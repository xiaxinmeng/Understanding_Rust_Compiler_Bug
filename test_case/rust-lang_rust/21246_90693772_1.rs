
rustdoc --test /Users/jonny/code/rslike/src/lib.rs --crate-name rslike -L dependency=/Users/jonny/code/rslike/target/debug/deps --extern tcod=/Users/jonny/code/rslike/target/debug/deps/libtcod-77d31af52492579f.rlib --extern rslike=/Users/jonny/code/rslike/target/debug/librslike-a5edd6cfc864fa5b.rlib -L dependency=/Users/jonny/code/rslike/target/debug/build/tcod-sys-101f3066d68732c8/out

running 3 tests
test engine::actor::Actor_0 ... FAILED
test engine::map::new_0 ... FAILED
test engine::map::new_1 ... FAILED

failures:

---- engine::actor::Actor_0 stdout ----
    thread 'engine::actor::Actor_0' panicked at 'test executable failed:

dyld: Library not loaded: libtcod.dylib
  Referenced from: /var/folders/6h/4xvb2jn91m5cj7p4wrrpl3c00000gn/T/rustdoctest.emkHOt9oudMQ/rust_out
  Reason: image not found
', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/librustdoc/test.rs:245


---- engine::map::new_0 stdout ----
    thread 'engine::map::new_0' panicked at 'test executable failed:

dyld: Library not loaded: libtcod.dylib
  Referenced from: /var/folders/6h/4xvb2jn91m5cj7p4wrrpl3c00000gn/T/rustdoctest.2LCMGcOqRjTO/rust_out
  Reason: image not found
', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/librustdoc/test.rs:245


---- engine::map::new_1 stdout ----
    thread 'engine::map::new_1' panicked at 'test executable failed:

dyld: Library not loaded: libtcod.dylib
  Referenced from: /var/folders/6h/4xvb2jn91m5cj7p4wrrpl3c00000gn/T/rustdoctest.V9OwlASfiVVM/rust_out
  Reason: image not found
', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/librustdoc/test.rs:245



failures:
    engine::actor::Actor_0
    engine::map::new_0
    engine::map::new_1

test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured
