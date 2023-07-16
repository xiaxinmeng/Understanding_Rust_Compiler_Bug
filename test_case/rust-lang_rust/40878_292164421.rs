
[00:06:09] error[E0026]: struct `hir::Block` does not have a field named `break_to_expr_id`
[00:06:09]    --> /checkout/src/librustc/ich/impls_hir.rs:347:13
[00:06:09]     |
[00:06:09] 347 |             break_to_expr_id,
[00:06:09]     |             ^^^^^^^^^^^^^^^^ struct `hir::Block` does not have field `break_to_expr_id`
[00:06:09] 
[00:06:09] error[E0027]: pattern does not mention field `targeted_by_break`
[00:06:09]    --> /checkout/src/librustc/ich/impls_hir.rs:341:13
[00:06:09]     |
[00:06:09] 341 |           let hir::Block {
[00:06:09]     |  _____________^ starting here...
[00:06:09] 342 | |             ref stmts,
[00:06:09] 343 | |             ref expr,
[00:06:09] 344 | |             id,
[00:06:09] 345 | |             rules,
[00:06:09] 346 | |             span,
[00:06:09] 347 | |             break_to_expr_id,
[00:06:09] 348 | |         } = *self;
[00:06:09]     | |_________^ ...ending here: missing field `targeted_by_break`
[00:06:09] 
[00:06:15] error: aborting due to 2 previous errors
