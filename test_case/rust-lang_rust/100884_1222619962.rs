rust
#![windows_subsystem = "windows"]

const STD_INPUT_HANDLE: -10_i32 as u32;
const STD_OUTPUT_HANDLE: -11_i32 as u32;
const STD_ERROR_HANDLE: -12_i32 as u32;

#[link(name = "windows")]
extern "system" {
    fn AllocConsole() -> i32;
    fn FreeConsole() -> i32;
    fn SetStdHandle(nStdHandle: u32, hHandle: isize) -> i32;
}

fn main() -> std::io::Result<()> {
    unsafe {
        AllocConsole();
        // unset stdio handles before free.
        SetStdHandle(STD_INPUT_HANDLE, 0);
        SetStdHandle(STD_OUTPUT_HANDLE, 0);
        SetStdHandle(STD_ERROR_HANDLE, 0);
        FreeConsole();
    }

    std::process::Command::new("notepad.exe").spawn()?;
    Ok(())
}
