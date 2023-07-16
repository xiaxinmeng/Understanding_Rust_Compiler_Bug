plain
---- [ui] src/test/ui/lexer/lex-bad-char-literals-3.rs stdout ----
diff of stderr:

6    |
7 help: if you meant to write a `str` literal, use double quotes
8    |
- LL | static c: char = "●●";
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL | static c: char = "\u{25cf}\u{25cf}";
11 
12 error: character literal may only contain one codepoint
13   --> $DIR/lex-bad-char-literals-3.rs:5:20


17    |
18 help: if you meant to write a `str` literal, use double quotes
19    |
- LL |     let ch: &str = "●●";
-    |                    ~~~~
+ LL |     let ch: &str = "\u{25cf}\u{25cf}";
22 
23 error: aborting due to 2 previous errors
24 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-3/lex-bad-char-literals-3.stderr
To only update this specific test, also pass `--test-args lexer/lex-bad-char-literals-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lexer/lex-bad-char-literals-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-3/auxiliary"
stdout: none
--- stderr -------------------------------
error: character literal may only contain one codepoint
   |
   |
LL | static c: char = '●●';
   |
   |
help: if you meant to write a `str` literal, use double quotes
   |
LL | static c: char = "\u{25cf}\u{25cf}";

error: character literal may only contain one codepoint
  --> /checkout/src/test/ui/lexer/lex-bad-char-literals-3.rs:5:20
   |
   |
LL |     let ch: &str = '●●';
   |
   |
help: if you meant to write a `str` literal, use double quotes
   |
LL |     let ch: &str = "\u{25cf}\u{25cf}";

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/lexer/lex-bad-char-literals-5.rs stdout ----
diff of stderr:

7 help: if you meant to write a `str` literal, use double quotes
8    |
9 LL | static c: char = "\x10\x10";
+    |                  ~~~~~~~~~~~~
11 
12 error: character literal may only contain one codepoint
13   --> $DIR/lex-bad-char-literals-5.rs:5:20
13   --> $DIR/lex-bad-char-literals-5.rs:5:20

18 help: if you meant to write a `str` literal, use double quotes
19    |
20 LL |     let ch: &str = "\x10\x10";
+    |                    ~~~~~~~~~~~~
22 
23 error: aborting due to 2 previous errors
24 
24 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-5/lex-bad-char-literals-5.stderr
To only update this specific test, also pass `--test-args lexer/lex-bad-char-literals-5.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lexer/lex-bad-char-literals-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-5" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-5/auxiliary"
stdout: none
--- stderr -------------------------------
error: character literal may only contain one codepoint
   |
   |
LL | static c: char = '\x10\x10';
   |
   |
help: if you meant to write a `str` literal, use double quotes
   |
LL | static c: char = "\\x10\\x10";

error: character literal may only contain one codepoint
  --> /checkout/src/test/ui/lexer/lex-bad-char-literals-5.rs:5:20
   |
   |
LL |     let ch: &str = '\x10\x10';
   |
   |
help: if you meant to write a `str` literal, use double quotes
   |
LL |     let ch: &str = "\\x10\\x10";

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/parser/issues/issue-64732.rs stdout ----
diff of stderr:

