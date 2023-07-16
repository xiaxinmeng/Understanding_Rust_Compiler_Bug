 rust
fn get_random_number() -> usize {
    4 /* guaranteed by xkcd 221 */
}
pub fn foo<T>(value: T) -> (T, usize) {
    (value, get_random_number())
}
