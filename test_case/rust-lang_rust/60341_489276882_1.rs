rust
pub fn get_the_thread_local() -> usize {
    thread_local! {
        static X: String = String::from("hello world. this string is long");
    }
    X.with(|x| x.len())
}
