
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:333:30: 335:6 error: `arena` does not live long enough
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:333     let result = arena.alloc(|| Outer {
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:334         inner: arena.alloc(|| Inner { value: 10 })
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:335     });
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:329:42: 338:2 note: reference must be valid for the block suffix following statement 1 at 329:41...
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:329     struct Outer<'a> { inner: &'a Inner }
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:330 
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:331     let arena = Arena::new();
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:332 
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:333     let result = arena.alloc(|| Outer {
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:334         inner: arena.alloc(|| Inner { value: 10 })
                                                                                       ...
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:331:29: 338:2 note: ...but borrowed value is only valid for the block suffix following statement 2 at 331:28
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:331     let arena = Arena::new();
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:332 
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:333     let result = arena.alloc(|| Outer {
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:334         inner: arena.alloc(|| Inner { value: 10 })
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:335     });
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libarena/lib.rs:336 
