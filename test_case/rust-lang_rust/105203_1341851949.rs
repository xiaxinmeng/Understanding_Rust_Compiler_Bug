rust
use std::os::windows::io::FromRawHandle;
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};

fn default_stdio() -> Stdio {
    unsafe { Stdio::from_raw_handle(std::ptr::null_mut()) }
}

    let mut res = Command::new(args.path)
        // create a new console for child process
        .creation_flags(0x00000010)
        .current_dir(dirp)
        // use the default handles
        .stdin(default_stdio())
        .stdout(default_stdio())
        .stderr(default_stdio())
        .spawn()
        .unwrap();

