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
......................i................................................................. 7040/14378
............................................................................i........... 7128/14378
................................................iiii........i....i...................... 7216/14378
.............................................................i.......................... 7304/14378
...................................F.............F...................................... 7392/14378
.........i..................i.............i............................................. 7568/14378
.....................i.................................................................. 7656/14378
...........................................i............................................ 7744/14378
........................................................................................ 7832/14378
---
2   --> $DIR/lex-bad-char-literals-1.rs:2:6
3    |
4 LL |     '\x1'
-    |      ^^^
+    |      ^^^ numeric character escape is too short
7 error: numeric character escape is too short
8   --> $DIR/lex-bad-char-literals-1.rs:6:6

9    |
9    |
10 LL |     "\x1"
-    |      ^^^
+    |      ^^^ numeric character escape is too short
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
12 
13 error: unknown character escape: `\u{25cf}`


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
   |      ^^^ numeric character escape is too short
error: numeric character escape is too short
  --> fake-test-src-base/lexer/lex-bad-char-literals-1.rs:6:6
   |
   |
LL |     "\x1" //~ ERROR: numeric character escape is too short
   |      ^^^ numeric character escape is too short

error: unknown character escape: `\u{25cf}`
  --> fake-test-src-base/lexer/lex-bad-char-literals-1.rs:10:7
   |
LL |     '\●' //~ ERROR: unknown character escape
   |       ^ unknown character escape
   |
   = help: for more information, visit <https://static.rust-lang.org/doc/master/reference.html#literals>
help: if you meant to write a literal backslash (perhaps escaping in a regular expression), consider a raw string literal
   |
LL |     r"\●" //~ ERROR: unknown character escape


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



---- [ui] tests/ui/lexer/lex-bare-cr-string-literal-doc-comment.rs stdout ----
diff of stderr:

32   --> $DIR/lex-bare-cr-string-literal-doc-comment.rs:22:19
33    |
34 LL |     let _s = r"bar
foo";
-    |                   ^
+    |                   ^ bare CR not allowed in raw string
36 
37 error: unknown character escape: `\r`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bare-cr-string-literal-doc-comment/lex-bare-cr-string-literal-doc-comment.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bare-cr-string-literal-doc-comment/lex-bare-cr-string-literal-doc-comment.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lexer/lex-bare-cr-string-literal-doc-comment.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lexer/lex-bare-cr-string-literal-doc-comment.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bare-cr-string-literal-doc-comment" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bare-cr-string-literal-doc-comment/auxiliary"
stdout: none
--- stderr -------------------------------
error: bare CR not allowed in doc-comment
  --> fake-test-src-base/lexer/lex-bare-cr-string-literal-doc-comment.rs:3:32
   |
LL | /// doc comment with bare CR: '
   |                                ^


error: bare CR not allowed in block doc-comment
  --> fake-test-src-base/lexer/lex-bare-cr-string-literal-doc-comment.rs:7:38
   |
