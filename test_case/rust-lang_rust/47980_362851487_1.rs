rust
use supersimd::{U16Constant, barbaz};

struct Leet;
impl U16Constant for Leet { const X: u8 = 1337; }

barbaz::<Leet>(4);
