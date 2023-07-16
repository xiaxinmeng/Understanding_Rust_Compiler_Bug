
---- [ui] ui\run-pass\structs-enums\enum-discrim-width-stuff.rs stdout ----
normalized stderr:
warning: literal out of range for isize
  --> $DIR/enum-discrim-width-stuff.rs:40:20
   |
LL |     check!(f, u32, 0xe8d8c8b8);
   |                    ^^^^^^^^^^
   |
   = note: #[warn(overflowing_literals)] on by default
   = note: the literal `0xe8d8c8b8` (decimal `3906521272`) does not fit into an `isize` and will become `-388446024isize`
