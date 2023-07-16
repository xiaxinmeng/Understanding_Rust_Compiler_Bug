 rust
struct PreTransmute {
    ptr: *const (),
}

struct PostTransmute {
    ptr: Box<Something>,
}

let some_value: RefCell<Option<PreTransmute>> = RefCell::new(None);
let tmp = some_value.borrow();
let some_other_value: Ref<Option<PostTransmute>> = unsafe { mem::transmute(tmp) };
