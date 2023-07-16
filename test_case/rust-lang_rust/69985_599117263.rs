rust
enum ArrayFillError<T, I: Iterator<Item=T>, const N: usize> {

    TooFew([MaybeUninit<T>; N], usize),

    TooMany([T; N], I)
}
