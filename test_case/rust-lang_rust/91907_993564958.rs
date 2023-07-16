plain
   Compiling clippy v0.1.59 (/checkout/src/tools/clippy)
    Checking bstr v0.2.13
    Checking quote v1.0.7
   Compiling libz-sys v1.1.3
error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:253:60
    |
253 |                 self.eq_expr(le, re) && self.eq_body(ll_id.body, rl_id.body)
    |
help: use parentheses to call the method
    |
    |
253 |                 self.eq_expr(le, re) && self.eq_body(ll_id.body(), rl_id.body)


error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:253:72
    |
253 |                 self.eq_expr(le, re) && self.eq_body(ll_id.body, rl_id.body)
    |
help: use parentheses to call the method
    |
    |
253 |                 self.eq_expr(le, re) && self.eq_body(ll_id.body, rl_id.body())


error[E0615]: attempted to take value of method `body` on type `rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:289:82
    |
289 |             (GenericArg::Const(l), GenericArg::Const(r)) => self.eq_body(l.value.body, r.value.body),
    |
help: use parentheses to call the method
    |
    |
289 |             (GenericArg::Const(l), GenericArg::Const(r)) => self.eq_body(l.value.body(), r.value.body),


error[E0615]: attempted to take value of method `body` on type `rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:289:96
    |
289 |             (GenericArg::Const(l), GenericArg::Const(r)) => self.eq_body(l.value.body, r.value.body),
    |
help: use parentheses to call the method
    |
    |
289 |             (GenericArg::Const(l), GenericArg::Const(r)) => self.eq_body(l.value.body, r.value.body()),


error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:391:58
    |
391 |                 self.eq_ty(lt, rt) && self.eq_body(ll_id.body, rl_id.body)
    |
help: use parentheses to call the method
    |
    |
391 |                 self.eq_ty(lt, rt) && self.eq_body(ll_id.body(), rl_id.body)


error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:391:70
    |
391 |                 self.eq_ty(lt, rt) && self.eq_body(ll_id.body, rl_id.body)
    |
help: use parentheses to call the method
    |
    |
391 |                 self.eq_ty(lt, rt) && self.eq_body(ll_id.body, rl_id.body())


error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:664:93
    |
664 |                         InlineAsmOperand::Const { anon_const } => self.hash_body(anon_const.body),
    |
help: use parentheses to call the method
    |
    |
664 |                         InlineAsmOperand::Const { anon_const } => self.hash_body(anon_const.body()),


error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:708:37
    |
708 |                 self.hash_body(l_id.body);
    |
help: use parentheses to call the method
    |
    |
708 |                 self.hash_body(l_id.body());


error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:712:37
    |
712 |                 self.hash_body(l_id.body);
    |
help: use parentheses to call the method
    |
    |
712 |                 self.hash_body(l_id.body());


error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:904:43
    |
904 |                 self.hash_body(anon_const.body);
    |
help: use parentheses to call the method
    |
    |
904 |                 self.hash_body(anon_const.body());


error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:943:43
    |
943 |                 self.hash_body(anon_const.body);
    |
help: use parentheses to call the method
    |
    |
943 |                 self.hash_body(anon_const.body());


error[E0615]: attempted to take value of method `body` on type `rustc_hir::AnonConst`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:961:70
    |
961 |                 GenericArg::Const(ref ca) => self.hash_body(ca.value.body),
    |
help: use parentheses to call the method
    |
    |
961 |                 GenericArg::Const(ref ca) => self.hash_body(ca.value.body()),

    Checking idna v0.2.0
    Checking getrandom v0.2.0
    Checking dirs-sys-next v0.1.2
    Checking dirs-sys-next v0.1.2
    Checking num_cpus v1.13.0
    Checking filetime v0.2.14
    Checking parking_lot_core v0.8.5
error[E0615]: attempted to take value of method `body` on type `&rustc_hir::AnonConst`
    |
    |
680 |             if let ExprKind::Lit(ref const_lit) = cx.tcx.hir().body(y.body).value.kind;
    |
help: use parentheses to call the method
    |
    |
680 |             if let ExprKind::Lit(ref const_lit) = cx.tcx.hir().body(y.body()).value.kind;

    Checking dirs-next v2.0.0
For more information about this error, try `rustc --explain E0615`.
error: could not compile `clippy_utils` due to 13 previous errors
