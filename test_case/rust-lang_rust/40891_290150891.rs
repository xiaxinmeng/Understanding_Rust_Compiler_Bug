rust
fn has_a_side_effect_and_returns_something() -> i32 {
    println!("I am a super useful and awesome side effect");
    42
}

fn do_a_thing() {
    has_a_side_effect_and_returns_something();
    has_a_side_effect_and_returns_something()
}

fn main() {
    do_a_thing();
}
