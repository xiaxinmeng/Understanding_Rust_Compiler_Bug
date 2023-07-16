rust
struct F(u32);

impl Drop for F {
    fn drop(&mut self) {
        eprintln!("drop");
        if self.0 == 0 { panic!() }
    }
}

fn main() {
    let _v = vec!(F(1), F(0), F(1), F(1));
}
