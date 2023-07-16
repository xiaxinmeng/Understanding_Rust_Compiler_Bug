 rust
trait Even {
        fn say(&self);
}

impl <'a, E: Even> Even for &'a E { 
        fn say(&self) { self.say() }
}

trait Odd {
        fn say(&self);
}

impl <'a, O: Odd> Odd for &'a O { 
        fn say(&self) { self.say() }
}

fn main() { }
