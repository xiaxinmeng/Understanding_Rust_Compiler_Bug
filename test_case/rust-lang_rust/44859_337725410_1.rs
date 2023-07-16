
(gdb) bt full
#0  0x00007ffff7337071 in __strlen_avx2 () from /usr/x86_64-pc-linux-gnu/lib/libc.so.6
No symbol table info available.
#1  0x000055555556072d in std::ffi::c_str::CStr::from_ptr (ptr=0x7ffff7f84195 <error: Cannot access memory at address 0x7ffff7f84195>) at src/libstd/ffi/c_str.rs:864
        len = 140737353630101
        ptr = 0x7ffff7f84195 <error: Cannot access memory at address 0x7ffff7f84195>
#2  0x0000555555572f12 in std::sys_common::gnu::libbacktrace::resolve_symname<closure> (frame=..., callback=...) at src/libstd/sys_common/gnu/libbacktrace.rs:89
        ret = 1
        data_addr = 0x7fffffffc420
        data = 0x7ffff7f84195 <error: Cannot access memory at address 0x7ffff7f84195>
        state = 0x7ffff7ff2000
        symname = core::option::Option<&str>::Some(&str {data_ptr: 0x7fffffffc464 "\000", length: 76155161039947})
        frame = std::sys_common::backtrace::Frame {exact_position: 0x55555558834b <std::sys::imp::backtrace::tracing::imp::unwind_backtrace+91>, symbol_addr: 0x5555555882f0 <std::sys::imp::backtrace::tracing::imp::unwind_backtrace>}
        callback = closure (closure (0x7fffffffc58f), 0x7fffffffc4f8, 0x7fffffffc510)
#3  0x0000555555588bda in std::sys::imp::backtrace::printing::resolve_symname<closure> (frame=..., callback=..., bc=0x7fffffffd3d8) at src/libstd/sys/unix/backtrace/printing/mod.rs:36
        frame = std::sys_common::backtrace::Frame {exact_position: 0x55555558834b <std::sys::imp::backtrace::tracing::imp::unwind_backtrace+91>, symbol_addr: 0x5555555882f0 <std::sys::imp::backtrace::tracing::imp::unwind_backtrace>}
        callback = closure (0x7fffffffc58f)
        bc = 0x7fffffffd3d8
#4  0x0000555555581ebb in std::sys_common::backtrace::filter_frames::{{closure}} (frame=0x7fffffffcd80) at src/libstd/sys_common/backtrace.rs:108
        is_marker = false
        frame = 0x7fffffffcd80
        context = 0x7fffffffd3d8
#5  0x000055555557cc2b in core::slice::{{impl}}::position::{{closure}}<std::sys_common::backtrace::Frame,closure> (elt=0x7fffffffcd80) at /var/tmp/paludis/build/dev-lang-rust-scm/work/rust-scm/src/libcore/slice/mod.rs:1204
        elt = 0x7fffffffcd80
        predicate = closure (0x7fffffffcc00)
        index = 0
#6  0x0000555555577be4 in core::slice::Iter<std::sys_common::backtrace::Frame>::search_while<std::sys_common::backtrace::Frame,core::option::Option<usize>,closure> (self=0x7fffffffcc28, default=..., g=...) at /var/tmp/paludis/build/dev-lang-rust-scm/work/rust-scm/src/libcore/slice/mod.rs:1271
        self = 0x7fffffffcc28
        default = core::option::Option::None
        g = closure (closure (0x7fffffffcc00), 0)
#7  0x000055555557ca9e in core::slice::{{impl}}::position<std::sys_common::backtrace::Frame,closure> (self=0x7fffffffcc28, predicate=...) at /var/tmp/paludis/build/dev-lang-rust-scm/work/rust-scm/src/libcore/slice/mod.rs:1203
        index = 0
        self = 0x7fffffffcc28
        predicate = closure (0x7fffffffcc00)
#8  0x0000555555581caf in std::sys_common::backtrace::filter_frames (frames=..., format=std::sys_common::backtrace::PrintFormat::Short, context=0x7fffffffd3d8) at src/libstd/sys_common/backtrace.rs:106
        skipped_after = 21
        skipped_before = 0
        frames = &[std::sys_common::backtrace::Frame] {data_ptr: 0x7fffffffcd80, length: 21}
        format = std::sys_common::backtrace::PrintFormat::Short
        context = 0x7fffffffd3d8
