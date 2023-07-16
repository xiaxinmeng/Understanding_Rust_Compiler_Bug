
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::panic::catch_unwind;

fn main() {
    let got = catch_unwind(||{
        let mut c = Command::new("echo");
        c.arg("hi");
        unsafe {
            c.pre_exec(|| panic!("crash now!"));
        }
        let st = c.status().expect("get status");
        println!("{:?}", st);
    });
    eprintln!("got={:?}", &got);
}
