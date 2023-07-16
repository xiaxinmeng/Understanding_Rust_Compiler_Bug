rust
struct NeedsDrop(i32);

impl NeedsDrop {
    const fn add_one(self)-> Self {
        NeedsDrop(self.0 + 1)
    }
}

impl Drop for NeedsDrop {
    fn drop(&mut self) {}
}

const _: NeedsDrop = {
    let x = NeedsDrop(0);
    let r = &mut x;
    let y = x.add_one();
    y
};
