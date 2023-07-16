rust
#[inline(never)]
#[cold]
fn init() -> String {
    String::from("hello world. this string is long")
}

pub fn get_the_thread_local() -> String {
    thread_local! {
        static X: String = init();
    }
    X.with(|x| x.clone())
}
