rust
macro_rules! create_string {
    () => {
        String::new()
    };
    ($($x:expr),*) => {
        [$($x),*].iter().collect::<String>()
    };
}
