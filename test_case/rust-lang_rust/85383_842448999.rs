rust
static NUM: i32 = 1;
struct A;
struct B<'a>(&'a i32);
impl A {
    fn get(&self) -> B<'_> {
        // Contains NO ref to self whatsoever!
        B(&NUM)
    }
}
fn a() -> A { A }
fn main() {
    let b = a().get();
    println!("{}", b.0.to_string());
}
