
   Compiling rls-span v0.4.0
   Compiling languageserver-types v0.12.0
error[E0432]: unresolved import `syntax::ast::BindingMode`
  --> C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\rustfmt-nightly-0.2.1\src\patterns.rs:11:25
   |
11 | use syntax::ast::{self, BindingMode, FieldPat, Pat, PatKind, RangeEnd};
   |                         ^^^^^^^^^^^ no `BindingMode` in `ast`
error[E0433]: failed to resolve. Could not find `BindingMode` in `ast`
    --> C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\rustfmt-nightly-0.2.1\src\items.rs:1725:37
     |
1725 |     if let ast::PatKind::Ident(ast::BindingMode::ByValue(mutability), _, _) = arg.pat.node {
     |                                     ^^^^^^^^^^^ Could not find `BindingMode` in `ast`
   Compiling syntex_errors v0.52.0
error: aborting due to 2 previous errors
error: Could not compile `rustfmt-nightly`.
warning: build failed, waiting for other jobs to finish...
error: build failed

