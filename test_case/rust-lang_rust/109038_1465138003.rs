rust
#![windows_subsystem = "windows"]
use std::io::{self, Read};

fn console_main() {
    println!("Hello world!");
}

fn main() {
    // Quick hack to see if a console is attached to the process.
    let has_console = unsafe { GetConsoleOutputCP() != 0 };
    if !has_console {
        // Try to allocate a console.
        unsafe { AllocConsole() };
    }

    console_main();

    if console_closes_on_exit() {
        println!("Press enter to exit");
        io::stdin().bytes().next();
    }
}

fn console_closes_on_exit() -> bool {
    // A console closes once the last process exits.
    // So check if we're the only process connected to this console.
    let mut list = [0_u32];
    let count = unsafe { GetConsoleProcessList(list.as_mut_ptr(), 1) };
    count == 1
}

// Ideally a crate should be used for these definitions.
#[link(name = "kernel32")]
extern "system" {
    fn GetConsoleOutputCP() -> u32;
    fn AllocConsole() -> i32;
    fn GetConsoleProcessList(list: *mut u32, count: u32) -> u32;
}
