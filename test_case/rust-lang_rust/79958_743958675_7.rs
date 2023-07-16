text

===================================================================================================

Failure due to:
   error[E0493]: destructors cannot be evaluated at compile-time
     --> /usr/local/google/home/richkadel/rust/src/test/ui/consts/unstable-precise-live-drops-in-libcore.rs:15:25
      |
   LL |     pub const fn unwrap(self) -> T {
      |                         ^^^^ constant functions cannot evaluate destructors
(this error is printed twice)

src/test/ui/consts/unstable-precise-live-drops-in-libcore.rs

