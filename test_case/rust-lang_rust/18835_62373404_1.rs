
$ rustc accum.rs
accum.rs:7:1: 12:2 error: conflicting implementations for trait `core::ops::FnMut` [E0119]
accum.rs:7 impl<T: Add<U, T> + Clone, U> FnMut<(U,), T> for G<T, U> {
accum.rs:8     extern "rust-call" fn call_mut(&mut self, (i,):(U,)) -> T {
accum.rs:9         self.n = self.n + i;
accum.rs:10         self.n.clone()
accum.rs:11     }
accum.rs:12 }
accum.rs:7:1: 12:2 note: conflicting implementation in crate `core`
accum.rs:7 impl<T: Add<U, T> + Clone, U> FnMut<(U,), T> for G<T, U> {
accum.rs:8     extern "rust-call" fn call_mut(&mut self, (i,):(U,)) -> T {
accum.rs:9         self.n = self.n + i;
accum.rs:10         self.n.clone()
accum.rs:11     }
accum.rs:12 }
error: aborting due to previous error
