rust
const fn coin() -> usize {
    let randomness = 1u8;
    (&randomness as *const u8 as usize) % 2
}

/* lots of other code */

const BAD : [f32; coin()] = [4.2; coin()];
