
[~/code/rust2] $ ./x86_64-apple-darwin/stage2/bin/rustc bar.rs                                          
warning: no debug symbols in executable (-arch x86_64)
[~/code/rust2] $ cat bar.rs                                                                             
struct Foo;
fn main() {
    let xs = ~[Foo, Foo, Foo];
    assert_eq!(fmt!("%?", xs.slice(0, 2).to_owned()), ~"~[{}, {}]");
[~/code/rust2] $ ./bar
[~/code/rust2] $
