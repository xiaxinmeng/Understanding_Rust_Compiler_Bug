
warning: literal out of range for u8
  --> $DIR/suggestions.rs:61:23
   |
LL |         for _ in 0u8..256 { }
   |                  -----^^^
   |                  |
   |                  help: use a closed range: `0u8..=255`
   |
   = note: #[warn(overflowing_literals)] on by default
