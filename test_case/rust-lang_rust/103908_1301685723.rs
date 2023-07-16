plain
    Checking strsim v0.10.0
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/tools/clippy/clippy_utils/src/ast_utils.rs:163:15
     |
163  |         (Loop(lt, ll), Loop(rt, rl)) => eq_label(ll, rl) && eq_block(lt, rt),
     |               ^^  ^^ expected 3 fields, found 2
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1364:10
     |
     |
1364 |     Loop(P<Block>, Option<Label>, Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
163  |         (Loop(lt, ll, _), Loop(rt, rl)) => eq_label(ll, rl) && eq_block(lt, rt),

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/tools/clippy/clippy_utils/src/ast_utils.rs:163:29
     |
     |
163  |         (Loop(lt, ll), Loop(rt, rl)) => eq_label(ll, rl) && eq_block(lt, rt),
     |                             ^^  ^^ expected 3 fields, found 2
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1364:10
     |
     |
1364 |     Loop(P<Block>, Option<Label>, Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
163  |         (Loop(lt, ll), Loop(rt, rl, _)) => eq_label(ll, rl) && eq_block(lt, rt),

    Checking tester v0.9.0
    Checking filetime v0.2.14
    Checking same-file v1.0.6
