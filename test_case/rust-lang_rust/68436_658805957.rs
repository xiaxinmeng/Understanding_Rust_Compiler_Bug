rust
type ArrayMul<T, const N: usize, const M: usize> = [T; N*M];
fn foo<const N: usize>()
    where ArrayMul<u8, 2, N>:, ArrayMul<u8, 4, N>:,
{
    let a: ArrayMul<u8, 2, N> = ..;
    let b: ArrayMul<u8, 4, N> = ..;
}
