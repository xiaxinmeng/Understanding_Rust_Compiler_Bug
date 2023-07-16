 rust
pub fn aaa() -> int {
    pub mod bbb {
        pub struct CCC {
            x: int
        }

        pub fn ddd() -> CCC {
            return CCC { x: 0 };
        }
    }

    return bbb::ddd().x;
}
