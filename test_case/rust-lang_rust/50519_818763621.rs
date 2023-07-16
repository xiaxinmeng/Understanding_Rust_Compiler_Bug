rust
const N: usize = 20_000_000;

pub fn sieve(vis: &mut [bool; N + 1]) {
    let mut i = 2;
    let mut count = 0;
    while i * i <= N {
        if vis[i] {
            count += 1; // This writes to stack every time.
            let mut k = 2 * i;
            while k <= N {
                vis[k] = true;
                k += i;
            }
        }
        i += 1;
    }

    // This loop can be optimized into SIMD when using walk-around below.
    while i <= N {
        if vis[i] {
            count += 1;  // This writes to stack every time.
        }
        i += 1;
    }

    // let count = count; // Uncomment this line to walk-around this issue.
    println!("{}", count);
}
