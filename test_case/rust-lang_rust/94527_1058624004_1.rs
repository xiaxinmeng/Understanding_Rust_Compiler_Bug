rust
enum ValidRange {
    FullRangeOrUninit,
    InitWithRange(WrappingRange),
}

let _ = Scalar { // maybe uninit
    value: Primitive::Pointer,
    valid_range: ValidRange::FullRangeOrUninit,
};
let _ = Scalar { // always null
    value: Primitive::Pointer,
    valid_range: ValidRange::InitWithRange(WrappingRange { start: 0, end: 0 }),
};
