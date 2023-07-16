
$ rustc --version
rustc 1.14.0-nightly (a3bc191b5 2016-10-10)
$ cat scratch.rs 
#[repr(u8)]
enum Foo {
    Foo(u8),
}

fn main() {
    match Foo::Foo(1) {
        _ => ()
    }
}
$ rustc scratch.rs 
$ echo $?
0
