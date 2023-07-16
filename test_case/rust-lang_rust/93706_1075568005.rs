rust
trait NotConstTrait {
    fn action(&self);
}

impl NotConstTrait for i32 {
    fn action(&self) {
        println!("{}", self);
    }
}

const X: &dyn NotConstTrait = &13;
