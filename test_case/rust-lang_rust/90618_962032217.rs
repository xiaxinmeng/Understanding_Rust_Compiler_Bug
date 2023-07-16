rust
fn main() {
    let inc1 = include_str!("inc1");
    let inc2 = include_str!("inc2");

    assert_eq!(inc1, "inc1\nab\n");
    assert_eq!(inc2, "inc2\nx\n");
}
