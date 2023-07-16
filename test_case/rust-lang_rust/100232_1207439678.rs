plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 5651759746e6eefa691f61522d19b46d07cf9244 and 5a6bf01ac1cc5b3eabaa96af806ee58aa58efa07
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/rls/racer/src/racer/ast.rs:621:34
     |
621  |             ExprKind::MethodCall(ref method_def, ref arguments, _) => {
     |
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1347:16
     |
     |
1347 |     MethodCall(PathSegment, P<Expr>, Vec<P<Expr>>, Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
621  |             ExprKind::MethodCall(ref method_def, ref arguments, _, _) => {

error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src/tools/rls/racer/src/racer/ast.rs:623:51
    |
    |
623 |                 debug!("method call ast name {}", methodname);
    |                 |                                 |
    |                 |                                 doesn't have a size known at compile-time
    |                 in this macro invocation (#1)
    |
---
    |
877 |     macro_rules! format_args {
    |     ------------------------ in this expansion of `format_args!` (#5)
    |
    = help: the trait `Sized` is not implemented for `str`
note: required by a bound in `ArgumentV1::<'a>::new_display`
    |
    |
313 | / macro_rules! arg_new {
314 | |     ($f: ident, $t: ident) => {
315 | |         #[doc(hidden)]
316 | |         #[unstable(feature = "fmt_internals", reason = "internal to format_args!", issue = "none")]
317 | |         #[inline]
318 | |         pub fn $f<'b, T: $t>(x: &'b T) -> ArgumentV1<'_> {
    | |                       ^ required by this bound in `ArgumentV1::<'a>::new_display`
321 | |     };
322 | | }
322 | | }
    | |_- in this expansion of `arg_new!`
...
341 |       arg_new!(new_display, Display);

   Compiling futures-executor v0.3.19
Some errors have detailed explanations: E0023, E0277.
For more information about an error, try `rustc --explain E0023`.
---
Verifying status of rls...
Verifying status of miri...
Verifying status of embedded-book...
Cloning into 'rust-toolstate'...
error: Tool `rls` has regressed from test-pass to build-fail during beta week.
