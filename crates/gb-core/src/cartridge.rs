pub struct Cartridge {
    rom: Vec<u8>,
    header: CartridgeHeader,
}

struct CartridgeHeader {
    title: String,
    cartridge_type: CartridgeType,
    rom_size_bytes: usize,
    rom_banks: usize,
    ram_size_bytes: usize,
    ram_banks: usize,
    header_checksum: u8,
    header_checksum_ok: bool,
}

enum CartridgeType {
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
        for byte in rom.iter().take(16) {
            print!("{:02X} ", byte);
        }
        println!();

        todo!("next: validate size and parse header")
    }
}
