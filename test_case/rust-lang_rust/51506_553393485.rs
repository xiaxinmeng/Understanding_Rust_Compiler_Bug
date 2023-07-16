rust
fn f<T: Trait>(a: T) {
    if let Some(iter) = a.f() {
        println!("Some");
        for x in iter {
            println!("x = {}", x);
        }
    }
}

pub fn main() {
    f(10);
}
