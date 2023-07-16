 rust
struct Attribute<'a, S, T> {
    priv context: &'a Context,
    priv contents: gl::GLuint
}
impl<'a, S, T> Attribute<'a, S, T> {
    unsafe fn set_contents(&mut self, x: gl::GLuint) { self.contents = x }
    // etc.
}