#9  0x00005555555810e7 in std::sys_common::backtrace::_print (w=..., format=std::sys_common::backtrace::PrintFormat::Short) at src/libstd/sys_common/backtrace.rs:71
        skipped_before = 93824995289048
        skipped_after = 140737331204104
        nb_frames = 21
        context = std::sys::imp::backtrace::BacktraceContext
        frames = [std::sys_common::backtrace::Frame {exact_position: 0x55555558834b <std::sys::imp::backtrace::tracing::imp::unwind_backtrace+91>, symbol_addr: 0x5555555882f0 <std::sys::imp::backtrace::tracing::imp::unwind_backtrace>}, std::sys_common::backtrace::Frame {exact_position: 0x5555555819c9 <std::sys_common::backtrace::_print+2729>, symbol_addr: 0x555555580f20 <std::sys_common::backtrace::_print>}, 
          std::sys_common::backtrace::Frame {exact_position: 0x555555580e9b <std::sys_common::backtrace::print+123>, symbol_addr: 0x555555580e20 <std::sys_common::backtrace::print>}, std::sys_common::backtrace::Frame {exact_position: 0x55555557047e <std::panicking::default_hook::{{closure}}+862>, symbol_addr: 0x555555570120 <std::panicking::default_hook::{{closure}}>}, 
          std::sys_common::backtrace::Frame {exact_position: 0x55555556ffbd <std::panicking::default_hook+1037>, symbol_addr: 0x55555556fbb0 <std::panicking::default_hook>}, std::sys_common::backtrace::Frame {exact_position: 0x555555570e6c <std::panicking::rust_panic_with_hook+396>, symbol_addr: 0x555555570ce0 <std::panicking::rust_panic_with_hook>}, 
          std::sys_common::backtrace::Frame {exact_position: 0x55555555ec55 <std::panicking::begin_panic<&str>+149>, symbol_addr: 0x55555555ebc0 <std::panicking::begin_panic<&str>>}, std::sys_common::backtrace::Frame {exact_position: 0x55555555eb1a <a::main+26>, symbol_addr: 0x55555555eb00 <a::main>}, 
          std::sys_common::backtrace::Frame {exact_position: 0x55555559779e <core::ops::function::FnOnce::call_once<fn(),()>+14>, symbol_addr: 0x555555597790 <core::ops::function::FnOnce::call_once<fn(),()>>}, 
          std::sys_common::backtrace::Frame {exact_position: 0x555555581efc <std::sys_common::backtrace::__rust_begin_short_backtrace<fn(),()>+28>, symbol_addr: 0x555555581ee0 <std::sys_common::backtrace::__rust_begin_short_backtrace<fn(),()>>}, std::sys_common::backtrace::Frame {exact_position: 0x5555555617f7 <std::rt::lang_start::{{closure}}+39>, symbol_addr: 0x5555555617d0 <std::rt::lang_start::{{closure}}>}, 
          std::sys_common::backtrace::Frame {exact_position: 0x5555555708c9 <std::panicking::try::do_call<closure,()>+73>, symbol_addr: 0x555555570880 <std::panicking::try::do_call<closure,()>>}, std::sys_common::backtrace::Frame {exact_position: 0x5555555e62a7 <__rust_try+23>, symbol_addr: 0x5555555e6290 <__rust_try>}, 
          std::sys_common::backtrace::Frame {exact_position: 0x5555555e61b7 <panic_unwind::__rust_maybe_catch_panic+87>, symbol_addr: 0x5555555e6160 <panic_unwind::__rust_maybe_catch_panic>}, std::sys_common::backtrace::Frame {exact_position: 0x5555555707fa <std::panicking::try<(),closure>+106>, symbol_addr: 0x555555570790 <std::panicking::try<(),closure>>}, 
          std::sys_common::backtrace::Frame {exact_position: 0x55555557e003 <std::panic::catch_unwind<closure,()>+51>, symbol_addr: 0x55555557dfd0 <std::panic::catch_unwind<closure,()>>}, std::sys_common::backtrace::Frame {exact_position: 0x55555556174d <std::rt::lang_start+269>, symbol_addr: 0x555555561640 <std::rt::lang_start>}, 
          std::sys_common::backtrace::Frame {exact_position: 0x55555555eb4d <main+45>, symbol_addr: 0x55555555eb20 <main>}, std::sys_common::backtrace::Frame {exact_position: 0x7ffff7201f49 <__libc_start_main+233>, symbol_addr: 0x7ffff7201e60 <__libc_start_main>}, std::sys_common::backtrace::Frame {exact_position: 0x55555555ea19 <_start+41>, symbol_addr: 0x55555555e9f0 <_start>}, 
          std::sys_common::backtrace::Frame {exact_position: 0x0, symbol_addr: 0x0} <repeats 80 times>]
        w = std::io::&mut Write
        format = std::sys_common::backtrace::PrintFormat::Short
