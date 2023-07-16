 rust
pub struct ProcessConfig<'a> {
    pub program: &'a [u8],
    pub args: &'a [Vec<u8>],
    pub env: Option<&'a [(Vec<u8>, Vec<u8>)]>,
    ...
}
