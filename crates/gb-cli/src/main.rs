use gb_core::cartridge::Cartridge;

fn main() {
    let rom_path = std::env::args().nth(1).expect("usage: gb-cli <rom-path>");

    let mut cartridge = Cartridge::load(&rom_path).expect("failed to load cartridge");

    let header = cartridge.header();

    println!("Title: {}", header.title);
    println!(
        "ROM size: {} bytes, {} banks",
        header.rom_size_bytes, header.rom_banks
    );
    println!(
        "RAM size: {} bytes, {} banks",
        header.ram_size_bytes, header.ram_banks
    );
    println!("Cartridge type: {:?}", header.cartridge_type);
    println!(
        "Header checksum: {}",
        if header.header_checksum_ok {
            "OK"
        } else {
            "BAD"
        }
    );

    println!("Read 0x0100: 0x{:02X}", cartridge.read_rom(0x0100));
    println!("Read 0x0147: 0x{:02X}", cartridge.read_rom(0x0147));
    println!("Read 0x4000: 0x{:02X}", cartridge.read_rom(0x4000));

    println!("Bank 1 read 0x4000: 0x{:02X}", cartridge.read_rom(0x4000));

    cartridge.write_rom_control(0x2000, 2);
    println!("Bank 2 read 0x4000: 0x{:02X}", cartridge.read_rom(0x4000));

    cartridge.write_rom_control(0x2000, 3);
    println!("Bank 3 read 0x4000: 0x{:02X}", cartridge.read_rom(0x4000));

    cartridge.write_rom_control(0x0000, 0x0A); // enable RAM

    cartridge.write_rom_control(0x4000, 0);
    cartridge.write_ram(0xA000, 0x11);

    cartridge.write_rom_control(0x4000, 1);
    cartridge.write_ram(0xA000, 0x22);

    cartridge.write_rom_control(0x4000, 0);
    println!("RAM bank 0: 0x{:02X}", cartridge.read_ram(0xA000));

    cartridge.write_rom_control(0x4000, 1);
    println!("RAM bank 1: 0x{:02X}", cartridge.read_ram(0xA000));
}
