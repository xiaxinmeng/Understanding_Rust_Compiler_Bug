rust
struct SimpleStrategy;
struct ComplicatedStrategy;

trait Strategy {
    fn do_thing<T: MyTrait>(tr: &T);
}

impl Strategy for SimpleStrategy {
    fn do_thing<T: MyTrait>(tr: &T) { println!("simple") }
}
impl Strategy for ComplicatedStrategy {
    fn do_thing<T: MyTrait>(tr: &T) { println!("complicated") }
}

trait MyTrait {
    type Strategy: Strategy;
    fn do_thing(&self) where Self: Sized {
        Self::Strategy::do_thing(self);
    }
}

impl MyTrait for u32 {
    type Strategy = SimpleStrategy;
}

fn main() {
    1u32.do_thing();
}
