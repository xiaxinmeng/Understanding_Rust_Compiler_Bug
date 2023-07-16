 rust
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(usize)]
pub enum E { A = 0, B = 1 }

impl E {
    pub fn foo(x: usize) -> Self {
        use self::E::{A, B};
        const E_A: usize = A as usize;
        const E_B: usize = B as usize;
        match x {
            E_A => A,
            E_B => B,
            _ => panic!("no match"),
        }
    }
}
