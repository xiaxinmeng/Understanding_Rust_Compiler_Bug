 rust
pub fn expand_type(t: P<ast::Ty>, fld: &mut MacroExpander) -> P<ast::Ty> {
    let t = match t.node.clone() {
        ast::TyKind::Mac(mac) => {
            if fld.cx.ecfg.features.unwrap().type_macros {
                expand_mac_invoc(mac, None, Vec::new(), t.span, fld)
            } else {
                feature_gate::emit_feature_err(
                    &fld.cx.parse_sess.span_diagnostic,
                    "type_macros",
                    t.span,
                    feature_gate::GateIssue::Language,
                    "type macros are experimental");

                DummyResult::raw_ty(t.span)
            }
        }
        _ => t
    };

    fold::noop_fold_ty(t, fld)
}
