
// main.rs
mod foo;
mod bar;
use foo::hello;

fn main() {
    hello();
    bar::by();
}
