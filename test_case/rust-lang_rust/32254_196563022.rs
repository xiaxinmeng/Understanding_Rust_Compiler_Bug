 rust
use std::env;
use std::io;
use std::process::Command;

fn main() {
    if env::args().len() == 1 {
        let s = Command::new(env::current_exe().unwrap())
                .arg("foo")
                .status()
                .unwrap();
        assert!(s.success());
    } else {
        let mut s = String::new();
        println!("{:?}", io::stdin().read_line(&mut s));
        println!("{}", s);
    }
}
