
strcat@thinktank i ~/projects/rust doc % rustc -v
rustc 0.8-pre (f92b75a 2013-07-11 13:28:38 -0700)
host: x86_64-unknown-linux-gnu
strcat@thinktank i ~/projects/rust doc % rustc foo.rs
strcat@thinktank i ~/projects/rust doc % ./foo 
~[]
strcat@thinktank i ~/projects/rust doc % cat foo.rs 
fn main() {
    struct Foo;
    let xs = ~[Foo, Foo, Foo];
    println(fmt!("%?", xs.slice(0, 2).to_owned())) // prints `~[]`
}