#10 0x0000555555580e9c in std::sys_common::backtrace::print (w=..., format=std::sys_common::backtrace::PrintFormat::Short) at src/libstd/sys_common/backtrace.rs:58
        res = <error reading variable>
        w = std::io::&mut Write
        format = std::sys_common::backtrace::PrintFormat::Short
#11 0x000055555557047f in std::panicking::default_hook::{{closure}} (err=...) at src/libstd/panicking.rs:381
        format = std::sys_common::backtrace::PrintFormat::Short
        err = std::io::&mut Write
        name = &str {data_ptr: 0x7ffff6a21000 "main\000", length: 4}
        msg = &str {data_ptr: 0x555555609ac4 "\000", length: 0}
        file = &str {data_ptr: 0x555555609ac0 <str> "a.rs\000", length: 4}
        line = 2
        col = 4
        log_backtrace = core::option::Option<std::sys_common::backtrace::PrintFormat>::Some(std::sys_common::backtrace::PrintFormat::Short)
#12 0x000055555556ffbe in std::panicking::default_hook (info=0x7fffffffde88) at src/libstd/panicking.rs:397
        err = 0x7fffffffdd40
        stderr = std::io::Box<Write>
        prev = <error reading variable>
        write = closure (0x7fffffffdcc8, 0x7fffffffdc60, 0x7fffffffdc48, 0x7fffffffdc58, 0x7fffffffdc5c, 0x7fffffffdc37)
        name = &str {data_ptr: 0x7ffff6a21000 "main\000", length: 4}
        thread = core::option::Option<std::thread::Thread>::Some(std::thread::Thread {inner: alloc::arc::Arc<std::thread::Inner> {ptr: core::ptr::Shared<alloc::arc::ArcInner<std::thread::Inner>> {pointer: core::nonzero::NonZero<*const alloc::arc::ArcInner<std::thread::Inner>> (0x7ffff6a25000), _marker: core::marker::PhantomData<alloc::arc::ArcInner<std::thread::Inner>>}}})
        err = core::option::Option::Some(std::sys::imp::stdio::Stderr (()))
        msg = &str {data_ptr: 0x555555609ac4 "\000", length: 0}
        col = 4
        line = 2
        file = &str {data_ptr: 0x555555609ac0 <str> "a.rs\000", length: 4}
        log_backtrace = core::option::Option<std::sys_common::backtrace::PrintFormat>::Some(std::sys_common::backtrace::PrintFormat::Short)
        info = 0x7fffffffde88
#13 0x0000555555570e6d in std::panicking::rust_panic_with_hook (msg=..., file_line_col=0x55555583dc30 <ref>) at src/libstd/panicking.rs:577
        info = std::panicking::PanicInfo {payload: core::any::&Any, location: std::panicking::Location {file: &str {data_ptr: 0x555555609ac0 <str> "a.rs\000", length: 4}, line: 2, col: 4}}
        panics = 1
        file = &str {data_ptr: 0x555555609ac0 <str> "a.rs\000", length: 4}
        line = 2
        col = 4
        msg = core::any::Box<Any>
        file_line_col = 0x55555583dc30 <ref>
#14 0x000055555555ec56 in std::panicking::begin_panic<&str> (msg=..., file_line_col=0x55555583dc30 <ref>) at /var/tmp/paludis/build/dev-lang-rust-scm/work/rust-scm/src/libstd/panicking.rs:538
        msg = &str {data_ptr: 0x555555609ac4 "\000", length: 0}
        file_line_col = 0x55555583dc30 <ref>
#15 0x000055555555eb1b in a::main () at a.rs:2
