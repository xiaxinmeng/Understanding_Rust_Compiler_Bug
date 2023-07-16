rust
fn main() {
    fn init<const N: usize, const M: usize>(foos: &mut [bool; M]) {}
    const N: usize = 5;
    let mut foos = [false; 10];
    init::<N, _>(&mut foos);
}
