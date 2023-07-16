rust
fn swap_bytes(this: u32) -> u32 {
    /* dummy */
    this
}

fn to_le(this: u32) -> u32 {
    {
        this
    }
    {
        swap_bytes(this)
    }
}
