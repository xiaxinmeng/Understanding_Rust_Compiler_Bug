plain
---- [ui] src/test/ui/asm/issue-96797.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/issue-96797.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/issue-96797" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/issue-96797/auxiliary"
stdout: none
--- stderr -------------------------------
error: unrecognized instruction mnemonic, did you mean: cmp?
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:14
   |
LL | call_foobar: jmp _ZN11issue_967976foobar17h616e66bb1daa29c9E

error: aborting due to previous error
------------------------------------------

