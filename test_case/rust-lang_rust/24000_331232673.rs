rust
type Function = Fn(i32) -> i32;

#[derive(Clone)]
struct FunctionContainer {
    function: Box<Function>
}

fn main() {
}
