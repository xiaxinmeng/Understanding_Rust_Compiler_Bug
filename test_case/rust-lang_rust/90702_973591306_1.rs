rust
extern crate foo;
extern crate bar;

use bar::baz;
use boo::buzz;

fn main() {
    foo::fuzz();
    boo::buzz();
    bar::buzz();
    baz::buzz();
}
