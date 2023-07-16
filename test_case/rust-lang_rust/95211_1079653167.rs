plain
failures:

---- [ui] ui/parser/issues/issue-93867.rs stdout ----
normalized stderr:
error: expected `,`, found `K`
   |
   |
LL | pub fn entry<'a, K, V>() -> Entry<'a K, V> {
   |                                      |
   |                                      expected `,`
   |                                      help: consider removing this ident

---
To only update this specific test, also pass `--test-args parser/issues/issue-93867.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-93867.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-93867" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-93867/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected `,`, found `K`
   |
   |
LL | pub fn entry<'a, K, V>() -> Entry<'a K, V> {
   |                                      |
   |                                      expected `,`
   |                                      help: consider removing this ident

