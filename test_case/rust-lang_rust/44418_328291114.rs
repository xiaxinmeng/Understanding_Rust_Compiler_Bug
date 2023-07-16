
[00:33:42] error[E0061]: this function takes 1 parameter but 2 parameters were supplied
[00:33:42]    --> /checkout/src/librustdoc/core.rs:149:38
[00:33:42]     |
[00:33:42] 149 |     let cstore = Rc::new(CStore::new(&dep_graph, box rustc_trans::LlvmMetadataLoader));
[00:33:42]     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 1 parameter
[00:33:42] 
[00:33:44] error[E0061]: this function takes 1 parameter but 2 parameters were supplied
[00:33:44]   --> /checkout/src/librustdoc/test.rs:88:38
[00:33:44]    |
[00:33:44] 88 |     let cstore = Rc::new(CStore::new(&dep_graph, box rustc_trans::LlvmMetadataLoader));
[00:33:44]    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 1 parameter
[00:33:44] 
[00:33:44] error[E0061]: this function takes 1 parameter but 2 parameters were supplied
[00:33:44]    --> /checkout/src/librustdoc/test.rs:241:38
[00:33:44]     |
[00:33:44] 241 |     let cstore = Rc::new(CStore::new(&dep_graph, box rustc_trans::LlvmMetadataLoader));
[00:33:44]     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 1 parameter
[00:33:44] 
[00:33:44] error: aborting due to 3 previous errors
[00:33:44] 
[00:33:44] error: Could not compile `rustdoc`.
