
error: patterns aren't allowed in functions without bodies
  --> src/lib.rs:11:45
   |
11 |     fn partition_dedup_by_new<F>(&mut self, mut same_bucket: F) -> (&mut [T], &mut [T])
   |                                             ^^^^^^^^^^^^^^^
   |
   = note: `#[deny(patterns_in_fns_without_body)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #35203 <https://github.com/rust-lang/rust/issues/35203>

error: aborting due to previous error

error: could not compile `alt-dedup-rs`

To learn more, run the command again with --verbose.
