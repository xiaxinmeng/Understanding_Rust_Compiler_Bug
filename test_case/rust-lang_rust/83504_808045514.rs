rust
struct MyBool(bool);

fn f() -> MyBool {
    MyBool(true)
}

fn main() {
    if let MyBool(b) = f() && true {
        if b {}
    }
}
