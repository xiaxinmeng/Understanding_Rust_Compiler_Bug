
error: `self` no longer imports values
   --> bs58-rs/src/decode.rs:150:19
    |
150 |     use decode::{ self, DecodeError };
    |                   ^^^^
    |
    = note: #[deny(legacy_imports)] on by default
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #38260 <https://github.com/rust-lang/rust/issues/38260>
