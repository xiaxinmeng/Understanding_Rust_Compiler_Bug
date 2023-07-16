
struct MyStruct {}

impl MyStruct {

    fn takes_mut_self(&mut self, _x:int) {}

    fn takes_self(&self) -> int { 42 }

    fn also_takes_mut_self(&mut self) {
        self.takes_mut_self(self.takes_self());
    }

}

fn main() {}
