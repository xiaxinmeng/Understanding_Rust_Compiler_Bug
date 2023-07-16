 Rust
struct Test;

struct Test2 {
    b: Option<Test>,
}

impl Drop for Test {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

#[cfg(leaks)]
impl Drop for Test2 {
    fn drop(&mut self) {}
}

fn stuff() {
    let mut t = Test2 { b: None };
    let u = Test;
    drop(t);
    t.b = Some(u);
}

fn main() {
    stuff();
}
