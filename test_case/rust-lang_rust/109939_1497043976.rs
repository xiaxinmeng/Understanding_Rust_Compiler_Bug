rust
struct S(u8);

impl Deref for S {
    type Target = u8;
    
    fn deref(&self) -> &u8 {
        &self.0
    }
}

impl DerefMut for S {
    fn deref_mut(&mut self) -> &mut u8 {
        &mut self.0
    }
}

fn main() {
    let x = S(0);
    dbg!(x.0); // 0
    *x = 1_u8;
    dbg!(x.0); // 1
}
