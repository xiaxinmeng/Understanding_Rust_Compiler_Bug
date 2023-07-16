plain

error[E0433]: failed to resolve: use of undeclared type `AttrVec`
   --> src/tools/rustfmt/src/types.rs:809:28
    |
809 |                     attrs: AttrVec::new(),
    |
help: consider importing one of these items
    |
1   | use crate::ast::AttrVec;
---

error[E0433]: failed to resolve: use of undeclared type `StructParts`
   --> src/tools/rustfmt/src/types.rs:820:22
    |
820 |                     &StructParts::from_variant(&variant),
    |
help: consider importing this struct
    |
1   | use crate::items::StructParts;
---

error[E0433]: failed to resolve: use of undeclared type `AttrVec`
   --> src/tools/rustfmt/src/types.rs:833:28
    |
833 |                     attrs: AttrVec::new(),
    |
help: consider importing one of these items
    |
1   | use crate::ast::AttrVec;
---

error[E0433]: failed to resolve: use of undeclared type `StructParts`
   --> src/tools/rustfmt/src/types.rs:844:22
    |
844 |                     &StructParts::from_variant(&variant),
    |
help: consider importing this struct
    |
1   | use crate::items::StructParts;
1   | use crate::items::StructParts;
    |

error[E0425]: cannot find value `DUMMY_SP` in this scope
    |
    |
101 |     span: DUMMY_SP,
    |
help: consider importing this constant
    |
27  | use rustc_span::DUMMY_SP;
