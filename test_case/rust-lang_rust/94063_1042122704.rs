plain
---- [ui] ui/proc-macro/issue-73933-procedural-masquerade.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/issue-73933-procedural-masquerade.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-73933-procedural-masquerade" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-73933-procedural-masquerade/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error: cannot find derive macro `Print` in this scope
  --> /checkout/src/test/ui/proc-macro/issue-73933-procedural-masquerade.rs:6:10
   |
LL | #[derive(Print)]

error: cannot find derive macro `Print` in this scope
  --> /checkout/src/test/ui/proc-macro/issue-73933-procedural-masquerade.rs:6:10
   |
   |
LL | #[derive(Print)]

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0463`.
