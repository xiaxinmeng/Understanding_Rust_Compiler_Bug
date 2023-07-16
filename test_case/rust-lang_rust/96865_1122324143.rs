rust
struct Foo<'a>(&'a str);
unsafe impl Send for Foo<'static> {}

fn assert_send_future() -> impl Send {
    async {
        let data: Foo<'static> = Foo("");
        std::future::ready(()).await;
    }
}
