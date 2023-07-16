plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: associated function has missing stability attribute
    --> library/core/src/slice/mod.rs:4000:5
     |
4000 | /     pub fn flatten(&self) -> &[T] {
4001 | |         let len = if crate::mem::size_of::<T>() == 0 {
4002 | |             self.len().checked_mul(N).expect("slice len overflow")
...    |
...    |
4009 | |         unsafe { from_raw_parts_mut(self.as_mut_ptr().cast(), len) }
     | |_____^

error: associated function has missing stability attribute
    --> library/core/src/slice/mod.rs:4012:5
    --> library/core/src/slice/mod.rs:4012:5
     |
4012 | /     pub fn flatten_mut(&mut self) -> &mut [T] {
4013 | |         let len = if crate::mem::size_of::<T>() == 0 {
4014 | |             self.len().checked_mul(N).expect("slice len overflow")
...    |
...    |
4021 | |         unsafe { from_raw_parts_mut(self.as_mut_ptr().cast(), len) }
     | |_____^

error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:03:31
