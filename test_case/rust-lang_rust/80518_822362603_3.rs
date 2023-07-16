rust
impl<'a> Xorcism<'a> {
    pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl Iterator<Item=u8> + 'b + CaptureLifetime<'a>;
}
