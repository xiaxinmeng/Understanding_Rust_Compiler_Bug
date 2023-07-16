rust
fn needs_zst<T>(_: T) where T: IsZst<ZST=ZstHelper<{true}>> {}
