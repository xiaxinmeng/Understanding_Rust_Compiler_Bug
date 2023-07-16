plain
    Checking semver v0.11.0
    Checking toml v0.5.7
    Checking clippy_utils v0.1.53 (/checkout/src/tools/clippy/clippy_utils)
    Checking url v2.1.1
error[E0532]: expected tuple struct or tuple variant, found struct variant `ExpnKind::Macro`
    |
    |
850 |             if let ExpnKind::Macro(MacroKind::Bang, mac_name) = data.kind {
    | 
   ::: /checkout/compiler/rustc_span/src/hygiene.rs:845:5
    |
845 |     Macro {
845 |     Macro {
    |     ----- `ExpnKind::Macro` defined here
help: use struct pattern syntax instead
    |
    |
850 |             if let ExpnKind::Macro { /* fields */ } = data.kind {
help: consider importing one of these items instead
    |
50  | use crate::DefKind::Macro;
    |
    |
50  | use rustc_hir::def::DefKind::Macro;
    |

error[E0532]: expected tuple struct or tuple variant, found struct variant `ExpnKind::Macro`
    |
    |
878 |         if let ExpnKind::Macro(MacroKind::Bang, mac_name) = data.kind {
    | 
   ::: /checkout/compiler/rustc_span/src/hygiene.rs:845:5
    |
845 |     Macro {
845 |     Macro {
    |     ----- `ExpnKind::Macro` defined here
help: use struct pattern syntax instead
    |
    |
878 |         if let ExpnKind::Macro { /* fields */ } = data.kind {
help: consider importing one of these items instead
    |
50  | use crate::DefKind::Macro;
    |
