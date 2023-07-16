
Building stage0 compiler artifacts (i686-pc-windows-gnu -> i686-pc-windows-gnu)
   Compiling syntax v0.0.0 (file:///C:/projects/rust/src/libsyntax)
   Compiling serialize v0.0.0 (file:///C:/projects/rust/src/libserialize)
   Compiling rustc-serialize v0.3.24
   Compiling stable_deref_trait v1.0.0
   Compiling log v0.3.8
   Compiling rustc_bitflags v0.0.0 (file:///C:/projects/rust/src/librustc_bitflags)
   Compiling rustc_driver v0.0.0 (file:///C:/projects/rust/src/librustc_driver)
   Compiling gcc v0.3.53
error: `#[deprecated]` cannot be used in staged api, use `#[rustc_deprecated]` instead
  --> C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\gcc-0.3.53\src\lib.rs:57:1
   |
57 | pub type Config = Build;
   | ^^^^^^^^^^^^^^^^^^^^^^^^
error: `#[deprecated]` cannot be used in staged api, use `#[rustc_deprecated]` instead
