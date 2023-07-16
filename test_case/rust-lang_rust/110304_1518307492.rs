plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
thread 'rustc' panicked at 'attempt to subtract with overflow', /cargo/registry/src/index.crates.io-6f17d22bba15001f/object-0.30.1/src/write/mod.rs:705:34
stack backtrace:
   0:     0x7f47ee8241e1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h99d41b7989943530
   1:     0x7f47ee88bb58 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7f47ee817de1 - std::io::Write::write_fmt::h0d88c7b0bf910f28
   3:     0x7f47ee823ff5 - std::sys_common::backtrace::print::h1940fb990ef00a99
   4:     0x7f47ee827154 - std::panicking::default_hook::{{closure}}::hc1eac4b73387c881
   5:     0x7f47ee826e42 - std::panicking::default_hook::h04915ebfc8268c91
   6:     0x7f47ef283315 - rustc_driver_impl[93cd919eb377a2e9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f47ee827889 - std::panicking::rust_panic_with_hook::h75e7ea4c0112022c
   8:     0x7f47ee8275c2 - std::panicking::begin_panic_handler::{{closure}}::he8caf66ef9f93dc4
   9:     0x7f47ee824696 - std::sys_common::backtrace::__rust_end_short_backtrace::h18c6d72531229c72
  10:     0x7f47ee8272d2 - rust_begin_unwind
  11:     0x7f47ee7def43 - core::panicking::panic_fmt::hb12531409cdb4ed6
  12:     0x7f47ee7defdd - core::panicking::panic::hc61b82565d1e8446
  13:     0x7f47f130bec0 - <object[196f51113b2a81d0]::write::Section>::append_data
  14:     0x7f47f10a9b91 - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::metadata::create_object_file
  15:     0x7f47f10a9d3c - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::metadata::create_wrapper_file
  16:     0x7f47f100d8ca - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::link::link_rlib
  17:     0x7f47f100c77e - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::link::link_binary
  18:     0x7f47ef4c8094 - <rustc_codegen_llvm[7640c7ee29cbb401]::LlvmCodegenBackend as rustc_codegen_ssa[a227ecdef4f6f8fb]::traits::backend::CodegenBackend>::link
  19:     0x7f47ef40318b - <rustc_interface[ce4c417bbf8a4c17]::queries::Linker>::link
  20:     0x7f47ef2f30dc - rustc_span[de5c6ef7fb663679]::set_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  21:     0x7f47ef29585a - std[9c6ad123400df44d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>
  22:     0x7f47ef2f87e8 - std[9c6ad123400df44d]::panicking::try::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, core[54ab13d2a06817e1]::panic::unwind_safe::AssertUnwindSafe<<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x7f47ef296a16 - <<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7f47ee833ace - std::sys::unix::thread::Thread::new::thread_start::h0ee55f1c660c8984
  25:     0x7f47ee5d0b43 - <unknown>
  26:     0x7f47ee662a00 - <unknown>
  27:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (c330fa7e5 2023-04-21) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `rustc-std-workspace-core`
warning: build failed, waiting for other jobs to finish...
thread 'rustc' panicked at 'attempt to subtract with overflow', /cargo/registry/src/index.crates.io-6f17d22bba15001f/object-0.30.1/src/write/mod.rs:705:34
stack backtrace:
   0:     0x7efd3fcfc1e1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h99d41b7989943530
   1:     0x7efd3fd63b58 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7efd3fcefde1 - std::io::Write::write_fmt::h0d88c7b0bf910f28
   3:     0x7efd3fcfbff5 - std::sys_common::backtrace::print::h1940fb990ef00a99
   4:     0x7efd3fcff154 - std::panicking::default_hook::{{closure}}::hc1eac4b73387c881
   5:     0x7efd3fcfee42 - std::panicking::default_hook::h04915ebfc8268c91
   6:     0x7efd4075b315 - rustc_driver_impl[93cd919eb377a2e9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7efd3fcff889 - std::panicking::rust_panic_with_hook::h75e7ea4c0112022c
   8:     0x7efd3fcff5c2 - std::panicking::begin_panic_handler::{{closure}}::he8caf66ef9f93dc4
   9:     0x7efd3fcfc696 - std::sys_common::backtrace::__rust_end_short_backtrace::h18c6d72531229c72
  10:     0x7efd3fcff2d2 - rust_begin_unwind
  11:     0x7efd3fcb6f43 - core::panicking::panic_fmt::hb12531409cdb4ed6
  12:     0x7efd3fcb6fdd - core::panicking::panic::hc61b82565d1e8446
  13:     0x7efd427e3ec0 - <object[196f51113b2a81d0]::write::Section>::append_data
  14:     0x7efd42581b91 - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::metadata::create_object_file
  15:     0x7efd42581d3c - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::metadata::create_wrapper_file
  16:     0x7efd424e58ca - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::link::link_rlib
  17:     0x7efd424e477e - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::link::link_binary
  18:     0x7efd409a0094 - <rustc_codegen_llvm[7640c7ee29cbb401]::LlvmCodegenBackend as rustc_codegen_ssa[a227ecdef4f6f8fb]::traits::backend::CodegenBackend>::link
  19:     0x7efd408db18b - <rustc_interface[ce4c417bbf8a4c17]::queries::Linker>::link
  20:     0x7efd407cb0dc - rustc_span[de5c6ef7fb663679]::set_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  21:     0x7efd4076d85a - std[9c6ad123400df44d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>
  22:     0x7efd407d07e8 - std[9c6ad123400df44d]::panicking::try::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, core[54ab13d2a06817e1]::panic::unwind_safe::AssertUnwindSafe<<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x7efd4076ea16 - <<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7efd3fd0bace - std::sys::unix::thread::Thread::new::thread_start::h0ee55f1c660c8984
  25:     0x7efd3faa8b43 - <unknown>
  26:     0x7efd3fb3aa00 - <unknown>
  27:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (c330fa7e5 2023-04-21) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=10000 -C debuginfo=0 -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `compiler_builtins`
thread 'rustc' panicked at 'attempt to subtract with overflow', /cargo/registry/src/index.crates.io-6f17d22bba15001f/object-0.30.1/src/write/mod.rs:705:34
stack backtrace:
   0:     0x7fd1e65961e1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h99d41b7989943530
   1:     0x7fd1e65fdb58 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7fd1e6589de1 - std::io::Write::write_fmt::h0d88c7b0bf910f28
   3:     0x7fd1e6595ff5 - std::sys_common::backtrace::print::h1940fb990ef00a99
   4:     0x7fd1e6599154 - std::panicking::default_hook::{{closure}}::hc1eac4b73387c881
   5:     0x7fd1e6598e42 - std::panicking::default_hook::h04915ebfc8268c91
   6:     0x7fd1e6ff5315 - rustc_driver_impl[93cd919eb377a2e9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fd1e6599889 - std::panicking::rust_panic_with_hook::h75e7ea4c0112022c
   8:     0x7fd1e65995c2 - std::panicking::begin_panic_handler::{{closure}}::he8caf66ef9f93dc4
   9:     0x7fd1e6596696 - std::sys_common::backtrace::__rust_end_short_backtrace::h18c6d72531229c72
  10:     0x7fd1e65992d2 - rust_begin_unwind
  11:     0x7fd1e6550f43 - core::panicking::panic_fmt::hb12531409cdb4ed6
  12:     0x7fd1e6550fdd - core::panicking::panic::hc61b82565d1e8446
  13:     0x7fd1e907dec0 - <object[196f51113b2a81d0]::write::Section>::append_data
  14:     0x7fd1e8e1bb91 - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::metadata::create_object_file
  15:     0x7fd1e8e1bd3c - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::metadata::create_wrapper_file
  16:     0x7fd1e8d7f8ca - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::link::link_rlib
  17:     0x7fd1e8d7e77e - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::link::link_binary
  18:     0x7fd1e723a094 - <rustc_codegen_llvm[7640c7ee29cbb401]::LlvmCodegenBackend as rustc_codegen_ssa[a227ecdef4f6f8fb]::traits::backend::CodegenBackend>::link
  19:     0x7fd1e717518b - <rustc_interface[ce4c417bbf8a4c17]::queries::Linker>::link
  20:     0x7fd1e70650dc - rustc_span[de5c6ef7fb663679]::set_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  21:     0x7fd1e700785a - std[9c6ad123400df44d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>
  22:     0x7fd1e706a7e8 - std[9c6ad123400df44d]::panicking::try::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, core[54ab13d2a06817e1]::panic::unwind_safe::AssertUnwindSafe<<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x7fd1e7008a16 - <<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7fd1e65a5ace - std::sys::unix::thread::Thread::new::thread_start::h0ee55f1c660c8984
  25:     0x7fd1e6342b43 - <unknown>
  26:     0x7fd1e63d4a00 - <unknown>
  27:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (c330fa7e5 2023-04-21) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z unstable-options -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `libc`
thread 'rustc' panicked at 'attempt to subtract with overflow', /cargo/registry/src/index.crates.io-6f17d22bba15001f/object-0.30.1/src/write/mod.rs:705:34
stack backtrace:
   0:     0x7fbaee6e01e1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h99d41b7989943530
   1:     0x7fbaee747b58 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7fbaee6d3de1 - std::io::Write::write_fmt::h0d88c7b0bf910f28
   3:     0x7fbaee6dfff5 - std::sys_common::backtrace::print::h1940fb990ef00a99
   4:     0x7fbaee6e3154 - std::panicking::default_hook::{{closure}}::hc1eac4b73387c881
   5:     0x7fbaee6e2e42 - std::panicking::default_hook::h04915ebfc8268c91
   6:     0x7fbaef13f315 - rustc_driver_impl[93cd919eb377a2e9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fbaee6e3889 - std::panicking::rust_panic_with_hook::h75e7ea4c0112022c
   8:     0x7fbaee6e35c2 - std::panicking::begin_panic_handler::{{closure}}::he8caf66ef9f93dc4
   9:     0x7fbaee6e0696 - std::sys_common::backtrace::__rust_end_short_backtrace::h18c6d72531229c72
  10:     0x7fbaee6e32d2 - rust_begin_unwind
  11:     0x7fbaee69af43 - core::panicking::panic_fmt::hb12531409cdb4ed6
  12:     0x7fbaee69afdd - core::panicking::panic::hc61b82565d1e8446
  13:     0x7fbaf11c7ec0 - <object[196f51113b2a81d0]::write::Section>::append_data
  14:     0x7fbaf0f65b91 - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::metadata::create_object_file
  15:     0x7fbaf0f65d3c - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::metadata::create_wrapper_file
  16:     0x7fbaf0ec98ca - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::link::link_rlib
  17:     0x7fbaf0ec877e - rustc_codegen_ssa[a227ecdef4f6f8fb]::back::link::link_binary
  18:     0x7fbaef384094 - <rustc_codegen_llvm[7640c7ee29cbb401]::LlvmCodegenBackend as rustc_codegen_ssa[a227ecdef4f6f8fb]::traits::backend::CodegenBackend>::link
  19:     0x7fbaef2bf18b - <rustc_interface[ce4c417bbf8a4c17]::queries::Linker>::link
  20:     0x7fbaef1af0dc - rustc_span[de5c6ef7fb663679]::set_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  21:     0x7fbaef15185a - std[9c6ad123400df44d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>
  22:     0x7fbaef1b47e8 - std[9c6ad123400df44d]::panicking::try::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, core[54ab13d2a06817e1]::panic::unwind_safe::AssertUnwindSafe<<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x7fbaef152a16 - <<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[ce4c417bbf8a4c17]::util::run_in_thread_pool_with_globals<rustc_interface[ce4c417bbf8a4c17]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[93cd919eb377a2e9]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7fbaee6eface - std::sys::unix::thread::Thread::new::thread_start::h0ee55f1c660c8984
  25:     0x7fbaee48cb43 - <unknown>
  26:     0x7fbaee51ea00 - <unknown>
  27:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (c330fa7e5 2023-04-21) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
