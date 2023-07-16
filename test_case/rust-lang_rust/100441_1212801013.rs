plain
    Checking thiserror v1.0.30
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    --> src/tools/rustfmt/src/skip.rs:61:38
     |
61   |         if let ast::AttrKind::Normal(attr_item, _) = &attr.kind {
     |                                      ^^^^^^^^^  ^ expected 1 field, found 2
    ::: /checkout/compiler/rustc_ast/src/ast.rs:2560:12
     |
     |
2560 |     Normal(P<Normal>),

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    --> src/tools/rustfmt/src/visitor.rs:814:43
     |
     |
814  |                     ast::AttrKind::Normal(ref attribute_item, _)
     |                                           ^^^^^^^^^^^^^^^^^^  ^ expected 1 field, found 2
    ::: /checkout/compiler/rustc_ast/src/ast.rs:2560:12
     |
     |
2560 |     Normal(P<Normal>),

For more information about this error, try `rustc --explain E0023`.
error: could not compile `rustfmt-nightly` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
