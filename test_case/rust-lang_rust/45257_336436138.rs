
abrr>vreiooieoeriotothread 'main' panicked at 'cannot remove a char from the end of a string', /checkout/src/liballoc/string.rs:1020:20
                                                       note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
                                                              stack backtrace:
                                                                                 0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
                                                                         at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
                                                          1: std::sys_common::backtrace::_print
                            at /checkout/src/libstd/sys_common/backtrace.rs:71
                                                                                 2: std::panicking::default_hook::{{closure}}
                                                          at /checkout/src/libstd/sys_common/backtrace.rs:60
                                         at /checkout/src/libstd/panicking.rs:380
    3: std::panicking::default_hook
                                                at /checkout/src/libstd/panicking.rs:396
           4: std::panicking::rust_panic_with_hook
                                                               at /checkout/src/libstd/panicking.rs:611
                          5: std::panicking::begin_panic_new
                                                                         at /checkout/src/libstd/panicking.rs:553
                                    6: std::panicking::begin_panic_fmt
                                                                                   at /checkout/src/libstd/panicking.rs:521
                                              7: rust_begin_unwind
                                                                               at /checkout/src/libstd/panicking.rs:497
                                          8: core::panicking::panic_fmt
                                                                                    at /checkout/src/libcore/panicking.rs:92
                                               9: core::panicking::panic
                                                                                     at /checkout/src/libcore/panicking.rs:51
                                               10: alloc::string::String::remove
                                                                                            at /checkout/obj/<panic macros>:4
                                               11: mouse_readline::read_line
                                                                                         at src/lib.rs:24
                           12: read::main
                                                      at examples/read.rs:4
