rust
#![windows_subsystem = "windows"]

fn main() {
    unsafe {
        let cmdline = windows::Win32::System::Environment::GetCommandLineW(); // returns wrong address with #![windows_subsystem = "windows"]
        // If you go to the parent stack on the debugger and call above "main" this works either way

        dbg!(cmdline); 
    }
}
