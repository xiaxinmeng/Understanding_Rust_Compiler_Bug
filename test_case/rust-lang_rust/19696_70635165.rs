 rust
// Faster one
pub fn test_1(upper: usize) -> [isize; 3] {
    let mut v = [1is, 1, 1];
    for _ in (0us..upper) {
        for i in 0us..3 {
            while v[i] > 0 { v[i] -= 1; }
            // In order to get here, the value of v[i] *must* be <= 0,
            // Therefore we replace the above loop with
            // if v[i] > 0 { v[i] = 0; )
        }
    }

    v
}

pub fn test_2(upper: usize) -> [isize; 3] {
    let mut v = [1is, 1, 1];
    for _ in (0us..upper) {
        for i in 0us..3 {
            if v[i] > 0 { v[i] -= 1; }
            // The value isn't necessarily 0, just smaller
        }
    }

    v
}
