rust
union MaybeUninit<T: Copy> {
    uninit: (),
    init: T
}

static mut FOO: MaybeUninit<u32> = MaybeUninit { uninit: () };
