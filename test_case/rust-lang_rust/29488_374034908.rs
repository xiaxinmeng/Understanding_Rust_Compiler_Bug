
struct Foo {}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("drop begin");
    }
}


thread_local! {
    static FOO: Foo = Foo{};
}


fn main() {
    println!("main begin");
    FOO.with(|_| {});
}

