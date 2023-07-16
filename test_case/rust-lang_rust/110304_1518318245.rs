plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
thread 'rustc' panicked at 'attempt to subtract with overflow', /cargo/registry/src/index.crates.io-6f17d22bba15001f/object-0.30.1/src/write/mod.rs:705:34
stack backtrace:
   0:     0x7fa325d7f1e1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h99d41b7989943530
   1:     0x7fa325de6b58 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7fa325d72de1 - std::io::Write::write_fmt::h0d88c7b0bf910f28
   3:     0x7fa325d7eff5 - std::sys_common::backtrace::print::h1940fb990ef00a99
   4:     0x7fa325d82154 - std::panicking::default_hook::{{closure}}::hc1eac4b73387c881
   5:     0x7fa325d81e42 - std::panicking::default_hook::h04915ebfc8268c91
   6:     0x7fa3267de315 - rustc_driver_impl[93cd919eb377a2e9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fa325d82889 - std::panicking::rust_panic_with_hook::h75e7ea4c0112022c
   8:     0x7fa325d825c2 - std::panicking::begin_panic_handler::{{closure}}::he8caf66ef9f93dc4
   9:     0x7fa325d7f696 - std::sys_common::backtrace::__rust_end_short_backtrace::h18c6d72531229c72
  10:     0x7fa325d822d2 - rust_begin_unwind
  11:     0x7fa325d39f43 - core::panicking::panic_fmt::hb12531409cdb4ed6
  12:     0x7fa325d39fdd - core::panicking::panic::hc61b82565d1e8446
  13:     0x7fa328866ec0 - <object[196f51113b2a81d0]::write::Section>::append_data
  14:     0x7fa328604b91 - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::metadata::create_object_file
  15:     0x7fa328604d3c - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::metadata::create_wrapper_file
  16:     0x7fa3285688ca - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::link::link_rlib
  17:     0x7fa32856777e - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::link::link_binary
  18:     0x7fa326a23094 - <rustc_codegen_llvm[7640c7ee29cbb401]::LlvmCodegenBackend as rustc_codegen_ssa[a227ecdef4f6f8fb]::traits::backend::CodegenBackend>::link
  19:     0x7fa32695e18b - <rustc_interface[ce4c417bbf8a4c17]::queries::Linker>::link
  20:     0x7fa32684e0dc - rustc_span[de5c6ef7fb663679]::set_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  21:     0x7fa3267f085a - std[9c6ad123400df44d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>
  22:     0x7fa3268537e8 - std[9c6ad123400df44d]::panicking::try::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, core[54ab13d2a06817e1]::panic::unwind_safe::AssertUnwindSafe<<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x7fa3267f1a16 - <<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7fa325d8eace - std::sys::unix::thread::Thread::new::thread_start::h0ee55f1c660c8984
  25:     0x7fa325b2bb43 - <unknown>
  26:     0x7fa325bbda00 - <unknown>
  27:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (5cce518ce 2023-04-21) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `rustc-std-workspace-core`
warning: build failed, waiting for other jobs to finish...
thread 'rustc' panicked at 'attempt to subtract with overflow', /cargo/registry/src/index.crates.io-6f17d22bba15001f/object-0.30.1/src/write/mod.rs:705:34
stack backtrace:
   0:     0x7f5af231a1e1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h99d41b7989943530
   1:     0x7f5af2381b58 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7f5af230dde1 - std::io::Write::write_fmt::h0d88c7b0bf910f28
   3:     0x7f5af2319ff5 - std::sys_common::backtrace::print::h1940fb990ef00a99
   4:     0x7f5af231d154 - std::panicking::default_hook::{{closure}}::hc1eac4b73387c881
   5:     0x7f5af231ce42 - std::panicking::default_hook::h04915ebfc8268c91
   6:     0x7f5af2d79315 - rustc_driver_impl[93cd919eb377a2e9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f5af231d889 - std::panicking::rust_panic_with_hook::h75e7ea4c0112022c
   8:     0x7f5af231d5c2 - std::panicking::begin_panic_handler::{{closure}}::he8caf66ef9f93dc4
   9:     0x7f5af231a696 - std::sys_common::backtrace::__rust_end_short_backtrace::h18c6d72531229c72
  10:     0x7f5af231d2d2 - rust_begin_unwind
  11:     0x7f5af22d4f43 - core::panicking::panic_fmt::hb12531409cdb4ed6
  12:     0x7f5af22d4fdd - core::panicking::panic::hc61b82565d1e8446
  13:     0x7f5af4e01ec0 - <object[196f51113b2a81d0]::write::Section>::append_data
  14:     0x7f5af4b9fb91 - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::metadata::create_object_file
  15:     0x7f5af4b9fd3c - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::metadata::create_wrapper_file
  16:     0x7f5af4b038ca - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::link::link_rlib
  17:     0x7f5af4b0277e - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::link::link_binary
  18:     0x7f5af2fbe094 - <rustc_codegen_llvm[7640c7ee29cbb401]::LlvmCodegenBackend as rustc_codegen_ssa[a227ecdef4f6f8fb]::traits::backend::CodegenBackend>::link
  19:     0x7f5af2ef918b - <rustc_interface[ce4c417bbf8a4c17]::queries::Linker>::link
  20:     0x7f5af2de90dc - rustc_span[de5c6ef7fb663679]::set_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  21:     0x7f5af2d8b85a - std[9c6ad123400df44d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>
  22:     0x7f5af2dee7e8 - std[9c6ad123400df44d]::panicking::try::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, core[54ab13d2a06817e1]::panic::unwind_safe::AssertUnwindSafe<<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x7f5af2d8ca16 - <<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7f5af2329ace - std::sys::unix::thread::Thread::new::thread_start::h0ee55f1c660c8984
  25:     0x7f5af20c6b43 - <unknown>
  26:     0x7f5af2158a00 - <unknown>
  27:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (5cce518ce 2023-04-21) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=10000 -C debuginfo=0 -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `compiler_builtins`
thread 'rustc' panicked at 'attempt to subtract with overflow', /cargo/registry/src/index.crates.io-6f17d22bba15001f/object-0.30.1/src/write/mod.rs:705:34
stack backtrace:
   0:     0x7f4a4ed141e1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h99d41b7989943530
   1:     0x7f4a4ed7bb58 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7f4a4ed07de1 - std::io::Write::write_fmt::h0d88c7b0bf910f28
   3:     0x7f4a4ed13ff5 - std::sys_common::backtrace::print::h1940fb990ef00a99
   4:     0x7f4a4ed17154 - std::panicking::default_hook::{{closure}}::hc1eac4b73387c881
   5:     0x7f4a4ed16e42 - std::panicking::default_hook::h04915ebfc8268c91
   6:     0x7f4a4f773315 - rustc_driver_impl[93cd919eb377a2e9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f4a4ed17889 - std::panicking::rust_panic_with_hook::h75e7ea4c0112022c
   8:     0x7f4a4ed175c2 - std::panicking::begin_panic_handler::{{closure}}::he8caf66ef9f93dc4
   9:     0x7f4a4ed14696 - std::sys_common::backtrace::__rust_end_short_backtrace::h18c6d72531229c72
  10:     0x7f4a4ed172d2 - rust_begin_unwind
  11:     0x7f4a4eccef43 - core::panicking::panic_fmt::hb12531409cdb4ed6
  12:     0x7f4a4eccefdd - core::panicking::panic::hc61b82565d1e8446
  13:     0x7f4a517fbec0 - <object[196f51113b2a81d0]::write::Section>::append_data
  14:     0x7f4a51599b91 - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::metadata::create_object_file
  15:     0x7f4a51599d3c - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::metadata::create_wrapper_file
  16:     0x7f4a514fd8ca - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::link::link_rlib
  17:     0x7f4a514fc77e - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::link::link_binary
  18:     0x7f4a4f9b8094 - <rustc_codegen_llvm[7640c7ee29cbb401]::LlvmCodegenBackend as rustc_codegen_ssa[a227ecdef4f6f8fb]::traits::backend::CodegenBackend>::link
  19:     0x7f4a4f8f318b - <rustc_interface[ce4c417bbf8a4c17]::queries::Linker>::link
  20:     0x7f4a4f7e30dc - rustc_span[de5c6ef7fb663679]::set_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  21:     0x7f4a4f78585a - std[9c6ad123400df44d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>
  22:     0x7f4a4f7e87e8 - std[9c6ad123400df44d]::panicking::try::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, core[54ab13d2a06817e1]::panic::unwind_safe::AssertUnwindSafe<<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x7f4a4f786a16 - <<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7f4a4ed23ace - std::sys::unix::thread::Thread::new::thread_start::h0ee55f1c660c8984
  25:     0x7f4a4eac0b43 - <unknown>
  26:     0x7f4a4eb52a00 - <unknown>
  27:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (5cce518ce 2023-04-21) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z unstable-options -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `libc`
thread 'rustc' panicked at 'attempt to subtract with overflow', /cargo/registry/src/index.crates.io-6f17d22bba15001f/object-0.30.1/src/write/mod.rs:705:34
stack backtrace:
   0:     0x7f0daf98a1e1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h99d41b7989943530
   1:     0x7f0daf9f1b58 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7f0daf97dde1 - std::io::Write::write_fmt::h0d88c7b0bf910f28
   3:     0x7f0daf989ff5 - std::sys_common::backtrace::print::h1940fb990ef00a99
   4:     0x7f0daf98d154 - std::panicking::default_hook::{{closure}}::hc1eac4b73387c881
   5:     0x7f0daf98ce42 - std::panicking::default_hook::h04915ebfc8268c91
   6:     0x7f0db03e9315 - rustc_driver_impl[93cd919eb377a2e9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f0daf98d889 - std::panicking::rust_panic_with_hook::h75e7ea4c0112022c
   8:     0x7f0daf98d5c2 - std::panicking::begin_panic_handler::{{closure}}::he8caf66ef9f93dc4
   9:     0x7f0daf98a696 - std::sys_common::backtrace::__rust_end_short_backtrace::h18c6d72531229c72
  10:     0x7f0daf98d2d2 - rust_begin_unwind
  11:     0x7f0daf944f43 - core::panicking::panic_fmt::hb12531409cdb4ed6
  12:     0x7f0daf944fdd - core::panicking::panic::hc61b82565d1e8446
  13:     0x7f0db2471ec0 - <object[196f51113b2a81d0]::write::Section>::append_data
  14:     0x7f0db220fb91 - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::metadata::create_object_file
  15:     0x7f0db220fd3c - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::metadata::create_wrapper_file
  16:     0x7f0db21738ca - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::link::link_rlib
  17:     0x7f0db217277e - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::link::link_binary
  18:     0x7f0db062e094 - <rustc_codegen_llvm[7640c7ee29cbb401]::LlvmCodegenBackend as rustc_codegen_ssa[a227ecdef4f6f8fb]::traits::backend::CodegenBackend>::link
  19:     0x7f0db056918b - <rustc_interface[ce4c417bbf8a4c17]::queries::Linker>::link
  20:     0x7f0db04590dc - rustc_span[de5c6ef7fb663679]::set_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  21:     0x7f0db03fb85a - std[9c6ad123400df44d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>
  22:     0x7f0db045e7e8 - std[9c6ad123400df44d]::panicking::try::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, core[54ab13d2a06817e1]::panic::unwind_safe::AssertUnwindSafe<<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x7f0db03fca16 - <<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7f0daf999ace - std::sys::unix::thread::Thread::new::thread_start::h0ee55f1c660c8984
  25:     0x7f0daf736b43 - <unknown>
  26:     0x7f0daf7c8a00 - <unknown>
  27:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (5cce518ce 2023-04-21) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
