
$ rustdoc --test src/libcollections/lib.rs 
src/libcollections/btree/map.rs:1392:32: 1392:58 error: slice pattern syntax is experimental
src/libcollections/btree/map.rs:1392                         if let [ref $($mutability)* edge] = slice.edges {
                                                                    ^~~~~~~~~~~~~~~~~~~~~~~~~~
src/libcollections/btree/map.rs:1342:1: 1470:2 note: in expansion of range_impl!
src/libcollections/btree/map.rs:1496:9: 1497:6 note: expansion site
src/libcollections/btree/map.rs:1392:58: 1392:58 help: add #![feature(slice_patterns)] to the crate attributes to enable
src/libcollections/btree/map.rs:1342:1: 1470:2 note: in expansion of range_impl!
src/libcollections/btree/map.rs:1496:9: 1497:6 note: expansion site
src/libcollections/btree/map.rs:1392:32: 1392:58 error: slice pattern syntax is experimental
src/libcollections/btree/map.rs:1392                         if let [ref $($mutability)* edge] = slice.edges {
                                                                    ^~~~~~~~~~~~~~~~~~~~~~~~~~
src/libcollections/btree/map.rs:1342:1: 1470:2 note: in expansion of range_impl!
src/libcollections/btree/map.rs:1523:9: 1525:6 note: expansion site
src/libcollections/btree/map.rs:1392:58: 1392:58 help: add #![feature(slice_patterns)] to the crate attributes to enable
src/libcollections/btree/map.rs:1342:1: 1470:2 note: in expansion of range_impl!
src/libcollections/btree/map.rs:1523:9: 1525:6 note: expansion site
error: aborting due to 2 previous errors
thread '<unnamed>' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:152
thread '<main>' panicked at 'child thread None panicked', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thread/mod.rs:775
