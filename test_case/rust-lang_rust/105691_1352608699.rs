plain
---- [ui] src/test/ui/codegen/issue-55976.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codegen/issue-55976.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codegen/issue-55976/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codegen/issue-55976/auxiliary"
stdout: none
--- stderr -------------------------------
Function return type does not match operand type of return inst!
  ret %"alloc::vec::Vec<alloc::boxed::Box<dyn for<'a> core::ops::function::Fn(&'a u8)>>"* %4
 %"alloc::vec::Vec<alloc::boxed::Box<dyn core::ops::function::Fn(&u8)>>"*LLVM ERROR: Broken module found, compilation aborted!



failures:
