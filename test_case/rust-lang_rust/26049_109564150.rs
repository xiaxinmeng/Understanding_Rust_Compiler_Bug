 rust
#![crate_type="lib"]

trait Foo {
    extern fn read_word(&mut self) -> u8;
}

fn do_foo(f: &mut Box<Foo>) -> u8 {
    f.read_word()
}
