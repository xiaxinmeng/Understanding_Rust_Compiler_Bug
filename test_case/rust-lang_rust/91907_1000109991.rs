plain
   Compiling clippy v0.1.59 (/checkout/src/tools/clippy)
    Checking bstr v0.2.13
    Checking quote v1.0.7
   Compiling libz-sys v1.1.3
error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:257:60
    |
257 |                 self.eq_expr(le, re) && self.eq_body(ll_id.body, rl_id.body)
    |
help: use parentheses to call the method
    |
    |
257 |                 self.eq_expr(le, re) && self.eq_body(ll_id.body(), rl_id.body)


error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:257:72
    |
257 |                 self.eq_expr(le, re) && self.eq_body(ll_id.body, rl_id.body)
    |
help: use parentheses to call the method
    |
    |
257 |                 self.eq_expr(le, re) && self.eq_body(ll_id.body, rl_id.body())


error[E0615]: attempted to take value of method `body` on type `rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:293:82
    |
293 |             (GenericArg::Const(l), GenericArg::Const(r)) => self.eq_body(l.value.body, r.value.body),
    |
help: use parentheses to call the method
    |
    |
293 |             (GenericArg::Const(l), GenericArg::Const(r)) => self.eq_body(l.value.body(), r.value.body),


error[E0615]: attempted to take value of method `body` on type `rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:293:96
    |
293 |             (GenericArg::Const(l), GenericArg::Const(r)) => self.eq_body(l.value.body, r.value.body),
    |
help: use parentheses to call the method
    |
    |
293 |             (GenericArg::Const(l), GenericArg::Const(r)) => self.eq_body(l.value.body, r.value.body()),


error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:395:58
    |
395 |                 self.eq_ty(lt, rt) && self.eq_body(ll_id.body, rl_id.body)
    |
help: use parentheses to call the method
    |
    |
395 |                 self.eq_ty(lt, rt) && self.eq_body(ll_id.body(), rl_id.body)


error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:395:70
    |
395 |                 self.eq_ty(lt, rt) && self.eq_body(ll_id.body, rl_id.body)
    |
help: use parentheses to call the method
    |
    |
395 |                 self.eq_ty(lt, rt) && self.eq_body(ll_id.body, rl_id.body())


error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:668:93
    |
668 |                         InlineAsmOperand::Const { anon_const } => self.hash_body(anon_const.body),
    |
help: use parentheses to call the method
    |
    |
668 |                         InlineAsmOperand::Const { anon_const } => self.hash_body(anon_const.body()),


error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:715:37
    |
715 |                 self.hash_body(l_id.body);
    |
help: use parentheses to call the method
    |
    |
715 |                 self.hash_body(l_id.body());


error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:719:37
    |
719 |                 self.hash_body(l_id.body);
    |
help: use parentheses to call the method
    |
    |
719 |                 self.hash_body(l_id.body());


error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:911:43
    |
911 |                 self.hash_body(anon_const.body);
    |
help: use parentheses to call the method
    |
    |
911 |                 self.hash_body(anon_const.body());


error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:950:43
    |
950 |                 self.hash_body(anon_const.body);
    |
help: use parentheses to call the method
    |
    |
950 |                 self.hash_body(anon_const.body());


error[E0615]: attempted to take value of method `body` on type `rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:968:70
    |
968 |                 GenericArg::Const(ref ca) => self.hash_body(ca.value.body),
    |
help: use parentheses to call the method
    |
    |
968 |                 GenericArg::Const(ref ca) => self.hash_body(ca.value.body()),

    Checking idna v0.2.0
    Checking getrandom v0.2.0
    Checking dirs-sys-next v0.1.2
    Checking dirs-sys-next v0.1.2
    Checking num_cpus v1.13.0
    Checking parking_lot_core v0.8.5
    Checking filetime v0.2.14
error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
    |
    |
679 |             if let ExprKind::Lit(ref const_lit) = cx.tcx.hir().body(y.body).value.kind;
    |
help: use parentheses to call the method
    |
    |
679 |             if let ExprKind::Lit(ref const_lit) = cx.tcx.hir().body(y.body()).value.kind;

    Checking dirs-next v2.0.0
For more information about this error, try `rustc --explain E0615`.
error: could not compile `clippy_utils` due to 13 previous errors
