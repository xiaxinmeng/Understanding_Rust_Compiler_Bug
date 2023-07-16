
use std::any::Any;
fn main() {
    println!("{}", (&5 as &Any).is::<uint>())
}
