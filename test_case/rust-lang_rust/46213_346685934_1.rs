rust
// use the RoundedRect.1.mode.tag niche
pub enum LocalClip {
    Rect(LayoutRect),
    RoundedRect(LayoutRect, ComplexClipRegion),
}

#[repr(C)]
pub struct ComplexClipRegion {
    pub rect: LayoutRect,
    pub radii: BorderRadius,
    pub mode: ClipMode,
}

#[repr(C)]
pub struct BorderRadius {
    pub top_left: LayoutSize,
    pub top_right: LayoutSize,
    pub bottom_left: LayoutSize,
    pub bottom_right: LayoutSize,
}

#[repr(C)]
pub enum ClipMode {
    Clip,
    ClipOut,
}

#[repr(C)]
struct LayoutRect(f32, f32, f32, f32);

#[repr(C)]
struct LayoutSize(f32, f32);
