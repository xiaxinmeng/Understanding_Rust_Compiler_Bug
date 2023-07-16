
alexgaynor@penguin /tmp> cat t.rs 
use std::process::Command;
use std::os::unix::process::CommandExt;

fn main() {
    eprintln!("About to exec");
    Command::new("doesnt-exist").env("VARIABLE", "X").exec();
    eprintln!("Didn't exist!");
    eprintln!("VARIABLE: {:?}", std::env::var("VARAIABLE"));
}
alexgaynor@penguin /tmp> rustc +nightly -Z sanitizer=address t.rs 
alexgaynor@penguin /tmp> env ASAN_OPTIONS=detect_odr_violation=0 ./t 
About to exec
Didn't exist!
fish: “env ASAN_OPTIONS=detect_odr_vio…” terminated by signal SIGSEGV (Address boundary error)
