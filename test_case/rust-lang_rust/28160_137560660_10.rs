 Rust
dirty |= SOMETHING_MODIFYING_DIRTY;
// equiv
let rhs = SOMETHING_MODIFYING_DIRTY;
dirty = dirty | rhs;
