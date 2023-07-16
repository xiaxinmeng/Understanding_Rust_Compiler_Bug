plain
travis_time:end:108f5240:start=1541592975447476803,finish=1541592976443542668,duration=996065865
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:37:33]    Compiling parking_lot_core v0.3.0
[00:37:33]    Compiling tempfile v3.0.3
[00:37:35]    Compiling parking_lot v0.6.4
[00:37:36]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:37:41] error[E0599]: no function or associated item named `new` found for type `syntax::ast::NodeId` in the current scope
[00:37:41]     |
[00:37:41] 246 |                 NodeId::new(0),
[00:37:41] 246 |                 NodeId::new(0),
[00:37:41]     |                 ^^^^^^^^^^^ function or associated item not found in `syntax::ast::NodeId`
[00:37:41]     = help: items from traits can only be used if the trait is in scope
[00:37:41]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:37:41]             `use rustc_data_structures::indexed_vec::Idx;`
[00:37:41] 
[00:37:41] 
[00:37:41] error[E0599]: no function or associated item named `new` found for type `syntax::ast::NodeId` in the current scope
[00:37:41]     |
[00:37:41]     |
[00:37:41] 284 |                     if item_node_id.unwrap() != NodeId::new(0) {
[00:37:41]     |                                                 ^^^^^^^^^^^ function or associated item not found in `syntax::ast::NodeId`
[00:37:41]     = help: items from traits can only be used if the trait is in scope
[00:37:41]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:37:41]             `use rustc_data_structures::indexed_vec::Idx;`
[00:37:41] 
[00:37:41] 
[00:37:41] error[E0599]: no function or associated item named `new` found for type `syntax::ast::NodeId` in the current scope
[00:37:41]     |
[00:37:41]     |
[00:37:41] 291 |                         Some(parent) if parent != NodeId::new(0) => {
[00:37:41]     |                                                   ^^^^^^^^^^^ function or associated item not found in `syntax::ast::NodeId`
[00:37:41]     = help: items from traits can only be used if the trait is in scope
[00:37:41]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:37:41]             `use rustc_data_structures::indexed_vec::Idx;`
[00:37:41] 
[00:37:41] 
[00:37:41]            ^^^^^^^^^^^ function or associated item not found in `syntax::ast::NodeId`
[00:37:41]     = help: items from traits can only be used if the trait is in scope
[00:37:41]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:37:41]             `use rustc_data_structures::indexed_vec::Idx;`
[00:37:41] 
[00:37:41] 
[00:37:41] error[E0599]: no function or associated item named `new` found for type `syntax::ast::NodeId` in the current scope
[00:37:41]     |
[00:37:41] 567 |                                      NodeId::new(0),
[00:37:41] 567 |                                      NodeId::new(0),
[00:37:41]     |                                      ^^^^^^^^^^^ function or associated item not found in `syntax::ast::NodeId`
[00:37:41]     = help: items from traits can only be used if the trait is in scope
[00:37:41]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:37:41]             `use rustc_data_structures::indexed_vec::Idx;`
3500600 .
---
151412 ./src/tools/clang
150256 ./obj/build/bootstrap/debug/incremental
149116 ./src/llvm-emscripten/test
134668 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u
134664 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u/s-f6g644o9i5-1jgvkpy-22tmsi8iacpi9
120932 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
120928 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
118732 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
111096 ./src/llvm/test/CodeGen
