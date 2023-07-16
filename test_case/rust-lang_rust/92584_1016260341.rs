plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0308]: mismatched types
   --> compiler/rustc_passes/src/check_attr.rs:119:77
    |
119 |                 sym::rustc_pass_by_value => self.check_pass_by_value(&attr, span, target),
    |                                                                             |
    |                                                                             expected `&rustc_span::Span`, found struct `rustc_span::Span`
    |                                                                             help: consider borrowing here: `&span`


error[E0308]: mismatched types
   --> compiler/rustc_passes/src/check_attr.rs:132:60
    |
132 |                 sym::link => self.check_link(hir_id, attr, span, target),
    |                                                            |
    |                                                            expected `&rustc_span::Span`, found struct `rustc_span::Span`
    |                                                            help: consider borrowing here: `&span`

