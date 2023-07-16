 rust
struct A(Box<i32>);
struct B<'a>(&'a mut A);

impl Drop for A {
    fn drop(&mut self) {
        if *self.0 == 0 {
            panic!("oh no");
        } else {
            // ...
        }
    }
}

impl<'a> Drop for B<'a> {
    fn drop(&mut self) {
        let a = &self.0;
        println!("{}", a.0);
    }
}

fn foo(b: &mut B) {
    *b.0 = A(Box::new(1));
}

fn main() {
    let mut bomb = A(Box::new(0));
    let mut b = B(&mut bomb);
    foo(&mut b);
}
