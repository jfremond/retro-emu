mod cartridge;
mod registers;
mod display;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {

    let args = std::env::args().skip(1).collect::<Box<[_]>>();

    // let rom_path = if args.len() == 1 {
    //     args[0].to_owned()
    // } else {
    //     panic!("usage")
    // };
    
    // let debug_arg = if args.len() == 2 {
    //     println!("{}", args[1])
    // } else {
    // };
    println!("{}", args.len());
    // if arg len < 1 => error
    // if arg len == 1 => arg[0] = rom_path
    // if arg len == 2 => arg[0] = rom_path && arg[1] = "debug"
    // if arg len == 2 && arg[1] != "debug" => error




    drop(args);

    // let cartridge = cartridge::load_cartridge_path(rom_path).unwrap();

    // _ = cartridge;
    // // debut thread
    // let index: i32 = 10;
    // let mut count: i32 = 0;
    // let emulator_thread = thread::spawn(move || {
    //     // send value of count
    //     while count < index {
    //         count += 1;
    //     }

    // });
    // let main_thread = thread::spawn(move || {
    //     // get value of count
    // });
    // let res = emulator_thread.join();
    // display::create_window();




    let index: i32 = 10;
    let (tx, rx) = mpsc::channel::<i32>();

    let emulator_thread = thread::spawn(move || {
        let mut count: i32 = 0;
        while count < index {
            count += 1;
            if tx.send(count).is_err() {
                break;
            } // send value of count
            thread::sleep(Duration::from_millis(500));
        }
    });
    display::create_window(rx);

    emulator_thread.join().unwrap();
}
