
pub const fn bad_hash(_bytes: &[u8]) -> u64 {
    0
}

struct Thing {
    pub h: u64,
    pub f: for<'a> fn(std::marker::PhantomData<&'a ()>),
}

static THINGS: &[Thing] = &[
    Thing {
        h: bad_hash(b"abc"),
        f: |_| (),
    },
    Thing {
        h: bad_hash(b"abc"),
        f: |_| (),
    },
];
