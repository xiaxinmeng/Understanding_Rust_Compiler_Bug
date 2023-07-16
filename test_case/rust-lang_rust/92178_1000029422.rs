
struct Dummy;

impl std::ops::Add<Dummy> for String {
    type Output = ();
    fn add(self, rhs: Dummy) -> Self::Output {}
}

fn main() {
    let teststring = "hi".to_string();
    println!("{}", String::from("/") + &teststring);
}
