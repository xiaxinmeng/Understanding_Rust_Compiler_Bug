 rust
use std::os;
use std::io::process;
use std::io::process::InheritFd;

fn main () {
    println!("args: {:?}", os::args());

    let args = os::args();
    if args.len() > 1 && args[1] == ~"child" {
        //for _ in range(0, 1000) {
        //    error!("hello?");
        //}
        //for _ in range(0, 1000) {
        //    println!("hello?");
        //}
    }

    let io = [InheritFd(0), InheritFd(1), InheritFd(2)];
    let config = process::ProcessConfig {
        program : args[0].as_slice(),
        args : [~"child"],
        env : None,
        cwd : None,
        io : io
    };

    let mut p = process::Process::new(config).unwrap();
    println!("{}", p.wait());
}
