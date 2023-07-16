rust=
#[warn(unused)]
#[deny(warnings)]
fn main() {
    #[warn(unused_variables)]
    let a = 5; // Still gets an error
}
