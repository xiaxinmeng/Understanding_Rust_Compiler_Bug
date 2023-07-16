
#0  0x00007f47d8f9acbb in elf_add ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/libstd-a81d7bd1df01b16f.so
#1  0x00007f47d8f9b5a7 in phdr_callback ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/libstd-a81d7bd1df01b16f.so
#2  0x00007f47d8c86b14 in __GI___dl_iterate_phdr (
    callback=0x7f47d8f9b510 <phdr_callback>, data=0x7f47cfc2f690) at dl-iteratephdr.c:76
#3  0x00007f47d8f9b684 in backtrace_initialize ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/libstd-a81d7bd1df01b16f.so
#4  0x00007f47d8f9a19e in fileline_initialize ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/libstd-a81d7bd1df01b16f.so
#5  0x00007f47d8f9a2a2 in backtrace_syminfo ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/libstd-a81d7bd1df01b16f.so
#6  0x00007f47d8f83668 in std::sys_common::gnu::libbacktrace::print (w=&mut Write, 
    idx=<optimized out>, addr=<optimized out>, symaddr=<optimized out>)
    at /opt/rust/src/libstd/sys_common/gnu/libbacktrace.rs:145
#7  std::sys::imp::backtrace::tracing::imp::write::trace_fn (ctx=<optimized out>, 
    arg=0x7f47cfc2fdf0)
    at /opt/rust/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:101
#8  0x00007f47d0e66609 in _Unwind_Backtrace (
    trace=0x7f47d8f83490 <std::sys::imp::backtrace::tracing::imp::write::trace_fn>, 
    trace_argument=0x7f47cfc2fdf0) at ../../../src/libgcc/unwind.inc:295
#9  0x00007f47d8f83213 in std::sys::imp::backtrace::tracing::imp::write (w=&mut Write)
    at /opt/rust/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
#10 0x00007f47d8f961e0 in std::panicking::default_hook::{{closure}} (err=&mut Write)
    at /opt/rust/src/libstd/panicking.rs:349
#11 0x00007f47d8f92599 in std::panicking::default_hook (info=<optimized out>)
    at /opt/rust/src/libstd/panicking.rs:359
#12 0x00007f47d8f92d68 in std::panicking::rust_panic_with_hook (msg=Box<Any>, 
    file_line=<optimized out>)
    at /opt/rust/src/libstd/panicking.rs:553
#13 0x00007f47d55f9efb in std::panicking::begin_panic<rustc_errors::FatalError> (
    msg=..., file_line=0x1)
    at /opt/rust/src/libstd/panicking.rs:515
#14 0x00007f47d576096d in syntax::parse::file_to_filemap (sess=0x7f47cfc34f10, 
    path=0x7f47c8002a40, 
    spanopt=<error reading variable: access outside bounds of object referenced via synthetic pointer>) at /opt/rust/src/libsyntax/parse/mod.rs:213
#15 0x00007f47d575fd47 in syntax::parse::new_parser_from_file (sess=0x7f47cfc34f10, 
    path=0x3) at /opt/rust/src/libsyntax/parse/mod.rs:160
#16 syntax::parse::parse_crate_from_file (input=0x3, sess=0x7f47cfc34f10)
    at /opt/rust/src/libsyntax/parse/mod.rs:98
#17 0x00007f47d92cce52 in rustc_driver::driver::phase_1_parse_input::{{closure}} ()
    at /opt/rust/src/librustc_driver/driver.rs:494
#18 rustc::util::common::time<core::result::Result<syntax::ast::Crate, rustc_errors::diagnostic_builder::DiagnosticBuilder>,closure> (
    what=<error reading variable: access outside bounds of object referenced via synthetic pointer>, do_it=<optimized out>, f=...)
    at /opt/rust/src/librustc/util/common.rs:34
#19 rustc_driver::driver::phase_1_parse_input (sess=0x7f47cfc344d0, 
    input=0x7f47cfc35a90)
    at /opt/rust/src/librustc_driver/driver.rs:491
#20 0x00007f47d92c90e8 in rustc_driver::driver::compile_input (sess=0x3, 
    cstore=0x7f47c8001060, input=0x7f47cfc35a90, outdir=0x7f47cfc35a78, 
    output=0x7f47cfc35a58, addl_plugins=..., control=0x7f47cfc353b8)
    at /opt/rust/src/librustc_driver/driver.rs:95
#21 0x00007f47d92f9283 in rustc_driver::run_compiler (args=..., 
    callbacks=&mut CompilerCalls, file_loader=..., emitter_dest=...)
    at /opt/rust/src/librustc_driver/lib.rs:221
#22 0x00007f47d923a262 in rustc_driver::main::{{closure}} ()
