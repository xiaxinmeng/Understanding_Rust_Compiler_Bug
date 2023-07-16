rust
macro_rules! mac {
    ($h(hello)*) => {
        println!("hello {} times", ${count(h)});
    };
}
mac!(hello hello hello);
