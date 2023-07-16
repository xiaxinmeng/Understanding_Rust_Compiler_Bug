rust
pub enum RenderJob<'queue> {
    ... other renderer primitives ...
    DrawMany(usize, &'queue [Rect]),
}
