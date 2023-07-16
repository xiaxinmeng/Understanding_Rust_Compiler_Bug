 rust
      #[cfg(target_word_size = "32")]
      {
        let n = 1i << 32; //~ ERROR: bitshift exceeds the type's number of bits
        let n = 1u << 32; //~ ERROR: bitshift exceeds the type's number of bits
      }
