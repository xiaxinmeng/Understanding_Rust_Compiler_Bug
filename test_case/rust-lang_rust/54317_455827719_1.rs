
let t = dbg!(2 + 1);

// instead of:
let _: u64 = dbg!((1 << 63) | (t << 8));

// this gives a nicer representation:
let _: u64 = dbg!("0x{:x}", (1 << 63) | (t << 8));
    
// also possible:
let _: u64 = dbg!("0x{:x}, with t={}", (1 << 63) | (t << 8), t);
