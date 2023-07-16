rust
error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
   --> /home/travis/build/servo/servo-with-rust-nightly/servo/components/style/stylesheets/stylesheet.rs:243:27
    |
243 |         self.iter_rules::<'a, 'b, EffectiveRules>(device, guard)
    |                           ^^
    |
note: lint level defined here
   --> /home/travis/build/servo/servo-with-rust-nightly/servo/components/style/lib.rs:26:9
    |
26  | #![deny(warnings)]
    |         ^^^^^^^^
    = note: #[deny(late_bound_lifetime_arguments)] implied by #[deny(warnings)]
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #42868 <https://github.com/rust-lang/rust/issues/42868>
