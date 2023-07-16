rust
fn check<'a, 'b>(a: Option<&'a i32>, b: &'b i32) -> bool where 'b: 'a {
    let tmp: Option<&'b i32> = Some(b);
    let eq1 = a == tmp;
    let eq2 = tmp == a; //< mismatched types
    assert_eq!(eq1, eq2);
    eq1
}
