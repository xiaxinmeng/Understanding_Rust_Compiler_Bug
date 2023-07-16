rust
// adapted from https://github.com/rust-lang/rust/blob/master/src/test/ui/consts/const-eval/infinite_loop.rs
pub const fn test() -> u64 {
    let mut n = 113383; // #20 in https://oeis.org/A006884
    while n != 1 {
        n = if n % 2 == 0 { n/2 } else { 3*n + 1 };
    }
    n
}

pub const fn test_1() -> u64 {
    test()
}
