
   Compiling bootloader v0.11.0
warning: unused import: `println`
  --> kernel/src/hardware/harddisk.rs:11:29
   |
11 | use crate::{serial_println, println, error::{Result, err}};
   |                             ^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `println`
 --> kernel/src/process.rs:6:13
  |
6 | use crate::{println, serial_println};
  |             ^^^^^^^

warning: unused imports: `BiosParameterBlock`, `Metadata`
 --> kernel/src/fs/fat.rs:3:15
  |
3 |     raw_fat::{BiosParameterBlock, Metadata, Name},
  |               ^^^^^^^^^^^^^^^^^^  ^^^^^^^^

warning: unused import: `Readable`
 --> kernel/src/fs/fat.rs:8:24
  |
8 |     hardware::{Device, Readable},
  |                        ^^^^^^^^

warning: unused import: `boxed::Box`
  --> kernel/src/fs/fat.rs:11:13
   |
11 | use alloc::{boxed::Box, vec::Vec};
   |             ^^^^^^^^^^

warning: unreachable statement
   --> kernel/src/process.rs:144:13
    |
140 | /             todo!(
141 | |                 "rax contains an int pointer,
142 | |             and the kernel should dereferenced the value"
143 | |             );
    | |_____________- any code following this expression is unreachable
144 |               saved_registers.rdi = self.curr_pid.0 as u64;
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
    |
    = note: `#[warn(unreachable_code)]` on by default

warning: unused variable: `regs`
   --> kernel/src/process.rs:125:24
    |
