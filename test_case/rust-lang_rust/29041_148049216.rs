 rust
pub fn get_a_foo() -> Foo {
    lazy_static! {
        static ref FOO: Mutex<Foo> = Mutex::new(Foo { … });
    }
    FOO.lock().unwrap().clone()
}
