rust
error[E0308]: mismatched types
   --> src/librustc_borrowck/borrowck/check_loans.rs:853:72
    |
853 |                                        self.bccx.tcx.hir.span_if_local(local_id),
    |                                                                        ^^^^^^^^ expected struct `rustc::hir::def_id::DefId`, found struct `syntax::ast::NodeId`
    |
    = note: expected type `rustc::hir::def_id::DefId`
               found type `syntax::ast::NodeId`

error[E0277]: the trait bound `syntax_pos::MultiSpan: std::convert::From<std::option::Option<syntax_pos::Span>>` is not satisfied
   --> src/librustc_borrowck/borrowck/check_loans.rs:851:35
    |
851 |                     self.bccx.tcx.lint_node(UNUSED_MUT,
    |                                   ^^^^^^^^^ the trait `std::convert::From<std::option::Option<syntax_pos::Span>>` is not implemented for `syntax_pos::MultiSpan`
    |
    = help: the following implementations were found:
              <syntax_pos::MultiSpan as std::convert::From<syntax_pos::Span>>
    = note: required because of the requirements on the impl of `std::convert::Into<syntax_pos::MultiSpan>` for `std::option::Option<syntax_pos::Span>`
