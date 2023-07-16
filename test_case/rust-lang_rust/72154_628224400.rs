rust
pub struct Wrap {
    pub t: [usize; 1]
}

impl Wrap {
    #[inline(always)]
    pub fn new(t: [usize; 1]) -> Self {
        Wrap { t }
    }
}

#[inline(always)]
pub fn assume_init() -> [usize; 1] {
    [1234]
}

fn main() {
    let x: [usize; 1] = assume_init();
    Wrap::new(x);
}
