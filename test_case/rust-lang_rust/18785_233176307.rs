 Rust
fn f(x: i32) -> String {
    if 0 < x {
        f(x - 1)
    } else {
        main_loop() // fn main_loop() -> !
    }
}
