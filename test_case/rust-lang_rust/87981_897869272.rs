plain
    Checking toml v0.5.7
    Checking cargo_metadata v0.8.2
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
 Documenting rustfmt-nightly v1.4.37 (/checkout/src/tools/rustfmt)
error[E0532]: expected unit struct, unit variant or constant, found struct variant `ast::StmtKind::Empty`
    |
    |
46  |         ast::StmtKind::Empty => stmt.span,
    |
help: use struct pattern syntax instead
    |
    |
46  |         ast::StmtKind::Empty { /* fields */ } => stmt.span,
help: consider importing one of these items instead
    |
    |
3   | use annotate_snippets::display_list::DisplaySourceLine::Empty;
3   | use core::num::IntErrorKind::Empty;
    |
    |
3   | use crate::ast::MacArgs::Empty;
    |
3   | use rustc_ast::MacArgs::Empty;
      and 5 other candidates


error[E0532]: expected unit struct, unit variant or constant, found struct variant `ast::StmtKind::Empty`
     |
     |
1141 |         .any(|stmt| !matches!(stmt.kind, ast::StmtKind::Empty))
     |
help: use struct pattern syntax instead
     |
     |
1141 |         .any(|stmt| !matches!(stmt.kind, ast::StmtKind::Empty { /* fields */ }))
help: consider importing one of these items instead
     |
     |
1    | use annotate_snippets::display_list::DisplaySourceLine::Empty;
1    | use core::num::IntErrorKind::Empty;
     |
     |
1    | use crate::ast::MacArgs::Empty;
     |
1    | use rustc_ast::MacArgs::Empty;
       and 5 other candidates


error[E0532]: expected unit struct, unit variant or constant, found struct variant `ast::StmtKind::Empty`
   --> src/tools/rustfmt/src/spanned.rs:76:13
    |
76  |             ast::StmtKind::Empty => self.span,
    |
help: use struct pattern syntax instead
    |
    |
76  |             ast::StmtKind::Empty { /* fields */ } => self.span,
help: consider importing one of these items instead
    |
    |
1   | use annotate_snippets::display_list::DisplaySourceLine::Empty;
1   | use core::num::IntErrorKind::Empty;
    |
    |
1   | use crate::ast::MacArgs::Empty;
    |
1   | use rustc_ast::MacArgs::Empty;
      and 5 other candidates


error[E0532]: expected unit struct, unit variant or constant, found struct variant `ast::StmtKind::Empty`
    |
    |
56  |         matches!(self.inner.kind, ast::StmtKind::Empty)
    |
help: use struct pattern syntax instead
    |
    |
56  |         matches!(self.inner.kind, ast::StmtKind::Empty { /* fields */ })
help: consider importing one of these items instead
    |
    |
1   | use annotate_snippets::display_list::DisplaySourceLine::Empty;
1   | use core::num::IntErrorKind::Empty;
    |
    |
1   | use crate::ast::MacArgs::Empty;
    |
1   | use rustc_ast::MacArgs::Empty;
      and 5 other candidates


error[E0532]: expected unit struct, unit variant or constant, found struct variant `ast::StmtKind::Empty`
    |
    |
113 |         ast::StmtKind::MacCall(..) | ast::StmtKind::Item(..) | ast::StmtKind::Empty => None,
    |
help: use struct pattern syntax instead
    |
    |
113 |         ast::StmtKind::MacCall(..) | ast::StmtKind::Item(..) | ast::StmtKind::Empty { /* fields */ } => None,
help: consider importing one of these items instead
    |
    |
1   | use annotate_snippets::display_list::DisplaySourceLine::Empty;
1   | use core::num::IntErrorKind::Empty;
    |
    |
1   | use crate::ast::MacArgs::Empty;
    |
1   | use rustc_ast::MacArgs::Empty;
      and 5 other candidates


error[E0532]: expected unit struct, unit variant or constant, found struct variant `ast::StmtKind::Empty`
   --> src/tools/rustfmt/src/visitor.rs:183:13
    |
183 |             ast::StmtKind::Empty => (),
    |
help: use struct pattern syntax instead
    |
    |
183 |             ast::StmtKind::Empty { /* fields */ } => (),
help: consider importing one of these items instead
    |
    |
1   | use annotate_snippets::display_list::DisplaySourceLine::Empty;
1   | use core::num::IntErrorKind::Empty;
    |
    |
1   | use crate::ast::MacArgs::Empty;
    |
1   | use rustc_ast::MacArgs::Empty;
      and 5 other candidates


error[E0532]: expected unit struct, unit variant or constant, found struct variant `ast::StmtKind::Empty`
   --> src/tools/rustfmt/src/visitor.rs:958:46
    |
958 |                     (ast::StmtKind::Item(_), ast::StmtKind::Empty)
    |
help: use struct pattern syntax instead
    |
    |
958 |                     (ast::StmtKind::Item(_), ast::StmtKind::Empty { /* fields */ })
help: consider importing one of these items instead
    |
    |
1   | use annotate_snippets::display_list::DisplaySourceLine::Empty;
1   | use core::num::IntErrorKind::Empty;
    |
    |
1   | use crate::ast::MacArgs::Empty;
    |
1   | use rustc_ast::MacArgs::Empty;
      and 5 other candidates


error[E0560]: struct `rustc_ast::Stmt` has no field named `id`
    |
    |
151 |             id: ast::NodeId::root(),
    |             ^^ `rustc_ast::Stmt` does not have this field
    = note: available fields are: `kind`, `span`

Some errors have detailed explanations: E0532, E0560.
For more information about an error, try `rustc --explain E0532`.
For more information about an error, try `rustc --explain E0532`.
error: could not compile `rustfmt-nightly` due to 8 previous errors


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustfmt/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "-Zskip-rustdoc-fingerprint" "--no-deps" "-p" "rustfmt-nightly" "-p" "rustfmt-config_proc_macro"


Build completed unsuccessfully in 0:08:15
