rust
struct Foo(i32);

fn main() {
    let x;
    {
        Foo(x) = Foo(1);
    }
    println!("x");
}
