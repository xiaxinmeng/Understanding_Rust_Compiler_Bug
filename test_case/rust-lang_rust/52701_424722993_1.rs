rust
    #![feature(exhaustive_integer_patterns)]
    #![feature(exclusive_range_pattern)]
    
    fn matcher(x: u8) {
      match x { // ok
        0 .. 32 => { /* foo */ }
        32 => { /* bar */ }
        33 ..= 255 => { /* baz */ }
      }
    }
    