plain
[00:13:56]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:13:57] error: unused import: `Place`
[00:13:57]   --> librustc_mir/borrow_check/borrow_set.rs:16:39
[00:13:57]    |
[00:13:57] 16 | use rustc::mir::{self, Location, Mir, Place, Local};
[00:13:57]    |
[00:13:57]    = note: `-D unused-imports` implied by `-D warnings`
[00:13:57] 
[00:14:00] error[E0308]: mismatched types
[00:14:00] error[E0308]: mismatched types
[00:14:00]    --> librustc_mir/borrow_check/borrow_set.rs:400:57
[00:14:00]     |
[00:14:00] 400 |         let old_value = self.pending_activations.insert(temp, borrow_index);
[00:14:00]     |                                                         |
[00:14:00]     |                                                         expected struct `rustc::mir::Local`, found reference
[00:14:00]     |                                                         expected struct `rustc::mir::Local`, found reference
[00:14:00]     |                                                         help: consider dereferencing the borrow: `*temp`
[00:14:00]     = note: expected type `rustc::mir::Local`
[00:14:00]                found type `&rustc::mir::Local`
[00:14:00] 
[00:14:08] error: aborting due to 2 previous errors
[00:14:08] error: aborting due to 2 previous errors
[00:14:08] 
[00:14:08] For more information about this error, try `rustc --explain E0308`.
[00:14:08] error: Could not compile `rustc_mir`.
[00:14:08] warning: build failed, waiting for other jobs to finish...
an/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0238cde7
$ dmesg | grep -i kill
