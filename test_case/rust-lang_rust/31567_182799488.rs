 rust
struct DropSet<'a>(&'a mut u8);

impl<'a> DropSet<'a> {
    fn get(self) -> &'a u8 {
        self.0
    }
}

impl<'a> Drop for DropSet<'a> {
    fn drop(&mut self) {
        *self.0 = 0;
    }
}

fn main() {
    let mut x = 64;
    let s = DropSet(&mut x);
    let r = s.get();
    println!("{}", r);
}
