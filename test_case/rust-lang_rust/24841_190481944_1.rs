
   Compiling parser v0.1.0 (file:///private/tmp/RustParser)
src/parser.rs:41:15: 41:25 error: attempted access of field `span` on type `ast::N<ast::Ty>`, but no field with that name was found
src/parser.rs:41        let start = $node.span;
                                    ^~~~~~~~~~
src/parser.rs:321:8: 324:6 note: in this expansion of extend! (defined in src/parser.rs)
src/parser.rs:41:21: 41:25 help: did you mean `val`?
src/parser.rs:41        let start = $node.span;
                                          ^~~~
src/parser.rs:321:8: 324:6 note: in this expansion of extend! (defined in src/parser.rs)
src/parser.rs:364:4: 380:5 error: match arms have incompatible types:
 expected `ast::Expr`,
    found `ast::N<ast::Expr>`
(expected enum `ast::Expr`,
    found struct `ast::N`) [E0308]
src/parser.rs:364           match self.tok() {
src/parser.rs:365               Token::Name(KW_LOOP) => {
src/parser.rs:366                   let baseline = self.lexer.indent;
src/parser.rs:367                   self.step();
src/parser.rs:368                   Expr::Loop(self.block(baseline, Parser::try_expr))
src/parser.rs:369               }
                  ...
src/parser.rs:363:3: 381:5 note: in this expansion of noded! (defined in src/parser.rs)
src/parser.rs:364:4: 380:5 help: run `rustc --explain E0308` to see a detailed explanation
src/parser.rs:379:11: 379:33 note: match arm with an incompatible type
src/parser.rs:379               _ =>  self.assign_operator()
                                      ^~~~~~~~~~~~~~~~~~~~~~
src/parser.rs:363:3: 381:5 note: in this expansion of noded! (defined in src/parser.rs)
src/parser.rs:385:16: 385:29 error: no method named `pred_operator` found for type `&mut parser::Parser<'c>` in the current scope
src/parser.rs:385       let r = self.pred_operator(self.unary());
                                     ^~~~~~~~~~~~~
src/parser.rs:385:35: 385:40 error: no method named `unary` found for type `&mut parser::Parser<'c>` in the current scope
src/parser.rs:385       let r = self.pred_operator(self.unary());
                                                        ^~~~~
src/parser.rs:41:15: 41:25 error: the type of this value must be known in this context
src/parser.rs:41        let start = $node.span;
                                    ^~~~~~~~~~
src/parser.rs:389:5: 392:7 note: in this expansion of extend! (defined in src/parser.rs)
src/parser.rs:391:6: 391:18 error: no associated item named `Assign` found for type `ast::Expr` in the current scope
src/parser.rs:391                   Expr::Assign(OP_ASSIGN, r, self.expr())
                                    ^~~~~~~~~~~~
src/parser.rs:389:5: 392:7 note: in this expansion of extend! (defined in src/parser.rs)
src/parser.rs:399:14: 399:24 error: no method named `is_prec_op` found for type `&mut parser::Parser<'c>` in the current scope
src/parser.rs:399       while self.is_prec_op() {
                                   ^~~~~~~~~~
src/parser.rs:402:15: 402:23 error: attempted access of field `src` on type `&mut parser::Parser<'c>`, but no field with that name was found
src/parser.rs:402           let prec = self.src.ctx.op_map.get(op);
                                       ^~~~~~~~
src/parser.rs:407:25: 407:30 error: no method named `unary` found for type `&mut parser::Parser<'c>` in the current scope
src/parser.rs:407           let mut right = self.unary();
                                                 ^~~~~
src/parser.rs:409:15: 409:25 error: no method named `is_prec_op` found for type `&mut parser::Parser<'c>` in the current scope
src/parser.rs:409           while self.is_prec_op() {
                                       ^~~~~~~~~~
src/parser.rs:411:21: 411:29 error: attempted access of field `src` on type `&mut parser::Parser<'c>`, but no field with that name was found
src/parser.rs:411               let next_prec = self.src.ctx.op_map.get(next_op);
                                                ^~~~~~~~
src/parser.rs:41:15: 41:25 error: attempted access of field `span` on type `ast::N<ast::Expr>`, but no field with that name was found
src/parser.rs:41        let start = $node.span;
                                    ^~~~~~~~~~
src/parser.rs:420:11: 420:60 note: in this expansion of extend! (defined in src/parser.rs)
src/parser.rs:41:21: 41:25 help: did you mean `val`?
src/parser.rs:41        let start = $node.span;
                                          ^~~~
src/parser.rs:420:11: 420:60 note: in this expansion of extend! (defined in src/parser.rs)
src/parser.rs:420:31: 420:42 error: no associated item named `BinOp` found for type `ast::Expr` in the current scope
src/parser.rs:420           left = extend!(self, left, Expr::BinOp(left, op, right))
                                                       ^~~~~~~~~~~
src/parser.rs:420:11: 420:60 note: in this expansion of extend! (defined in src/parser.rs)
error: aborting due to 13 previous errors
Could not compile `parser`.

To learn more, run the command again with --verbose.
