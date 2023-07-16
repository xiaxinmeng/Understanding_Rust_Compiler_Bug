 rust
fn f<T>() -> T { panic!() }
fn main() {
    let a = f();
    let b = -a;
    let c : i32 = a;
}
