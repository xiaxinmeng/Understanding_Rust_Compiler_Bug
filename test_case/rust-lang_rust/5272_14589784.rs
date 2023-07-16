
tony@tony-ubuntu:~/Projects/rust-apl$ RUST_LOG=rustc=1,::rt::backtrace rustc src/rust-apl.rc -o bin/test --test
src/tests.rs:7:9: 7:17 warning: unused variable: `string`
src/tests.rs:7         let string = ~"1";
                        ^~~~~~~~
src/tests.rs:8:12: 8:20 warning: unused variable: `string`
src/tests.rs:8         let string = ~"321";
                           ^~~~~~~~
src/tests.rs:11:12: 11:20 warning: unused variable: `string`
src/tests.rs:11         let string = ~"'This is a string'";
                            ^~~~~~~~
src/tests.rs:14:12: 14:20 warning: unused variable: `string`
src/tests.rs:14         let string = ~"⍒";
                            ^~~~~~~~
src/tests.rs:15:12: 15:20 warning: unused variable: `string`
src/tests.rs:15         let string = ~"÷";
                            ^~~~~~~~
rust: task failed at 'Assertion bpos == mbc.pos || bpos.to_uint() >= mbc.pos.to_uint() + mbc.bytes failed', /home/tony/Projects/rust/src/libsyntax/codemap.rs:500
/usr/local/bin/../lib/librustrt.so(_ZN9rust_task13begin_failureEPKcS1_m+0x4b)[0x7f0ba053ef9b]
/usr/local/bin/../lib/librustrt.so(+0x2b109)[0x7f0ba0550109]
/usr/local/bin/../lib/librustrt.so(upcall_fail+0x188)[0x7f0ba0540d78]
/usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(+0xf8a5b)[0x7f0ba2081a5b]
/usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(+0x413fe)[0x7f0ba1fca3fe]
/usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so(_ZN7codemap14__extensions__9meth_925124bytepos_to_local_charpos17_2ef2f680debabbad3_06E+0x3de)[0x7f0ba155149e]
/usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so(_ZN7codemap14__extensions__9meth_911110lookup_pos17_fa9d322f72fd53163_06E+0x7b)[0x7f0ba154f55b]
/usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so(_ZN7codemap14__extensions__9meth_911319lookup_char_pos_adj16_49afbe1de9c0f593_06E+0x54)[0x7f0ba154fc64]
/usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so(_ZN7codemap14__extensions__9meth_851411span_to_str17_cac44644182bf88b3_06E+0xe6)[0x7f0ba154c5b6]
/usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so(_ZN10diagnostic4emit16_d2f47fc894c93203_06E+0xe5)[0x7f0ba1547fb5]
/usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so(+0x2811b8)[0x7f0ba17661b8]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x8ea193)[0x7f0ba106b193]
/usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so(_ZN10diagnostic14__extensions__9meth_81704emit16_d2f47fc894c93203_06E+0x4d)[0x7f0ba15463bd]
/usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so(_ZN10diagnostic14__extensions__9meth_81209span_warn14_2f48a8a4d33843_06E+0x10e)[0x7f0ba154547e]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x6dc93e)[0x7f0ba0e5d93e]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(_ZN6middle8liveness14__extensions__10meth_6921217warn_about_unused17_eaa396e49e4084683_06E+0xe4)[0x7f0ba0e5a174]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x6dc5b6)[0x7f0ba0e5d5b6]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x6d03dc)[0x7f0ba0e513dc]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x701f2a)[0x7f0ba0e82f2a]
/usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so(_ZN8ast_util8walk_pat17_b8bf7b22f55e85f03_06E+0x45)[0x7f0ba1598205]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(_ZN6middle8liveness14__extensions__10meth_6883112pat_bindings16_ddf8809261f68fb3_06E+0xaa)[0x7f0ba0e5129a]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x6cc99c)[0x7f0ba0e4d99c]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x1f9226)[0x7f0ba097a226]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x1f7ce1)[0x7f0ba0978ce1]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x1eaafa)[0x7f0ba096bafa]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x6bd832)[0x7f0ba0e3e832]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x1f6388)[0x7f0ba0977388]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x1f5c24)[0x7f0ba0976c24]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x1f63f9)[0x7f0ba09773f9]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x1f5c24)[0x7f0ba0976c24]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x1f63f9)[0x7f0ba09773f9]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x1f5c24)[0x7f0ba0976c24]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(_ZN6middle8liveness11check_crate17_1628ef44bfc587c93_06E+0x40e)[0x7f0ba0e3448e]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x8a92f9)[0x7f0ba102a2f9]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x8a7f38)[0x7f0ba1028f38]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(_ZN6driver6driver12compile_rest17_ae80a5494db8f5f03_06E+0x1d98)[0x7f0ba10262e8]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x8ec184)[0x7f0ba106d184]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(_ZN6driver6driver12compile_upto16_3511ccef3fbc8173_06E+0x107)[0x7f0ba102a677]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(_ZN6driver6driver13compile_input17_da16f137dd9fced53_06E+0xb9)[0x7f0ba102aa79]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(_ZN12run_compiler16_a31e58461d354de3_06E+0x223a)[0x7f0ba1058d2a]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x8e90e1)[0x7f0ba106a0e1]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x8e774e)[0x7f0ba106874e]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x8ec184)[0x7f0ba106d184]
/usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(+0xc3e5f)[0x7f0ba204ce5f]
/usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(+0x129f60)[0x7f0ba20b2f60]
/usr/local/bin/../lib/librustrt.so(_Z18task_start_wrapperP10spawn_args+0x24)[0x7f0ba053f7e4]
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug
note: try running with RUST_LOG=rustc=1,::rt::backtrace to get further details and report the results to github.com/mozilla/rust/issues
rust: task failed at 'explicit failure', /home/tony/Projects/rust/src/librustc/rustc.rc:364
/usr/local/bin/../lib/librustrt.so(_ZN9rust_task13begin_failureEPKcS1_m+0x4b)[0x7f0ba053ef9b]
/usr/local/bin/../lib/librustrt.so(+0x2b109)[0x7f0ba0550109]
/usr/local/bin/../lib/librustrt.so(upcall_fail+0x188)[0x7f0ba0540d78]
/usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(+0xf8a5b)[0x7f0ba2081a5b]
/usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(+0xf8a02)[0x7f0ba2081a02]
/usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(_ZN3sys12begin_unwind17_7cd364c41f10422f3_06E+0x71)[0x7f0ba1fca891]
/usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(+0x129f60)[0x7f0ba20b2f60]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(_ZN7monitor17_fcefdec84997cb1d3_06E+0x5c85)[0x7f0ba105f8a5]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(+0x8ec184)[0x7f0ba106d184]
/usr/local/bin/../lib/librustc-c84825241471686d-0.6.so(_ZN4main16_706f4ee7413ae583_06E+0x7e)[0x7f0ba106cd9e]
/usr/local/bin/../lib/librustrt.so(_Z18task_start_wrapperP10spawn_args+0x24)[0x7f0ba053f7e4]
rust: domain main @0x1cae3c0 root task failed
