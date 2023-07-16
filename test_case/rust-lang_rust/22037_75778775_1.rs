
impl A for u32 {
    type Output = u32;
    fn a(&self) -> u32 {
        0
    }
}

fn main() {
    let a: u32 = 0;
    let b: u32 = a.a();
}
