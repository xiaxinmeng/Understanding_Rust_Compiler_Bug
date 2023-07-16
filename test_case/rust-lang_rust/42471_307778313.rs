
[00:58:45] error[E0512]: transmute called with differently sized types: rls_data::Analysis (2176 bits) to data::Analysis (1984 bits)
[00:58:45]    --> /checkout/src/tools/rls/src/build.rs:588:57
[00:58:45]     |
[00:58:45] 588 |                                                         ::std::mem::transmute(a.clone())
[00:58:45]     |                                                         ^^^^^^^^^^^^^^^^^^^^^ transmuting between 2176 bits and 1984 bits
