mod cartridge;
mod registers;
mod display;
use std::thread;

fn main() {
    let args = std::env::args().skip(1).collect::<Box<[_]>>();

    let rom_path = if args.len() == 1 {
        args[0].to_owned()
    } else {
        panic!("usage")
    };

    drop(args);

    let cartridge = cartridge::load_cartridge_path(rom_path).unwrap();

    _ = cartridge;
    // debut thread
    let emulator_thread = thread::spawn(move || {
        // recupere des trucs
    });
    let res = emulator_thread.join();
    display::create_window();
}
