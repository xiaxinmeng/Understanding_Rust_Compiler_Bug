rust
const UNINIT: core::mem::MaybeUninit<core::cell::Cell<&'static ()>> =
    core::mem::MaybeUninit::uninit();
static mut BUF: [MaybeUninit<Cell<&'static ()>>; $LEN] = [UNINIT; $LEN];
