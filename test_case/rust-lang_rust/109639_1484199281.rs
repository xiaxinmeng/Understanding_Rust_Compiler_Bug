plain
   Compiling adler v1.0.2
   Compiling rustc-demangle v0.1.21
[RUSTC-TIMING] cfg_if test:false 0.042
[RUSTC-TIMING] profiler_builtins test:false 0.051
thread 'rustc' panicked at 'no CodegenUnitDebugContext found on LLVM context', compiler/rustc_codegen_llvm/src/debuginfo/mod.rs:298:39
stack backtrace:
   0:     0x7f55ea1b60aa - std::backtrace_rs::backtrace::libunwind::trace::h0ffe58418c003787
   1:     0x7f55ea1b60aa - std::backtrace_rs::backtrace::trace_unsynchronized::h761f0fdb0d0a58fb
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f55ea1b60aa - std::sys_common::backtrace::_print_fmt::h771f9efeb63e9fd5
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f55ea1b60aa - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45d751481ad35228
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f55ea219e1e - core::fmt::write::h895534c79fa6aae4
   5:     0x7f55ea1a8bf5 - std::io::Write::write_fmt::h1175d487c21b67a7
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/std/src/io/mod.rs:1698:15
   6:     0x7f55ea1b5e75 - std::sys_common::backtrace::_print::h83d8fe6baea6ee93
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/std/src/sys_common/backtrace.rs:47:5
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f55ea1b5e75 - std::sys_common::backtrace::print::h6e968a4fc8e207df
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f55ea1b8bef - std::panicking::default_hook::{{closure}}::h73fd81d072a513e4
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/std/src/panicking.rs:271:22
   9:     0x7f55ea1b892b - std::panicking::default_hook::hbbab2ae13b778dd2
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/std/src/panicking.rs:290:9
  10:     0x7f55e73b6825 - <rustc_driver_impl[8acb53825cb3ac80]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[3564f90f2d2fc29]::ops::function::FnOnce<(&core[3564f90f2d2fc29]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7f55ea1b942d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h2646502c7cee6a2c
  12:     0x7f55ea1b942d - std::panicking::rust_panic_with_hook::h40731192c9688d0e
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/std/src/panicking.rs:696:13
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/std/src/panicking.rs:696:13
  13:     0x7f55ea1b91a9 - std::panicking::begin_panic_handler::{{closure}}::heaad7ca588b10871
  14:     0x7f55ea1b6516 - std::sys_common::backtrace::__rust_end_short_backtrace::h8f21f57b0b49fde6
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/std/src/sys_common/backtrace.rs:150:18
  15:     0x7f55ea1b8eb2 - rust_begin_unwind
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/std/src/panicking.rs:579:5
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/std/src/panicking.rs:579:5
  16:     0x7f55ea216163 - core::panicking::panic_fmt::hacbcd6ff7873fbfc
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/core/src/panicking.rs:67:14
  17:     0x7f55ea2162d1 - core::panicking::panic_display::h0068b862c75f1ecd
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/core/src/panicking.rs:150:5
  18:     0x7f55ea21627b - core::panicking::panic_str::hcb90af2adb86df92
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/core/src/panicking.rs:134:5
  19:     0x7f55ea215ee6 - core::option::expect_failed::hdd65a7aa37e65d4a
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/core/src/option.rs:2055:5
  20:     0x7f55e74f560b - rustc_codegen_ssa[ec2a1977c2e71b3a]::meth::get_vtable::<rustc_codegen_llvm[ec651b3cd0038b49]::context::CodegenCx>
  21:     0x7f55e755245d - rustc_codegen_ssa[ec2a1977c2e71b3a]::base::unsized_info::<rustc_codegen_llvm[ec651b3cd0038b49]::builder::Builder>
  22:     0x7f55e7551a8c - rustc_codegen_ssa[ec2a1977c2e71b3a]::base::unsize_ptr::<rustc_codegen_llvm[ec651b3cd0038b49]::builder::Builder>
  23:     0x7f55e75ed574 - <rustc_codegen_ssa[ec2a1977c2e71b3a]::mir::FunctionCx<rustc_codegen_llvm[ec651b3cd0038b49]::builder::Builder>>::codegen_rvalue_operand
  24:     0x7f55e75e7313 - rustc_codegen_ssa[ec2a1977c2e71b3a]::mir::codegen_mir::<rustc_codegen_llvm[ec651b3cd0038b49]::builder::Builder>
  25:     0x7f55e755472d - rustc_codegen_ssa[ec2a1977c2e71b3a]::base::codegen_instance::<rustc_codegen_llvm[ec651b3cd0038b49]::builder::Builder>
  26:     0x7f55e75b7970 - <rustc_middle[592f5aac4adc36c1]::mir::mono::MonoItem as rustc_codegen_ssa[ec2a1977c2e71b3a]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[ec651b3cd0038b49]::builder::Builder>
  27:     0x7f55e7543cc4 - rustc_codegen_llvm[ec651b3cd0038b49]::base::compile_codegen_unit::module_codegen
  28:     0x7f55e75430d4 - rustc_codegen_llvm[ec651b3cd0038b49]::base::compile_codegen_unit
  29:     0x7f55e7553d40 - rustc_codegen_ssa[ec2a1977c2e71b3a]::base::codegen_crate::<rustc_codegen_llvm[ec651b3cd0038b49]::LlvmCodegenBackend>
  30:     0x7f55e75105d8 - <rustc_codegen_llvm[ec651b3cd0038b49]::LlvmCodegenBackend as rustc_codegen_ssa[ec2a1977c2e71b3a]::traits::backend::CodegenBackend>::codegen_crate
  31:     0x7f55e7428c6f - <rustc_session[a7ad64d0e9eb3c09]::session::Session>::time::<alloc[f07b92181fce68c9]::boxed::Box<dyn core[3564f90f2d2fc29]::any::Any>, rustc_interface[bdec339a16c21f39]::passes::start_codegen::{closure#0}>
  32:     0x7f55e74ac9d2 - rustc_interface[bdec339a16c21f39]::passes::start_codegen
  33:     0x7f55e740d3cb - <rustc_middle[592f5aac4adc36c1]::ty::context::GlobalCtxt>::enter::<<rustc_interface[bdec339a16c21f39]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[3564f90f2d2fc29]::result::Result<alloc[f07b92181fce68c9]::boxed::Box<dyn core[3564f90f2d2fc29]::any::Any>, rustc_span[88232e7a35cbe17e]::ErrorGuaranteed>>
  34:     0x7f55e749d260 - <rustc_interface[bdec339a16c21f39]::queries::Queries>::ongoing_codegen
  35:     0x7f55e73825c3 - <rustc_interface[bdec339a16c21f39]::interface::Compiler>::enter::<rustc_driver_impl[8acb53825cb3ac80]::run_compiler::{closure#1}::{closure#2}, core[3564f90f2d2fc29]::result::Result<core[3564f90f2d2fc29]::option::Option<rustc_interface[bdec339a16c21f39]::queries::Linker>, rustc_span[88232e7a35cbe17e]::ErrorGuaranteed>>
  36:     0x7f55e7377bd0 - rustc_span[88232e7a35cbe17e]::with_source_map::<core[3564f90f2d2fc29]::result::Result<(), rustc_span[88232e7a35cbe17e]::ErrorGuaranteed>, rustc_interface[bdec339a16c21f39]::interface::run_compiler<core[3564f90f2d2fc29]::result::Result<(), rustc_span[88232e7a35cbe17e]::ErrorGuaranteed>, rustc_driver_impl[8acb53825cb3ac80]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  37:     0x7f55e7333759 - std[8d98d01f26af9fdf]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[bdec339a16c21f39]::util::run_in_thread_pool_with_globals<rustc_interface[bdec339a16c21f39]::interface::run_compiler<core[3564f90f2d2fc29]::result::Result<(), rustc_span[88232e7a35cbe17e]::ErrorGuaranteed>, rustc_driver_impl[8acb53825cb3ac80]::run_compiler::{closure#1}>::{closure#0}, core[3564f90f2d2fc29]::result::Result<(), rustc_span[88232e7a35cbe17e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3564f90f2d2fc29]::result::Result<(), rustc_span[88232e7a35cbe17e]::ErrorGuaranteed>>
  38:     0x7f55e735680d - <<std[8d98d01f26af9fdf]::thread::Builder>::spawn_unchecked_<rustc_interface[bdec339a16c21f39]::util::run_in_thread_pool_with_globals<rustc_interface[bdec339a16c21f39]::interface::run_compiler<core[3564f90f2d2fc29]::result::Result<(), rustc_span[88232e7a35cbe17e]::ErrorGuaranteed>, rustc_driver_impl[8acb53825cb3ac80]::run_compiler::{closure#1}>::{closure#0}, core[3564f90f2d2fc29]::result::Result<(), rustc_span[88232e7a35cbe17e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3564f90f2d2fc29]::result::Result<(), rustc_span[88232e7a35cbe17e]::ErrorGuaranteed>>::{closure#1} as core[3564f90f2d2fc29]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7f55ea1c34c3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hcc70c0f6700de579
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/alloc/src/boxed.rs:1988:9
  40:     0x7f55ea1c34c3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h4c1bedc4f309f2b9
  41:     0x7f55ea1c34c3 - std::sys::unix::thread::Thread::new::thread_start::h5c9f5eba54220185
                               at /rustc/29b20b5366e9a6c540c3dac4742d9cea3ffa67b5/library/std/src/sys/unix/thread.rs:108:17
  42:     0x7f55e5bb3ea5 - start_thread
  43:     0x7f55e58dcb0d - clone
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (29b20b536 2023-03-26) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -Z unstable-options -C linker=clang -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -C prefer-dynamic -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
---
Total duration:                          10m 25s
------------------------------------------------
root INFO: Free disk space: 509.10 GiB out of total 581.32 GiB (12.42% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 839, in <module>
    raise e
  File "../src/ci/stage-build.py", line 836, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 760, in execute_build_pipeline
    LLVM_PROFILE_DIR=str(pipeline.llvm_profile_dir_root() / "prof-%p")
  File "../src/ci/stage-build.py", line 571, in build_rustc
    cmd(arguments, env=env)
  File "../src/ci/stage-build.py", line 452, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/usr/bin/python3', '/checkout/x.py', 'build', '--target', 'x86_64-unknown-linux-gnu', '--host', 'x86_64-unknown-linux-gnu', '--stage', '2', 'library/std', '--llvm-profile-generate']' returned non-zero exit status 1.