125 |     fn exit(&mut self, regs: &mut Registers, ret_code: u64) {
    |                        ^^^^ help: if this is intentional, prefix it with an underscore: `_regs`
    |
    = note: `#[warn(unused_variables)]` on by default

warning: variable does not need to be mutable
   --> kernel/src/process.rs:138:17
    |
138 |             let mut saved_registers = &mut parent_proc.saved_registers;
    |                 ----^^^^^^^^^^^^^^^
    |                 |
    |                 help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` on by default

warning: unused variable: `regs`
   --> kernel/src/process.rs:153:24
    |
153 |     fn kill(&mut self, regs: &mut Registers, pid: isize, signal_name: isize) {
    |                        ^^^^ help: if this is intentional, prefix it with an underscore: `_regs`

warning: unused variable: `pid`
   --> kernel/src/process.rs:153:46
    |
153 |     fn kill(&mut self, regs: &mut Registers, pid: isize, signal_name: isize) {
    |                                              ^^^ help: if this is intentional, prefix it with an underscore: `_pid`

warning: unused variable: `signal_name`
   --> kernel/src/process.rs:153:58
    |
153 |     fn kill(&mut self, regs: &mut Registers, pid: isize, signal_name: isize) {
    |                                                          ^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_signal_name`

warning: field `channels` is never read
  --> kernel/src/process.rs:54:5
   |
50 | pub struct KernelState {
   |            ----------- field in this struct
...
54 |     channels: Vec<ChannelState>,
   |     ^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: fields `free_clusters` and `free_clusters_start_at` are never read
   --> kernel/src/fs/raw_fat.rs:288:5
    |
282 | struct FileSystemInfo {
    |        -------------- fields in this struct
...
288 |     free_clusters: u32,
    |     ^^^^^^^^^^^^^
...
293 |     free_clusters_start_at: u32,
    |     ^^^^^^^^^^^^^^^^^^^^^^

warning: fields `deci_sec`, `second`, `minute`, and `hour` are never read
   --> kernel/src/fs/raw_fat.rs:327:5
    |
326 | pub struct Timestamp {
    |            --------- fields in this struct
327 |     deci_sec: u8,
    |     ^^^^^^^^
328 |     second: u8,
    |     ^^^^^^
329 |     minute: u8,
    |     ^^^^^^
330 |     hour: u8,
    |     ^^^^
    |
    = note: `Timestamp` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: fields `year`, `month`, and `day` are never read
   --> kernel/src/fs/raw_fat.rs:350:5
    |
349 | pub struct Date {
    |            ---- fields in this struct
350 |     year: u8,
    |     ^^^^
351 |     month: u8,
    |     ^^^^^
352 |     day: u8,
    |     ^^^
    |
    = note: `Date` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: fields `creation_time`, `creation_date`, `access_date`, `modification_date`, and `modification_time` are never read
   --> kernel/src/fs/raw_fat.rs:393:16
    |
390 | pub struct Metadata {
    |            -------- fields in this struct
...
393 |     pub(super) creation_time: Timestamp,
    |                ^^^^^^^^^^^^^
394 |     pub(super) creation_date: Date,
    |                ^^^^^^^^^^^^^
395 |     pub(super) access_date: Date,
    |                ^^^^^^^^^^^
396 |     pub(super) chain_head: u32,
397 |     pub(super) modification_date: Date,
    |                ^^^^^^^^^^^^^^^^^
398 |     pub(super) modification_time: Timestamp,
    |                ^^^^^^^^^^^^^^^^^
    |
    = note: `Metadata` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: `kernel` (lib) generated 16 warnings (run `cargo fix --lib -p kernel` to apply 10 suggestions)
warning: unused imports: `mutex::Mutex`, `yield_now`
  --> kernel/src/main.rs:13:54
   |
13 | ...eyboard::KeycodeStream, mutex::Mutex, utils::{yield_now, sleep}, Task,
   |                            ^^^^^^^^^^^^          ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `kernel` (bin "kernel") generated 1 warning (run `cargo fix --bin "kernel"` to apply 1 suggestion)
error: failed to run custom build command for `bootloader v0.11.0`
note: To improve backtraces for build dependencies, set the CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true environment variable to enable debug information generation.

Caused by:
  process didn't exit successfully: `/home/mm/mm/rust/nicos/target/debug/build/bootloader-ce00a0df52972587/build-script-build` (exit status: 101)
  --- stderr
      Updating crates.io index
    Installing bootloader-x86_64-uefi v0.11.0
      Updating crates.io index
      Updating crates.io index
     Compiling compiler_builtins v0.1.91
     Compiling core v0.0.0 (/home/mm/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
     Compiling proc-macro2 v1.0.47
     Compiling quote v1.0.21
     Compiling autocfg v1.1.0
     Compiling unicode-ident v1.0.5
     Compiling rustversion v1.0.9
     Compiling syn v1.0.105
     Compiling log v0.4.17
     Compiling bootloader_api v0.11.0
     Compiling lock_api v0.4.9
     Compiling uefi-macros v0.7.1
     Compiling rustc-std-workspace-core v1.99.0 (/home/mm/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
     Compiling bit_field v0.10.1
     Compiling bitflags v1.3.2
     Compiling scopeguard v1.1.0
     Compiling cfg-if v1.0.0
     Compiling rand_core v0.6.4
     Compiling zero v0.1.3
     Compiling conquer-util v0.3.0
     Compiling volatile v0.4.5
     Compiling usize_conversions v0.2.0
     Compiling noto-sans-mono-bitmap v0.2.0
     Compiling raw-cpuid v10.6.0
     Compiling xmas-elf v0.8.0
     Compiling conquer-once v0.3.2
     Compiling rand_hc v0.3.1
     Compiling rand v0.8.5
     Compiling ucs2 v0.3.2
     Compiling x86_64 v0.14.10
     Compiling uefi v0.16.1
     Compiling spinning_top v0.2.4
     Compiling bootloader-x86_64-common v0.11.0
     Compiling bootloader-x86_64-uefi v0.11.0
  error: internal compiler error: compiler/rustc_monomorphize/src/collector.rs:1038:9: no MIR available for DefId(25:1196 ~ uefi[1c2c]::proto::media::file::info::{impl#3}::name_offset)

  thread 'rustc' panicked at 'Box<dyn Any>', /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/compiler/rustc_errors/src/lib.rs:1650:9
  stack backtrace:
     0:     0x7f53fa7dd361 - std::backtrace_rs::backtrace::libunwind::trace::he9e9030cf5bcddaa
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
     1:     0x7f53fa7dd361 - std::backtrace_rs::backtrace::trace_unsynchronized::hb5e44a77ff0586eb
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
     2:     0x7f53fa7dd361 - std::sys_common::backtrace::_print_fmt::h38e8723c5d83f22f
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/sys_common/backtrace.rs:65:5
     3:     0x7f53fa7dd361 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5e394c4b1f1cb045
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/sys_common/backtrace.rs:44:22
     4:     0x7f53fa83d6ff - core::fmt::rt::Argument::fmt::ha5dde4cb71578cc2
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/core/src/fmt/rt.rs:138:9
     5:     0x7f53fa83d6ff - core::fmt::write::hb1413899fd777907
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/core/src/fmt/mod.rs:1094:21
     6:     0x7f53fa7d05c1 - std::io::Write::write_fmt::h08b4eac68313bb0d
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/io/mod.rs:1712:15
     7:     0x7f53fa7dd175 - std::sys_common::backtrace::_print::h2bb25f41b1777226
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/sys_common/backtrace.rs:47:5
     8:     0x7f53fa7dd175 - std::sys_common::backtrace::print::ha21981aa5a1c5802
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/sys_common/backtrace.rs:34:9
     9:     0x7f53fa7dfe07 - std::panicking::default_hook::{{closure}}::h603ce04329232051
    10:     0x7f53fa7dfbf4 - std::panicking::default_hook::hd4285764fbdb24e1
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/panicking.rs:288:9
    11:     0x7f53fda3294b - <rustc_driver_impl[27c68de0ac55b06a]::install_ice_hook::{closure#0} as core[3afeaedfb140cde7]::ops::function::FnOnce<(&core[3afeaedfb140cde7]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
    12:     0x7f53fa7e0527 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h401f21a7538313bc
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/alloc/src/boxed.rs:1999:9
    13:     0x7f53fa7e0527 - std::panicking::rust_panic_with_hook::h2322eda2481f33bc
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/panicking.rs:695:13
    14:     0x7f53fdf21031 - std[53b54636b4f52653]::panicking::begin_panic::<rustc_errors[2afb0e2cc7c4c42d]::ExplicitBug>::{closure#0}
    15:     0x7f53fdf1d7e6 - std[53b54636b4f52653]::sys_common::backtrace::__rust_end_short_backtrace::<std[53b54636b4f52653]::panicking::begin_panic<rustc_errors[2afb0e2cc7c4c42d]::ExplicitBug>::{closure#0}, !>
    16:     0x7f53fdf1d6d6 - std[53b54636b4f52653]::panicking::begin_panic::<rustc_errors[2afb0e2cc7c4c42d]::ExplicitBug>
    17:     0x7f53fdf18264 - <rustc_errors[2afb0e2cc7c4c42d]::HandlerInner>::bug::<alloc[a4f318becb59a05]::string::String>
    18:     0x7f53fdf17d76 - <rustc_errors[2afb0e2cc7c4c42d]::Handler>::bug::<alloc[a4f318becb59a05]::string::String>
    19:     0x7f53fdfa4bac - rustc_middle[232bd29a687eeda3]::util::bug::opt_span_bug_fmt::<rustc_span[811089187935bcb6]::span_encoding::Span>::{closure#0}
    20:     0x7f53fdfa437a - rustc_middle[232bd29a687eeda3]::ty::context::tls::with_opt::<rustc_middle[232bd29a687eeda3]::util::bug::opt_span_bug_fmt<rustc_span[811089187935bcb6]::span_encoding::Span>::{closure#0}, !>::{closure#0}
    21:     0x7f53fdfa434a - rustc_middle[232bd29a687eeda3]::ty::context::tls::with_context_opt::<rustc_middle[232bd29a687eeda3]::ty::context::tls::with_opt<rustc_middle[232bd29a687eeda3]::util::bug::opt_span_bug_fmt<rustc_span[811089187935bcb6]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
    22:     0x7f53fbc3bf8d - rustc_middle[232bd29a687eeda3]::util::bug::bug_fmt
    23:     0x7f53fc8b4845 - rustc_monomorphize[6a4925edab24b3f4]::collector::collect_neighbours
    24:     0x7f53fc8a4676 - rustc_monomorphize[6a4925edab24b3f4]::collector::collect_items_rec
    25:     0x7f53fc8a4ae7 - rustc_monomorphize[6a4925edab24b3f4]::collector::collect_items_rec
    26:     0x7f53fc8a4ae7 - rustc_monomorphize[6a4925edab24b3f4]::collector::collect_items_rec
    27:     0x7f53fc8a4ae7 - rustc_monomorphize[6a4925edab24b3f4]::collector::collect_items_rec
    28:     0x7f53fc8a4ae7 - rustc_monomorphize[6a4925edab24b3f4]::collector::collect_items_rec
    29:     0x7f53fc8a4ae7 - rustc_monomorphize[6a4925edab24b3f4]::collector::collect_items_rec
    30:     0x7f53fc8a4ae7 - rustc_monomorphize[6a4925edab24b3f4]::collector::collect_items_rec
    31:     0x7f53fc8a4ae7 - rustc_monomorphize[6a4925edab24b3f4]::collector::collect_items_rec
    32:     0x7f53fc8a4ae7 - rustc_monomorphize[6a4925edab24b3f4]::collector::collect_items_rec
    33:     0x7f53fc8a4ae7 - rustc_monomorphize[6a4925edab24b3f4]::collector::collect_items_rec
    34:     0x7f53fce8e250 - rustc_data_structures[fba7df231146955e]::sync::par_for_each_in::<alloc[a4f318becb59a05]::vec::Vec<rustc_middle[232bd29a687eeda3]::mir::mono::MonoItem>, rustc_monomorphize[6a4925edab24b3f4]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
    35:     0x7f53fce8df74 - <rustc_session[f2a4139b8ec92988]::session::Session>::time::<(), rustc_monomorphize[6a4925edab24b3f4]::collector::collect_crate_mono_items::{closure#1}>
    36:     0x7f53fce8d050 - rustc_monomorphize[6a4925edab24b3f4]::collector::collect_crate_mono_items
    37:     0x7f53fce8b8f7 - rustc_monomorphize[6a4925edab24b3f4]::partitioning::collect_and_partition_mono_items
    38:     0x7f53fd24f5dc - rustc_query_system[8be5ea0cc4c28ea6]::query::plumbing::try_execute_query::<rustc_query_impl[94005537e54250be]::queries::collect_and_partition_mono_items, rustc_query_impl[94005537e54250be]::plumbing::QueryCtxt>
    39:     0x7f53fd24f2bd - rustc_query_impl[94005537e54250be]::get_query::collect_and_partition_mono_items
    40:     0x7f53fcef439c - rustc_codegen_ssa[4e498a107e2a3c27]::base::codegen_crate::<rustc_codegen_llvm[f9a4b87b46d3b0ee]::LlvmCodegenBackend>
    41:     0x7f53fcef4182 - <rustc_codegen_llvm[f9a4b87b46d3b0ee]::LlvmCodegenBackend as rustc_codegen_ssa[4e498a107e2a3c27]::traits::backend::CodegenBackend>::codegen_crate
    42:     0x7f53fcc44590 - rustc_interface[1005215f39c21569]::passes::start_codegen
    43:     0x7f53fcc400d7 - <rustc_middle[232bd29a687eeda3]::ty::context::GlobalCtxt>::enter::<<rustc_interface[1005215f39c21569]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[3afeaedfb140cde7]::result::Result<alloc[a4f318becb59a05]::boxed::Box<dyn core[3afeaedfb140cde7]::any::Any>, rustc_span[811089187935bcb6]::ErrorGuaranteed>>
    44:     0x7f53fcc3f84d - <rustc_interface[1005215f39c21569]::queries::Queries>::ongoing_codegen
    45:     0x7f53fcc3ece3 - <rustc_interface[1005215f39c21569]::interface::Compiler>::enter::<rustc_driver_impl[27c68de0ac55b06a]::run_compiler::{closure#1}::{closure#2}, core[3afeaedfb140cde7]::result::Result<core[3afeaedfb140cde7]::option::Option<rustc_interface[1005215f39c21569]::queries::Linker>, rustc_span[811089187935bcb6]::ErrorGuaranteed>>
    46:     0x7f53fcc3c33b - std[53b54636b4f52653]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1005215f39c21569]::util::run_in_thread_pool_with_globals<rustc_interface[1005215f39c21569]::interface::run_compiler<core[3afeaedfb140cde7]::result::Result<(), rustc_span[811089187935bcb6]::ErrorGuaranteed>, rustc_driver_impl[27c68de0ac55b06a]::run_compiler::{closure#1}>::{closure#0}, core[3afeaedfb140cde7]::result::Result<(), rustc_span[811089187935bcb6]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3afeaedfb140cde7]::result::Result<(), rustc_span[811089187935bcb6]::ErrorGuaranteed>>
    47:     0x7f53fd30bf55 - <<std[53b54636b4f52653]::thread::Builder>::spawn_unchecked_<rustc_interface[1005215f39c21569]::util::run_in_thread_pool_with_globals<rustc_interface[1005215f39c21569]::interface::run_compiler<core[3afeaedfb140cde7]::result::Result<(), rustc_span[811089187935bcb6]::ErrorGuaranteed>, rustc_driver_impl[27c68de0ac55b06a]::run_compiler::{closure#1}>::{closure#0}, core[3afeaedfb140cde7]::result::Result<(), rustc_span[811089187935bcb6]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3afeaedfb140cde7]::result::Result<(), rustc_span[811089187935bcb6]::ErrorGuaranteed>>::{closure#1} as core[3afeaedfb140cde7]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
    48:     0x7f53fa7ea9d5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h20fe0a24dda744f1
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/alloc/src/boxed.rs:1985:9
    49:     0x7f53fa7ea9d5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hb11b4529d66356f4
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/alloc/src/boxed.rs:1985:9
    50:     0x7f53fa7ea9d5 - std::sys::unix::thread::Thread::new::thread_start::h896be1b4ceadbc98
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/sys/unix/thread.rs:108:17
    51:     0x7f53fa68e609 - start_thread
                                 at /build/glibc-SzIz7B/glibc-2.31/nptl/pthread_create.c:477:8
    52:     0x7f53fa5b1133 - clone
                                 at /build/glibc-SzIz7B/glibc-2.31/misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:95
    53:                0x0 - <unknown>

  note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

  note: rustc 1.71.0-nightly (2f2c438dc 2023-05-08) running on x86_64-unknown-linux-gnu

  note: compiler flags: --crate-type bin -C opt-level=3 -C embed-bitcode=no -Z unstable-options

  note: some of the compiler flags provided by cargo are hidden

  query stack during panic:
  #0 [collect_and_partition_mono_items] collect_and_partition_mono_items
  end of query stack
  error: could not compile `bootloader-x86_64-uefi` (bin "bootloader-x86_64-uefi")
  error: failed to compile `bootloader-x86_64-uefi v0.11.0`, intermediate artifacts can be found at `/tmp/cargo-installih7YZw`
  thread 'main' panicked at 'failed to build uefi bootloader', /home/mm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bootloader-0.11.0/build.rs:76:9
  stack backtrace:
     0:     0x564d1ad665a1 - std::backtrace_rs::backtrace::libunwind::trace::he9e9030cf5bcddaa
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
     1:     0x564d1ad665a1 - std::backtrace_rs::backtrace::trace_unsynchronized::hb5e44a77ff0586eb
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
     2:     0x564d1ad665a1 - std::sys_common::backtrace::_print_fmt::h38e8723c5d83f22f
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/sys_common/backtrace.rs:65:5
     3:     0x564d1ad665a1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5e394c4b1f1cb045
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/sys_common/backtrace.rs:44:22
     4:     0x564d1ad8704f - core::fmt::rt::Argument::fmt::ha5dde4cb71578cc2
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/core/src/fmt/rt.rs:138:9
     5:     0x564d1ad8704f - core::fmt::write::hb1413899fd777907
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/core/src/fmt/mod.rs:1094:21
     6:     0x564d1ad63d81 - std::io::Write::write_fmt::h08b4eac68313bb0d
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/io/mod.rs:1712:15
     7:     0x564d1ad663b5 - std::sys_common::backtrace::_print::h2bb25f41b1777226
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/sys_common/backtrace.rs:47:5
     8:     0x564d1ad663b5 - std::sys_common::backtrace::print::ha21981aa5a1c5802
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/sys_common/backtrace.rs:34:9
     9:     0x564d1ad67e57 - std::panicking::default_hook::{{closure}}::h603ce04329232051
    10:     0x564d1ad67c44 - std::panicking::default_hook::hd4285764fbdb24e1
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/panicking.rs:288:9
    11:     0x564d1ad682f1 - std::panicking::rust_panic_with_hook::h2322eda2481f33bc
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/panicking.rs:691:13
    12:     0x564d1ad681a1 - std::panicking::begin_panic_handler::{{closure}}::h3c6baee49f38de13
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/panicking.rs:580:13
    13:     0x564d1ad669e6 - std::sys_common::backtrace::__rust_end_short_backtrace::he4c32e2cde881fbf
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/sys_common/backtrace.rs:150:18
    14:     0x564d1ad67f52 - rust_begin_unwind
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/panicking.rs:578:5
    15:     0x564d1ad3a283 - core::panicking::panic_fmt::hbd6b294c82b9b3dd
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/core/src/panicking.rs:67:14
    16:     0x564d1ad3b8a4 - build_script_build::build_uefi_bootloader::ha64e41b8f0bcb115
    17:     0x564d1ad3ac51 - build_script_build::main::ha5edc8c5ccfa7ea8
    18:     0x564d1ad3ef43 - core::ops::function::FnOnce::call_once::h668ae0fb8cb7d0b8
    19:     0x564d1ad3e916 - std::sys_common::backtrace::__rust_begin_short_backtrace::ha27587cb3abe07a4
    20:     0x564d1ad3eea9 - std::rt::lang_start::{{closure}}::h4784aefd3aad7ab1
    21:     0x564d1ad60a25 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::h0e92181d85ba76c5
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/core/src/ops/function.rs:284:13
    22:     0x564d1ad60a25 - std::panicking::try::do_call::h944df4f4fbb6020c
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/panicking.rs:485:40
    23:     0x564d1ad60a25 - std::panicking::try::h7f6c068eda6ae039
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/panicking.rs:449:19
    24:     0x564d1ad60a25 - std::panic::catch_unwind::hf8faebd1c590a761
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/panic.rs:140:14
    25:     0x564d1ad60a25 - std::rt::lang_start_internal::{{closure}}::he97ac956f184b101
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/rt.rs:148:48
    26:     0x564d1ad60a25 - std::panicking::try::do_call::h9b14e7fc2cfcb97a
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/panicking.rs:485:40
    27:     0x564d1ad60a25 - std::panicking::try::heca3ee258815924e
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/panicking.rs:449:19
    28:     0x564d1ad60a25 - std::panic::catch_unwind::h6e7e553c0a3c470a
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/panic.rs:140:14
    29:     0x564d1ad60a25 - std::rt::lang_start_internal::h3117708736c316f3
                                 at /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/library/std/src/rt.rs:148:20
    30:     0x564d1ad3ee87 - std::rt::lang_start::h4fa2b67ca858ac4f
    31:     0x564d1ad3df95 - main
    32:     0x7fe86254d083 - __libc_start_main
                                 at /build/glibc-SzIz7B/glibc-2.31/csu/../csu/libc-start.c:308:16
    33:     0x564d1ad3a8ee - _start
    34:                0x0 - <unknown>
