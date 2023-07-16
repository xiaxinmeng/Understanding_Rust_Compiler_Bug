
---- mem::forget_2 stdout ----
    <anon>:5:5: 20:6 error: visibility has no effect inside functions
<anon>:5     pub fn swap<T>(x: &mut T, y: &mut T) {
<anon>:6         unsafe {
<anon>:7             // Give ourselves some scratch space to work with
<anon>:8             let mut t: T = mem::uninitialized();
<anon>:9     
<anon>:10             // Perform the swap, `&mut` pointers never alias
          ...
error: aborting due to previous error
thread 'mem::forget_2' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:211