7 help: if you meant to write a byte string literal, use double quotes
8    |
9 LL |     let _foo = b"hello\0";
+    |                ~~~~~~~~~~~
11 
12 error: character literal may only contain one codepoint
13   --> $DIR/issue-64732.rs:6:16
---
To only update this specific test, also pass `--test-args parser/issues/issue-64732.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-64732.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-64732" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-64732/auxiliary"
stdout: none
--- stderr -------------------------------
error: character literal may only contain one codepoint
   |
   |
LL |     let _foo = b'hello\0';
   |
   |
help: if you meant to write a byte string literal, use double quotes
   |
LL |     let _foo = b"hello\\0";

error: character literal may only contain one codepoint
  --> /checkout/src/test/ui/parser/issues/issue-64732.rs:6:16
   |
   |
LL |     let _bar = 'hello';
   |
   |
help: if you meant to write a `str` literal, use double quotes
   |
LL |     let _bar = "hello";

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/parser/unicode-character-literal.rs stdout ----
diff of stderr:

11    |                   ^
12 help: if you meant to write a `str` literal, use double quotes
13    |
- LL |     let _spade = "♠️";
-    |                  ~~~
+ LL |     let _spade = "\u{2660}\u{fe0f}";
16 
17 error: character literal may only contain one codepoint
18   --> $DIR/unicode-character-literal.rs:12:14


27    |               ^
28 help: if you meant to write a `str` literal, use double quotes
29    |
- LL |     let _s = "ṩ̂̊";
-    |              ~~~
+ LL |     let _s = "s\u{323}\u{307}\u{302}\u{30a}";
32 
33 error: character literal may only contain one codepoint
34   --> $DIR/unicode-character-literal.rs:17:14

---

4 // run-rustfix
5 
6 fn main() {
-     let _spade = "♠️";
+     let _spade = "\u{2660}\u{fe0f}";
8     //~^ ERROR: character literal may only contain one codepoint
9     //~| NOTE: this `♠` is followed by the combining mark `\u{fe0f}`
10     //~| HELP: if you meant to write a `str` literal, use double quotes
11 
-     let _s = "ṩ̂̊";
-     let _s = "ṩ̂̊";
+     let _s = "s\u{323}\u{307}\u{302}\u{30a}";
13     //~^ ERROR: character literal may only contain one codepoint
14     //~| NOTE: this `s` is followed by the combining marks `\u{323}\u{307}\u{302}\u{30a}`
15     //~| HELP: if you meant to write a `str` literal, use double quotes

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-character-literal/unicode-character-literal.fixed
To only update this specific test, also pass `--test-args parser/unicode-character-literal.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unicode-character-literal.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-character-literal" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-character-literal/auxiliary"
stdout: none
--- stderr -------------------------------
error: character literal may only contain one codepoint
   |
   |
LL |     let _spade = '♠️';
   |
   |
note: this `♠` is followed by the combining mark `\u{fe0f}`
   |
   |
LL |     let _spade = '♠️';
   |                   ^
help: if you meant to write a `str` literal, use double quotes
   |
LL |     let _spade = "\u{2660}\u{fe0f}";

error: character literal may only contain one codepoint
  --> /checkout/src/test/ui/parser/unicode-character-literal.rs:12:14
   |
   |
LL |     let _s = 'ṩ̂̊';
   |
   |
note: this `s` is followed by the combining marks `\u{323}\u{307}\u{302}\u{30a}`
   |
   |
LL |     let _s = 'ṩ̂̊';
   |               ^
help: if you meant to write a `str` literal, use double quotes
   |
LL |     let _s = "s\u{323}\u{307}\u{302}\u{30a}";

error: character literal may only contain one codepoint
  --> /checkout/src/test/ui/parser/unicode-character-literal.rs:17:14
   |
   |
LL |     let _a = 'Å';
   |              ^-^
   |               |
   |               help: consider using the normalized form `\u{c5}` of this character: `Å`
   |
note: this `A` is followed by the combining mark `\u{30a}`
   |
   |
LL |     let _a = 'Å';

error: aborting due to 3 previous errors
------------------------------------------



---- [ui] src/test/ui/str/str-as-char.rs stdout ----
diff of stderr:

6    |
7 help: if you meant to write a `str` literal, use double quotes
- LL |     println!("●●");
-    |              ~~~~
-    |              ~~~~
+ LL |     println!("\u{25cf}\u{25cf}");
11 
12 error: aborting due to previous error
13 

---

1 // run-rustfix
2 
3 fn main() {
-     println!("●●"); //~ ERROR character literal may only contain one codepoint
+     println!("\u{25cf}\u{25cf}"); //~ ERROR character literal may only contain one codepoint
6 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-as-char/str-as-char.fixed
To only update this specific test, also pass `--test-args str/str-as-char.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/str/str-as-char.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-as-char" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-as-char/auxiliary"
stdout: none
--- stderr -------------------------------
error: character literal may only contain one codepoint
   |
   |
LL |     println!('●●'); //~ ERROR character literal may only contain one codepoint
   |
   |
help: if you meant to write a `str` literal, use double quotes
   |
LL |     println!("\u{25cf}\u{25cf}"); //~ ERROR character literal may only contain one codepoint

error: aborting due to previous error
------------------------------------------

