
---- [ui] ui/threads-sendsync/thread-local-extern-static.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/r/src/rust/rustc.2/src/test/ui/threads-sendsync/thread-local-extern-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/thread-local-extern-static/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/thread-local-extern-static/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: broken MIR in DefId(0:7 ~ thread_local_extern_static[317d]::main) (_6 = &/*tls*/ FOO): bad assignment (*const std::cell::Cell<u32> = &'static std::cell::Cell<u32>): NoSolution
  --> /home/r/src/rust/rustc.2/src/test/ui/threads-sendsync/thread-local-extern-static.rs:22:20
   |
LL |         assert_eq!(FOO.get(), 3);
   |                    ^^^
   |
   = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:253:27

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:7 ~ thread_local_extern_static[317d]::main), const_param_did: None }) (end of phase Optimization) at bb0[5]:
encountered `Assign((_5, &/*tls*/ FOO))` with incompatible types:
left-hand side has type: *const Cell<u32>
right-hand side has type: &'static Cell<u32>
  --> /home/r/src/rust/rustc.2/src/test/ui/threads-sendsync/thread-local-extern-static.rs:22:20
   |
LL |         assert_eq!(FOO.get(), 3);
   |                    ^^^
   |
   = note: delayed at compiler/rustc_mir/src/transform/validate.rs:156:36
