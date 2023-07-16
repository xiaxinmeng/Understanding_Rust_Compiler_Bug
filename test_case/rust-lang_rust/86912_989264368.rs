rust
pub fn map<F, U>(self, mut f: F) -> [U; N]
where
   F: FnMut(T) -> U,
{
    let mut result: [U; N] = unsafe { std::mem::zeroed() };
    for (i, x) in self.into_iter().enumerate() {
        result[i] = f(x);
    }
    result
}
