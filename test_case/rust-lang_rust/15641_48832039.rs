 rust
use std::io::Command;
use std::os;

fn main() {
    let args = os::args();
    if args.len() == 1 {
        let child = Command::new(args.get(0).as_slice())
                            .arg("child")
                            .detached().spawn().unwrap();
        println!("child: {}", child.id());
        child.forget();
    } else {
        println!("I'm a daemon!");
    }
}
