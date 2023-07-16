plain
   Compiling clippy_lints v0.1.63 (/checkout/src/tools/clippy/clippy_lints)
error[E0106]: missing lifetime specifier
  --> src/tools/clippy/clippy_lints/src/unused_io_amount.rs:80:60
   |
80 | fn try_remove_await<'a>(expr: &'a hir::Expr<'a>) -> Option<&hir::Expr<'a>> {
   |                               -----------------            ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value with an elided lifetime, but the lifetime cannot be derived from the arguments
help: consider using the `'a` lifetime
   |
80 | fn try_remove_await<'a>(expr: &'a hir::Expr<'a>) -> Option<&'a hir::Expr<'a>> {


error[E0759]: `call` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
    |
    |
95  | fn check_map_error(cx: &LateContext<'_>, call: &hir::Expr<'_>, expr: &hir::Expr<'_>) {
    |                                                -------------- this data with an anonymous lifetime `'_`...
96  |     let mut call = call;
    |                    ^^^^ ...is used here...
...
105 |     if let Some(call) = try_remove_await(call) {
    |                         ---------------------- ...and is required to live as long as `'static` here
Some errors have detailed explanations: E0106, E0759.
For more information about an error, try `rustc --explain E0106`.
error: could not compile `clippy_lints` due to 2 previous errors
Building stage2 tool rls (x86_64-unknown-linux-gnu)
---
   Compiling proc-macro-crate v0.1.5
error[E0106]: missing lifetime specifier
  --> src/tools/clippy/clippy_lints/src/unused_io_amount.rs:80:60
   |
80 | fn try_remove_await<'a>(expr: &'a hir::Expr<'a>) -> Option<&hir::Expr<'a>> {
   |                               -----------------            ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value with an elided lifetime, but the lifetime cannot be derived from the arguments
help: consider using the `'a` lifetime
   |
80 | fn try_remove_await<'a>(expr: &'a hir::Expr<'a>) -> Option<&'a hir::Expr<'a>> {

   Compiling jsonrpc-core v18.0.0
warning: passing `Span` by reference
   --> src/tools/rls/racer/src/racer/ast.rs:255:43
---
   Compiling jsonrpc-derive v18.0.0
   Compiling jsonrpc-server-utils v18.0.0
   Compiling jsonrpc-pubsub v18.0.0
   Compiling jsonrpc-ipc-server v18.0.0
error[E0759]: `call` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
    |
    |
95  | fn check_map_error(cx: &LateContext<'_>, call: &hir::Expr<'_>, expr: &hir::Expr<'_>) {
    |                                                -------------- this data with an anonymous lifetime `'_`...
96  |     let mut call = call;
    |                    ^^^^ ...is used here...
...
105 |     if let Some(call) = try_remove_await(call) {
    |                         ---------------------- ...and is required to live as long as `'static` here
Some errors have detailed explanations: E0106, E0759.
For more information about an error, try `rustc --explain E0106`.
error: could not compile `clippy_lints` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
---
   Compiling git2 v0.14.2
error[E0106]: missing lifetime specifiers
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/vergen-5.1.0/src/gen.rs:106:73
    |
106 | fn some_vals<'a>(tuple: (&'a VergenKey, &'a Option<String>)) -> Option<(&VergenKey, &String)> {
    |                         -----------------------------------             ^           ^ expected named lifetime parameter
    |                                                                         expected named lifetime parameter
    |
    |
    = help: this function's return type contains a borrowed value with an elided lifetime, but the lifetime cannot be derived from the arguments
help: consider using the `'a` lifetime
    |
106 | fn some_vals<'a>(tuple: (&'a VergenKey, &'a Option<String>)) -> Option<(&'a VergenKey, &'a String)> {
    |                                                                          ++             ++

error[E0759]: `tuple` has lifetime `'a` but it needs to satisfy a `'static` lifetime requirement
    |
    |
106 | fn some_vals<'a>(tuple: (&'a VergenKey, &'a Option<String>)) -> Option<(&VergenKey, &String)> {
    |                         ----------------------------------- this data with lifetime `'a`...
107 |     if tuple.1.is_some() {
108 |         Some((tuple.0, tuple.1.as_ref().unwrap()))
    |               |
    |               |
    |               ...is used and required to live as long as `'static` here
    |
note: `'static` lifetime requirement introduced by the return type
    |
    |
106 | fn some_vals<'a>(tuple: (&'a VergenKey, &'a Option<String>)) -> Option<(&VergenKey, &String)> {
    |                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ requirement introduced by this return type
107 |     if tuple.1.is_some() {
108 |         Some((tuple.0, tuple.1.as_ref().unwrap()))
    |         ------------------------------------------ because of this returned expression

error[E0759]: `tuple` has lifetime `'a` but it needs to satisfy a `'static` lifetime requirement
    |
    |
106 | fn some_vals<'a>(tuple: (&'a VergenKey, &'a Option<String>)) -> Option<(&VergenKey, &String)> {
    |                         ----------------------------------- this data with lifetime `'a`...
107 |     if tuple.1.is_some() {
108 |         Some((tuple.0, tuple.1.as_ref().unwrap()))
    |                        |
    |                        |
    |                        ...is used and required to live as long as `'static` here
    |
note: `'static` lifetime requirement introduced by the return type
    |
    |
106 | fn some_vals<'a>(tuple: (&'a VergenKey, &'a Option<String>)) -> Option<(&VergenKey, &String)> {
    |                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ requirement introduced by this return type
107 |     if tuple.1.is_some() {
108 |         Some((tuple.0, tuple.1.as_ref().unwrap()))
    |         ------------------------------------------ because of this returned expression
Some errors have detailed explanations: E0106, E0759.
For more information about an error, try `rustc --explain E0106`.
For more information about an error, try `rustc --explain E0106`.
error: could not compile `vergen` due to 3 previous errors
failed to test miri: could not build
Build completed successfully in 0:23:35
{"rls":"build-fail","rustbook":"test-fail","embedded-book":"test-pass","rust-by-example":"test-pass","book":"test-pass","nomicon":"test-pass","reference":"test-pass","miri":"build-fail","cargo-miri":"build-fail","edition-guide":"test-pass"}Building rustbuild
    Finished dev [unoptimized] target(s) in 0.23s
---
   Compiling clippy_lints v0.1.63 (/checkout/src/tools/clippy/clippy_lints)
error[E0106]: missing lifetime specifier
  --> src/tools/clippy/clippy_lints/src/unused_io_amount.rs:80:60
   |
80 | fn try_remove_await<'a>(expr: &'a hir::Expr<'a>) -> Option<&hir::Expr<'a>> {
   |                               -----------------            ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value with an elided lifetime, but the lifetime cannot be derived from the arguments
help: consider using the `'a` lifetime
   |
80 | fn try_remove_await<'a>(expr: &'a hir::Expr<'a>) -> Option<&'a hir::Expr<'a>> {


error[E0759]: `call` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
    |
    |
95  | fn check_map_error(cx: &LateContext<'_>, call: &hir::Expr<'_>, expr: &hir::Expr<'_>) {
    |                                                -------------- this data with an anonymous lifetime `'_`...
96  |     let mut call = call;
    |                    ^^^^ ...is used here...
...
105 |     if let Some(call) = try_remove_await(call) {
    |                         ---------------------- ...and is required to live as long as `'static` here
Some errors have detailed explanations: E0106, E0759.
For more information about an error, try `rustc --explain E0106`.
error: could not compile `clippy_lints` due to 2 previous errors
error: could not compile `clippy_lints` due to 2 previous errors
thread 'main' panicked at 'in-tree tool', src/bootstrap/test.rs:655:14
Build completed unsuccessfully in 0:00:11
