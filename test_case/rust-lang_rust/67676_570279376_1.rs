rust
error: truncating cast: the value 4294967292 requires 32 bits but the target type is only 16 bits
     --> /tmp/.tmpmCOmcQ/target/debug/build/style-f46d78432390fdd0/out/gecko_properties.rs:10050:21
      |
10050 |                     structs::NS_FONT_STRETCH_ULTRA_CONDENSED as i16,
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
      = note: `#[deny(const_err)]` on by default
