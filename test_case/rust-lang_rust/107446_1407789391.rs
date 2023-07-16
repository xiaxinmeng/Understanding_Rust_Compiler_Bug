plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ece894d15649f0f9d7884f915dc821f00bd0418b)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
- error: empty character literal
+ error: invalid trailing slash in literal
2   --> $DIR/lex-bad-char-literals-7.rs:2:20
3    |
4 LL |     let _: char = '';
-    |                    ^ empty character literal
-    |                    ^ empty character literal
+    |                    ^ invalid trailing slash in literal
7 error: empty unicode escape
8   --> $DIR/lex-bad-char-literals-7.rs:4:20



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-7/lex-bad-char-literals-7.stderr
To only update this specific test, also pass `--test-args lexer/lex-bad-char-literals-7.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lexer/lex-bad-char-literals-7.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-7" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-7/auxiliary"
stdout: none
--- stderr -------------------------------
error: invalid trailing slash in literal
  --> fake-test-src-base/lexer/lex-bad-char-literals-7.rs:2:20
   |
LL |     let _: char = '';
   |                    ^ invalid trailing slash in literal
error: empty unicode escape
  --> fake-test-src-base/lexer/lex-bad-char-literals-7.rs:4:20
   |
   |
LL |     let _: char = '\u{}';
   |                    ^^^^ this escape must have at least 1 hex digit
error[E0762]: unterminated character literal
  --> fake-test-src-base/lexer/lex-bad-char-literals-7.rs:11:13
   |
   |
LL |     let _ = ' hello // here's a comment

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0762`.
