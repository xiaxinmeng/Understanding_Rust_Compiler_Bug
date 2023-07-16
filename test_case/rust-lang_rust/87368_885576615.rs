rust
#[repr(C, packed)]
struct FooBazUnAligned {
    bar: u64,
    baz: usize,
}

#[repr(C, align(8))]
struct FooBazAligned {
    inner: FooBazUnAligned
}

fn main() {
    let aligned = FooBazAligned { inner: FooBazUnAligned { bar: 0, baz: 0 } };
    let _ = &aligned.inner.bar;
}
