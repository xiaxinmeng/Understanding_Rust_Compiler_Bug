rust
pub enum RenderJob {
    ... other renderer primitives ...
    DrawMany(usize, Vec<Rect>),
}
