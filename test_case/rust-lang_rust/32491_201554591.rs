 rust
dup.rs:4:5: 4:31 warning: duplicate definitions with name `get`, #[warn(overlapping_inherent_impls)] on by default
dup.rs:4     fn get(&self) -> u8 { 42 }
             ^~~~~~~~~~~~~~~~~~~~~~~~~~
dup.rs:4:5: 4:31 warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
dup.rs:4:5: 4:31 note: for more information, see issue #22889 <https://github.com/rust-lang/rust/issues/22889>
dup.rs:4:5: 4:31 warning: method is never used: `get`, #[warn(dead_code)] on by default
dup.rs:4     fn get(&self) -> u8 { 42 }
