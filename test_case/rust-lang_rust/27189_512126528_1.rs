rust
fn main() {
    println!(
        "Hello, world! {}",
        #[cfg(not(feature = "bad"))]
        "arg when not bad",
        #[cfg(feature = "bad")]
        "arg when bad",
    );
}
