
/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-c/build/src/libcore/any.rs:188:12: 188:16 error: mismatched types: expected `&any::Any+'static`, found `&any::Any+'a` (lifetime mismatch)
/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-c/build/src/libcore/any.rs:188         if self.is::<T>() {
                                                                                                  ^~~~
/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-c/build/src/libcore/any.rs:187:60: 199:6 note: the lifetime 'a as defined on the block at 187:59...
/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-c/build/src/libcore/any.rs:187     fn downcast_mut<T: 'static>(self) -> Option<&'a mut T> {
/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-c/build/src/libcore/any.rs:188         if self.is::<T>() {
/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-c/build/src/libcore/any.rs:189             unsafe {
/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-c/build/src/libcore/any.rs:190                 // Get the raw representation of the trait object
/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-c/build/src/libcore/any.rs:191                 let to: TraitObject = transmute_copy(&self);
/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-c/build/src/libcore/any.rs:192 
                                                                                       ...
note: ...does not necessarily outlive the static lifetime
error: aborting due to previous error
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.core] Error 101
program finished with exit code 2
