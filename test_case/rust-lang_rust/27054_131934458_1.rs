 rust
#[derive(Debug, Copy, Clone)]
struct CoolInt(i32);
impl std::ops::Add<CoolInt> for CoolInt {
    type Output = CoolInt;
    fn add(self, other: CoolInt) -> CoolInt { CoolInt(self.0 + other.0) }
}

fn main() {
    let x = Box::new(CoolInt(0));
    let mut y = CoolInt(0);
    println!("{:?}", *x + *{ drop(x); let _ = Box::new(main); &mut y });
}
