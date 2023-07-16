rust
extern "C" {
    type ExternType;
}

struct ExternTypeWrapper {
    _a: ExternType,
}

let nullptr = 0 as *const ();
let extern_nullptr = nullptr as *const ExternTypeWrapper;
extern_nullptr as *const ();
let slice_ptr = &[] as *const [u8];
slice_ptr as *const u8;
