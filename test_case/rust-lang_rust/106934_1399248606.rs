rust
assert_eq!(addr_of!(base).addr() + offset_of!(base_ty, field), addr_of!(base.field));
