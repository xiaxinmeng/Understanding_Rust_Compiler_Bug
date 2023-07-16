plain

---- [ui] ui/parser/macro/macro-repeat.rs stdout ----
diff of stderr:

- error: variable 'v' is still repeating at this depth
+ error: variable `v` is still repeating at this depth
3    |
4 LL |         $v

5    |         ^^
5    |         ^^
6 
- error: variable 'v' is still repeating at this depth
+ error: variable `v` is still repeating at this depth
9    |
10 LL |         $v



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro/macro-repeat/macro-repeat.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/macro/macro-repeat.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/macro/macro-repeat.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro/macro-repeat" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro/macro-repeat/auxiliary"
------------------------------------------

------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stderr:
------------------------------------------
error: variable `v` is still repeating at this depth
   |
LL |         $v
   |         ^^


error: variable `v` is still repeating at this depth
   |
LL |         $v
   |         ^^


error: aborting due to 2 previous errors


------------------------------------------


---- [ui] ui/rfc-3086-metavar-expr/syntax-errors.rs stdout ----


78 LL |     ( $i:ident ) => { ${ count($i) } };
80 
- error: variable 'i' is still repeating at this depth
- error: variable 'i' is still repeating at this depth
+ error: variable `i` is still repeating at this depth
83    |
83    |
84 LL |     ( $( $i:ident ),* ) => { count($i };

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-3086-metavar-expr/syntax-errors/syntax-errors.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-3086-metavar-expr/syntax-errors.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-3086-metavar-expr/syntax-errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-3086-metavar-expr/syntax-errors/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: mismatched closing delimiter: `}`
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:47:35
   |
LL |     ( $( $i:ident ),* ) => { count($i };
   |                            -      ^   ^ mismatched closing delimiter
   |                            |      unclosed delimiter
   |                            closing delimiter possibly meant for this

error: expected meta-variable expression argument
error: expected meta-variable expression argument
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:17:39
   |
LL |     ( $( $i:ident ),* ) => { ${ count($i) } };

error: expected ','
error: expected ','
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:17:40
   |
LL |     ( $( $i:ident ),* ) => { ${ count($i) } };

error: count requires an identifier to count
error: count requires an identifier to count
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:17:33
   |
LL |     ( $( $i:ident ),* ) => { ${ count($i) } };


error: expected one of: `*`, `+`, or `?`
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:17:31
   |
LL |     ( $( $i:ident ),* ) => { ${ count($i) } };

error: expected meta-variable expression argument
error: expected meta-variable expression argument
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:26:32
   |
LL |     ( $i:ident ) => { ${ count($i) } };

error: expected ','
error: expected ','
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:26:33
   |
LL |     ( $i:ident ) => { ${ count($i) } };

error: count requires an identifier to count
error: count requires an identifier to count
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:26:26
   |
LL |     ( $i:ident ) => { ${ count($i) } };


error: expected one of: `*`, `+`, or `?`
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:26:24
   |
LL |     ( $i:ident ) => { ${ count($i) } };


error: variable 'i' does not repeat at this depth
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:12:24
   |
LL |     ( $i:ident ) => { ${ count(i) } };


error: expected one of `.`, `;`, `?`, `}`, or an operator, found `count`
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:17:33
   |
LL |     ( $( $i:ident ),* ) => { ${ count($i) } };
   |                                 ^^^^^ expected one of `.`, `;`, `?`, `}`, or an operator
...
LL |     curly__rhs_dollar__round!(a, b, c);
   |
   |
   = note: this error originates in the macro `curly__rhs_dollar__round` (in Nightly builds, run with -Z macro-backtrace for more info)
error: attempted to repeat an expression containing no syntax variables matched as repeating at this depth
error: attempted to repeat an expression containing no syntax variables matched as repeating at this depth
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:26:24
   |
LL |     ( $i:ident ) => { ${ count($i) } };


error: variable `i` is still repeating at this depth
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:47:36
   |
LL |     ( $( $i:ident ),* ) => { count($i };

error[E0425]: cannot find function `count` in this scope
error[E0425]: cannot find function `count` in this scope
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:35:30
   |
LL |     ( $( $i:ident ),* ) => { count(i) };
...
...
LL |     no_curly__no_rhs_dollar__round!(a, b, c);
   |
   |
   = note: this error originates in the macro `no_curly__no_rhs_dollar__round` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0425]: cannot find value `i` in this scope
error[E0425]: cannot find value `i` in this scope
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:35:36
   |
LL |     ( $( $i:ident ),* ) => { count(i) };
...
...
LL |     no_curly__no_rhs_dollar__round!(a, b, c);
   |
   |
   = note: this error originates in the macro `no_curly__no_rhs_dollar__round` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0425]: cannot find function `count` in this scope
error[E0425]: cannot find function `count` in this scope
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:41:23
   |
LL |     ( $i:ident ) => { count(i) };
...
...
LL |     no_curly__no_rhs_dollar__no_round!(a);
   |
   |
   = note: this error originates in the macro `no_curly__no_rhs_dollar__no_round` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0425]: cannot find value `i` in this scope
error[E0425]: cannot find value `i` in this scope
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:41:29
   |
LL |     ( $i:ident ) => { count(i) };
...
...
LL |     no_curly__no_rhs_dollar__no_round!(a);
   |
   |
   = note: this error originates in the macro `no_curly__no_rhs_dollar__no_round` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0425]: cannot find function `count` in this scope
error[E0425]: cannot find function `count` in this scope
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:53:23
   |
LL |     ( $i:ident ) => { count($i) };
...
...
LL |     no_curly__rhs_dollar__no_round!(a);
   |
   |
   = note: this error originates in the macro `no_curly__rhs_dollar__no_round` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0425]: cannot find value `a` in this scope
error[E0425]: cannot find value `a` in this scope
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/syntax-errors.rs:65:37
   |
LL |     no_curly__rhs_dollar__no_round!(a);

error: aborting due to 19 previous errors

For more information about this error, try `rustc --explain E0425`.
For more information about this error, try `rustc --explain E0425`.

------------------------------------------



failures:
    [ui] ui/parser/macro/macro-repeat.rs
    [ui] ui/rfc-3086-metavar-expr/syntax-errors.rs
test result: FAILED. 12456 passed; 2 failed; 121 ignored; 0 measured; 0 filtered out; finished in 98.73s

Build completed unsuccessfully in 0:10:19
