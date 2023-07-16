 rust
#![warn(unused_qualifications)]
mod foo {
    pub fn bar() {}
}
use foo::bar;

fn main() {
    foo::bar(); // warning: unnecessary qualification
}
