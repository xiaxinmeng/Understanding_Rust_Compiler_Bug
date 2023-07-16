 rust
#[cfg(target_word_size = "32")]
      let n = 1i << 32; //~ ERROR: bitshift exceeds the type's number of bits
#[cfg(target_word_size = "64")]
      let n = 1i << 64; //~ ERROR: bitshift exceeds the type's number of bits
