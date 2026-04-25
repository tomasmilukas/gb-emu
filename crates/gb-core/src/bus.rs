use crate::cartridge::Cartridge;

pub struct Bus {
    cartridge: Cartridge,
    wram: [u8; 0x2000],
    hram: [u8; 0x7F],
    interrupt_enable: u8,
}

impl Bus {
    pub fn read8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read_rom(address),

            0xA000..=0xBFFF => self.cartridge.read_ram(address),

            0xC000..=0xDFFF => {
                let index = address as usize - 0xC000;
                self.wram[index]
            }

            0xFF80..=0xFFFE => {
                let index = address as usize - 0xFF80;
                self.hram[index]
            }

            0xFFFF => self.interrupt_enable,

            _ => 0xFF,
        }
    }

    pub fn write8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.cartridge.write_rom_control(address, value),

            0xA000..=0xBFFF => self.cartridge.write_ram(address, value),

            0xC000..=0xDFFF => {
                let index = address as usize - 0xC000;
                self.wram[index] = value;
            }

            0xFF80..=0xFFFE => {
                let index = address as usize - 0xFF80;
                self.hram[index] = value;
            }

            0xFFFF => {
                self.interrupt_enable = value;
            }

            _ => {}
        }
    }
}
