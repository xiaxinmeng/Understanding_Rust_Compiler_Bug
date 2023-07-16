plain
iii................................................................................................. 12700/12705
.....
failures:

---- [ui] ui/macros/rfc-3086-metavar-expr/syntax-errors.rs stdout ----


16 LL |     ( $( $i:ident ),* ) => { ${ length() a b c } };
18 
- error: expected `(`, found `{`
-   --> $DIR/syntax-errors.rs:63:8
-    |
-    |
- LL |     ( ${ length() } ) => {
- 
- 
- error: expected one of: `*`, `+`, or `?`
-   --> $DIR/syntax-errors.rs:63:8
-    |
- LL |     ( ${ length() } ) => {
- 
31 error: meta-variable expression depth must be a literal
32   --> $DIR/syntax-errors.rs:69:33
33    |
33    |

230 LL |     no_curly__rhs_dollar__no_round!(a);
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
232 
- error: aborting due to 27 previous errors
+ error: aborting due to 25 previous errors
234 
234 
235 For more information about this error, try `rustc --explain E0425`.
236 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/rfc-3086-metavar-expr/syntax-errors/syntax-errors.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/rfc-3086-metavar-expr/syntax-errors.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/rfc-3086-metavar-expr/syntax-errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/rfc-3086-metavar-expr/syntax-errors/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:16:33
   |
LL |     ( $( $i:ident ),* ) => { ${ count($i) } };
   |                                 ^^^^^ - help: try removing `$`
error: expected identifier, found `$`
error: expected identifier, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:22:26
   |
LL |     ( $i:ident ) => { ${ count($i) } };
   |                          ^^^^^ - help: try removing `$`
error: meta-variable expression must not have trailing statements
error: meta-variable expression must not have trailing statements
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:57:42
   |
LL |     ( $( $i:ident ),* ) => { ${ length() a b c } };

error: meta-variable expression depth must be a literal
error: meta-variable expression depth must be a literal
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:69:33
   |
LL |     ( $( $i:ident ),* ) => { ${ index(IDX) } };

error: expected identifier
error: expected identifier
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:75:33
   |
LL |     ( $( $i:ident ),* ) => { ${ ignore() } };

error: only unsuffixes literals are not supported in meta-variable expressions
error: only unsuffixes literals are not supported in meta-variable expressions
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:81:33
   |
LL |     ( $( $i:ident ),* ) => { ${ index(1u32) } };

error: meta-variable expression parameter must be wrapped in parentheses
error: meta-variable expression parameter must be wrapped in parentheses
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:87:33
   |
LL |     ( $( $i:ident ),* ) => { ${ count{i} } };

error: expected identifier
error: expected identifier
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:93:31
   |
LL |     ( $( $i:ident ),* ) => { ${ {} } };

error: unrecognized meta-variable expression
error: unrecognized meta-variable expression
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:99:33
   |
LL |     ( $( $i:ident ),* ) => { ${ aaaaaaaaaaaaaa(i) } };
   |                                 ^^^^^^^^^^^^^^ help: supported expressions are count, ignore, index and length
error: expected expression, found `$`
error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:16:30
   |
LL |     ( $( $i:ident ),* ) => { ${ count($i) } };
   |                              ^ expected expression
...
LL |     curly__rhs_dollar__round!(a, b, c);
   |
   |
   = note: this error originates in the macro `curly__rhs_dollar__round` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected expression, found `$`
error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:22:23
   |
LL |     ( $i:ident ) => { ${ count($i) } };
   |                       ^ expected expression
...
LL |     curly__rhs_dollar__no_round!(a);
   |
   |
   = note: this error originates in the macro `curly__rhs_dollar__no_round` (in Nightly builds, run with -Z macro-backtrace for more info)
error: variable 'i' is still repeating at this depth
error: variable 'i' is still repeating at this depth
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:40:36
   |
LL |     ( $( $i:ident ),* ) => { count($i) };

error: expected expression, found `$`
error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:57:30
   |
LL |     ( $( $i:ident ),* ) => { ${ length() a b c } };
   |                              ^ expected expression
...
LL |     extra_garbage_after_metavar!(a);
   |
   |
   = note: this error originates in the macro `extra_garbage_after_metavar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected expression, found `$`
error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:99:30
   |
LL |     ( $( $i:ident ),* ) => { ${ aaaaaaaaaaaaaa(i) } };
   |                              ^ expected expression
...
LL |     unknown_metavar!(a);
   |
   |
   = note: this error originates in the macro `unknown_metavar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected expression, found `$`
error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:87:30
   |
LL |     ( $( $i:ident ),* ) => { ${ count{i} } };
   |                              ^ expected expression
...
LL |     metavar_without_parens!(a);
   |
   |
   = note: this error originates in the macro `metavar_without_parens` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected expression, found `$`
error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:75:30
   |
LL |     ( $( $i:ident ),* ) => { ${ ignore() } };
   |                              ^ expected expression
...
LL |     metavar_token_without_ident!(a);
   |
   |
   = note: this error originates in the macro `metavar_token_without_ident` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected expression, found `$`
error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:69:30
   |
LL |     ( $( $i:ident ),* ) => { ${ index(IDX) } };
   |                              ^ expected expression
...
LL |     metavar_depth_is_not_literal!(a);
   |
   |
   = note: this error originates in the macro `metavar_depth_is_not_literal` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected expression, found `$`
error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:81:30
   |
LL |     ( $( $i:ident ),* ) => { ${ index(1u32) } };
   |                              ^ expected expression
...
LL |     metavar_with_literal_suffix!(a);
   |
   |
   = note: this error originates in the macro `metavar_with_literal_suffix` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected expression, found `$`
error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:93:30
   |
LL |     ( $( $i:ident ),* ) => { ${ {} } };
   |                              ^ expected expression
...
LL |     open_brackets_without_tokens!(a)
   |
   |
   = note: this error originates in the macro `open_brackets_without_tokens` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0425]: cannot find function `count` in this scope
error[E0425]: cannot find function `count` in this scope
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:28:30
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
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:28:36
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
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:34:23
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
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:34:29
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
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:45:23
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
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:112:37
   |
LL |     no_curly__rhs_dollar__no_round!(a);

error: aborting due to 25 previous errors

For more information about this error, try `rustc --explain E0425`.
For more information about this error, try `rustc --explain E0425`.
------------------------------------------



failures:
    [ui] ui/macros/rfc-3086-metavar-expr/syntax-errors.rs
test result: FAILED. 12587 passed; 1 failed; 117 ignored; 0 measured; 0 filtered out; finished in 130.39s

Build completed unsuccessfully in 0:13:17
