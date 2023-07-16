Rust
#![feature(bindings_after_at)]
#![feature(const_generics)]

trait Splitter {
    type Item;
    fn split_array<const N: usize, const M: usize>(&self) -> (&[Self::Item; N], &[Self::Item; M]);
}

impl<T, const Q: usize> Splitter for [T; Q] {
    type Item = T;

    fn split_array<const N: usize, const M: usize>(&self) -> (&[Self::Item; N], &[Self::Item; M]) {
        // Check that the sum of the lengths of the splitted arrays equal to the original.
        assert!(N + M == Q);

        let (lhs, rhs) = self.split_at(N);
        
        // See the assert above for sanity sake
        unsafe {
            let lhs_array = &(*lhs.as_ptr().cast());
            let rhs_array = &(*rhs.as_ptr().cast());

            (lhs_array, rhs_array)
        }
    }
}

fn main() {
    let array = [1, 2, 3, 4, 5];

    match array.split_array() {
        (a1 @ [a, b, c], a2 @ [d, e]) => {
            dbg!(a1);
            dbg!(a2);
        }
    }
}
