rust
impl AsFormat for Position {
    // AsFormat and its impl on `[f32; 3]` are themselves pulled from a separate crate, `gfx-hal`
    const SELF: Format = <[f32; 3] as AsFormat>::SELF;
}
