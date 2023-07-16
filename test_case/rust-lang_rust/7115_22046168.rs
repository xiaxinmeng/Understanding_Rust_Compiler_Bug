 rust
fn main() {
    test_nd_dr((8,  3), 2);
}

fn test_nd_dr((n, d): (i8,i8), r: i8) {
    let separate_div_rem = (n / d, n % d);
    let combined_div_rem = n.div_rem(&d);

    let (_, b) = separate_div_rem;
    assert_eq!(b, r);
}
