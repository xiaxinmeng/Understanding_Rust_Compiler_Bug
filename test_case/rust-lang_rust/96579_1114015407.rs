rs
fn main() {
    struct L;
    impl Drop for L {
        fn drop(&mut self) {
            println!("dropped L")
        }
    }
    struct M;
    impl Drop for M {
        fn drop(&mut self) {
            println!("dropped M")
        }
    }

    let mut x1 = L;
    x1 = (&M, L).1;

    println!("---"); // -------------

    let mut x2 = L;
    [x2] = [(&M, L).1];

    println!("---\ndropping local variables next…"); // -------------
}