LL | /** block doc comment with bare CR: '
   |                                      ^


error: bare CR not allowed in doc-comment
  --> fake-test-src-base/lexer/lex-bare-cr-string-literal-doc-comment.rs:12:36
   |
LL |     //! doc comment with bare CR: '
   |                                    ^


error: bare CR not allowed in block doc-comment
  --> fake-test-src-base/lexer/lex-bare-cr-string-literal-doc-comment.rs:15:42
   |
LL |     /*! block doc comment with bare CR: '
   |                                          ^


error: bare CR not allowed in string, use `\r` instead
  --> fake-test-src-base/lexer/lex-bare-cr-string-literal-doc-comment.rs:19:18
LL |     let _s = "foo
LL |     let _s = "foo
bar"; //~ ERROR: bare CR not allowed in string
   |                  ^ help: escape the character: `\r`

error: bare CR not allowed in raw string
  --> fake-test-src-base/lexer/lex-bare-cr-string-literal-doc-comment.rs:22:19
   |
LL |     let _s = r"bar
foo"; //~ ERROR: bare CR not allowed in raw string
   |                   ^ bare CR not allowed in raw string

error: unknown character escape: `\r`
  --> fake-test-src-base/lexer/lex-bare-cr-string-literal-doc-comment.rs:25:19
LL |     let _s = "foo\
LL |     let _s = "foo\
bar"; //~ ERROR: unknown character escape: `\r`
   |                   ^ unknown character escape
   = help: this is an isolated carriage return; consider checking your editor and version control settings

error: aborting due to 7 previous errors
------------------------------------------
---
3    |
4 LL |     br"a
";
-    |         ^
+    |         ^ bare CR not allowed in raw string
6 
7 error: non-ASCII character in raw byte string literal


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw-byte-string-literals/raw-byte-string-literals.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw-byte-string-literals/raw-byte-string-literals.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/raw/raw-byte-string-literals.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/raw/raw-byte-string-literals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw-byte-string-literals" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw-byte-string-literals/auxiliary"
stdout: none
--- stderr -------------------------------
error: bare CR not allowed in raw string
  --> fake-test-src-base/parser/raw/raw-byte-string-literals.rs:4:9
LL |     br"a
LL |     br"a
"; //~ ERROR bare CR not allowed in raw string
   |         ^ bare CR not allowed in raw string

error: non-ASCII character in raw byte string literal
  --> fake-test-src-base/parser/raw/raw-byte-string-literals.rs:5:8
   |
LL |     br"é";  //~ ERROR non-ASCII character in raw byte string literal
   |        ^ must be ASCII

error: found invalid character; only `#` is allowed in raw string delimitation: ~
  --> fake-test-src-base/parser/raw/raw-byte-string-literals.rs:6:5
   |
LL |     br##~"a"~##;  //~ ERROR only `#` is allowed in raw string delimitation

error: aborting due to 3 previous errors
------------------------------------------



---- [ui] tests/ui/parser/unicode-quote-chars.rs stdout ----
diff of stderr:

4 LL |     println!(“hello world”);
6    |
6    |
- help: Unicode characters '“' (Left Double Quotation Mark) and '”' (Right Double Quotation Mark) look like '"' (Quotation Mark), but are not
+ help: Unicode characters '“' (Left Double Quotation Mark) and '”' (Right Double Quotation Mark) look like '"' (Quotation Mark), but are not",
9 LL |     println!("hello world");
10    |              ~~~~~~~~~~~~~



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-quote-chars/unicode-quote-chars.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/unicode-quote-chars.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/unicode-quote-chars.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-quote-chars" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-quote-chars/auxiliary"
stdout: none
--- stderr -------------------------------
error: unknown start of token: \u{201c}
  --> fake-test-src-base/parser/unicode-quote-chars.rs:2:14
   |
LL |     println!(“hello world”);
   |
   |
help: Unicode characters '“' (Left Double Quotation Mark) and '”' (Right Double Quotation Mark) look like '"' (Quotation Mark), but are not",
LL |     println!("hello world");
   |              ~~~~~~~~~~~~~


error: unknown start of token: \u{201d}
  --> fake-test-src-base/parser/unicode-quote-chars.rs:2:26
   |
LL |     println!(“hello world”);
   |
   |
help: Unicode character '”' (Right Double Quotation Mark) looks like '"' (Quotation Mark), but it is not
   |
LL |     println!(“hello world");


error: expected `,`, found `world`
  --> fake-test-src-base/parser/unicode-quote-chars.rs:2:21
   |
LL |     println!(“hello world”);
   |                     ^^^^^ expected `,`
error: aborting due to 3 previous errors
------------------------------------------



---- [ui] tests/ui/str/str-escape.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/str/str-escape.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-escape" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-escape/auxiliary"
stdout: none
--- stderr -------------------------------
error: multiple lines skipped by escaped newline
  --> fake-test-src-base/str/str-escape.rs:3:14
LL |       let s = "\
   |  ______________^
LL | |
LL | |              ";
LL | |              ";
   | |_____________^ skipping everything up to and including this point

warning: non-ASCII whitespace symbol '\u{a0}' is not skipped
  --> fake-test-src-base/str/str-escape.rs:7:17
LL |       let s = "foo\
   |  _________________^
LL | |              bar
LL | |              bar
   | |   ^ non-ASCII whitespace symbol '\u{a0}' is not skipped
   | |___|

error: aborting due to previous error; 1 warning emitted
------------------------------------------

