plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0061]: this function takes 1 argument but 2 arguments were supplied
   --> src/librustdoc/passes/check_code_block_syntax.rs:49:29
    |
49  |         let span = DUMMY_SP.fresh_expansion(expn_data, self.cx.tcx.create_stable_hashing_context());
    |                             |
    |                             expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_span/src/hygiene.rs:877:12
    |
877 |     pub fn fresh_expansion(self, expn_id: LocalExpnId) -> Span {

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:06
