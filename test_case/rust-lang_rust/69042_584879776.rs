plain
2020-02-11T20:52:11.3332551Z ========================== Starting Command Output ===========================
2020-02-11T20:52:11.3333965Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/56c1d556-4b87-4683-83dc-87ac2340e74e.sh
2020-02-11T20:52:11.3333994Z 
2020-02-11T20:52:11.3337115Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-11T20:52:11.3343235Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69042/merge to s
2020-02-11T20:52:11.3345004Z Task         : Get sources
2020-02-11T20:52:11.3345078Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T20:52:11.3345104Z Version      : 1.0.0
2020-02-11T20:52:11.3345130Z Author       : Microsoft
---
2020-02-11T20:52:12.3364752Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-11T20:52:12.3376630Z ##[command]git config gc.auto 0
2020-02-11T20:52:12.3398550Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-11T20:52:12.3400703Z ##[command]git config --get-all http.proxy
2020-02-11T20:52:12.3407862Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69042/merge:refs/remotes/pull/69042/merge
---
2020-02-11T21:55:41.1506357Z .................................................................................................... 1700/9627
2020-02-11T21:55:46.7117410Z .................................................................................................... 1800/9627
2020-02-11T21:55:59.8541497Z ..............................i..................................................................... 1900/9627
2020-02-11T21:56:08.4463723Z .................................................................................................... 2000/9627
2020-02-11T21:56:24.4820143Z ....................iiiii........................................................................... 2100/9627
2020-02-11T21:56:35.2613634Z .................................................................................................... 2300/9627
2020-02-11T21:56:37.9362569Z .................................................................................................... 2400/9627
2020-02-11T21:56:43.1475242Z .................................................................................................... 2500/9627
2020-02-11T21:57:05.8590952Z .................................................................................................... 2600/9627
---
2020-02-11T22:00:00.3112577Z .......................................................................i...............i............ 4900/9627
2020-02-11T22:00:08.9156464Z .................................................................................................... 5000/9627
2020-02-11T22:00:17.7023619Z .................................................................................................... 5100/9627
2020-02-11T22:00:22.8196277Z .............i...................................................................................... 5200/9627
2020-02-11T22:00:35.2020610Z .......................................................................................ii.ii........ 5300/9627
2020-02-11T22:00:39.4880130Z i...i............................................................................................... 5400/9627
2020-02-11T22:00:52.7123330Z .................................................................................................... 5600/9627
2020-02-11T22:01:01.7306211Z ...........................................................................i........................ 5700/9627
2020-02-11T22:01:09.8458338Z .................................................................................................... 5800/9627
2020-02-11T22:01:16.9125331Z .................................................................................................... 5900/9627
2020-02-11T22:01:16.9125331Z .................................................................................................... 5900/9627
2020-02-11T22:01:28.1918301Z ...................................................................ii...i..ii...........i........... 6000/9627
2020-02-11T22:01:52.3031275Z .................................................................................................... 6200/9627
2020-02-11T22:02:00.8293031Z .................................................................................................... 6300/9627
2020-02-11T22:02:09.7821646Z ...............................................................................................i..ii 6400/9627
2020-02-11T22:02:28.8739352Z .................................................................................................... 6500/9627
---
2020-02-11T22:04:45.2960967Z .................................................................................................... 7600/9627
2020-02-11T22:04:50.0706888Z .................................................................................................... 7700/9627
2020-02-11T22:04:56.5788851Z .................................................................................................... 7800/9627
2020-02-11T22:05:05.9952018Z .................................................................................................... 7900/9627
2020-02-11T22:05:16.0686129Z .....................................................................iiiiiii.i...................... 8000/9627
2020-02-11T22:05:33.9881312Z .........i......i................................................................................... 8200/9627
2020-02-11T22:05:40.0638105Z .................................................................................................... 8300/9627
2020-02-11T22:05:54.9802114Z ..................................F................................................................. 8400/9627
2020-02-11T22:06:05.5975243Z .................................................................................................... 8500/9627
---
2020-02-11T22:08:15.7037880Z 
2020-02-11T22:08:15.7038255Z ------------------------------------------
2020-02-11T22:08:15.7038456Z stderr:
2020-02-11T22:08:15.7038826Z ------------------------------------------
2020-02-11T22:08:15.7039342Z thread 'main' panicked at 'bad output: thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace.rs:18:9
2020-02-11T22:08:15.7039567Z    0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
2020-02-11T22:08:15.7039875Z    2: std::io::Write::write_fmt
2020-02-11T22:08:15.7040041Z    3: std::panicking::default_hook::{{closure}}
2020-02-11T22:08:15.7040188Z    4: std::panicking::default_hook
2020-02-11T22:08:15.7040336Z    5: std::panicking::rust_panic_with_hook
---
2020-02-11T22:08:15.7042197Z   12: std::rt::lang_start_internal
2020-02-11T22:08:15.7042363Z   13: main
2020-02-11T22:08:15.7042497Z   14: __libc_start_main
2020-02-11T22:08:15.7042631Z   15: _start
2020-02-11T22:08:15.7042796Z note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
2020-02-11T22:08:15.7043174Z ', /checkout/src/test/ui/backtrace.rs:68:5
2020-02-11T22:08:15.7043517Z 
2020-02-11T22:08:15.7044078Z ------------------------------------------
2020-02-11T22:08:15.7044263Z 
2020-02-11T22:08:15.7044421Z 
---
2020-02-11T22:08:15.7046262Z ------------------------------------------
2020-02-11T22:08:15.7046884Z ---------------------------------------
2020-02-11T22:08:15.7047089Z no backtrace found in stderr:
2020-02-11T22:08:15.7047232Z test case 0
2020-02-11T22:08:15.7047639Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:86:5
2020-02-11T22:08:15.7048160Z    0:     0x7f0121d81d98 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h773996ea8c7d87fd
2020-02-11T22:08:15.7048583Z    1:     0x7f0121dbb70c - core::fmt::write::hc6fac2af734bcbe6
2020-02-11T22:08:15.7049030Z    2:     0x7f0121d72857 - std::io::Write::write_fmt::h32f7c9d1c0ecde3b
2020-02-11T22:08:15.7049582Z    3:     0x7f0121d86d84 - std::panicking::default_hook::{{closure}}::h590e3843a43b4171
2020-02-11T22:08:15.7050061Z    4:     0x7f0121d86aae - std::panicking::default_hook::h4f44564c405cb696
2020-02-11T22:08:15.7050534Z    5:     0x7f0121d873e5 - std::panicking::rust_panic_with_hook::h75d505592f54cc4e
2020-02-11T22:08:15.7050973Z    6:     0x55cd7c968522 - std::panicking::begin_panic::h835adbb927912903
2020-02-11T22:08:15.7051171Z                                at /checkout/src/libstd/panicking.rs:397
2020-02-11T22:08:15.7051596Z    7:     0x55cd7c9603bb - backtrace_debuginfo::inner::hba9feb2184e80f2b
2020-02-11T22:08:15.7052032Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:86
2020-02-11T22:08:15.7052480Z    8:     0x55cd7c960c41 - backtrace_debuginfo::outer::h681fb285db800a84
2020-02-11T22:08:15.7052933Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:125
2020-02-11T22:08:15.7053579Z    9:     0x55cd7c961941 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-02-11T22:08:15.7054137Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-11T22:08:15.7054591Z   10:     0x55cd7c9570ee - std::rt::lang_start::{{closure}}::h713fbc1b2737288b
2020-02-11T22:08:15.7054801Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7055207Z   11:     0x7f0121d86ea3 - std::panicking::try::do_call::h73491d1a398c4d9f
2020-02-11T22:08:15.7055632Z   12:     0x7f0121d99da7 - __rust_maybe_catch_panic
2020-02-11T22:08:15.7056088Z   13:     0x7f0121d87958 - std::rt::lang_start_internal::h5c3aea9777b113bf
2020-02-11T22:08:15.7056515Z   14:     0x55cd7c9570d5 - std::rt::lang_start::he9bb1e94b62ebf5d
2020-02-11T22:08:15.7056714Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7057142Z   15:     0x55cd7c9619e2 - main
2020-02-11T22:08:15.7057574Z   16:     0x7f012172cb97 - __libc_start_main
2020-02-11T22:08:15.7057989Z   17:     0x55cd7c950aba - _start
2020-02-11T22:08:15.7058423Z   18:                0x0 - <unknown>
2020-02-11T22:08:15.7058978Z ---------------------------------------
2020-02-11T22:08:15.7059193Z no backtrace found in stderr:
2020-02-11T22:08:15.7059342Z test case 1
2020-02-11T22:08:15.7059778Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:87:5
2020-02-11T22:08:15.7059778Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:87:5
2020-02-11T22:08:15.7060329Z    0:     0x7f0a95a4fd98 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h773996ea8c7d87fd
2020-02-11T22:08:15.7061049Z    1:     0x7f0a95a8970c - core::fmt::write::hc6fac2af734bcbe6
2020-02-11T22:08:15.7061561Z    2:     0x7f0a95a40857 - std::io::Write::write_fmt::h32f7c9d1c0ecde3b
2020-02-11T22:08:15.7062273Z    3:     0x7f0a95a54d84 - std::panicking::default_hook::{{closure}}::h590e3843a43b4171
2020-02-11T22:08:15.7064614Z    4:     0x7f0a95a54aae - std::panicking::default_hook::h4f44564c405cb696
2020-02-11T22:08:15.7065112Z    5:     0x7f0a95a553e5 - std::panicking::rust_panic_with_hook::h75d505592f54cc4e
2020-02-11T22:08:15.7065400Z    6:     0x555ba0193522 - std::panicking::begin_panic::h835adbb927912903
2020-02-11T22:08:15.7065475Z                                at /checkout/src/libstd/panicking.rs:397
2020-02-11T22:08:15.7065745Z    7:     0x555ba018b4bd - backtrace_debuginfo::inner::hba9feb2184e80f2b
2020-02-11T22:08:15.7066010Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:87
2020-02-11T22:08:15.7066452Z    8:     0x555ba018bc41 - backtrace_debuginfo::outer::h681fb285db800a84
2020-02-11T22:08:15.7066725Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:125
2020-02-11T22:08:15.7066992Z    9:     0x555ba018c941 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-02-11T22:08:15.7067278Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-11T22:08:15.7067553Z   10:     0x555ba01820ee - std::rt::lang_start::{{closure}}::h713fbc1b2737288b
2020-02-11T22:08:15.7067608Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7067971Z   11:     0x7f0a95a54ea3 - std::panicking::try::do_call::h73491d1a398c4d9f
2020-02-11T22:08:15.7068247Z   12:     0x7f0a95a67da7 - __rust_maybe_catch_panic
2020-02-11T22:08:15.7068518Z   13:     0x7f0a95a55958 - std::rt::lang_start_internal::h5c3aea9777b113bf
2020-02-11T22:08:15.7068808Z   14:     0x555ba01820d5 - std::rt::lang_start::he9bb1e94b62ebf5d
2020-02-11T22:08:15.7068861Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7069087Z   15:     0x555ba018c9e2 - main
2020-02-11T22:08:15.7069322Z   16:     0x7f0a953fab97 - __libc_start_main
2020-02-11T22:08:15.7069571Z   17:     0x555ba017baba - _start
2020-02-11T22:08:15.7069796Z   18:                0x0 - <unknown>
2020-02-11T22:08:15.7070066Z ---------------------------------------
2020-02-11T22:08:15.7070113Z no backtrace found in stderr:
2020-02-11T22:08:15.7070154Z test case 2
2020-02-11T22:08:15.7070440Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:89:9
2020-02-11T22:08:15.7070440Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:89:9
2020-02-11T22:08:15.7070787Z    0:     0x7f0234d9cd98 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h773996ea8c7d87fd
2020-02-11T22:08:15.7071048Z    1:     0x7f0234dd670c - core::fmt::write::hc6fac2af734bcbe6
2020-02-11T22:08:15.7071338Z    2:     0x7f0234d8d857 - std::io::Write::write_fmt::h32f7c9d1c0ecde3b
2020-02-11T22:08:15.7071620Z    3:     0x7f0234da1d84 - std::panicking::default_hook::{{closure}}::h590e3843a43b4171
2020-02-11T22:08:15.7071886Z    4:     0x7f0234da1aae - std::panicking::default_hook::h4f44564c405cb696
2020-02-11T22:08:15.7072176Z    5:     0x7f0234da23e5 - std::panicking::rust_panic_with_hook::h75d505592f54cc4e
2020-02-11T22:08:15.7072447Z    6:     0x5573579b5522 - std::panicking::begin_panic::h835adbb927912903
2020-02-11T22:08:15.7072500Z                                at /checkout/src/libstd/panicking.rs:397
2020-02-11T22:08:15.7072790Z    7:     0x5573579a5dcb - backtrace_debuginfo::inner::{{closure}}::he203707f23085cd6
2020-02-11T22:08:15.7073067Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:89
2020-02-11T22:08:15.7073344Z    8:     0x5573579a5c86 - backtrace_debuginfo::aux::callback::h7593cc62965c93e6
2020-02-11T22:08:15.7073631Z                                at /checkout/src/test/ui/backtrace-debuginfo-aux.rs:6
2020-02-11T22:08:15.7073915Z    9:     0x5573579ad41f - backtrace_debuginfo::inner::hba9feb2184e80f2b
2020-02-11T22:08:15.7074181Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:88
2020-02-11T22:08:15.7074465Z   10:     0x5573579adc41 - backtrace_debuginfo::outer::h681fb285db800a84
2020-02-11T22:08:15.7075002Z   11:     0x5573579ae941 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-02-11T22:08:15.7075290Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-11T22:08:15.7075290Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-11T22:08:15.7075572Z   12:     0x5573579a40ee - std::rt::lang_start::{{closure}}::h713fbc1b2737288b
2020-02-11T22:08:15.7075626Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7075912Z   13:     0x7f0234da1ea3 - std::panicking::try::do_call::h73491d1a398c4d9f
2020-02-11T22:08:15.7076158Z   14:     0x7f0234db4da7 - __rust_maybe_catch_panic
2020-02-11T22:08:15.7076525Z   15:     0x7f0234da2958 - std::rt::lang_start_internal::h5c3aea9777b113bf
2020-02-11T22:08:15.7076809Z   16:     0x5573579a40d5 - std::rt::lang_start::he9bb1e94b62ebf5d
2020-02-11T22:08:15.7076861Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7077086Z   17:     0x5573579ae9e2 - main
2020-02-11T22:08:15.7077339Z   18:     0x7f0234747b97 - __libc_start_main
2020-02-11T22:08:15.7077564Z   19:     0x55735799daba - _start
2020-02-11T22:08:15.7077786Z   20:                0x0 - <unknown>
2020-02-11T22:08:15.7078055Z ---------------------------------------
2020-02-11T22:08:15.7078171Z no backtrace found in stderr:
2020-02-11T22:08:15.7078218Z test case 3
2020-02-11T22:08:15.7078537Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:92:9
2020-02-11T22:08:15.7078537Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:92:9
2020-02-11T22:08:15.7078863Z    0:     0x7f700f269d98 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h773996ea8c7d87fd
2020-02-11T22:08:15.7079135Z    1:     0x7f700f2a370c - core::fmt::write::hc6fac2af734bcbe6
2020-02-11T22:08:15.7079418Z    2:     0x7f700f25a857 - std::io::Write::write_fmt::h32f7c9d1c0ecde3b
2020-02-11T22:08:15.7079701Z    3:     0x7f700f26ed84 - std::panicking::default_hook::{{closure}}::h590e3843a43b4171
2020-02-11T22:08:15.7079967Z    4:     0x7f700f26eaae - std::panicking::default_hook::h4f44564c405cb696
2020-02-11T22:08:15.7080259Z    5:     0x7f700f26f3e5 - std::panicking::rust_panic_with_hook::h75d505592f54cc4e
2020-02-11T22:08:15.7080528Z    6:     0x55905c818522 - std::panicking::begin_panic::h835adbb927912903
2020-02-11T22:08:15.7080590Z                                at /checkout/src/libstd/panicking.rs:397
2020-02-11T22:08:15.7080886Z    7:     0x55905c81066b - backtrace_debuginfo::inner::{{closure}}::hfb5f4b951951d06d
2020-02-11T22:08:15.7081156Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:92
2020-02-11T22:08:15.7081448Z    8:     0x55905c8100d6 - backtrace_debuginfo::aux::callback_inlined::h5a51bb5d7c9bc904
2020-02-11T22:08:15.7081738Z                                at /checkout/src/test/ui/backtrace-debuginfo-aux.rs:13
2020-02-11T22:08:15.7082008Z    9:     0x55905c810515 - backtrace_debuginfo::inner::hba9feb2184e80f2b
2020-02-11T22:08:15.7082273Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:91
2020-02-11T22:08:15.7084366Z   10:     0x55905c810c41 - backtrace_debuginfo::outer::h681fb285db800a84
2020-02-11T22:08:15.7084928Z   11:     0x55905c811941 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-02-11T22:08:15.7085212Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-11T22:08:15.7085212Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-11T22:08:15.7085477Z   12:     0x55905c8070ee - std::rt::lang_start::{{closure}}::h713fbc1b2737288b
2020-02-11T22:08:15.7085529Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7085813Z   13:     0x7f700f26eea3 - std::panicking::try::do_call::h73491d1a398c4d9f
2020-02-11T22:08:15.7086053Z   14:     0x7f700f281da7 - __rust_maybe_catch_panic
2020-02-11T22:08:15.7086313Z   15:     0x7f700f26f958 - std::rt::lang_start_internal::h5c3aea9777b113bf
2020-02-11T22:08:15.7086583Z   16:     0x55905c8070d5 - std::rt::lang_start::he9bb1e94b62ebf5d
2020-02-11T22:08:15.7086633Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7086852Z   17:     0x55905c8119e2 - main
2020-02-11T22:08:15.7087098Z   18:     0x7f700ec14b97 - __libc_start_main
2020-02-11T22:08:15.7087320Z   19:     0x55905c800aba - _start
2020-02-11T22:08:15.7087544Z   20:                0x0 - <unknown>
2020-02-11T22:08:15.7087813Z ---------------------------------------
2020-02-11T22:08:15.7087858Z no backtrace found in stderr:
2020-02-11T22:08:15.7088160Z test case 4
2020-02-11T22:08:15.7088515Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:100:5
2020-02-11T22:08:15.7088515Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:100:5
2020-02-11T22:08:15.7089007Z    0:     0x7f619211dd98 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h773996ea8c7d87fd
2020-02-11T22:08:15.7089259Z    1:     0x7f619215770c - core::fmt::write::hc6fac2af734bcbe6
2020-02-11T22:08:15.7089536Z    2:     0x7f619210e857 - std::io::Write::write_fmt::h32f7c9d1c0ecde3b
2020-02-11T22:08:15.7089810Z    3:     0x7f6192122d84 - std::panicking::default_hook::{{closure}}::h590e3843a43b4171
2020-02-11T22:08:15.7090069Z    4:     0x7f6192122aae - std::panicking::default_hook::h4f44564c405cb696
2020-02-11T22:08:15.7090431Z    5:     0x7f61921233e5 - std::panicking::rust_panic_with_hook::h75d505592f54cc4e
2020-02-11T22:08:15.7090718Z    6:     0x5589dbdb0522 - std::panicking::begin_panic::h835adbb927912903
2020-02-11T22:08:15.7090770Z                                at /checkout/src/libstd/panicking.rs:397
2020-02-11T22:08:15.7091052Z    7:     0x5589dbda874b - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2020-02-11T22:08:15.7091327Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:100
2020-02-11T22:08:15.7091583Z    8:     0x5589dbda8c83 - backtrace_debuginfo::outer::h681fb285db800a84
2020-02-11T22:08:15.7091859Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:126
2020-02-11T22:08:15.7092121Z    9:     0x5589dbda9941 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-02-11T22:08:15.7092380Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-11T22:08:15.7092764Z   10:     0x5589dbd9f0ee - std::rt::lang_start::{{closure}}::h713fbc1b2737288b
2020-02-11T22:08:15.7092824Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7093086Z   11:     0x7f6192122ea3 - std::panicking::try::do_call::h73491d1a398c4d9f
2020-02-11T22:08:15.7093409Z   12:     0x7f6192135da7 - __rust_maybe_catch_panic
2020-02-11T22:08:15.7093673Z   13:     0x7f6192123958 - std::rt::lang_start_internal::h5c3aea9777b113bf
2020-02-11T22:08:15.7093932Z   14:     0x5589dbd9f0d5 - std::rt::lang_start::he9bb1e94b62ebf5d
2020-02-11T22:08:15.7094001Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7094218Z   15:     0x5589dbda99e2 - main
2020-02-11T22:08:15.7094444Z   16:     0x7f6191ac8b97 - __libc_start_main
2020-02-11T22:08:15.7094678Z   17:     0x5589dbd98aba - _start
2020-02-11T22:08:15.7094899Z   18:                0x0 - <unknown>
2020-02-11T22:08:15.7095140Z ---------------------------------------
2020-02-11T22:08:15.7095203Z no backtrace found in stderr:
2020-02-11T22:08:15.7095243Z test case 5
2020-02-11T22:08:15.7095520Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:101:5
2020-02-11T22:08:15.7095520Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:101:5
2020-02-11T22:08:15.7095852Z    0:     0x7f55cb4a4d98 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h773996ea8c7d87fd
2020-02-11T22:08:15.7096106Z    1:     0x7f55cb4de70c - core::fmt::write::hc6fac2af734bcbe6
2020-02-11T22:08:15.7096371Z    2:     0x7f55cb495857 - std::io::Write::write_fmt::h32f7c9d1c0ecde3b
2020-02-11T22:08:15.7096658Z    3:     0x7f55cb4a9d84 - std::panicking::default_hook::{{closure}}::h590e3843a43b4171
2020-02-11T22:08:15.7096922Z    4:     0x7f55cb4a9aae - std::panicking::default_hook::h4f44564c405cb696
2020-02-11T22:08:15.7097185Z    5:     0x7f55cb4aa3e5 - std::panicking::rust_panic_with_hook::h75d505592f54cc4e
2020-02-11T22:08:15.7097461Z    6:     0x55b3777f0522 - std::panicking::begin_panic::h835adbb927912903
2020-02-11T22:08:15.7097515Z                                at /checkout/src/libstd/panicking.rs:397
2020-02-11T22:08:15.7097791Z    7:     0x55b3777e8878 - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2020-02-11T22:08:15.7098071Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:101
2020-02-11T22:08:15.7098331Z    8:     0x55b3777e8c83 - backtrace_debuginfo::outer::h681fb285db800a84
2020-02-11T22:08:15.7098692Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:126
2020-02-11T22:08:15.7098970Z    9:     0x55b3777e9941 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-02-11T22:08:15.7099231Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-11T22:08:15.7099491Z   10:     0x55b3777df0ee - std::rt::lang_start::{{closure}}::h713fbc1b2737288b
2020-02-11T22:08:15.7099561Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7099819Z   11:     0x7f55cb4a9ea3 - std::panicking::try::do_call::h73491d1a398c4d9f
2020-02-11T22:08:15.7100052Z   12:     0x7f55cb4bcda7 - __rust_maybe_catch_panic
2020-02-11T22:08:15.7100397Z   13:     0x7f55cb4aa958 - std::rt::lang_start_internal::h5c3aea9777b113bf
2020-02-11T22:08:15.7100676Z   14:     0x55b3777df0d5 - std::rt::lang_start::he9bb1e94b62ebf5d
2020-02-11T22:08:15.7100726Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7100961Z   15:     0x55b3777e99e2 - main
2020-02-11T22:08:15.7101202Z   16:     0x7f55cae4fb97 - __libc_start_main
2020-02-11T22:08:15.7101419Z   17:     0x55b3777d8aba - _start
2020-02-11T22:08:15.7101658Z   18:                0x0 - <unknown>
2020-02-11T22:08:15.7101902Z ---------------------------------------
2020-02-11T22:08:15.7101947Z no backtrace found in stderr:
2020-02-11T22:08:15.7102006Z test case 6
2020-02-11T22:08:15.7102274Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:107:9
2020-02-11T22:08:15.7102274Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:107:9
2020-02-11T22:08:15.7102583Z    0:     0x7f9173769d98 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h773996ea8c7d87fd
2020-02-11T22:08:15.7103015Z    1:     0x7f91737a370c - core::fmt::write::hc6fac2af734bcbe6
2020-02-11T22:08:15.7103268Z    2:     0x7f917375a857 - std::io::Write::write_fmt::h32f7c9d1c0ecde3b
2020-02-11T22:08:15.7103529Z    3:     0x7f917376ed84 - std::panicking::default_hook::{{closure}}::h590e3843a43b4171
2020-02-11T22:08:15.7103807Z    4:     0x7f917376eaae - std::panicking::default_hook::h4f44564c405cb696
2020-02-11T22:08:15.7104068Z    5:     0x7f917376f3e5 - std::panicking::rust_panic_with_hook::h75d505592f54cc4e
2020-02-11T22:08:15.7104317Z    6:     0x5568db8a5522 - std::panicking::begin_panic::h835adbb927912903
2020-02-11T22:08:15.7104384Z                                at /checkout/src/libstd/panicking.rs:397
2020-02-11T22:08:15.7104660Z    7:     0x5568db89dbe3 - backtrace_debuginfo::inner_inlined::inner_further_inlined::hc6cb011b12fdc07c
2020-02-11T22:08:15.7104913Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:107
2020-02-11T22:08:15.7105200Z    8:     0x5568db89d7da - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2020-02-11T22:08:15.7105455Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:109
2020-02-11T22:08:15.7105703Z    9:     0x5568db89dc83 - backtrace_debuginfo::outer::h681fb285db800a84
2020-02-11T22:08:15.7105975Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:126
2020-02-11T22:08:15.7106234Z   10:     0x5568db89e941 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-02-11T22:08:15.7106483Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-11T22:08:15.7106799Z   11:     0x5568db8940ee - std::rt::lang_start::{{closure}}::h713fbc1b2737288b
2020-02-11T22:08:15.7106849Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7107097Z   12:     0x7f917376eea3 - std::panicking::try::do_call::h73491d1a398c4d9f
2020-02-11T22:08:15.7107346Z   13:     0x7f9173781da7 - __rust_maybe_catch_panic
2020-02-11T22:08:15.7107603Z   14:     0x7f917376f958 - std::rt::lang_start_internal::h5c3aea9777b113bf
2020-02-11T22:08:15.7107848Z   15:     0x5568db8940d5 - std::rt::lang_start::he9bb1e94b62ebf5d
2020-02-11T22:08:15.7107912Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7108123Z   16:     0x5568db89e9e2 - main
2020-02-11T22:08:15.7108445Z   17:     0x7f9173114b97 - __libc_start_main
2020-02-11T22:08:15.7108674Z   18:     0x5568db88daba - _start
2020-02-11T22:08:15.7108885Z   19:                0x0 - <unknown>
2020-02-11T22:08:15.7109121Z ---------------------------------------
2020-02-11T22:08:15.7109185Z no backtrace found in stderr:
2020-02-11T22:08:15.7109224Z test case 7
2020-02-11T22:08:15.7109484Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:112:9
2020-02-11T22:08:15.7109484Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:112:9
2020-02-11T22:08:15.7109804Z    0:     0x7f3521c9ad98 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h773996ea8c7d87fd
2020-02-11T22:08:15.7122055Z    1:     0x7f3521cd470c - core::fmt::write::hc6fac2af734bcbe6
2020-02-11T22:08:15.7122651Z    2:     0x7f3521c8b857 - std::io::Write::write_fmt::h32f7c9d1c0ecde3b
2020-02-11T22:08:15.7122955Z    3:     0x7f3521c9fd84 - std::panicking::default_hook::{{closure}}::h590e3843a43b4171
2020-02-11T22:08:15.7123200Z    4:     0x7f3521c9faae - std::panicking::default_hook::h4f44564c405cb696
2020-02-11T22:08:15.7123464Z    5:     0x7f3521ca03e5 - std::panicking::rust_panic_with_hook::h75d505592f54cc4e
2020-02-11T22:08:15.7123728Z    6:     0x5572d0856522 - std::panicking::begin_panic::h835adbb927912903
2020-02-11T22:08:15.7123778Z                                at /checkout/src/libstd/panicking.rs:397
2020-02-11T22:08:15.7124262Z    7:     0x5572d0846f0b - backtrace_debuginfo::inner_inlined::{{closure}}::hfb3950c4d50c2864
2020-02-11T22:08:15.7124541Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:112
2020-02-11T22:08:15.7124973Z    8:     0x5572d0846c16 - backtrace_debuginfo::aux::callback::h2d349a730207a6f1
2020-02-11T22:08:15.7125584Z                                at /checkout/src/test/ui/backtrace-debuginfo-aux.rs:6
2020-02-11T22:08:15.7126045Z    9:     0x5572d084e8d0 - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2020-02-11T22:08:15.7126461Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:111
2020-02-11T22:08:15.7126725Z   10:     0x5572d084ec83 - backtrace_debuginfo::outer::h681fb285db800a84
2020-02-11T22:08:15.7126996Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:126
2020-02-11T22:08:15.7127246Z   11:     0x5572d084f941 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-02-11T22:08:15.7127750Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-11T22:08:15.7128216Z   12:     0x5572d08450ee - std::rt::lang_start::{{closure}}::h713fbc1b2737288b
2020-02-11T22:08:15.7128270Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7128545Z   13:     0x7f3521c9fea3 - std::panicking::try::do_call::h73491d1a398c4d9f
2020-02-11T22:08:15.7128817Z   14:     0x7f3521cb2da7 - __rust_maybe_catch_panic
2020-02-11T22:08:15.7129085Z   15:     0x7f3521ca0958 - std::rt::lang_start_internal::h5c3aea9777b113bf
2020-02-11T22:08:15.7129501Z   16:     0x5572d08450d5 - std::rt::lang_start::he9bb1e94b62ebf5d
2020-02-11T22:08:15.7129577Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7129955Z   17:     0x5572d084f9e2 - main
2020-02-11T22:08:15.7130436Z   18:     0x7f3521645b97 - __libc_start_main
2020-02-11T22:08:15.7130675Z   19:     0x5572d083eaba - _start
2020-02-11T22:08:15.7130890Z   20:                0x0 - <unknown>
2020-02-11T22:08:15.7131377Z ---------------------------------------
2020-02-11T22:08:15.7131438Z no backtrace found in stderr:
2020-02-11T22:08:15.7131477Z test case 8
2020-02-11T22:08:15.7131738Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:115:9
2020-02-11T22:08:15.7131738Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:115:9
2020-02-11T22:08:15.7132069Z    0:     0x7fea82202d98 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h773996ea8c7d87fd
2020-02-11T22:08:15.7132316Z    1:     0x7fea8223c70c - core::fmt::write::hc6fac2af734bcbe6
2020-02-11T22:08:15.7132561Z    2:     0x7fea821f3857 - std::io::Write::write_fmt::h32f7c9d1c0ecde3b
2020-02-11T22:08:15.7132998Z    3:     0x7fea82207d84 - std::panicking::default_hook::{{closure}}::h590e3843a43b4171
2020-02-11T22:08:15.7133249Z    4:     0x7fea82207aae - std::panicking::default_hook::h4f44564c405cb696
2020-02-11T22:08:15.7133503Z    5:     0x7fea822083e5 - std::panicking::rust_panic_with_hook::h75d505592f54cc4e
2020-02-11T22:08:15.7133775Z    6:     0x562726eda522 - std::panicking::begin_panic::h835adbb927912903
2020-02-11T22:08:15.7133824Z                                at /checkout/src/libstd/panicking.rs:397
2020-02-11T22:08:15.7134085Z    7:     0x562726ed2afb - backtrace_debuginfo::inner_inlined::{{closure}}::h99e04af60b7bda32
2020-02-11T22:08:15.7134428Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:115
2020-02-11T22:08:15.7134719Z    8:     0x562726ed2146 - backtrace_debuginfo::aux::callback_inlined::hbb5dd8dbdf473045
2020-02-11T22:08:15.7134976Z                                at /checkout/src/test/ui/backtrace-debuginfo-aux.rs:13
2020-02-11T22:08:15.7135455Z    9:     0x562726ed2926 - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2020-02-11T22:08:15.7135871Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:114
2020-02-11T22:08:15.7136118Z   10:     0x562726ed2c83 - backtrace_debuginfo::outer::h681fb285db800a84
2020-02-11T22:08:15.7136389Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:126
2020-02-11T22:08:15.7136637Z   11:     0x562726ed3941 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-02-11T22:08:15.7136886Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-11T22:08:15.7137167Z   12:     0x562726ec90ee - std::rt::lang_start::{{closure}}::h713fbc1b2737288b
2020-02-11T22:08:15.7137216Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7137465Z   13:     0x7fea82207ea3 - std::panicking::try::do_call::h73491d1a398c4d9f
2020-02-11T22:08:15.7137712Z   14:     0x7fea8221ada7 - __rust_maybe_catch_panic
2020-02-11T22:08:15.7137970Z   15:     0x7fea82208958 - std::rt::lang_start_internal::h5c3aea9777b113bf
2020-02-11T22:08:15.7138211Z   16:     0x562726ec90d5 - std::rt::lang_start::he9bb1e94b62ebf5d
2020-02-11T22:08:15.7138277Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7138489Z   17:     0x562726ed39e2 - main
2020-02-11T22:08:15.7138707Z   18:     0x7fea81badb97 - __libc_start_main
2020-02-11T22:08:15.7138937Z   19:     0x562726ec2aba - _start
2020-02-11T22:08:15.7139145Z   20:                0x0 - <unknown>
2020-02-11T22:08:15.7139396Z ---------------------------------------
2020-02-11T22:08:15.7139448Z no backtrace found in stderr:
2020-02-11T22:08:15.7139487Z test case 9
2020-02-11T22:08:15.7139748Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:107:9
2020-02-11T22:08:15.7139748Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:107:9
2020-02-11T22:08:15.7140067Z    0:     0x7fa611c05d98 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h773996ea8c7d87fd
2020-02-11T22:08:15.7140319Z    1:     0x7fa611c3f70c - core::fmt::write::hc6fac2af734bcbe6
2020-02-11T22:08:15.7140565Z    2:     0x7fa611bf6857 - std::io::Write::write_fmt::h32f7c9d1c0ecde3b
2020-02-11T22:08:15.7140848Z    3:     0x7fa611c0ad84 - std::panicking::default_hook::{{closure}}::h590e3843a43b4171
2020-02-11T22:08:15.7141095Z    4:     0x7fa611c0aaae - std::panicking::default_hook::h4f44564c405cb696
2020-02-11T22:08:15.7141351Z    5:     0x7fa611c0b3e5 - std::panicking::rust_panic_with_hook::h75d505592f54cc4e
2020-02-11T22:08:15.7141626Z    6:     0x55e60b480522 - std::panicking::begin_panic::h835adbb927912903
2020-02-11T22:08:15.7141684Z                                at /checkout/src/libstd/panicking.rs:397
2020-02-11T22:08:15.7141959Z    7:     0x55e60b478be3 - backtrace_debuginfo::inner_inlined::inner_further_inlined::hc6cb011b12fdc07c
2020-02-11T22:08:15.7142230Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:107
2020-02-11T22:08:15.7142579Z    8:     0x55e60b4789a7 - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2020-02-11T22:08:15.7142847Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:120
2020-02-11T22:08:15.7143098Z    9:     0x55e60b478c83 - backtrace_debuginfo::outer::h681fb285db800a84
2020-02-11T22:08:15.7143346Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:126
2020-02-11T22:08:15.7143614Z   10:     0x55e60b479941 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-02-11T22:08:15.7143867Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-11T22:08:15.7144181Z   11:     0x55e60b46f0ee - std::rt::lang_start::{{closure}}::h713fbc1b2737288b
2020-02-11T22:08:15.7144254Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7144525Z   12:     0x7fa611c0aea3 - std::panicking::try::do_call::h73491d1a398c4d9f
2020-02-11T22:08:15.7144751Z   13:     0x7fa611c1dda7 - __rust_maybe_catch_panic
2020-02-11T22:08:15.7145183Z   14:     0x7fa611c0b958 - std::rt::lang_start_internal::h5c3aea9777b113bf
2020-02-11T22:08:15.7145437Z   15:     0x55e60b46f0d5 - std::rt::lang_start::he9bb1e94b62ebf5d
2020-02-11T22:08:15.7145483Z                                at /checkout/src/libstd/rt.rs:67
2020-02-11T22:08:15.7145685Z   16:     0x55e60b4799e2 - main
2020-02-11T22:08:15.7146085Z   17:     0x7fa6115b0b97 - __libc_start_main
2020-02-11T22:08:15.7146488Z   18:     0x55e60b468aba - _start
2020-02-11T22:08:15.7146886Z   19:                0x0 - <unknown>
2020-02-11T22:08:15.7146961Z 
2020-02-11T22:08:15.7147188Z ------------------------------------------
2020-02-11T22:08:15.7147234Z stderr:
2020-02-11T22:08:15.7147663Z ------------------------------------------
---
2020-02-11T22:08:15.7149735Z 
2020-02-11T22:08:15.7150142Z ------------------------------------------
2020-02-11T22:08:15.7150195Z stderr:
2020-02-11T22:08:15.7150418Z ------------------------------------------
2020-02-11T22:08:15.7150934Z thread 'main' panicked at 'assertion failed: String::from_utf8_lossy(&p.stdout).contains("stack backtrace:\n")', /checkout/src/test/ui/std-backtrace.rs:34:5
2020-02-11T22:08:15.7151032Z 
2020-02-11T22:08:15.7151253Z ------------------------------------------
2020-02-11T22:08:15.7151301Z 
2020-02-11T22:08:15.7151325Z 
---
2020-02-11T22:08:15.7153076Z test result: FAILED. 9572 passed; 3 failed; 52 ignored; 0 measured; 0 filtered out
2020-02-11T22:08:15.7153110Z 
2020-02-11T22:08:15.7154224Z 
2020-02-11T22:08:15.7158221Z 
2020-02-11T22:08:15.7160279Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-11T22:08:15.7160759Z 
2020-02-11T22:08:15.7160807Z 
2020-02-11T22:08:15.7160853Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-11T22:08:15.7160902Z Build completed unsuccessfully in 1:09:36
2020-02-11T22:08:15.7160902Z Build completed unsuccessfully in 1:09:36
2020-02-11T22:08:15.7160965Z == clock drift check ==
2020-02-11T22:08:15.7161546Z   local time: thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-11T22:08:15.7161610Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-11T22:08:15.7179061Z Tue Feb 11 22:08:15 UTC 2020
2020-02-11T22:08:15.8814163Z   network time: Tue, 11 Feb 2020 22:08:15 GMT
2020-02-11T22:08:15.8817988Z == end clock drift check ==
2020-02-11T22:08:16.3847697Z 
2020-02-11T22:08:16.3946327Z ##[error]Bash exited with code '1'.
2020-02-11T22:08:16.3959370Z ##[section]Finishing: Run build
2020-02-11T22:08:16.3982468Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69042/merge to s
2020-02-11T22:08:16.3984765Z Task         : Get sources
2020-02-11T22:08:16.3984812Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T22:08:16.3984857Z Version      : 1.0.0
2020-02-11T22:08:16.3984897Z Author       : Microsoft
2020-02-11T22:08:16.3984897Z Author       : Microsoft
2020-02-11T22:08:16.3984959Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-11T22:08:16.3985986Z ==============================================================================
2020-02-11T22:08:16.8821861Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-11T22:08:16.8887877Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69042/merge to s
2020-02-11T22:08:16.9005747Z Cleaning up task key
2020-02-11T22:08:16.9006734Z Start cleaning up orphan processes.
2020-02-11T22:08:16.9118667Z Terminate orphan process: pid (3833) (python)
2020-02-11T22:08:16.9359847Z ##[section]Finishing: Finalize Job
