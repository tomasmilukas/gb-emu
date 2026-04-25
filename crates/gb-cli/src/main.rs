use gb_core::cartridge::Cartridge;

fn main() {
    let rom_path = std::env::args().nth(1).expect("usage: gb-cli <rom-path>");

    let cartridge = Cartridge::load(&rom_path).expect("failed to load cartridge");

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
}
