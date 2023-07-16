plain
..........................iii........................................................... 13288/13333
.............................................
failures:

---- [ui] src/test/ui/lint/unused_parens_multibyte_recovery.rs stdout ----

1 error: this file contains an unclosed delimiter
-   --> $DIR/unused_parens_multibyte_recovery.rs:9:17
+   --> $DIR/unused_parens_multibyte_recovery.rs:11:17
+   --> $DIR/unused_parens_multibyte_recovery.rs:11:17
3    |
4 LL | fn f(){(print!(á
5    |       --      - ^
9    |       unclosed delimiter
10 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
11 error: this file contains an unclosed delimiter
11 error: this file contains an unclosed delimiter
-   --> $DIR/unused_parens_multibyte_recovery.rs:9:17
+   --> $DIR/unused_parens_multibyte_recovery.rs:11:17
13    |
14 LL | fn f(){(print!(á
15    |       --      - ^
19    |       unclosed delimiter
20 
21 error: this file contains an unclosed delimiter
-   --> $DIR/unused_parens_multibyte_recovery.rs:9:17
-   --> $DIR/unused_parens_multibyte_recovery.rs:9:17
+   --> $DIR/unused_parens_multibyte_recovery.rs:11:17
23    |
24 LL | fn f(){(print!(á
25    |       --      - ^
29    |       unclosed delimiter
30 
31 error: format argument must be a string literal
-   --> $DIR/unused_parens_multibyte_recovery.rs:9:16
-   --> $DIR/unused_parens_multibyte_recovery.rs:9:16
+   --> $DIR/unused_parens_multibyte_recovery.rs:11:16
33    |
34 LL | fn f(){(print!(á


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_multibyte_recovery/unused_parens_multibyte_recovery.stderr
To only update this specific test, also pass `--test-args lint/unused_parens_multibyte_recovery.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused_parens_multibyte_recovery.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_multibyte_recovery" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_multibyte_recovery/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lint/unused_parens_multibyte_recovery.rs:11:17
   |
   |
LL | fn f(){(print!(á
   |       --      - ^
   |       ||      unclosed delimiter
   |       |unclosed delimiter
   |       unclosed delimiter


error: this file contains an unclosed delimiter
  --> /checkout/src/test/ui/lint/unused_parens_multibyte_recovery.rs:11:17
   |
LL | fn f(){(print!(á
   |       --      - ^
   |       ||      unclosed delimiter
   |       |unclosed delimiter
   |       unclosed delimiter


error: this file contains an unclosed delimiter
  --> /checkout/src/test/ui/lint/unused_parens_multibyte_recovery.rs:11:17
   |
LL | fn f(){(print!(á
   |       --      - ^
   |       ||      unclosed delimiter
   |       |unclosed delimiter
   |       unclosed delimiter


error: format argument must be a string literal
  --> /checkout/src/test/ui/lint/unused_parens_multibyte_recovery.rs:11:16
   |
LL | fn f(){(print!(á
   |
help: you might be missing a string literal to format with
   |
   |
LL | fn f(){(print!("{}", á

error: aborting due to 4 previous errors
------------------------------------------

