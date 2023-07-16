rust
#[inline(always)]
fn inline_me() -> u32 {
    42
}

pub fn root() -> u32 {
    inline_me()
}
