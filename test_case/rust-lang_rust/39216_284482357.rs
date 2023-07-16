rust
warning: code relies on type inference rules which are likely to change
   --> src/algo/dominators.rs:260:9
    |
260 |         assert_eq!(None, doms.dominators(99).map(|_| unreachable!()));
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: #[warn(resolve_trait_on_defaulted_unit)] on by default
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #39216 <https://github.com/rust-lang/rust/issues/39216>
    = note: this error originates in a macro outside of the current crate
