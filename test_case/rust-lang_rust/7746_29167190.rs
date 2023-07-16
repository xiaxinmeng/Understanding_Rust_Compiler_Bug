 rust
yield fn slice_iter<'a, T>(slice: &'a [T]) -> struct SliceIter<'a, T> for &'a T {
    let mut start = 0;
    let mut end = slice.len();
    while start < end {
        yield in {
            [>..] => { let x = &slice[start]; start += 1; x }
            [..<] => { let x = &slice[end-1];   end -= 1; x }
        }
    }
}
