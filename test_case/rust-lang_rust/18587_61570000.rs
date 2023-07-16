 rust
#[cfg(target_word_size = "32")]
fn dead_but_still_linted() {
        let n = 1i << 32; //~ ERROR: bitshift exceeds the type's number of bits
        let n = 1u << 32; //~ ERROR: bitshift exceeds the type's number of bits
}
