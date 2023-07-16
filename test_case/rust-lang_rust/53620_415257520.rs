plain
[00:03:47]       Memory: 8 GB
[00:03:47]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:47]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:47]       SMC Version (system): 2.8f0
[00:03:47]       Serial Number (system): VMZxAZIpVqGX
[00:03:47] 
[00:03:47] hw.ncpu: 4
[00:03:47] hw.byteorder: 1234
[00:03:47] hw.memsize: 8589934592
---
Building stage2 tool miri (i686-apple-darwin)
[01:12:31]    Compiling miri v0.1.0 (file:///Users/travis/build/rust-lang/rust/src/tools/miri)
[01:12:31]    Compiling byteorder v1.2.3
[01:12:32] [RUSTC-TIMING] byteorder test:false 0.872
[01:12:32] error: found removed `do catch` syntax
[01:12:32]     |
[01:12:32]     |
[01:12:32] 639 |         let res: EvalResult<'tcx> = do catch {
[01:12:32]     |
[01:12:32]     |
[01:12:32]     = help: Following RFC #2388, the new non-placeholder syntax is `try`
[01:12:32] 
[01:12:32] error: found removed `do catch` syntax
[01:12:32]     |
[01:12:32]     |
[01:12:32] 253 |     let res: EvalResult = do catch {
[01:12:32]     |
[01:12:32]     |
[01:12:32]     = help: Following RFC #2388, the new non-placeholder syntax is `try`
[01:12:33] error[E0432]: unresolved import `syntax::codemap`
[01:12:33]   --> tools/miri/src/lib.rs:33:13
[01:12:33]    |
[01:12:33] 33 | use syntax::codemap::Span;
---
[01:12:34] 
[01:12:34] error[E0423]: expected value, found struct variant `$crate::mir::interpret::EvalErrorKind::Panic`
[01:12:34]    --> tools/miri/src/fn_call.rs:736:50
[01:12:34]     |
[01:12:34] 736 |             "std::rt::begin_panic_fmt" => return err!(Panic),
[01:12:34]     |                                                  ^^^^^^^^^^^ did you mean `$crate::mir::interpret::EvalErrorKind::Panic { /* fields */ }`?
[01:12:34]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:12:34] 
[01:12:34] error[E0531]: cannot find unit struct/variant or constant `TyBool` in module `ty`
[01:12:34]    --> tools/miri/src/validation.rs:387:17
---
[01:12:34]   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:12:34]   |
[01:12:34]   = note: #[warn(unused_imports)] on by default
[01:12:34] 
[01:12:34] warning: unused import: `Instance`
[01:12:34]   |
[01:12:34]   |
[01:12:34] 5 | use rustc::ty::{self, Ty, TypeFoldable, TyCtxt, Instance};
[01:12:34] 
[01:12:34] warning: unused import: `ConstValue`
[01:12:34]   --> tools/miri/src/validation.rs:11:29
[01:12:34]    |
---
[01:12:34] 
[01:12:34] warning: unused import: `rustc::ty::AdtKind`
[01:12:34]    --> tools/miri/src/validation.rs:535:13
[01:12:34]     |
[01:12:34] 535 |         use rustc::ty::AdtKind;
[01:12:34] 
[01:12:34] 
[01:12:34] warning: unused import: `tls::EvalContextExt as TlsEvalContextExt`
[01:12:34]    |
[01:12:34]    |
[01:12:34] 54 | use tls::EvalContextExt as TlsEvalContextExt;
[01:12:34] 
[01:12:34] error[E0050]: method `eval_fn_call` has 6 parameters but the declaration in trait `rustc_mir::interpret::Machine::eval_fn_call` has 5
[01:12:34]    --> tools/miri/src/lib.rs:398:14
[01:12:34]     |
---
travis_fold:start:dpl_0
travis_time:start:17f64e54
$ rvm $(travis_internal_ruby) --fuzzy do ruby -S gem install dpl
ERROR:  While executing gem ... (Gem::RemoteFetcher::UnknownHostError)
    no such name (https://api.rubygems.org/quick/Marshal.4.8/dpl-1.10.0.gemspec.rz)


The command "rvm $(travis_internal_ruby) --fuzzy do ruby -S gem install dpl" failed and exited with 1 during .
Your build has been stopped.
