Rust
use std::ops::Add;

struct Ishmael;
struct Maybe;
struct CallMe;

impl Add<Ishmael> for CallMe {
    type Output = ();
    fn add(self, _args: (Ishmael)) -> () {
        println!("Split your lungs with blood and thunder!");
    }
}

impl Add<Maybe> for CallMe {
    type Output = ();
    fn add(self, _args: (Maybe)) -> () {
        println!("So we just met, and this is crazy");
    }
}

impl Add<(Ishmael, Maybe)> for CallMe {
    type Output = ();
    fn add(self, _args: (Ishmael, Maybe)) -> () {
        println!("But I didn't know salty captains liked pop");
    }
}

fn main() {
    CallMe+(Ishmael);
    CallMe+(Maybe);
    CallMe+(Ishmael, Maybe);
}
