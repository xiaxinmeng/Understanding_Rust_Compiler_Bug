
$ cat foo.rs
#[no_mangle]
pub extern fn foo() {}
fn main() {}
$ rustc foo.rs
$ nm -g foo | foo
# should see the symbol `foo` defined here
