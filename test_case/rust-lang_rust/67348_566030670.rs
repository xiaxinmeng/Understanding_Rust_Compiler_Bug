
$ RUST_BACKTRACE=full  ./x.py build
Updating only changed submodules
Submodules updated in 0.02 seconds
    Finished dev [unoptimized] target(s) in 0.06s
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.13s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: In file included from ../rustllvm/rustllvm.h:4,
warning:                  from ../rustllvm/PassWrapper.cpp:6:
warning: /w/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h: In instantiation of ‘llvm::ArrayRef<T>::ArrayRef(const std::initializer_list<_Tp>&) [with T = long unsigned int]’:
warning: /w/rust/src/llvm-project/llvm/include/llvm/IR/DIBuilder.h:644:74:   required from here
warning: /w/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h:101:37: warning: initializing ‘llvm::ArrayRef<long unsigned int>::Data’ from ‘std::initializer_list<long unsigned int>::begin’ does not extend the lifetime of the underlying array [-Winit-list-lifetime]
warning:   101 |     : Data(Vec.begin() == Vec.end() ? (T*)nullptr : Vec.begin()),
warning: In file included from ../rustllvm/rustllvm.h:4,
warning:                  from ../rustllvm/RustWrapper.cpp:1:
warning: /w/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h: In instantiation of ‘llvm::ArrayRef<T>::ArrayRef(const std::initializer_list<_Tp>&) [with T = long unsigned int]’:
warning: /w/rust/src/llvm-project/llvm/include/llvm/IR/DIBuilder.h:644:74:   required from here
warning: /w/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h:101:37: warning: initializing ‘llvm::ArrayRef<long unsigned int>::Data’ from ‘std::initializer_list<long unsigned int>::begin’ does not extend the lifetime of the underlying array [-Winit-list-lifetime]
warning:   101 |     : Data(Vec.begin() == Vec.end() ? (T*)nullptr : Vec.begin()),
warning: In file included from ../rustllvm/rustllvm.h:4,
warning:                  from ../rustllvm/ArchiveWrapper.cpp:1:
warning: /w/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h: In instantiation of ‘llvm::ArrayRef<T>::ArrayRef(const std::initializer_list<_Tp>&) [with T = long unsigned int]’:
warning: /w/rust/src/llvm-project/llvm/include/llvm/IR/DIBuilder.h:644:74:   required from here
warning: /w/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h:101:37: warning: initializing ‘llvm::ArrayRef<long unsigned int>::Data’ from ‘std::initializer_list<long unsigned int>::begin’ does not extend the lifetime of the underlying array [-Winit-list-lifetime]
warning:   101 |     : Data(Vec.begin() == Vec.end() ? (T*)nullptr : Vec.begin()),
warning: In file included from /w/rust/src/llvm-project/llvm/include/llvm/Linker/IRMover.h:12,
warning:                  from /w/rust/src/llvm-project/llvm/include/llvm/Linker/Linker.h:13,
warning:                  from ../rustllvm/Linker.cpp:1:
warning: /w/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h: In instantiation of ‘llvm::ArrayRef<T>::ArrayRef(const std::initializer_list<_Tp>&) [with T = long unsigned int]’:
warning: /w/rust/src/llvm-project/llvm/include/llvm/IR/DIBuilder.h:644:74:   required from here
warning: /w/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h:101:37: warning: initializing ‘llvm::ArrayRef<long unsigned int>::Data’ from ‘std::initializer_list<long unsigned int>::begin’ does not extend the lifetime of the underlying array [-Winit-list-lifetime]
warning:   101 |     : Data(Vec.begin() == Vec.end() ? (T*)nullptr : Vec.begin()),
    Finished release [optimized] target(s) in 0.13s
