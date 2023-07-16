
fn main() {
    let mut v = ~[];
    v.reserve(-1);
    v.push(1);
    v.push(2);
}
// *** glibc detected *** ./crash~: free(): invalid next size (fast): 0x00000000022e9040 ***
