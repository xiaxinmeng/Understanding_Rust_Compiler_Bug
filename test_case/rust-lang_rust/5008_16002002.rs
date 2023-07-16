
struct Foo {
    x: int
}
struct Bar {
    x: float
}
trait Tra {
    fn get_value(&self) -> int;
}
impl Tra for Foo {
    fn get_value(&self) -> int {
        self.x
    }
}
impl Tra for Bar {
    fn get_value(&self) -> int {
        self.x as int
    }
}

fn test(a: &~Tra) -> int {
    a.get_value()
}

fn main() {
    let list = ~[~Foo{x: 3} as ~Tra, ~Bar{x: 5.3} as ~Tra];
    let mapped = list.map(test);
    io::println(fmt!("%?", mapped));
}
