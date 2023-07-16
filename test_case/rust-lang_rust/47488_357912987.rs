rust
fn main() {
    #[inline(tralala = "trulala")] // Not an error!
    let _x = 1 + 1;
}
