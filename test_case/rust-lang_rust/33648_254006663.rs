
$ rustc --version
rustc 1.14.0-nightly (a3bc191b5 2016-10-10)
$ cat scratch.rs 
fn main() {
    let _ = &*"sth";
}
$ rustc scratch.rs 
$ echo $?
0
