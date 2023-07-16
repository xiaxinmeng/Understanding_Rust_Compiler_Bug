 rust
struct DroppableStruct;

impl Drop for DroppableStruct {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

trait MyTrait { }
impl MyTrait for ~DroppableStruct {}

struct Whatever { w: ~MyTrait }
impl  Whatever {
    fn new(w: ~MyTrait) -> Whatever {
        Whatever { w: w }
    }
}

fn main() {
    let f = ~DroppableStruct;
    let _a = Whatever::new(~f as ~MyTrait);
}
