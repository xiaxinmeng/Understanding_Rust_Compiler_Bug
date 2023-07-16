
fn main() {}
struct MyStruct;

trait Test {
}

impl Test for MyStruct {
    const TEST: fn() -> _ = 42; 
}
