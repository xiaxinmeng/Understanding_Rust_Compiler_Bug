
use std::num::Zero;

pub trait MyTrait {
    pub fn method(&mut self);
}

pub struct MyStruct<U> {
    pub attr: U,
}

impl<U: Num> MyStruct<U> {
   pub fn other_method(&mut self) {
        self.attr = self.attr+ Zero::zero();
   }
}

impl MyTrait for MyStruct<f64> {
   pub fn method(&mut self) {
        self.other_method();
    }
}

fn main() {
    let x = MyStruct {attr: 0.0f64};
    let _ = ~x as ~MyTrait;
}
