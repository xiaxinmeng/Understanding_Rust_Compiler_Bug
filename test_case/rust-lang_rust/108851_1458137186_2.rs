rust
fn foo<const N: usize>() {
    struct Check<const N: usize>;
    impl<const N: usize> Check<N> {
        const CHECK: () = assert!(N % 2 == 0, "N must be even"); // or any other condition
    }
    let _ = Check::<N>::CHECK;
}

fn main() {
    foo::<2>(); // OK
    foo::<3>(); // ERROR
}
