plain
 ---> 2afb3e7bef8f
Step 3/8 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   gcc-multilib   make   ninja-build   file   curl   ca-certificates   python2.7   python3.9   git   cmake   sudo   gdb   llvm-12-tools   llvm-12-dev   libedit-dev   libssl-dev   pkg-config   zlib1g-dev   xz-utils   nodejs
 ---> Using cache
 ---> 357fae1e02d2
Step 4/8 : RUN apt-get update &&     apt-get install -y apt-transport-https software-properties-common &&     curl -s "https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/packages-microsoft-prod.deb" > packages-microsoft-prod.deb &&     dpkg -i packages-microsoft-prod.deb &&     apt-get update &&     apt-get install -y powershell
 ---> 080d1843107f
Step 5/8 : COPY scripts/sccache.sh /scripts/
 ---> Using cache
 ---> 4a6c76c56ba3
---
---- [ui] src/test/ui/fmt/incorrect-separator.rs stdout ----
diff of stderr:

3    |
4 LL |     format!("A number: {}" ̦ iter::once(42).next().unwrap());
+    |
+    |
+    = help: Unicode character \u{326} might not be visible when rendered
6 
7 error: expected `,`, found `.`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/incorrect-separator/incorrect-separator.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/incorrect-separator/incorrect-separator.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fmt/incorrect-separator.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/incorrect-separator.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/incorrect-separator" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/incorrect-separator/auxiliary"
stdout: none
--- stderr -------------------------------
error: unknown start of token: \u{326}
   |
   |
LL |     format!("A number: {}" ̦ iter::once(42).next().unwrap());
   |
   |
   = help: Unicode character \u{326} might not be visible when rendered

error: expected `,`, found `.`
   |
   |
LL |     format!("A number: {}". iter::once(42).next().unwrap());
   |                           ^ expected `,`

error: expected `,`, found `/`
   |
   |
LL |     format!("A number: {}" / iter::once(42).next().unwrap());
   |                            ^ expected `,`

error: expected `,`, found `;`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL |     format!("A number: {}"; iter::once(42).next().unwrap());
   |                           ^ expected `,`

error: expected `,`, found `iter`
   |
   |
LL |     format!("A number: {}" ̦ iter::once(42).next().unwrap());
   |                             ^^^^ expected `,`

error: expected `,`, found `.`
   |
   |
LL |     format!("{}". compile_error!("fail"));
   |                 ^ expected `,`
error: fail
  --> /checkout/src/test/ui/fmt/incorrect-separator.rs:26:19
   |
   |
LL |     format!("{}". compile_error!("fail"));

error: aborting due to 7 previous errors
------------------------------------------



---- [ui] src/test/ui/issues/issue-69130.rs stdout ----
diff of stderr:

3    |
4 LL | M (§& u8)}
+    |
+    |
+    = help: Unicode character \u{a7} might not be visible when rendered
7 error[E0106]: missing lifetime specifier
8   --> $DIR/issue-69130.rs:4:5



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69130/issue-69130.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-69130.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-69130.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69130" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69130/auxiliary"
stdout: none
--- stderr -------------------------------
error: unknown start of token: \u{a7}
   |
   |
LL | M (§& u8)}
   |
   |
   = help: Unicode character \u{a7} might not be visible when rendered
error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/issues/issue-69130.rs:4:5
   |
   |
LL | M (§& u8)}
   |     ^ expected named lifetime parameter
help: consider introducing a named lifetime parameter
   |
   |
LL ~ enum F<'a> {
LL ~ M (§&'a  u8)}

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0106`.
