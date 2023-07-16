
fn main() {
    let ten = Character::Digit(10);
    println!("The character is {}", ten.Digit);
}

enum Character {
    Digit(i32),
    Other,
}
