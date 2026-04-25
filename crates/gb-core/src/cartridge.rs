pub struct Cartridge {
    rom: Vec<u8>,
    header: CartridgeHeader,
    selected_rom_bank: usize,
    ram: Vec<u8>,
    selected_ram_bank: usize,
    ram_enabled: bool,
}

pub struct CartridgeHeader {
    pub title: String,
    pub cartridge_type: CartridgeType,
    pub rom_size_bytes: usize,
    pub rom_banks: usize,
    pub ram_size_bytes: usize,
    pub ram_banks: usize,
    pub header_checksum: u8,
    pub header_checksum_ok: bool,
}

#[derive(Debug)]
pub enum CartridgeType {
    RomOnly,
    Mbc1,
    Mbc1Ram,
    Mbc1RamBattery,
    Mbc3,
    Mbc3Ram,
    Mbc3RamBattery,
    Mbc3TimerBattery,
    Mbc3TimerRamBattery,
}

impl Cartridge {
    pub fn load(path: &str) -> Result<Cartridge, String> {
        println!("Trying to load ROM from path: {}", path);

        let rom = std::fs::read(path).map_err(|err| format!("Failed to read ROM file: {}", err))?;

        println!("Loaded {} bytes", rom.len());

        println!("First 16 bytes:");
        for byte in rom.iter().take(32) {
            print!("{:02X} ", byte);
        }
        println!();
        println!("rom[0x0147] = 0x{:02X}", rom[0x0147]);
        println!("rom[327]    = 0x{:02X}", rom[327]);

        println!("0x0147 as decimal = {}", 0x0147);

        println!("Title bytes:");
        for address in 0x0134..=0x0143 {
            let byte = rom[address];
            println!("0x{:04X}: 0x{:02X} '{}'", address, byte, byte as char);
        }

        println!("Cartridge type byte: 0x{:02X}", rom[0x0147]);
        println!("ROM size byte:       0x{:02X}", rom[0x0148]);
        println!("RAM size byte:       0x{:02X}", rom[0x0149]);
        println!("Destination byte:    0x{:02X}", rom[0x014A]);
        println!("Version byte:        0x{:02X}", rom[0x014C]);
        println!("Header checksum:     0x{:02X}", rom[0x014D]);

        if rom.len() < 0x150 {
            return Err("ROM is too small to contain a valid Game Boy header".to_string());
        }

        let header = Self::parse_header(&rom)?;
        let ram = vec![0; header.ram_size_bytes];

        Ok(Cartridge {
            rom,
            ram,
            header,
            selected_rom_bank: 1,
            selected_ram_bank: 0,
            ram_enabled: false,
        })
    }

    pub fn header(&self) -> &CartridgeHeader {
        &self.header
    }

    fn parse_header(rom: &[u8]) -> Result<CartridgeHeader, String> {
        let title = Self::parse_title(rom);
        let cartridge_type = Self::decode_cartridge_type(rom[0x0147])?;
        let (rom_size_bytes, rom_banks) = Self::decode_rom_size(rom[0x0148])?;
        let (ram_size_bytes, ram_banks) = Self::decode_ram_size(rom[0x0149])?;

        let expected_checksum = rom[0x014D];
        let computed_checksum = Self::compute_header_checksum(rom);
        let header_checksum_ok = expected_checksum == computed_checksum;

        Ok(CartridgeHeader {
            title,
            cartridge_type,
            rom_size_bytes,
            rom_banks,
            ram_size_bytes,
            ram_banks,
            header_checksum: expected_checksum,
            header_checksum_ok,
        })
    }

    fn parse_title(rom: &[u8]) -> String {
        let mut title = String::new();

        for address in 0x0134..=0x0143 {
            let byte = rom[address];

            if byte == 0x00 {
                break;
            }

            title.push(byte as char);
        }

        title
    }

    fn decode_rom_size(value: u8) -> Result<(usize, usize), String> {
        match value {
            0x00..=0x08 => {
                let rom_banks = 2usize << value;
                let rom_size_bytes = 32 * 1024 * (1usize << value);

                Ok((rom_size_bytes, rom_banks))
            }

            _ => Err(format!("Unsupported ROM size byte: 0x{:02X}", value)),
        }
    }

    fn decode_ram_size(value: u8) -> Result<(usize, usize), String> {
        match value {
            0x00 => Ok((0, 0)),
            0x02 => Ok((8 * 1024, 1)),
            0x03 => Ok((32 * 1024, 4)),
            0x04 => Ok((128 * 1024, 16)),
            0x05 => Ok((64 * 1024, 8)),

            _ => Err(format!("Unsupported RAM size byte: 0x{:02X}", value)),
        }
    }

    fn decode_cartridge_type(value: u8) -> Result<CartridgeType, String> {
        match value {
            0x00 => Ok(CartridgeType::RomOnly),
            0x01 => Ok(CartridgeType::Mbc1),
            0x02 => Ok(CartridgeType::Mbc1Ram),
            0x03 => Ok(CartridgeType::Mbc1RamBattery),

            0x0F => Ok(CartridgeType::Mbc3TimerBattery),
            0x10 => Ok(CartridgeType::Mbc3TimerRamBattery),
            0x11 => Ok(CartridgeType::Mbc3),
            0x12 => Ok(CartridgeType::Mbc3Ram),
            0x13 => Ok(CartridgeType::Mbc3RamBattery),

            _ => Err(format!("Unsupported cartridge type: 0x{:02X}", value)),
        }
    }

    fn compute_header_checksum(rom: &[u8]) -> u8 {
        let mut checksum: u8 = 0;

        for address in 0x0134..=0x014C {
            checksum = checksum.wrapping_sub(rom[address]).wrapping_sub(1);
        }

        checksum
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom[address as usize],

            0x4000..=0x7FFF => {
                let offset = address as usize - 0x4000;
                let index = self.selected_rom_bank * 0x4000 + offset;

                self.rom[index]
            }

            _ => panic!("Cartridge ROM read out of range: 0x{:04X}", address),
        }
    }

    pub fn write_rom_control(&mut self, address: u16, value: u8) {
        match address {
            0x2000..=0x3FFF => {
                let mut bank = (value & 0x7F) as usize;

                if bank == 0 {
                    bank = 1;
                }

                self.selected_rom_bank = bank % self.header.rom_banks;
            }
            0x0000..=0x1FFF => {
                self.ram_enabled = (value & 0x0F) == 0x0A;
            }
            0x4000..=0x5FFF => {
                self.selected_ram_bank = (value as usize) & 0x03;
            }

            _ => {
                // ignore for now
            }
        }
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled || self.ram.is_empty() {
            return 0xFF;
        }

        let offset = address as usize - 0xA000;
        let index = self.selected_ram_bank * 0x2000 + offset;

        self.ram[index]
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled || self.ram.is_empty() {
            return;
        }

        let offset = address as usize - 0xA000;
        let index = self.selected_ram_bank * 0x2000 + offset;

        self.ram[index] = value;
    }
}
