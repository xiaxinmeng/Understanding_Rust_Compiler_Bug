plain
    Checking structopt v0.3.25
error[E0382]: use of moved value: `ts`
   --> src/tools/rustfmt/src/macros.rs:238:41
    |
211 |     let ts = mac.args.inner_tokens();
    |         -- move occurs because `ts` has type `TokenStream`, which does not implement the `Copy` trait
...
229 |         if let success @ Some(..) = format_lazy_static(context, shape, ts.into_trees().collect()) {
    |                                                                           ------------ `ts` moved due to this method call
...
238 |     } = match parse_macro_args(context, ts, style, is_forced_bracket) {
    |                                         ^^ value used here after move
    |
note: this function takes ownership of the receiver `self`, which moves `ts`
    |
    |
449 |     pub fn into_trees(self) -> Cursor {

For more information about this error, try `rustc --explain E0382`.
error: could not compile `rustfmt-nightly` due to previous error
warning: build failed, waiting for other jobs to finish...
