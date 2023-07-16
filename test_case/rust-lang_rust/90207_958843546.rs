rust
struct ArrayStorage<T, const N: usize = 2> {
    arr: [T; N],
}

impl<T> ArrayStorage<T> {
    fn new(a: T, b: T) -> ArrayStorage<T> {
        ArrayStorage {
            arr: [a, b],
        }
    }
}


struct Image<
    const WIDTH: usize,
    const HEIGHT: usize,
    // Without loosening the ordering restriction,
    // we could not add a default for this type parameter.
    FORMAT: ImageFormat = PngFormat, 
> {
    // ...
}

// This order is a lot clearer than
//
//    T, U, V, F, const N: usize, const M: usize
fn cartesian_product<
    T, const N: usize,
    U, const M: usize,
    V, F
>(a: [T; N], b: [U; M]) -> [[V; N]; M]
where
    F: FnMut(&T, &U) -> V
{
    // ...    
}
