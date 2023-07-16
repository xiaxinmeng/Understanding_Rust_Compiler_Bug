
warning: private type `foo::Private` in public interface (error E0446)
  --> src/lib.rs:12:5
   |
12 |     pub fn bar() -> impl AsRef<Private> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(private_in_public)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
