rust
union U {
    x: u32,
    y: f32,
    z: [u16; 2],
    w: [u8; 4],
}

unsafe {
    let mut u = U { x: 1 };
    // x: written, y: -, z: -, w: -
    let y = u.y; 
    // x: written, y: read, z: -, w: -
    // -> x: used, y: used, z: -, w: - 
    // `y` is used because it is read.
    // `x` is used because the write to `x` happens before read of `y`.
    u.z = [5, 6];
    // x: used, y: used, z: written, w: -
    drop(u);
    // w is entirely untouched, so unused.
    // z is written, but the union is no longer read, so also unused.
    // (current Rust treats `z` is used, but that's fine.)
}
