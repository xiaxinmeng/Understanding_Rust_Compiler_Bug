
/home/strcat/projects/rust/src/test/bench/core-map.rs:26:22: 26:23 error: Illegal lifetime 'r: only 'static is allowed here
/home/strcat/projects/rust/src/test/bench/core-map.rs:26 fn ascending<M: Map<'r, uint, uint>>(map: &mut M, n_keys: uint) {
                                                                               ^
/home/strcat/projects/rust/src/test/bench/core-map.rs:48:23: 48:24 error: Illegal lifetime 'r: only 'static is allowed here
/home/strcat/projects/rust/src/test/bench/core-map.rs:48 fn descending<M: Map<'r, uint, uint>>(map: &mut M, n_keys: uint) {
                                                                                ^
/home/strcat/projects/rust/src/test/bench/core-map.rs:70:19: 70:20 error: Illegal lifetime 'r: only 'static is allowed here
/home/strcat/projects/rust/src/test/bench/core-map.rs:70 fn vector<M: Map<'r, uint, uint>>(map: &mut M, n_keys: uint, dist: &[uint]) {
