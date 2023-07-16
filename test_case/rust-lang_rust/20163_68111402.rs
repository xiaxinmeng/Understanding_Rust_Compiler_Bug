
rustc: x86_64-unknown-linux-gnu/stage1/test/collectionstest-x86_64-unknown-linux-gnu
rustc: x86_64-unknown-linux-gnu/stage1/test/stdtest-x86_64-unknown-linux-gnu
/home/ben/devl/rust/rust/src/libcollections/btree/map.rs:1576:17: 1576:27 error: type `btree::map::VacantEntry<'_, str, string::String, int>` does not implement any method in scope named `insert`
/home/ben/devl/rust/rust/src/libcollections/btree/map.rs:1576           entry.insert(30);
                                                                              ^~~~~~~~~~
error: aborting due to previous error
/home/ben/devl/rust/rust/mk/tests.mk:409: recipe for target 'x86_64-unknown-linux-gnu/stage1/test/collectionstest-x86_64-unknown-linux-gnu' failed
make: *** [x86_64-unknown-linux-gnu/stage1/test/collectionstest-x86_64-unknown-linux-gnu] Error 101
make: *** Waiting for unfinished jobs....
/home/ben/devl/rust/rust/src/libstd/collections/hash/map.rs:2192:17: 2192:27 error: type `collections::hash::map::VacantEntry<'_, str, collections::string::String, int>` does not implement any method in scope named `insert`
/home/ben/devl/rust/rust/src/libstd/collections/hash/map.rs:2192           entry.insert(30);
                                                                                 ^~~~~~~~~~
