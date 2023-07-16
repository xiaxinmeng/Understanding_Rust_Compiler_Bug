rust
fn foo(f: impl Fn(i32) + Copy) {
    f(10);
}

fn bar() -> impl Fn(i32) + Copy {
    |x: i32| {
        println!("{}", x);
    }
}

fn main() {
    foo(bar());
}
