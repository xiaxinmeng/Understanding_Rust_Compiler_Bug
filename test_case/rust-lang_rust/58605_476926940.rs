plain
travis_time:end:1fcb0774:start=1553649020780369545,finish=1553649094775824007,duration=73995454462
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:24:48]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:24:52] error[E0425]: cannot find value `sess` in this scope
[00:24:52]   --> src/librustc_codegen_llvm/back/write.rs:98:28
[00:24:52]    |
[00:24:52] 98 |     target_machine_factory(sess, tcx.backend_optimization_level(LOCAL_CRATE), find_features)()
[00:24:52] 
[00:24:52] error[E0425]: cannot find value `sess` in this scope
[00:24:52]    --> src/librustc_codegen_llvm/back/write.rs:100:18
[00:24:52]     |
[00:24:52]     |
[00:24:52] 100 |         llvm_err(sess.diagnostic(), &err).raise()
[00:24:52] 
[00:24:52] 
[00:24:53] error[E0599]: no method named `new` found for type `LlvmCodegenBackend` in the current scope
[00:24:53]    --> src/librustc_codegen_llvm/base.rs:169:35
[00:24:53]     |
[00:24:53] 169 |         let llvm_module = backend.new(tcx, &cgu_name.as_str());
[00:24:53]     |                           |       |
[00:24:53]     |                           |       this is an associated function, not a method
[00:24:53]     |                           help: use associated function syntax instead: `LlvmCodegenBackend::new`
[00:24:53]     | 
[00:24:53]     | 
[00:24:53]    ::: src/librustc_codegen_llvm/lib.rs:111:1
[00:24:53]     |
[00:24:53] 111 | pub struct LlvmCodegenBackend(());
[00:24:53]     | ---------------------------------- method `new` not found for this
[00:24:53]     |
[00:24:53]     = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
[00:24:53] note: the candidate is defined in an impl for the type `LlvmCodegenBackend`
[00:24:53]     |
[00:24:53]     |
[00:24:53] 224 |     pub fn new() -> Box<dyn CodegenBackend> {
[00:24:53]     = help: items from traits can only be used if the trait is implemented and in scope
[00:24:53]     = help: items from traits can only be used if the trait is implemented and in scope
[00:24:53]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[00:24:53]             candidate #1: `abi::FnTypeExt`
[00:24:53]             candidate #2: `rustc_data_structures::indexed_vec::Idx`
[00:24:53]             candidate #3: `proc_macro::bridge::server::TokenStream`
[00:24:53]             candidate #4: `proc_macro::bridge::server::TokenStreamBuilder`
[00:24:53]             candidate #5: `proc_macro::bridge::server::Group`
[00:24:53]             candidate #6: `proc_macro::bridge::server::Punct`
[00:24:53]             candidate #7: `proc_macro::bridge::server::Ident`
[00:24:53]             candidate #8: `proc_macro::bridge::server::MultiSpan`
[00:24:53]             candidate #9: `proc_macro::bridge::server::Diagnostic`
[00:24:53]             candidate #10: `rand::distributions::uniform::UniformSampler`
[00:24:54] error: aborting due to 3 previous errors
[00:24:54] 
[00:24:54] Some errors occurred: E0425, E0599.
[00:24:54] For more information about an error, try `rustc --explain E0425`.
