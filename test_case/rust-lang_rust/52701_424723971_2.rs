rust
    fn matcher(x: u8) {
      match x { //~ ERROR
        0 .. 32 => { /* foo */ }
      }
    }
    