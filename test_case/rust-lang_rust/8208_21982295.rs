 rust
rust: task failed at 'borrowed', /home/huon/rust/src/librustc/middle/resolve.rs:469
/usr/local/bin/../lib/librustrt.so(_ZN9rust_task13begin_failureEPKcS1_m+0x4b)[0x7f697c3c185b]
/usr/local/bin/../lib/librustrt.so(+0x2ab51)[0x7f697c3d2b51]
/usr/local/bin/../lib/librustrt.so(upcall_fail+0x198)[0x7f697c3c3808]
/usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so(_ZN3sys13begin_unwind_16_89e154cd091567114_0$x2e8$x2dpreE+0x569)[0x7f697f7a4809]
/usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so(+0x172a5f)[0x7f697f802a5f]
/usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so(+0xf96fa)[0x7f697f7896fa]
/usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so(_ZN2rt8borrowck13fail_borrowed15_075e556c6b348a14_0$x2e8$x2dpreE+0x168)[0x7f697f8023d8]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(_ZN6middle7resolve14__extensions__10meth_4817020all_imports_resolved17_dc424d9ab87fb95b14_0$x2e8$x2dpreE+0x65)[0x7f697e1bdfb5]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(_ZN6middle7resolve14__extensions__10meth_5031219resolve_glob_import17_48c3c6a071d4b2e514_0$x2e8$x2dpreE+0xb1)[0x7f697e1f5a61]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(_ZN6middle7resolve14__extensions__10meth_5027725resolve_import_for_module17_fe96ed4c25c477db14_0$x2e8$x2dpreE+0x234)[0x7f697e1f17f4]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(_ZN6middle7resolve14__extensions__10meth_5020926resolve_imports_for_module17_dcb755815ce2399b14_0$x2e8$x2dpreE+0x1a4)[0x7f697e1f0ff4]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(_ZN6middle7resolve14__extensions__10meth_5020534resolve_imports_for_module_subtree17_dcb755815ce2399b14_0$x2e8$x2dpreE+0x48)[0x7f697e1ef458]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(_ZN6middle7resolve14__extensions__10meth_4878515resolve_imports15_f3d16eaf7d573814_0$x2e8$x2dpreE+0x7b)[0x7f697e1c739b]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(_ZN6middle7resolve14__extensions__10meth_487717resolve15_f3d16eaf7d573814_0$x2e8$x2dpreE+0x52)[0x7f697e1c6662]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(_ZN6middle7resolve13resolve_crate17_97bb199cc8ea87b814_0$x2e8$x2dpreE+0xb1)[0x7f697e230681]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(_ZN6driver6driver12compile_rest15_4bcc908e4730a114_0$x2e8$x2dpreE+0xea6)[0x7f697e6878c6]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(_ZN6driver6driver12compile_upto17_137ef03814afa89c14_0$x2e8$x2dpreE+0x134)[0x7f697e68ba34]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(_ZN6driver6driver13compile_input17_b7dc6f2c2099232314_0$x2e8$x2dpreE+0xd5)[0x7f697e68bd95]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(_ZN12run_compiler17_7aefb92a92c4c2b414_0$x2e8$x2dpreE+0x18ec)[0x7f697e6af66c]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(+0x80e99e)[0x7f697e6ce99e]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(+0x80a664)[0x7f697e6ca664]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(+0x803d02)[0x7f697e6c3d02]
/usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so(+0xec0af)[0x7f697f77c0af]
/usr/local/bin/../lib/librustrt.so(_Z18task_start_wrapperP10spawn_args+0x22)[0x7f697c3c2192]
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug
note: try running with RUST_LOG=rustc=1,::rt::backtrace to get further details and report the results to github.com/mozilla/rust/issues
rust: task failed at 'explicit failure', /home/huon/rust/src/librustc/rustc.rs:364
/usr/local/bin/../lib/librustrt.so(_ZN9rust_task13begin_failureEPKcS1_m+0x4b)[0x7f697c3c185b]
/usr/local/bin/../lib/librustrt.so(+0x2ab51)[0x7f697c3d2b51]
/usr/local/bin/../lib/librustrt.so(upcall_fail+0x198)[0x7f697c3c3808]
/usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so(_ZN3sys13begin_unwind_16_89e154cd091567114_0$x2e8$x2dpreE+0x569)[0x7f697f7a4809]
/usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so(+0x114942)[0x7f697f7a4942]
/usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so(+0x114118)[0x7f697f7a4118]
/usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so(+0x1148f1)[0x7f697f7a48f1]
/usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so(+0x114118)[0x7f697f7a4118]
/usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so(+0x8b71c)[0x7f697f71b71c]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(_ZN7monitor16_cd19199377934ec14_0$x2e8$x2dpreE+0x2f85)[0x7f697e6b3185]
/usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so(_ZN4main15_f3d16eaf7d573814_0$x2e8$x2dpreE+0x69)[0x7f697e6ce8d9]
/usr/local/bin/../lib/librustrt.so(_Z18task_start_wrapperP10spawn_args+0x22)[0x7f697c3c2192]
rust: domain main @0xa3dcd0 root task failed
