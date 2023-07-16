Rust
struct Struct {
    field: f32,
}

let base_ptr = core::mem::MaybeUninit::<Struct>::uninit().as_ptr();
let field_ptr = &raw const base_ptr.field; // Is this allowed by this RFC?
