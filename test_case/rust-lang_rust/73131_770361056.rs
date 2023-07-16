
rustcargo@zealot:~$ cat t.rs
use std::process::Command;
fn main() {
    println!("Hello, world!");
    let mut cmd = Command::new("sh");
    cmd.args(&["-xec","
        echo hi
        sleep 5
        echo ho
    "]);
    let child = cmd.spawn().expect("spawn");
    dbg!(&child);
}
rustcargo@zealot:~$ rustc t.rs
rustcargo@zealot:~$ ./t
Hello, world!
[t.rs:11] &child = Child {
    stdin: None,
    stdout: None,
    stderr: None,
}
rustcargo@zealot:~$ + echo hi
hi
+ sleep 5
+ echo ho
ho
