plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:46007752205b5430f5cabe1357251ea7621a9e98)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
diff of stderr:

17    |       ^ unknown character escape
18    |
19    = help: for more information, visit <https://static.rust-lang.org/doc/master/reference.html#literals>
- help: if you meant to write a literal backslash (perhaps escaping in a regular expression), consider a raw string literal
- LL |     r"\●"
-    |     ~~~~~
24 
24 
25 error: unknown character escape: `\u{25cf}`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-1/lex-bad-char-literals-1.stderr
To only update this specific test, also pass `--test-args lexer/lex-bad-char-literals-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lexer/lex-bad-char-literals-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: numeric character escape is too short
  --> fake-test-src-base/lexer/lex-bad-char-literals-1.rs:2:6
   |
LL |     '\x1' //~ ERROR: numeric character escape is too short

error: numeric character escape is too short
  --> fake-test-src-base/lexer/lex-bad-char-literals-1.rs:6:6
   |
   |
LL |     "\x1" //~ ERROR: numeric character escape is too short


error: unknown character escape: `\u{25cf}`
  --> fake-test-src-base/lexer/lex-bad-char-literals-1.rs:10:7
   |
LL |     '\●' //~ ERROR: unknown character escape
   |       ^ unknown character escape
   |
   = help: for more information, visit <https://static.rust-lang.org/doc/master/reference.html#literals>

error: unknown character escape: `\u{25cf}`
  --> fake-test-src-base/lexer/lex-bad-char-literals-1.rs:14:7
   |
LL |     "\●" //~ ERROR: unknown character escape
   |       ^ unknown character escape
   |
   = help: for more information, visit <https://static.rust-lang.org/doc/master/reference.html#literals>
help: if you meant to write a literal backslash (perhaps escaping in a regular expression), consider a raw string literal
   |
LL |     r"\●" //~ ERROR: unknown character escape

error: aborting due to 4 previous errors
------------------------------------------



---- [ui] tests/ui/rfcs/rfc-3348-c-string-literals/no-nuls.rs stdout ----

1 error: null character in C string
-   --> $DIR/no-nuls.rs:4:7
-    |
-    |
- LL |     c"\0";
-    |       ^^ null character in C string
- error: null character in C string
8   --> $DIR/no-nuls.rs:7:7
9    |
9    |
10 LL |     c"\u{00}";
17    |       ^ null character in C string
18 
19 error: null character in C string
-   --> $DIR/no-nuls.rs:13:7
-   --> $DIR/no-nuls.rs:13:7
-    |
- LL |     c"\x00";
-    |       ^^^^ null character in C string
- error: null character in C string
26   --> $DIR/no-nuls.rs:16:8
27    |
28 LL |     cr"";
---
33 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-3348-c-string-literals/no-nuls/no-nuls.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfcs/rfc-3348-c-string-literals/no-nuls.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/rfcs/rfc-3348-c-string-literals/no-nuls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-3348-c-string-literals/no-nuls" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-3348-c-string-literals/no-nuls/auxiliary"
stdout: none
--- stderr -------------------------------
error: null character in C string
  --> fake-test-src-base/rfcs/rfc-3348-c-string-literals/no-nuls.rs:7:7
   |
LL |     c"\u{00}";
   |       ^^^^^^ null character in C string
error: null character in C string
error: null character in C string
  --> fake-test-src-base/rfcs/rfc-3348-c-string-literals/no-nuls.rs:10:7
LL |     c"";
   |       ^ null character in C string

error: null character in C string
error: null character in C string
  --> fake-test-src-base/rfcs/rfc-3348-c-string-literals/no-nuls.rs:16:8
LL |     cr"";
   |        ^ null character in C string

error: aborting due to 3 previous errors
error: aborting due to 3 previous errors
------------------------------------------



failures:
    [ui] tests/ui/lexer/lex-bad-char-literals-1.rs
    [ui] tests/ui/rfcs/rfc-3348-c-string-literals/no-nuls.rs
test result: FAILED. 14427 passed; 2 failed; 135 ignored; 0 measured; 0 filtered out; finished in 144.38s

Build completed unsuccessfully in 0:13:18
