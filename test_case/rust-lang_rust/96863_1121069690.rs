plain
   Compiling libz-sys v1.1.3
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    --> src/tools/clippy/clippy_utils/src/hir_utils.rs:304:27
     |
304  |             (Guard::IfLet(lp, le), Guard::IfLet(rp, re)) => self.eq_pat(lp, rp) && self.eq_expr(le, re),
     |                           ^^  ^^ expected 1 field, found 2
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1323:11
     |
     |
1323 |     IfLet(&'hir Let<'hir>),

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    --> src/tools/clippy/clippy_utils/src/hir_utils.rs:304:49
     |
     |
304  |             (Guard::IfLet(lp, le), Guard::IfLet(rp, re)) => self.eq_pat(lp, rp) && self.eq_expr(le, re),
     |                                                 ^^  ^^ expected 1 field, found 2
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1323:11
     |
     |
1323 |     IfLet(&'hir Let<'hir>),

    Checking pulldown-cmark v0.9.1
    Checking aho-corasick v0.7.18
    Checking bstr v0.2.13
    Checking bstr v0.2.13
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    --> src/tools/clippy/clippy_utils/src/hir_utils.rs:897:44
     |
897  |             Guard::If(expr) | Guard::IfLet(_, expr) => {
     |                                            ^  ^^^^ expected 1 field, found 2
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1323:11
     |
     |
1323 |     IfLet(&'hir Let<'hir>),

    Checking idna v0.2.0
    Checking getrandom v0.2.0
    Checking dirs-sys-next v0.1.2
