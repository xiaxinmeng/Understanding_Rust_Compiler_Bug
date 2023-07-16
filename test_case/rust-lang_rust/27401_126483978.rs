 rust
struct A<'a>(&'a mut i32);

impl<'a> Drop for A<'a> {
    fn drop(&mut self) {
        *self.0 += 1;
    }
}

fn main() {
    let mut cnt = 0;
    for i in 0..2 {
        let a; a = A(&mut cnt); // This was `let a = A(&mut cnt);` before
        if i == 1 {
            break
        }
        drop(a);
    }
    assert_eq!(cnt, 2);
}
