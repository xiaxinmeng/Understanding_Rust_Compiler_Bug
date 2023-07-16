rust
error[E0308]: mismatched types
--> library/core/src/sync/atomic.rs:326:9
|
325|    pub fn get_mut(&mut self) -> &mut bool {
| ---------expected `&mut bool` because of return type
326|        self.v.get_mut()
| ^^^^^^^^^^^^^^^^expected `bool`, found `u8`
|
= note: expected mutable reference `&mut bool`
               found mutable reference `&mut u8`
