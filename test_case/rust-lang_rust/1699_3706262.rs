
joshua@nyus:~/rust/build$ RUST_LOG=rustc=0,::rt::backtrace !!
RUST_LOG=rustc=0,::rt::backtrace rustc smash.rs
smash.rs:4:4: 4:20 error: unresolved modulename: optn
smash.rs:4     core::optn::some() { }
               ^~~~~~~~~~~~~~~~
rust: upcall fail 'option none', ../src/libcore/option.rs:34
/usr/local/bin/../lib/librustrt.so(_ZN9rust_task4failEv+0x25)[0x7fd54306a9f5]
/usr/local/bin/../lib/librustrt.so(+0x33119)[0x7fd543081119]
/usr/local/bin/../lib/librustrt.so(upcall_fail+0x39)[0x7fd54306dec9]
/usr/local/bin/../lib/libcore-14bd852465126fe7-0.1.so(_ZN6option3get17_e9fe5bbdaa8f1343E+0xb4)[0x7fd543ebded4]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(_ZN6middle7resolve13resolve_names8walk_pat17_df5733ccc384791dE+0x1d3)[0x7fd54365e183]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(_ZN6middle7resolve20visit_arm_with_scope17_a3ffd851e87a2377E+0x81)[0x7fd54365fed1]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(_ZN6syntax5visit10visit_expr17_2191d7c991221f0bE+0x7e3)[0x7fd5436fb3c3]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(_ZN6middle7resolve21visit_expr_with_scope17_3a65aad0de13a35eE+0x1da)[0x7fd54366023a]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(_ZN6middle7resolve13resolve_names9walk_expr17_222d86f3ff88133eE+0x4d)[0x7fd54365dafd]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(_ZN6middle7resolve22visit_block_with_scope17_ee5d1a2a4be1e211E+0x2d9)[0x7fd54365fc29]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(_ZN6syntax5visit8visit_fn17_caa06b313b8ad9b4E+0xe7)[0x7fd5436fa6b7]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(_ZN6middle7resolve19visit_fn_with_scope17_a811fd977ead6f7eE+0x745)[0x7fd54365f8d5]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(+0x2b3494)[0x7fd5437eb494]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(_ZN6syntax5visit10visit_item17_5c3aa7827bc98c1fE+0x184)[0x7fd5436f96e4]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(_ZN6middle7resolve21visit_item_with_scope17_cf55e0e4dfaaafecE+0xa8a)[0x7fd54365ef1a]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(+0x304318)[0x7fd54383c318]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(_ZN6syntax5visit9visit_mod17_24466afaa9e1a8b7E+0xca)[0x7fd5436f944a]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(+0x2de702)[0x7fd543816702]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(_ZN6middle7resolve13resolve_names17_e9f61c552e89e646E+0x41a)[0x7fd54365d91a]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(_ZN6middle7resolve13resolve_crate17_c4c27222d2158d0aE+0x2fa)[0x7fd54365ab3a]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(_ZN6driver6driver4time17_6224b963f79affc8E+0x8c)[0x7fd5437a843c]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(_ZN6driver6driver12compile_upto17_bd712d43acc149bfE+0x957)[0x7fd5437a9677]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(+0x304318)[0x7fd54383c318]
/usr/local/bin/../lib/librustc-4171d83aef249987-0.1.so(_ZN6driver6driver13compile_input17_7c468f6ed9ece9faE+0xc2)[0x7fd5437aac62]
rustc[0x405116]
rustc[0x407918]
rustc[0x40745a]
rustc[0x406f5e]
/usr/local/bin/../lib/libcore-14bd852465126fe7-0.1.so(+0x40452)[0x7fd543ecd452]
/usr/local/bin/../lib/librustrt.so(task_start_wrapper+0x32)[0x7fd543069bd2]
error: internal compiler error unexpected failure
note: The compiler hit an unexpected failure path. This is a bug. Try running with RUST_LOG=rustc=0,::rt::backtrace to get further details and report the results to github.com/mozilla/rust/issues
rust: upcall fail 'explicit failure', ../src/comp/driver/rustc.rs:175
/usr/local/bin/../lib/librustrt.so(_ZN9rust_task4failEv+0x25)[0x7fd54306a9f5]
/usr/local/bin/../lib/librustrt.so(+0x33119)[0x7fd543081119]
/usr/local/bin/../lib/librustrt.so(upcall_fail+0x39)[0x7fd54306dec9]
rustc[0x4054c2]
rustc[0x4056fc]
/usr/local/bin/../lib/librustrt.so(task_start_wrapper+0x32)[0x7fd543069bd2]
rust: domain main @0x1ed1c80 root task failed
