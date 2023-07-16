 rust
struct Foo { a: &'static int }
impl Drop for Foo { fn drop(&mut self) { } }
impl Copy for Foo { }

fn main() {
    static I: int = 42;
    let x = Foo { a: &I };
    let y = x;
    println!("{}\n{}", y.a, x.a);
}