Installing libLLVM.so to stage 0 (x86_64-unknown-linux-gnu)
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.13s
Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: In file included from ../rustllvm/rustllvm.h:4,
warning:                  from ../rustllvm/PassWrapper.cpp:6:
warning: /w/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h: In instantiation of ‘llvm::ArrayRef<T>::ArrayRef(const std::initializer_list<_Tp>&) [with T = long unsigned int]’:
warning: /w/rust/src/llvm-project/llvm/include/llvm/IR/DIBuilder.h:644:74:   required from here
warning: /w/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h:101:37: warning: initializing ‘llvm::ArrayRef<long unsigned int>::Data’ from ‘std::initializer_list<long unsigned int>::begin’ does not extend the lifetime of the underlying array [-Winit-list-lifetime]
warning:   101 |     : Data(Vec.begin() == Vec.end() ? (T*)nullptr : Vec.begin()),
warning: In file included from ../rustllvm/rustllvm.h:4,
warning:                  from ../rustllvm/RustWrapper.cpp:1:
warning: /w/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h: In instantiation of ‘llvm::ArrayRef<T>::ArrayRef(const std::initializer_list<_Tp>&) [with T = long unsigned int]’:
warning: /w/rust/src/llvm-project/llvm/include/llvm/IR/DIBuilder.h:644:74:   required from here
warning: /w/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h:101:37: warning: initializing ‘llvm::ArrayRef<long unsigned int>::Data’ from ‘std::initializer_list<long unsigned int>::begin’ does not extend the lifetime of the underlying array [-Winit-list-lifetime]
warning:   101 |     : Data(Vec.begin() == Vec.end() ? (T*)nullptr : Vec.begin()),
warning: In file included from ../rustllvm/rustllvm.h:4,
warning:                  from ../rustllvm/ArchiveWrapper.cpp:1:
warning: /w/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h: In instantiation of ‘llvm::ArrayRef<T>::ArrayRef(const std::initializer_list<_Tp>&) [with T = long unsigned int]’:
warning: /w/rust/src/llvm-project/llvm/include/llvm/IR/DIBuilder.h:644:74:   required from here
warning: /w/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h:101:37: warning: initializing ‘llvm::ArrayRef<long unsigned int>::Data’ from ‘std::initializer_list<long unsigned int>::begin’ does not extend the lifetime of the underlying array [-Winit-list-lifetime]
warning:   101 |     : Data(Vec.begin() == Vec.end() ? (T*)nullptr : Vec.begin()),
warning: In file included from /w/rust/src/llvm-project/llvm/include/llvm/Linker/IRMover.h:12,
warning:                  from /w/rust/src/llvm-project/llvm/include/llvm/Linker/Linker.h:13,
warning:                  from ../rustllvm/Linker.cpp:1:
warning: /w/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h: In instantiation of ‘llvm::ArrayRef<T>::ArrayRef(const std::initializer_list<_Tp>&) [with T = long unsigned int]’:
warning: /w/rust/src/llvm-project/llvm/include/llvm/IR/DIBuilder.h:644:74:   required from here
warning: /w/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h:101:37: warning: initializing ‘llvm::ArrayRef<long unsigned int>::Data’ from ‘std::initializer_list<long unsigned int>::begin’ does not extend the lifetime of the underlying array [-Winit-list-lifetime]
warning:   101 |     : Data(Vec.begin() == Vec.end() ? (T*)nullptr : Vec.begin()),
   Compiling rustc_errors v0.0.0 (/w/rust/src/librustc_errors)
   Compiling rustc_feature v0.0.0 (/w/rust/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/w/rust/src/libfmt_macros)
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: "read_option: expected 0 for None or 1 for Some"', src/libcore/macros/mod.rs:23:13
stack backtrace:
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: "read_option: expected 0 for None or 1 for Some"', src/libcore/macros/mod.rs:23:13
stack backtrace:
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: "read_option: expected 0 for None or 1 for Some"', src/libcore/macros/mod.rs:23:13
stack backtrace:
   0:     0x7f3fe56b6fc4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h88dc5c093d40d2e6
   1:     0x7f3fe56e532d - core::fmt::write::hc556d781e39e5bb0
   2:     0x7f3fe56b6a57 - std::io::Write::write_fmt::he5aa9dc64aea0f7b
   3:     0x7f3fe56889fe - std::panicking::default_hook::{{closure}}::hb2da15cc31fdb219
   4:     0x7f3fe5688710 - std::panicking::default_hook::h5a5ead8ed092f78f
   5:     0x7f3fe5d119c3 - rustc_driver::report_ice::h38b41ab231c7c12c
   6:     0x7f3fe5689270 - std::panicking::rust_panic_with_hook::h6874967c7a7eccee
   7:     0x7f3fe5688d4e - rust_begin_unwind
   8:     0x7f3fe56ea7de - core::panicking::panic_fmt::hfa45f9ae4c778937
   9:     0x7f3fe56e3cf7 - core::result::unwrap_failed::h77bb7222c6dd1a62
  10:     0x7f3fe956c21b - rustc_metadata::rmeta::decoder::<impl rustc_metadata::rmeta::Lazy<T>>::decode::h7f3ed0e1e019b72f
  11:     0x7f3fe963932b - rustc_metadata::rmeta::decoder::MetadataBlob::get_root::h9b81ec3d7ebf42ab
  12:     0x7f3fe9514e90 - rustc_metadata::locator::CrateLocator::extract_one::hb309dfaec40caea4
  13:     0x7f3fe9514302 - rustc_metadata::locator::CrateLocator::extract_lib::hd95ce92cf254056e
  14:     0x7f3fe95137ba - rustc_metadata::locator::CrateLocator::find_library_crate::h4dc4a5eb01bb0ed9
  15:     0x7f3fe9511520 - rustc_metadata::locator::CrateLocator::maybe_load_library_crate::h6332317e6c4cfeea
  16:     0x7f3fe952b93a - rustc_metadata::creader::CrateLoader::load::hc72bd280e8ffda7a
  17:     0x7f3fe95292fd - rustc_metadata::creader::CrateLoader::maybe_resolve_crate::h59d2db93e01e4101
  18:     0x7f3fe96912ee - <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::hc13b74ae05a2a1d3
  19:     0x7f3fe9657e44 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter::hca74828c4f044247
  20:     0x7f3fe952a749 - rustc_metadata::creader::CrateLoader::maybe_resolve_crate::h59d2db93e01e4101
  21:     0x7f3fe952e3e3 - rustc_metadata::creader::CrateLoader::maybe_process_path_extern::h89acb10afb588aaa
  22:     0x7f3fe87eeed5 - rustc_resolve::Resolver::extern_prelude_get::h6cfda1ec26e128fb
  23:     0x7f3fe87d1506 - rustc_resolve::macros::<impl rustc_resolve::Resolver>::early_resolve_ident_in_lexical_scope::hb3791b591a1be724
 24:     0x7f3fe87e968e - rustc_resolve::Resolver::resolve_path_with_ribs::h755703ad6ae2cf23
  25:     0x7f3fe878470d - rustc_resolve::resolve_imports::ImportResolver::resolve_imports::h061eea07e50273ff
  26:     0x7f3fe87cdf2e - rustc_resolve::macros::<impl syntax_expand::base::Resolver for rustc_resolve::Resolver>::resolve_imports::h01371a835e09adf2
  27:     0x7f3fe96fdc78 - syntax_expand::expand::MacroExpander::fully_expand_fragment::hbe63a95672f1bf00
  28:     0x7f3fe96fd5d3 - syntax_expand::expand::MacroExpander::expand_crate::h20e212031955041f
  29:     0x7f3fe5dfd24e - rustc_interface::passes::configure_and_expand_inner::{{closure}}::h29c8716a567e4b82
  30:     0x7f3fe5dfb503 - rustc_interface::passes::configure_and_expand_inner::h104589ff132bc2e8
  31:     0x7f3fe5e29b85 - rustc_interface::passes::configure_and_expand::{{closure}}::h3a6f97e0023ef2a2
  32:     0x7f3fe5e06712 - rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new::h093db3a7447fb716
  33:     0x7f3fe5df997d - rustc_interface::passes::configure_and_expand::h59a2c97cd705d3ce
  34:     0x7f3fe5ead302 - rustc_interface::queries::Queries::expansion::hbe1f5a0972521435
  35:     0x7f3fe5d7194a - rustc_interface::interface::run_compiler_in_existing_thread_pool::h0a94a658344077b8
  36:     0x7f3fe5d20452 - std::thread::local::LocalKey<T>::with::h46f0851e68859e24
  37:     0x7f3fe5d21dbe - scoped_tls::ScopedKey<T>::set::h3dfa7d458040e6d0
  38:     0x7f3fe5d1dc84 - syntax::with_globals::h8d0eee513238454f
  39:     0x7f3fe5d27bb0 - std::sys_common::backtrace::__rust_begin_short_backtrace::h63ce72cd7ae4664b
  40:     0x7f3fe56b7f7a - __rust_maybe_catch_panic
  41:     0x7f3fe5d45d89 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hbd0bfcd2a07ae417
  42:     0x7f3fe567d7ef - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hbdf0fe210237da86
  43:     0x7f3fe568d7c0 - std::sys_common::thread::start_thread::he35082e636cc571f
  44:     0x7f3fe56b45a6 - std::sys::unix::thread::Thread::new::thread_start::hba2926e74afffc5e
  45:     0x7f3fe536b669 - start_thread
  46:     0x7f3fe5542323 - clone
  47:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack

