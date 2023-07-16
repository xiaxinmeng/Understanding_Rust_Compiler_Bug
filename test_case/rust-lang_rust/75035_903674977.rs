rust
fn euler76() -> u32 {
    const N: usize = 100; // Input.
    let mut ways = [0; N + 1];
    ways[0] = 1;
    for j in 1 .. N {
        //for i in j ..= N { // Adds a bound test.
        for i in j .. N + 1 {
            ways[i] += ways[i - j];
        }
    }
    ways[N]
}

fn main() { assert_eq!(euler76(), 190_569_291); }
