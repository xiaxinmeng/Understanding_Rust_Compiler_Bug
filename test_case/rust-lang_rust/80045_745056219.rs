
> rustc +stage1 src/test/ui/consts/unstable-precise-live-drops-in-libcore.rs
warning: enum is never used: `Either`
 --> src/test/ui/consts/unstable-precise-live-drops-in-libcore.rs:7:6
  |
7 | enum Either<T, S> {
  |      ^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: associated function is never used: `unwrap`
  --> src/test/ui/consts/unstable-precise-live-drops-in-libcore.rs:15:18
   |
15 |     pub const fn unwrap(self) -> T {
   |                  ^^^^^^

warning: 2 warnings emitted
