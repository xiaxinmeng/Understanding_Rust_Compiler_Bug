rust
// a functional-ish head-tail pattern
match [9,8,7] {
    [head @ 0.., tail] => assert_eq!([9, 8], head),
    /* other patterns can go here
     * just imagine!
     */
}

// destructuring some bytes
let [octobyte 0..8, rest @ ..] = byteslice;
let num = u64::from_le_bytes(octobyte);
