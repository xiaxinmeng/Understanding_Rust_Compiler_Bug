rust
struct Foo<T>(T);

unsafe impl Send for Foo<&'static str> {}

fn main() {
    let sendable_future: &dyn Send = &async {
        let s = Foo::<&'static str>("");
        async{}.await;
    };
}
