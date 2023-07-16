
error[E0365]: `Sealed` is private, and cannot be re-exported
  --> src/main.rs:10:13
   |
10 |     pub use sealed::Sealed as _;
   |             ^^^^^^^^^^^^^^^^^^^ re-export of private `Sealed`
   |
   = note: consider declaring type or module `Sealed` with `pub`
