
[00:14:00] error[E0308]: mismatched types
[00:14:00]    --> /checkout/src/librustc_mir/borrow_check.rs:594:75
[00:14:00]     |
[00:14:00] 594 |                 self.report_illegal_reassignment(context, (lvalue, span), assignment_span);
[00:14:00]     |                                                                           ^^^^^^^^^^^^^^^ expected struct `syntax_pos::Span`, found reference
[00:14:00]     |
[00:14:00]     = note: expected type `syntax_pos::Span`
[00:14:00]                found type `&rustc::mir::Statement<'_>`
