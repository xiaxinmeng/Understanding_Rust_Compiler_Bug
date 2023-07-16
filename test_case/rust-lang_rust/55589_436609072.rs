plain
travis_time:end:0eb40226:start=1541593257983284114,finish=1541593260358697043,duration=2375412929
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:13:43]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:13:58] error[E0599]: no method named `erase_tag` found for type `u128` in the current scope
[00:13:58]    --> librustc_mir/interpret/operand.rs:774:82
[00:13:58]     |
[00:13:58] 774 |                     .ok_or_else(|| EvalErrorKind::InvalidDiscriminant(real_discr.erase_tag()))?;
[00:13:58] 
[00:13:59] error[E0308]: mismatched types
[00:13:59]    --> librustc_mir/interpret/validity.rs:165:54
[00:13:59]     |
[00:13:59]     |
[00:13:59] 165 |                         PathElem::Field(def.variants[field].name)
[00:13:59]     |                                                      ^^^^^ expected struct `rustc_target::abi::VariantIdx`, found usize
[00:13:59]     |
[00:13:59]     = note: expected type `rustc_target::abi::VariantIdx`
[00:13:59] 
[00:14:00] error[E0308]: mismatched types
[00:14:00]   --> librustc_mir/interpret/visitor.rs:75:36
[00:14:00]    |
[00:14:00]    |
[00:14:00] 75 |         ecx.operand_downcast(self, variant)
[00:14:00]    |                                    ^^^^^^^ expected struct `rustc_target::abi::VariantIdx`, found usize
[00:14:00]    |
[00:14:00]    = note: expected type `rustc_target::abi::VariantIdx`
[00:14:00] 
[00:14:00] error[E0308]: mismatched types
[00:14:00]    --> librustc_mir/interpret/visitor.rs:114:35
[00:14:00]     |
[00:14:00]     |
[00:14:00] 114 |         ecx.mplace_downcast(self, variant)
[00:14:00]     |                                   ^^^^^^^ expected struct `rustc_target::abi::VariantIdx`, found usize
[00:14:00]     |
[00:14:00]     = note: expected type `rustc_target::abi::VariantIdx`
[00:14:00] 
[00:14:01] error[E0308]: mismatched types
[00:14:01]    --> librustc_mir/interpret/visitor.rs:221:68
[00:14:01]     |
[00:14:01]     |
[00:14:01] 221 |                         let inner = v.project_downcast(self.ecx(), idx)?;
[00:14:01]     |                                                                    ^^^ expected usize, found struct `rustc_target::abi::VariantIdx`
[00:14:01] ...
[00:14:01] 319 | make_value_visitor!(ValueVisitor,);
[00:14:01]     | ----------------------------------- in this macro invocation
[00:14:01]     = note: expected type `usize`
[00:14:01]     = note: expected type `usize`
[00:14:01]                found type `rustc_target::abi::VariantIdx`
[00:14:01] error[E0308]: mismatched types
[00:14:01]    --> librustc_mir/interpret/visitor.rs:224:52
[00:14:01]     |
[00:14:01]     |
[00:14:01] 224 |                         return self.visit_field(v, idx, inner);
[00:14:01]     |                                                    ^^^ expected usize, found struct `rustc_target::abi::VariantIdx`
[00:14:01] ...
[00:14:01] 319 | make_value_visitor!(ValueVisitor,);
[00:14:01]     | ----------------------------------- in this macro invocation
[00:14:01]     = note: expected type `usize`
[00:14:01]     = note: expected type `usize`
[00:14:01]                found type `rustc_target::abi::VariantIdx`
[00:14:01] error[E0308]: mismatched types
[00:14:01]    --> librustc_mir/interpret/visitor.rs:221:68
[00:14:01]     |
[00:14:01]     |
[00:14:01] 221 |                         let inner = v.project_downcast(self.ecx(), idx)?;
[00:14:01]     |                                                                    ^^^ expected usize, found struct `rustc_target::abi::VariantIdx`
[00:14:01] ...
[00:14:01] 320 | make_value_visitor!(MutValueVisitor,mut);
[00:14:01]     | ----------------------------------------- in this macro invocation
[00:14:01]     = note: expected type `usize`
[00:14:01]     = note: expected type `usize`
[00:14:01]                found type `rustc_target::abi::VariantIdx`
[00:14:02] error[E0308]: mismatched types
[00:14:02]    --> librustc_mir/interpret/visitor.rs:224:52
[00:14:02]     |
[00:14:02]     |
[00:14:02] 224 |                         return self.visit_field(v, idx, inner);
[00:14:02]     |                                                    ^^^ expected usize, found struct `rustc_target::abi::VariantIdx`
[00:14:02] ...
[00:14:02] 320 | make_value_visitor!(MutValueVisitor,mut);
[00:14:02]     | ----------------------------------------- in this macro invocation
[00:14:02]     = note: expected type `usize`
[00:14:02]     = note: expected type `usize`
[00:14:02]                found type `rustc_target::abi::VariantIdx`
[00:14:03] error: aborting due to 8 previous errors
[00:14:03] 
[00:14:03] Some errors occurred: E0308, E0599.
[00:14:03] For more information about an error, try `rustc --explain E0308`.
[00:14:03] For more information about an error, try `rustc --explain E0308`.
[00:14:03] error: Could not compile `rustc_mir`.
[00:14:03] warning: build failed, waiting for other jobs to finish...
[00:14:52] error: build failed
[00:14:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:14:52] expected success, got: exit code: 101
[00:14:52] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:14:52] travis_fold:end:stage0-rustc

[00:14:52] travis_time:end:stage0-rustc:start=1541593585383497900,finish=1541594163327630351,duration=577944132451


[00:14:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:14:52] Build completed unsuccessfully in 0:10:48
[00:14:52] make: *** [all] Error 1
[00:14:52] Makefile:28: recipe for target 'all' failed
32464 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
32392 ./src/libcompiler_builtins/compiler-rt/test
32168 ./.git/modules/src/tools/lld
31780 ./.git/modules/src/tools/lld/objects
