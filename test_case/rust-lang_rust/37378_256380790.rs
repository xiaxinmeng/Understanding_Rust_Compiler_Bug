
error: patterns aren't allowed in methods without bodies
   --> tests\cargotest\support/mod.rs:604:34
    |
604 |     fn tap<F: FnOnce(&mut Self)>(mut self, callback: F) -> Self;
    |                                  ^^^^^^^^
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #35203 <https://github.com/rust-lang/rust/issues/35203>
note: lint level defined here
   --> tests\cargotest\lib.rs:1:9
    |
1   | #![deny(warnings)]
    |         ^^^^^^^^

error: aborting due to previous error
