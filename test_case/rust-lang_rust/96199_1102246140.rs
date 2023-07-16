plain
....................i................i.............i.................................... 6952/12959
...............i........................................................................ 7040/12959
................................i....................................................... 7128/12959
........................................................................................ 7216/12959
............................F.F.FF...................................................... 7304/12959
............i........................................................................... 7480/12959
........................................................................................ 7568/12959
............i........................................................................... 7656/12959
........................................................................................ 7744/12959
---
---- [ui] src/test/ui/lto/lto-opt-level-s.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto/lto-opt-level-s.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/lto-opt-level-s" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Clinker-plugin-lto" "-Copt-level=s" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/lto-opt-level-s/auxiliary"
stdout: none
--- stderr -------------------------------

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lto/lto-opt-level-z.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto/lto-opt-level-z.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/lto-opt-level-z" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Clinker-plugin-lto" "-Copt-level=z" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/lto-opt-level-z/auxiliary"
stdout: none
--- stderr -------------------------------

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lto/lto-rustc-loads-linker-plugin.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui/lto/auxiliary/lto-rustc-loads-linker-plugin.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto/auxiliary/lto-rustc-loads-linker-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/lto-rustc-loads-linker-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Clinker-plugin-lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/lto-rustc-loads-linker-plugin/auxiliary"
stdout: none
--- stderr -------------------------------

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lto/lto-thin-rustc-loads-linker-plugin.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui/lto/auxiliary/lto-rustc-loads-linker-plugin.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto/auxiliary/lto-rustc-loads-linker-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/lto-thin-rustc-loads-linker-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Clinker-plugin-lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/lto-thin-rustc-loads-linker-plugin/auxiliary"
stdout: none
--- stderr -------------------------------

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/std-backtrace.rs stdout ----

error: test run failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/std-backtrace/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: String::from_utf8_lossy(&p.stdout).contains(\"backtrace::main\")', /checkout/src/test/ui/std-backtrace.rs:34:5
------------------------------------------



