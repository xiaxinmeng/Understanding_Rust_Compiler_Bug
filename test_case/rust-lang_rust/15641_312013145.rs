
use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if &args[1] == "start" {
                let child = Command::new(&args[0])
                                    .spawn().expect("Child process failed to start.");
                println!("child pid: {}", child.id());
                // child.forget() No Child Left Behind
        }
    } else {
        println!("This is an incredibly simple daemon!");
    }
}
