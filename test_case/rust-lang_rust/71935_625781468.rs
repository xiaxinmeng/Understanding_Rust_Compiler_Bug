rust
use std::mem::swap;

fn foo() -> u32 {
    const LIMIT: u32 = 1_000_000;
    const N_LIMIT: usize = 100;
    const NR: usize = N_LIMIT / 2 + 1;

    let mut row0 = &mut [0; NR];
    row0[0] = 1;
    let mut row1 = &mut [0; NR];
    row1[0] = 1;
    let mut result = 0;

    for n in 1 .. N_LIMIT + 1 {
        let mut r = 1;
        while r < n / 2 + 1 { //****
            row1[r] = row0[r] + row0[r - 1];
            if r == n / 2 && n % 2 == 0 {
                row1[r] += row0[r - 1];
            }

            if row1[r] > LIMIT {
                row1[r] = LIMIT;
                result += (n - 2 * r + 1) as u32;
                break;
            }
            r += 1;
        }
        swap(&mut row0, &mut row1);
    }
    result
}
