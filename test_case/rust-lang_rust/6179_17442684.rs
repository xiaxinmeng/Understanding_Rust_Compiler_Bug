
$ cat trait-recursive-iloop.rs 
trait Foo {
    fn x(&self);
}

impl Foo for int {
    fn x(&self) {}
}

impl<F:Foo> Foo for @F {
    fn x(&self) {
        (*self).x()
    }
}

fn main() {
    (@1i).x()
}
$ rustc --opt-level 0 trait-recursive-iloop.rs 
$ ./trait-recursive-iloop 
rust: task 794060 ran out of stack
Aborted (core dumped)
$ rustc --opt-level 1 trait-recursive-iloop.rs 
$ ./trait-recursive-iloop 
$ rustc --version
rustc 0.6 (1f65e4a 2013-05-04 00:48:37 -0700)
host: x86_64-unknown-linux-gnu
