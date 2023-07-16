
#[derive(Debug)]
pub enum Unprintable {
    X
}
#[test]
fn test_x() {
    println!("Hello {0:5}!", format!("{:?}", Unprintable::X));
}
