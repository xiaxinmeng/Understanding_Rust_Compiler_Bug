rust
const fn each_ref<T, const N: usize>(arr: &[T; N]) -> [&T; N] {
    array![i => &arr[i]; N]
}
