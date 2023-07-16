rust
const USIZE: TypeId = TypeId::of::<usize>();

match TypeId::of::<Self>() {
    // Use a closure for `usize` that transmutes the generic `Self` to
    // a concrete `usize` and dispatches to `Self::usize`.
    USIZE => |x| Self::usize(unsafe { &*(x as *const Self as *const usize) }),
    // For other types, dispatch to the generic `Self::default`.
    _ => Self::default,
}
