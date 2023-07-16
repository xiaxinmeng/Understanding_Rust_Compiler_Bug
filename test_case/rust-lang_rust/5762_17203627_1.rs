
trait MyTrait {
    fn get_by_ref(&self) -> uint;
}

struct MyStruct {
    f: uint
}

impl MyTrait for MyStruct {
    fn get_by_ref(&self) -> uint {
        self.f
    }
}

fn main() {
    let s: @MyTrait = (@MyStruct {f: 22}) as @MyTrait;
    assert!(s.get_by_ref() == 22);
}
