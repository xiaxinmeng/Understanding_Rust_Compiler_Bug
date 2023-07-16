rust
fn check<'a, 'b>(a: Option<&'a i32>, b: &'b i32) -> bool {
    let tmp: Option<&'b i32> = Some(b);
    a == tmp //< mismatched types
}
