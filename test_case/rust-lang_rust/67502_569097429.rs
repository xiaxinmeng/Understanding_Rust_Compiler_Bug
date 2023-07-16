plain
2019-12-26T16:18:02.6939652Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-26T16:18:02.6952370Z ##[command]git config gc.auto 0
2019-12-26T16:18:02.6955430Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-26T16:18:02.6957426Z ##[command]git config --get-all http.proxy
2019-12-26T16:18:02.6960412Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67502/merge:refs/remotes/pull/67502/merge
---
2019-12-26T17:18:59.8883587Z .................................................................................................... 1600/9454
2019-12-26T17:19:04.6581314Z .................................................................................................... 1700/9454
2019-12-26T17:19:14.2840309Z .............................................................................................i...... 1800/9454
2019-12-26T17:19:22.7633936Z .................................................................................................... 1900/9454
2019-12-26T17:19:29.8049014Z ...............................................................................iiiii................ 2000/9454
2019-12-26T17:19:51.6753772Z .................................................................................................... 2200/9454
2019-12-26T17:19:54.0756694Z .................................................................................................... 2300/9454
2019-12-26T17:19:56.6773311Z .................................................................................................... 2400/9454
2019-12-26T17:20:03.0072145Z .................................................................................................... 2500/9454
---
2019-12-26T17:23:03.3324968Z ..........i...............i......................................................................... 4900/9454
2019-12-26T17:23:13.4498286Z .................................................................................................... 5000/9454
2019-12-26T17:23:19.1726150Z ......................................................i............................................. 5100/9454
2019-12-26T17:23:28.7762580Z .................................................................................................... 5200/9454
2019-12-26T17:23:35.1456649Z .....................ii.ii...........i.............................................................. 5300/9454
2019-12-26T17:23:44.0944078Z .................................................................................................... 5500/9454
2019-12-26T17:23:54.9394285Z .................................................................................................... 5600/9454
2019-12-26T17:24:02.1469513Z ...i................................................................................................ 5700/9454
2019-12-26T17:24:07.8399773Z .................................................................................................... 5800/9454
2019-12-26T17:24:07.8399773Z .................................................................................................... 5800/9454
2019-12-26T17:24:17.8703782Z ...........................................................................................ii...i..i 5900/9454
2019-12-26T17:24:30.5776281Z i...........i....................................................................................... 6000/9454
2019-12-26T17:24:48.2667361Z .................................................................................................... 6200/9454
2019-12-26T17:24:55.7709511Z .........................................................................................F.......... 6300/9454
2019-12-26T17:24:55.7709511Z .........................................................................................F.......... 6300/9454
2019-12-26T17:25:09.8699322Z ..................i..ii............................................................................. 6400/9454
2019-12-26T17:25:29.8256764Z .............................FF..F............................................................i..... 6600/9454
2019-12-26T17:25:32.0440710Z .................................................................................................... 6700/9454
2019-12-26T17:25:34.3508387Z ..............................................................................................i..... 6800/9454
2019-12-26T17:25:36.9896678Z .................................................................................................... 6900/9454
---
2019-12-26T17:27:15.5252036Z .................................................................................................... 7500/9454
2019-12-26T17:27:20.2751553Z .................................................................................................... 7600/9454
2019-12-26T17:27:26.7995208Z .................................................................................................... 7700/9454
2019-12-26T17:27:37.2354542Z .................................................................................................... 7800/9454
2019-12-26T17:27:43.8126140Z .........................iiii....................................................................... 7900/9454
2019-12-26T17:27:58.4735769Z .................................................................................................... 8100/9454
2019-12-26T17:28:08.2366846Z .................................................................................................... 8200/9454
2019-12-26T17:28:22.0832479Z .................................................................................................... 8300/9454
2019-12-26T17:28:29.2318346Z .................................................................................................... 8400/9454
---
2019-12-26T17:30:25.4434514Z failures:
2019-12-26T17:30:25.4434709Z 
2019-12-26T17:30:25.4435173Z ---- [ui] ui/no-landing-pads.rs stdout ----
2019-12-26T17:30:25.4435348Z 
2019-12-26T17:30:25.4435501Z error: test run failed!
2019-12-26T17:30:25.4435705Z status: signal: 6
2019-12-26T17:30:25.4436482Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-landing-pads/a" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-landing-pads/a"
2019-12-26T17:30:25.4437141Z ------------------------------------------
2019-12-26T17:30:25.4437301Z 
2019-12-26T17:30:25.4437672Z ------------------------------------------
2019-12-26T17:30:25.4437842Z stderr:
---
2019-12-26T17:30:25.4440684Z ---- [ui] ui/panic-runtime/abort-link-to-unwinding-crates.rs stdout ----
2019-12-26T17:30:25.4440836Z 
2019-12-26T17:30:25.4441203Z error: test compilation failed although it shouldn't!
2019-12-26T17:30:25.4441402Z status: exit code: 1
2019-12-26T17:30:25.4442424Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/abort-link-to-unwinding-crates.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/auxiliary"
2019-12-26T17:30:25.4443034Z ------------------------------------------
2019-12-26T17:30:25.4443213Z 
2019-12-26T17:30:25.4443567Z ------------------------------------------
2019-12-26T17:30:25.4443732Z stderr:
2019-12-26T17:30:25.4443732Z stderr:
2019-12-26T17:30:25.4444101Z ------------------------------------------
2019-12-26T17:30:25.4444273Z error: linking with `cc` failed: exit code: 1
2019-12-26T17:30:25.4444426Z    |
2019-12-26T17:30:25.4451314Z    = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a.abort_link_to_unwinding_crates.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a.abort_link_to_unwinding_crates.7rcbfp3g-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a.abort_link_to_unwinding_crates.7rcbfp3g-cgu.10.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a.abort_link_to_unwinding_crates.7rcbfp3g-cgu.11.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a.abort_link_to_unwinding_crates.7rcbfp3g-cgu.12.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a.abort_link_to_unwinding_crates.7rcbfp3g-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a.abort_link_to_unwinding_crates.7rcbfp3g-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a.abort_link_to_unwinding_crates.7rcbfp3g-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a.abort_link_to_unwinding_crates.7rcbfp3g-cgu.5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a.abort_link_to_unwinding_crates.7rcbfp3g-cgu.6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a.abort_link_to_unwinding_crates.7rcbfp3g-cgu.7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a.abort_link_to_unwinding_crates.7rcbfp3g-cgu.8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a.abort_link_to_unwinding_crates.7rcbfp3g-cgu.9.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a.1vb5ndsi2uz2y594.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/auxiliary/libexit_success_if_unwind.rlib" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b586cf616217cb61.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-54ce8b4246cb06b0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-50762b7d1f500aae.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-43af671ddf03ee81.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace-0d59c4282cbc8e4b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-661fc2e981ab6285.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-0cd4fe277e96d107.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-8ec6d240e6d918f8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-818d3bf4d09f33eb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-3f4b91103c2ee14f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-289cdd5064e4da02.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-87f2141e797e5ef0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-987003bc1262d91a.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-6d0715333b74ba49.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
2019-12-26T17:30:25.4453133Z    = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b586cf616217cb61.rlib(std-b586cf616217cb61.std.2pbd4t20-cgu.0.rcgu.o): In function `std::panicking::try::cleanup':
2019-12-26T17:30:25.4453656Z            std.2pbd4t20-cgu.0:(.text._ZN3std9panicking3try7cleanup17hd2a574b8686b676cE+0xd): undefined reference to `__rust_panic_cleanup'
2019-12-26T17:30:25.4453892Z            collect2: error: ld returned 1 exit status
2019-12-26T17:30:25.4454149Z 
2019-12-26T17:30:25.4454308Z error: aborting due to previous error
2019-12-26T17:30:25.4454425Z 
2019-12-26T17:30:25.4454538Z 
2019-12-26T17:30:25.4454538Z 
2019-12-26T17:30:25.4454918Z ------------------------------------------
2019-12-26T17:30:25.4455071Z 
2019-12-26T17:30:25.4455227Z 
2019-12-26T17:30:25.4455638Z ---- [ui] ui/panic-runtime/abort.rs stdout ----
2019-12-26T17:30:25.4456648Z 
2019-12-26T17:30:25.4457172Z error: test compilation failed although it shouldn't!
2019-12-26T17:30:25.4458449Z status: exit code: 1
2019-12-26T17:30:25.4459454Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/abort.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/auxiliary"
2019-12-26T17:30:25.4459888Z ------------------------------------------
2019-12-26T17:30:25.4459922Z 
2019-12-26T17:30:25.4460163Z ------------------------------------------
2019-12-26T17:30:25.4460208Z stderr:
2019-12-26T17:30:25.4460208Z stderr:
2019-12-26T17:30:25.4460426Z ------------------------------------------
2019-12-26T17:30:25.4460476Z error: linking with `cc` failed: exit code: 1
2019-12-26T17:30:25.4460536Z    |
2019-12-26T17:30:25.4465701Z    = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a.abort.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a.abort.7rcbfp3g-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a.abort.7rcbfp3g-cgu.10.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a.abort.7rcbfp3g-cgu.11.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a.abort.7rcbfp3g-cgu.12.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a.abort.7rcbfp3g-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a.abort.7rcbfp3g-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a.abort.7rcbfp3g-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a.abort.7rcbfp3g-cgu.5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a.abort.7rcbfp3g-cgu.6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a.abort.7rcbfp3g-cgu.7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a.abort.7rcbfp3g-cgu.8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a.abort.7rcbfp3g-cgu.9.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a.1ksge58qc8gpwchm.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b586cf616217cb61.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-54ce8b4246cb06b0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-50762b7d1f500aae.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-43af671ddf03ee81.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace-0d59c4282cbc8e4b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-661fc2e981ab6285.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-0cd4fe277e96d107.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-8ec6d240e6d918f8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-818d3bf4d09f33eb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-3f4b91103c2ee14f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-289cdd5064e4da02.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-87f2141e797e5ef0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-987003bc1262d91a.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-6d0715333b74ba49.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
2019-12-26T17:30:25.4467329Z    = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b586cf616217cb61.rlib(std-b586cf616217cb61.std.2pbd4t20-cgu.0.rcgu.o): In function `std::panicking::try::cleanup':
2019-12-26T17:30:25.4467718Z            std.2pbd4t20-cgu.0:(.text._ZN3std9panicking3try7cleanup17hd2a574b8686b676cE+0xd): undefined reference to `__rust_panic_cleanup'
2019-12-26T17:30:25.4467781Z            collect2: error: ld returned 1 exit status
2019-12-26T17:30:25.4467873Z 
2019-12-26T17:30:25.4467916Z error: aborting due to previous error
2019-12-26T17:30:25.4467944Z 
2019-12-26T17:30:25.4467970Z 
2019-12-26T17:30:25.4467970Z 
2019-12-26T17:30:25.4468211Z ------------------------------------------
2019-12-26T17:30:25.4468243Z 
2019-12-26T17:30:25.4468268Z 
2019-12-26T17:30:25.4468498Z ---- [ui] ui/panic-runtime/link-to-abort.rs stdout ----
2019-12-26T17:30:25.4468549Z 
2019-12-26T17:30:25.4468779Z error: test compilation failed although it shouldn't!
2019-12-26T17:30:25.4468827Z status: exit code: 1
2019-12-26T17:30:25.4469665Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/link-to-abort.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/auxiliary"
2019-12-26T17:30:25.4470006Z ------------------------------------------
2019-12-26T17:30:25.4470039Z 
2019-12-26T17:30:25.4470256Z ------------------------------------------
2019-12-26T17:30:25.4470301Z stderr:
2019-12-26T17:30:25.4470301Z stderr:
2019-12-26T17:30:25.4470534Z ------------------------------------------
2019-12-26T17:30:25.4470583Z error: linking with `cc` failed: exit code: 1
2019-12-26T17:30:25.4470626Z    |
2019-12-26T17:30:25.4474681Z    = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.link_to_abort.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.link_to_abort.7rcbfp3g-cgu.1.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.4hgw2zitnpvuzihs.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b586cf616217cb61.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-54ce8b4246cb06b0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-50762b7d1f500aae.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-43af671ddf03ee81.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace-0d59c4282cbc8e4b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-661fc2e981ab6285.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-0cd4fe277e96d107.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-8ec6d240e6d918f8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-818d3bf4d09f33eb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-3f4b91103c2ee14f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-289cdd5064e4da02.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-87f2141e797e5ef0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-987003bc1262d91a.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-6d0715333b74ba49.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
2019-12-26T17:30:25.4475856Z    = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b586cf616217cb61.rlib(std-b586cf616217cb61.std.2pbd4t20-cgu.0.rcgu.o): In function `std::panicking::try::cleanup':
2019-12-26T17:30:25.4476229Z            std.2pbd4t20-cgu.0:(.text._ZN3std9panicking3try7cleanup17hd2a574b8686b676cE+0xd): undefined reference to `__rust_panic_cleanup'
2019-12-26T17:30:25.4476291Z            collect2: error: ld returned 1 exit status
2019-12-26T17:30:25.4476570Z 
2019-12-26T17:30:25.4476646Z error: aborting due to previous error
2019-12-26T17:30:25.4476675Z 
2019-12-26T17:30:25.4476711Z 
2019-12-26T17:30:25.4476711Z 
2019-12-26T17:30:25.4476968Z ------------------------------------------
2019-12-26T17:30:25.4477017Z 
2019-12-26T17:30:25.4477043Z 
2019-12-26T17:30:25.4477264Z ---- [ui] ui/test-panic-abort.rs stdout ----
2019-12-26T17:30:25.4477296Z 
2019-12-26T17:30:25.4477538Z error: test compilation failed although it shouldn't!
2019-12-26T17:30:25.4477607Z status: exit code: 1
2019-12-26T17:30:25.4478440Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-panic-abort.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-Cpanic=abort" "-Zpanic_abort_tests" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/auxiliary" "-A" "unused"
2019-12-26T17:30:25.4478769Z ------------------------------------------
2019-12-26T17:30:25.4478816Z 
2019-12-26T17:30:25.4479037Z ------------------------------------------
2019-12-26T17:30:25.4479081Z stderr:
2019-12-26T17:30:25.4479081Z stderr:
2019-12-26T17:30:25.4479294Z ------------------------------------------
2019-12-26T17:30:25.4479527Z error: linking with `cc` failed: exit code: 1
2019-12-26T17:30:25.4479575Z    |
2019-12-26T17:30:25.4485651Z    = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a.test_panic_abort.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a.test_panic_abort.7rcbfp3g-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a.test_panic_abort.7rcbfp3g-cgu.10.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a.test_panic_abort.7rcbfp3g-cgu.11.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a.test_panic_abort.7rcbfp3g-cgu.12.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a.test_panic_abort.7rcbfp3g-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a.test_panic_abort.7rcbfp3g-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a.test_panic_abort.7rcbfp3g-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a.test_panic_abort.7rcbfp3g-cgu.5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a.test_panic_abort.7rcbfp3g-cgu.6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a.test_panic_abort.7rcbfp3g-cgu.7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a.test_panic_abort.7rcbfp3g-cgu.8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a.test_panic_abort.7rcbfp3g-cgu.9.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a.4jwkzzjxw5w87ant.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-2ad3a661ee4cd26e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-3867360917dc180d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-5d9095139821c0e2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-8e948ed2950a8bcf.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-3cff50dcefbc28ec.rlib" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b586cf616217cb61.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-54ce8b4246cb06b0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-50762b7d1f500aae.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-43af671ddf03ee81.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace-0d59c4282cbc8e4b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-661fc2e981ab6285.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-0cd4fe277e96d107.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-8ec6d240e6d918f8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-818d3bf4d09f33eb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-3f4b91103c2ee14f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-289cdd5064e4da02.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-87f2141e797e5ef0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-987003bc1262d91a.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-6d0715333b74ba49.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
2019-12-26T17:30:25.4487451Z    = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b586cf616217cb61.rlib(std-b586cf616217cb61.std.2pbd4t20-cgu.0.rcgu.o): In function `std::panicking::try::cleanup':
2019-12-26T17:30:25.4487812Z            std.2pbd4t20-cgu.0:(.text._ZN3std9panicking3try7cleanup17hd2a574b8686b676cE+0xd): undefined reference to `__rust_panic_cleanup'
2019-12-26T17:30:25.4487891Z            collect2: error: ld returned 1 exit status
2019-12-26T17:30:25.4487963Z 
2019-12-26T17:30:25.4488005Z error: aborting due to previous error
2019-12-26T17:30:25.4488053Z 
2019-12-26T17:30:25.4488079Z 
---
2019-12-26T17:30:25.4494110Z 
2019-12-26T17:30:25.4494187Z 
2019-12-26T17:30:25.4494635Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-26T17:30:25.4494753Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-26T17:30:25.4513191Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-26T17:30:25.4513768Z 
2019-12-26T17:30:25.4513799Z 
2019-12-26T17:30:25.4554322Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-26T17:30:25.4554410Z Build completed unsuccessfully in 1:05:53
2019-12-26T17:30:25.4554410Z Build completed unsuccessfully in 1:05:53
2019-12-26T17:30:25.4578245Z == clock drift check ==
2019-12-26T17:30:25.4596286Z   local time: Thu Dec 26 17:30:25 UTC 2019
2019-12-26T17:30:25.7527868Z   network time: Thu, 26 Dec 2019 17:30:25 GMT
2019-12-26T17:30:25.7528738Z == end clock drift check ==
2019-12-26T17:30:26.7994663Z 
2019-12-26T17:30:26.8100582Z ##[error]Bash exited with code '1'.
2019-12-26T17:30:26.8141938Z ##[section]Starting: Checkout
2019-12-26T17:30:26.8144064Z ==============================================================================
2019-12-26T17:30:26.8144149Z Task         : Get sources
2019-12-26T17:30:26.8144217Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