error: could not compile `fmt_macros`.
warning: build failed, waiting for other jobs to finish...
   0:     0x7f5286f9dfc4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h88dc5c093d40d2e6
   1:     0x7f5286fcc32d - core::fmt::write::hc556d781e39e5bb0
   2:     0x7f5286f9da57 - std::io::Write::write_fmt::he5aa9dc64aea0f7b
   3:     0x7f5286f6f9fe - std::panicking::default_hook::{{closure}}::hb2da15cc31fdb219
   4:     0x7f5286f6f710 - std::panicking::default_hook::h5a5ead8ed092f78f
   5:     0x7f52875f89c3 - rustc_driver::report_ice::h38b41ab231c7c12c
   6:     0x7f5286f70270 - std::panicking::rust_panic_with_hook::h6874967c7a7eccee
   7:     0x7f5286f6fd4e - rust_begin_unwind
   8:     0x7f5286fd17de - core::panicking::panic_fmt::hfa45f9ae4c778937
   9:     0x7f5286fcacf7 - core::result::unwrap_failed::h77bb7222c6dd1a62
  10:     0x7f528ae5321b - rustc_metadata::rmeta::decoder::<impl rustc_metadata::rmeta::Lazy<T>>::decode::h7f3ed0e1e019b72f
  11:     0x7f528af2032b - rustc_metadata::rmeta::decoder::MetadataBlob::get_root::h9b81ec3d7ebf42ab
  12:     0x7f528adfbe90 - rustc_metadata::locator::CrateLocator::extract_one::hb309dfaec40caea4
  13:     0x7f528adfb302 - rustc_metadata::locator::CrateLocator::extract_lib::hd95ce92cf254056e
  14:     0x7f528adfa7ba - rustc_metadata::locator::CrateLocator::find_library_crate::h4dc4a5eb01bb0ed9
  15:     0x7f528adf8520 - rustc_metadata::locator::CrateLocator::maybe_load_library_crate::h6332317e6c4cfeea
  16:     0x7f528ae1293a - rustc_metadata::creader::CrateLoader::load::hc72bd280e8ffda7a
  17:     0x7f528ae102fd - rustc_metadata::creader::CrateLoader::maybe_resolve_crate::h59d2db93e01e4101
  18:     0x7f528af782ee - <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::hc13b74ae05a2a1d3
  19:     0x7f528af3ee44 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter::hca74828c4f044247
  20:     0x7f528ae11749 - rustc_metadata::creader::CrateLoader::maybe_resolve_crate::h59d2db93e01e4101
  21:     0x7f528ae153e3 - rustc_metadata::creader::CrateLoader::maybe_process_path_extern::h89acb10afb588aaa
  22:     0x7f528a0d5ed5 - rustc_resolve::Resolver::extern_prelude_get::h6cfda1ec26e128fb
  23:     0x7f528a0b8506 - rustc_resolve::macros::<impl rustc_resolve::Resolver>::early_resolve_ident_in_lexical_scope::hb3791b591a1be724
  24:     0x7f528a0d068e - rustc_resolve::Resolver::resolve_path_with_ribs::h755703ad6ae2cf23
  25:     0x7f528a06b70d - rustc_resolve::resolve_imports::ImportResolver::resolve_imports::h061eea07e50273ff
  26:     0x7f528a0b4f2e - rustc_resolve::macros::<impl syntax_expand::base::Resolver for rustc_resolve::Resolver>::resolve_imports::h01371a835e09adf2
  27:     0x7f528afe4c78 - syntax_expand::expand::MacroExpander::fully_expand_fragment::hbe63a95672f1bf00
  28:     0x7f528afe45d3 - syntax_expand::expand::MacroExpander::expand_crate::h20e212031955041f
  29:     0x7f52876e424e - rustc_interface::passes::configure_and_expand_inner::{{closure}}::h29c8716a567e4b82
  30:     0x7f52876e2503 - rustc_interface::passes::configure_and_expand_inner::h104589ff132bc2e8
  31:     0x7f5287710b85 - rustc_interface::passes::configure_and_expand::{{closure}}::h3a6f97e0023ef2a2
  32:     0x7f52876ed712 - rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new::h093db3a7447fb716
  33:     0x7f52876e097d - rustc_interface::passes::configure_and_expand::h59a2c97cd705d3ce
  34:     0x7f5287794302 - rustc_interface::queries::Queries::expansion::hbe1f5a0972521435
  35:     0x7f528765894a - rustc_interface::interface::run_compiler_in_existing_thread_pool::h0a94a658344077b8
  36:     0x7f5287607452 - std::thread::local::LocalKey<T>::with::h46f0851e68859e24
  37:     0x7f5287608dbe - scoped_tls::ScopedKey<T>::set::h3dfa7d458040e6d0
  38:     0x7f5287604c84 - syntax::with_globals::h8d0eee513238454f
  39:     0x7f528760ebb0 - std::sys_common::backtrace::__rust_begin_short_backtrace::h63ce72cd7ae4664b
  40:     0x7f5286f9ef7a - __rust_maybe_catch_panic
  41:     0x7f528762cd89 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hbd0bfcd2a07ae417
  42:     0x7f5286f647ef - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hbdf0fe210237da86
  43:     0x7f5286f747c0 - std::sys_common::thread::start_thread::he35082e636cc571f
  44:     0x7f5286f9b5a6 - std::sys::unix::thread::Thread::new::thread_start::hba2926e74afffc5e
  45:     0x7f5286c52669 - start_thread
  46:     0x7f5286e29323 - clone
  47:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
   0:     0x7f0575eb7fc4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h88dc5c093d40d2e6
   1:     0x7f0575ee632d - core::fmt::write::hc556d781e39e5bb0
   2:     0x7f0575eb7a57 - std::io::Write::write_fmt::he5aa9dc64aea0f7b
   3:     0x7f0575e899fe - std::panicking::default_hook::{{closure}}::hb2da15cc31fdb219
   4:     0x7f0575e89710 - std::panicking::default_hook::h5a5ead8ed092f78f
   5:     0x7f05765129c3 - rustc_driver::report_ice::h38b41ab231c7c12c
   6:     0x7f0575e8a270 - std::panicking::rust_panic_with_hook::h6874967c7a7eccee
   7:     0x7f0575e89d4e - rust_begin_unwind
   8:     0x7f0575eeb7de - core::panicking::panic_fmt::hfa45f9ae4c778937
   9:     0x7f0575ee4cf7 - core::result::unwrap_failed::h77bb7222c6dd1a62
  10:     0x7f0579d6d21b - rustc_metadata::rmeta::decoder::<impl rustc_metadata::rmeta::Lazy<T>>::decode::h7f3ed0e1e019b72f
  11:     0x7f0579e3a32b - rustc_metadata::rmeta::decoder::MetadataBlob::get_root::h9b81ec3d7ebf42ab
  12:     0x7f0579d15e90 - rustc_metadata::locator::CrateLocator::extract_one::hb309dfaec40caea4
  13:     0x7f0579d15302 - rustc_metadata::locator::CrateLocator::extract_lib::hd95ce92cf254056e
  14:     0x7f0579d147ba - rustc_metadata::locator::CrateLocator::find_library_crate::h4dc4a5eb01bb0ed9
  15:     0x7f0579d12520 - rustc_metadata::locator::CrateLocator::maybe_load_library_crate::h6332317e6c4cfeea
  16:     0x7f0579d2c93a - rustc_metadata::creader::CrateLoader::load::hc72bd280e8ffda7a
  17:     0x7f0579d2a2fd - rustc_metadata::creader::CrateLoader::maybe_resolve_crate::h59d2db93e01e4101
  18:     0x7f0579e922ee - <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::hc13b74ae05a2a1d3
  19:     0x7f0579e58e44 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter::hca74828c4f044247
  20:     0x7f0579d2b749 - rustc_metadata::creader::CrateLoader::maybe_resolve_crate::h59d2db93e01e4101
  21:     0x7f0579d2f3e3 - rustc_metadata::creader::CrateLoader::maybe_process_path_extern::h89acb10afb588aaa
  22:     0x7f0578fefed5 - rustc_resolve::Resolver::extern_prelude_get::h6cfda1ec26e128fb
  23:     0x7f0578fd2506 - rustc_resolve::macros::<impl rustc_resolve::Resolver>::early_resolve_ident_in_lexical_scope::hb3791b591a1be724
  24:     0x7f0578fea68e - rustc_resolve::Resolver::resolve_path_with_ribs::h755703ad6ae2cf23
  25:     0x7f0578f8570d - rustc_resolve::resolve_imports::ImportResolver::resolve_imports::h061eea07e50273ff
  26:     0x7f0578fcef2e - rustc_resolve::macros::<impl syntax_expand::base::Resolver for rustc_resolve::Resolver>::resolve_imports::h01371a835e09adf2
  27:     0x7f0579efec78 - syntax_expand::expand::MacroExpander::fully_expand_fragment::hbe63a95672f1bf00
  28:     0x7f0579efe5d3 - syntax_expand::expand::MacroExpander::expand_crate::h20e212031955041f
  29:     0x7f05765fe24e - rustc_interface::passes::configure_and_expand_inner::{{closure}}::h29c8716a567e4b82
  30:     0x7f05765fc503 - rustc_interface::passes::configure_and_expand_inner::h104589ff132bc2e8
  31:     0x7f057662ab85 - rustc_interface::passes::configure_and_expand::{{closure}}::h3a6f97e0023ef2a2
  32:     0x7f0576607712 - rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new::h093db3a7447fb716
  33:     0x7f05765fa97d - rustc_interface::passes::configure_and_expand::h59a2c97cd705d3ce
  34:     0x7f05766ae302 - rustc_interface::queries::Queries::expansion::hbe1f5a0972521435
  35:     0x7f057657294a - rustc_interface::interface::run_compiler_in_existing_thread_pool::h0a94a658344077b8
  36:     0x7f0576521452 - std::thread::local::LocalKey<T>::with::h46f0851e68859e24
  37:     0x7f0576522dbe - scoped_tls::ScopedKey<T>::set::h3dfa7d458040e6d0
  38:     0x7f057651ec84 - syntax::with_globals::h8d0eee513238454f
  39:     0x7f0576528bb0 - std::sys_common::backtrace::__rust_begin_short_backtrace::h63ce72cd7ae4664b
  40:     0x7f0575eb8f7a - __rust_maybe_catch_panic
  41:     0x7f0576546d89 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hbd0bfcd2a07ae417
  42:     0x7f0575e7e7ef - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hbdf0fe210237da86
  43:     0x7f0575e8e7c0 - std::sys_common::thread::start_thread::he35082e636cc571f
  44:     0x7f0575eb55a6 - std::sys::unix::thread::Thread::new::thread_start::hba2926e74afffc5e
  45:     0x7f0575b6c669 - start_thread
  46:     0x7f0575d43323 - clone
  47:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `rustc_errors`.
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_feature`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/w/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--features" " llvm" "--manifest-path" "/w/rust/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: /w/rust/build/bootstrap/debug/bootstrap build
Build completed unsuccessfully in 0:00:01
