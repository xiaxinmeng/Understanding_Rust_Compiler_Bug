
warning: private type `[closure@src/lib.rs:5:5: 5:10]` in public interface (error E0446)
 --> src/lib.rs:3:1
  |
3 | pub existential type X: FnOnce();
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(private_in_public)] on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>

    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
