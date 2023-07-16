 rust
fn main() {
    #[allow(unused_parens)]
    let _ = (9);

    let _ = #[allow(unused_parens)] (9);

    // This now works:
    #[allow(unused_parens)]
    {
        let _ = (9);
    }
}
