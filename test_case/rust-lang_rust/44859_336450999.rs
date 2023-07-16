
(gdb) bt full
#0  0x00007ffff7337071 in __strlen_avx2 () from /usr/x86_64-pc-linux-gnu/lib/libc.so.6
No symbol table info available.
#1  0x0000555555565efe in std::ffi::c_str::CStr::from_ptr (ptr=0x7ffff7fcafdb <error: Cannot access memory at address 0x7ffff7fcafdb>) at src/libstd/ffi/c_str.rs:749
        len = <optimized out>
        ptr = 0x7ffff7fcafdb <error: Cannot access memory at address 0x7ffff7fcafdb>
#2  0x000055555555bde2 in std::sys_common::gnu::libbacktrace::resolve_symname<closure> (frame=..., callback=...) at src/libstd/sys_common/gnu/libbacktrace.rs:89
        ret = 1
        data_addr = <optimized out>
        data = 0x7ffff7fcafdb <error: Cannot access memory at address 0x7ffff7fcafdb>
        state = <optimized out>
#3  0x000055555555db88 in std::sys::imp::backtrace::printing::resolve_symname<closure> (frame=..., callback=..., bc=0x1b) at src/libstd/sys/unix/backtrace/printing/mod.rs:36
        bc = 0x1b
        callback = <optimized out>
#4  0x0000555555567405 in std::sys_common::backtrace::filter_frames::{{closure}} (frame=<optimized out>) at src/libstd/sys_common/backtrace.rs:110
        is_marker = false
        frame = <optimized out>
#5  core::slice::{{impl}}::position::{{closure}}<std::sys_common::backtrace::Frame,closure> (elt=<optimized out>) at /home/keruspe/Sources/rust/src/libcore/slice/mod.rs:1204
        elt = <optimized out>
        index = 0
#6  0x0000555555567202 in core::slice::Iter<std::sys_common::backtrace::Frame>::search_while<std::sys_common::backtrace::Frame,core::option::Option<usize>,closure> (self=<optimized out>, default=..., g=...) at /home/keruspe/Sources/rust/src/libcore/slice/mod.rs:1271
        self = <optimized out>
#7  core::slice::{{impl}}::position<std::sys_common::backtrace::Frame,closure> (self=0x7fffffffdda0, predicate=...) at /home/keruspe/Sources/rust/src/libcore/slice/mod.rs:1203
        self = <optimized out>
        predicate = <optimized out>
#8  0x0000555555562262 in std::sys_common::backtrace::filter_frames (frames=..., format=<optimized out>, context=<optimized out>) at src/libstd/sys_common/backtrace.rs:108
        skipped_after = <optimized out>
        frames = &[std::sys_common::backtrace::Frame] {data_ptr: 0x7fffffffd6b8, length: 14}
        context = <optimized out>
        format = std::sys_common::backtrace::PrintFormat::Short
#9  std::sys_common::backtrace::_print (format=<optimized out>, w=...) at src/libstd/sys_common/backtrace.rs:73
        skipped_after = <optimized out>
        nb_frames = 14
        context = <optimized out>
        frames = [std::sys_common::backtrace::Frame {exact_position: 0x55555555c2eb <std::sys::imp::backtrace::tracing::imp::unwind_backtrace+43>, symbol_addr: 0x55555555c2c0 <std::sys::imp::backtrace::tracing::imp::unwind_backtrace>}, std::sys_common::backtrace::Frame {exact_position: 0x5555555621d4 <std::sys_common::backtrace::print+100>, symbol_addr: 0x555555562170 <std::sys_common::backtrace::print>}, 
          std::sys_common::backtrace::Frame {exact_position: 0x55555555e919 <std::panicking::default_hook::{{closure}}+537>, symbol_addr: 0x55555555e700 <std::panicking::default_hook::{{closure}}>}, std::sys_common::backtrace::Frame {exact_position: 0x55555555e5c4 <std::panicking::default_hook+660>, symbol_addr: 0x55555555e330 <std::panicking::default_hook>}, 
          std::sys_common::backtrace::Frame {exact_position: 0x55555555ed9b <std::panicking::rust_panic_with_hook+251>, symbol_addr: 0x55555555eca0 <std::panicking::rust_panic_with_hook>}, std::sys_common::backtrace::Frame {exact_position: 0x55555555a5b5 <std::panicking::begin_panic<&str>+149>, symbol_addr: 0x55555555a520 <std::panicking::begin_panic<&str>>}, 
          std::sys_common::backtrace::Frame {exact_position: 0x55555555ac0d <a::main+29>, symbol_addr: 0x55555555abf0 <a::main>}, std::sys_common::backtrace::Frame {exact_position: 0x5555555a5727 <panic_unwind::__rust_maybe_catch_panic+39>, symbol_addr: 0x5555555a5700 <panic_unwind::__rust_maybe_catch_panic>}, 
          std::sys_common::backtrace::Frame {exact_position: 0x55555555ea67 <std::panicking::try<(),closure>+55>, symbol_addr: 0x55555555ea30 <std::panicking::try<(),closure>>}, std::sys_common::backtrace::Frame {exact_position: 0x55555555c485 <std::rt::lang_start+117>, symbol_addr: 0x55555555c410 <std::rt::lang_start>}, 
          std::sys_common::backtrace::Frame {exact_position: 0x55555555ac3d <main+45>, symbol_addr: 0x55555555ac10 <main>}, std::sys_common::backtrace::Frame {exact_position: 0x7ffff7201f49 <__libc_start_main+233>, symbol_addr: 0x7ffff7201e60 <__libc_start_main>}, std::sys_common::backtrace::Frame {exact_position: 0x55555555a3d9 <_start+41>, symbol_addr: 0x55555555a3b0 <_start>}, 
          std::sys_common::backtrace::Frame {exact_position: 0x0, symbol_addr: 0x0} <repeats 87 times>]
        format = std::sys_common::backtrace::PrintFormat::Short
#10 std::sys_common::backtrace::print (w=..., format=std::sys_common::backtrace::PrintFormat::Short) at src/libstd/sys_common/backtrace.rs:60
        format = std::sys_common::backtrace::PrintFormat::Short
#11 0x000055555555e91a in std::panicking::default_hook::{{closure}} (err=...) at src/libstd/panicking.rs:381
        format = 1
        err = std::io::&mut Write
        log_backtrace = core::option::Option<std::sys_common::backtrace::PrintFormat>::Some(std::sys_common::backtrace::PrintFormat::Short)
#12 0x000055555555e5c5 in std::panicking::default_hook (info=<optimized out>) at src/libstd/panicking.rs:397
        stderr = <optimized out>
        thread = <optimized out>
        col = <optimized out>
        line = <optimized out>
        log_backtrace = <optimized out>
        info = <optimized out>
#13 0x000055555555ed9c in std::panicking::rust_panic_with_hook (msg=..., file_line_col=<optimized out>) at src/libstd/panicking.rs:577
        file = <optimized out>
        line = <optimized out>
        col = <optimized out>
        file_line_col = <optimized out>
        msg = <optimized out>
#14 0x000055555555a5b6 in std::panicking::begin_panic<&str> (msg=..., file_line_col=0x5555557ca350 <ref>) at /home/keruspe/Sources/rust/src/libstd/panicking.rs:538
        msg = &str {data_ptr: 0x5555555acb44 <str> "blah\000", length: 4}
        file_line_col = 0x5555557ca350 <ref>
#15 0x000055555555ac0e in a::main () at a.rs:2
No locals.
