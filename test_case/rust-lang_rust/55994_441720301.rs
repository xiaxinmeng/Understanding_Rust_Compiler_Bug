
   Compiling unicode-normalization v0.1.4
error[E0119]: conflicting implementations of trait `Trait` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`:
  --> C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\traitobject-0.1.0\src\impls.rs:72:1
   |
71 | unsafe impl Trait for ::std::marker::Send + Sync { }
   | ------------------------------------------------ first implementation here
72 | unsafe impl Trait for ::std::marker::Send + Send + Sync { }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`
error: aborting due to previous error
