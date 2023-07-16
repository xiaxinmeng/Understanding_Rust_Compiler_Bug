rust
let _ = Scalar { // uninit or null?
    value: Primitive::Pointer { undef: true },
    valid_range: WrappingRange { start: 0, end: 0 },
};
