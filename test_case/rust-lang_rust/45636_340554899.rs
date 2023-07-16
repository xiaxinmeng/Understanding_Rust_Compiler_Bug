rust
#![feature(swap_with_slice)]
fn main() {
    let mut slice = [3, 4, 1, 2];
    {
        let (fst, snd) = slice.split_at_mut(2);
        fst.swap_with_slice(snd);
    }
    println!("{:?}", slice); // prints [1, 2, 3, 4]
}
