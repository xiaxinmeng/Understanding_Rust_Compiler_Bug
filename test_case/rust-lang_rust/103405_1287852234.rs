plain
   Compiling getopts v0.2.21
error: unexpected `if` in the condition expression
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.102/src/path.rs:298:22
    |
298 |                 } && if input.peek(Token![=]) {
    |
help: remove the `if`
    |
    |
298 -                 } && if input.peek(Token![=]) {
298 +                 } && input.peek(Token![=]) {

   Compiling perf-event-open-sys v1.0.1
   Compiling rand_core v0.6.2
   Compiling memmap2 v0.2.1
---
   Compiling md-5 v0.10.0
error[E0308]: mismatched types
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.102/src/path.rs:301:21
    |
286 | /                 if match &argument {
287 | |                     Type::Path(argument)
288 | |                         if argument.qself.is_none()
289 | |                             && argument.path.leading_colon.is_none()
301 | |                     true
    | |                     ^^^^ expected `()`, found `bool`
...   |
307 | |                     false
---
   Compiling adler v0.2.3
error[E0308]: mismatched types
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.102/src/path.rs:305:21
    |
302 |                   } else if input.peek(Token![:]) {
    |  ________________________-
303 | |                     input.parse::<Token![:]>()?;
304 | |                     input.call(constraint_bounds)?;
    | |                     ^^^^ expected `()`, found `bool`
306 | |                 } else {
307 | |                     false
308 | |                 } {
308 | |                 } {
    | |_________________- expected this to be `()`

   Compiling unicode-script v0.5.5
   Compiling miniz_oxide v0.4.0
error[E0308]: mismatched types
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.102/src/path.rs:307:21
    |
302 |                   } else if input.peek(Token![:]) {
    |  ________________________-
303 | |                     input.parse::<Token![:]>()?;
304 | |                     input.call(constraint_bounds)?;
306 | |                 } else {
307 | |                     false
    | |                     ^^^^^ expected `()`, found `bool`
308 | |                 } {
