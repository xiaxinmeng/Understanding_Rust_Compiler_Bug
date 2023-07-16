 rust
#![feature(phase)]


mod foo {
    #[phase(plugin, link)] extern crate log;

    fn bar() {
        debug!("hi")
    }
}

fn main() {}
