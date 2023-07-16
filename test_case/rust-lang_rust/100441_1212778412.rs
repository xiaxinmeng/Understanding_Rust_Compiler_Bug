plain
    Checking walkdir v2.3.2
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    --> src/tools/clippy/clippy_utils/src/attrs.rs:62:49
     |
62   |         let attr = if let ast::AttrKind::Normal(ref attr, _) = attr.kind {
     |                                                 ^^^^^^^^  ^ expected 1 field, found 2
    ::: /checkout/compiler/rustc_ast/src/ast.rs:2560:12
     |
     |
2560 |     Normal(P<Normal>),

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    --> src/tools/clippy/clippy_utils/src/ast_utils.rs:698:21
     |
     |
698  |             (Normal(l, _), Normal(r, _)) => eq_path(&l.path, &r.path) && eq_mac_args(&l.args, &r.args),
     |                     ^  ^ expected 1 field, found 2
    ::: /checkout/compiler/rustc_ast/src/ast.rs:2560:12
     |
     |
2560 |     Normal(P<Normal>),

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    --> src/tools/clippy/clippy_utils/src/ast_utils.rs:698:35
     |
     |
698  |             (Normal(l, _), Normal(r, _)) => eq_path(&l.path, &r.path) && eq_mac_args(&l.args, &r.args),
     |                                   ^  ^ expected 1 field, found 2
    ::: /checkout/compiler/rustc_ast/src/ast.rs:2560:12
     |
     |
2560 |     Normal(P<Normal>),

   Compiling libz-sys v1.1.3
    Checking pulldown-cmark v0.9.2
    Checking aho-corasick v0.7.18
---
    Checking filetime v0.2.14
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    --> src/tools/clippy/clippy_utils/src/lib.rs:1897:38
     |
1897 |         if let ast::AttrKind::Normal(ref attr, _) = attr.kind {
     |                                      ^^^^^^^^  ^ expected 1 field, found 2
    ::: /checkout/compiler/rustc_ast/src/ast.rs:2560:12
     |
     |
2560 |     Normal(P<Normal>),

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    --> src/tools/clippy/clippy_utils/src/lib.rs:1907:38
     |
     |
1907 |         if let ast::AttrKind::Normal(ref attr, _) = attr.kind {
     |                                      ^^^^^^^^  ^ expected 1 field, found 2
    ::: /checkout/compiler/rustc_ast/src/ast.rs:2560:12
     |
     |
2560 |     Normal(P<Normal>),

    Checking dirs-next v2.0.0
For more information about this error, try `rustc --explain E0023`.
error: could not compile `clippy_utils` due to 5 previous errors
