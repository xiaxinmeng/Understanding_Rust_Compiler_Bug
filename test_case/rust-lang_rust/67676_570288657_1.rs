rust
   error: truncating cast: the value 65535 requires 16 bits but the target type is only 8 bits
    --> a.rs:2:13
     |
   2 |     let _ = -1_i16 as i8;
     |             ^^^^^^^^^^^^
     |
     = note: `#[deny(const_err)]` on by default
   