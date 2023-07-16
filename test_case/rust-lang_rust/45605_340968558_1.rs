rust
mod X {
    pub struct CrateNum(pub u32);
    impl CrateNum {
        pub fn new(val: u32) -> Self {
            CrateNum(val)
        }
    }
}

use X::CrateNum;

pub enum Enum {
    CrateNumVal(CrateNum)
}

fn main() {
    let val = Enum::CrateNumVal(CrateNum::new(4));
    match val {
        Enum::CrateNumVal(CrateNum) => {
            println!("hello world");
        }
    }
}
