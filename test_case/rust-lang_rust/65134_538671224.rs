plain
2019-10-05T16:32:16.9063425Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-05T16:32:16.9271127Z ##[command]git config gc.auto 0
2019-10-05T16:32:16.9336063Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-05T16:32:16.9387427Z ##[command]git config --get-all http.proxy
2019-10-05T16:32:16.9550780Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65134/merge:refs/remotes/pull/65134/merge
---
2019-10-05T17:31:43.9019316Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-10-05T17:31:43.9023867Z    Compiling cc v1.0.35
2019-10-05T17:31:45.9825883Z    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
2019-10-05T17:31:50.4953907Z    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
2019-10-05T17:31:59.0532582Z error: `extern` block uses type `rustc_codegen_ssa::back::write::CodegenContext<LlvmCodegenBackend>`, which is not FFI-safe
2019-10-05T17:31:59.0534154Z    --> src/librustc_codegen_llvm/back/write.rs:242:46
2019-10-05T17:31:59.0534858Z     |
2019-10-05T17:31:59.0535620Z 242 | unsafe extern "C" fn report_inline_asm(cgcx: &CodegenContext<LlvmCodegenBackend>,
2019-10-05T17:31:59.0536406Z     |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
2019-10-05T17:31:59.0537725Z     = note: `-D improper-ctypes` implied by `-D warnings`
2019-10-05T17:31:59.0538442Z     = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
2019-10-05T17:31:59.0539574Z     = note: this struct has unspecified layout
2019-10-05T17:31:59.0539903Z 
2019-10-05T17:31:59.0539903Z 
2019-10-05T17:31:59.0545283Z error: `extern` block uses type `str`, which is not FFI-safe
2019-10-05T17:31:59.0546247Z    --> src/librustc_codegen_llvm/back/write.rs:243:45
2019-10-05T17:31:59.0546869Z     |
2019-10-05T17:31:59.0547523Z 243 | ...                   msg: &str,
2019-10-05T17:31:59.0548264Z     |                            ^^^^ not FFI-safe
2019-10-05T17:31:59.0549405Z     |
2019-10-05T17:31:59.0550666Z     = help: consider using `*const u8` and a length instead
2019-10-05T17:31:59.0552394Z     = note: string slices have no C equivalent
2019-10-05T17:31:59.0552685Z 
2019-10-05T17:31:59.0726363Z error: `extern` block uses type `std::cell::RefCell<std::vec::Vec<u8>>`, which is not FFI-safe
2019-10-05T17:31:59.0727711Z   --> src/librustc_codegen_llvm/llvm/mod.rs:91:54
2019-10-05T17:31:59.0728190Z    |
2019-10-05T17:31:59.0728697Z 91 | pub unsafe extern "C" fn LLVMRustStringWriteImpl(sr: &RustString,
2019-10-05T17:31:59.0729286Z    |                                                      ^^^^^^^^^^^ not FFI-safe
2019-10-05T17:31:59.0731002Z    = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
2019-10-05T17:31:59.0731655Z    = note: this struct has unspecified layout
2019-10-05T17:31:59.0731874Z 
2019-10-05T17:31:59.0731874Z 
2019-10-05T17:31:59.0840611Z error: internal compiler error: src/librustc/hir/map/mod.rs:932: expected item, found foreign item llvm_::ffi::LLVMCreatePassManager (hir_id=HirId { owner: DefIndex(2104), local_id: 3302 })
2019-10-05T17:31:59.0841773Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:915:9
2019-10-05T17:31:59.0847206Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-05T17:31:59.0847645Z 
2019-10-05T17:31:59.0851989Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-05T17:31:59.0851989Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-05T17:31:59.0852226Z 
2019-10-05T17:31:59.0856557Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-05T17:31:59.0863204Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-05T17:31:59.0868693Z 
2019-10-05T17:31:59.0868693Z 
2019-10-05T17:31:59.0870072Z note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type dylib
2019-10-05T17:31:59.0879201Z note: some of the compiler flags provided by cargo are hidden
2019-10-05T17:31:59.0879408Z 
2019-10-05T17:31:59.1919067Z error: aborting due to 4 previous errors
2019-10-05T17:31:59.1919209Z 
---
2019-10-05T17:31:59.2288171Z == clock drift check ==
2019-10-05T17:31:59.2306566Z   local time: Sat Oct  5 17:31:59 UTC 2019
2019-10-05T17:32:00.8970549Z   network time: Sat, 05 Oct 2019 17:32:00 GMT
2019-10-05T17:32:00.8970896Z == end clock drift check ==
2019-10-05T17:32:03.1646027Z ##[error]Bash exited with code '1'.
2019-10-05T17:32:03.1700963Z ##[section]Starting: Checkout
2019-10-05T17:32:03.1703516Z ==============================================================================
2019-10-05T17:32:03.1703604Z Task         : Get sources
2019-10-05T17:32:03.1703673Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
