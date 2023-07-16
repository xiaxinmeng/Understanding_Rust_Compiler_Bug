rust
#![feature(const_fn)]
const fn bad(input: fn()) {
    input()
}
