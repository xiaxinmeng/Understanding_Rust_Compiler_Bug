rust
extern crate foo;

#[foo::foo]
#[allow(dead_code)]
fn main() {
    let warn_about_me = 3;

    struct A { a: i32 }
}
