 rust
// span-lint.rs
#![feature(phase, plugin_registrar)]
#![crate_type = "dylib"]

extern crate syntax;
#[phase(plugin, link)] extern crate rustc;

use syntax::ast;
use rustc::plugin::Registry;
use rustc::lint::{Context, LintArray, LintPass};
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(box SpanLint);
}

declare_lint! { SPAN_LINT, Warn, "..." }

struct SpanLint;

impl LintPass for SpanLint {
    fn get_lints(&self) -> LintArray {
        lint_array!(SPAN_LINT)
    }

    fn check_expr(&mut self, cx: &Context, e: &ast::Expr) {
        let mut sp = e.span;
        let mut stop = false;
        cx.tcx.sess.span_warn(sp, "starts here");
        while !stop {
            cx.tcx.sess.codemap().with_expn_info(sp.expn_id, |expn_info| {
                match expn_info {
                    None => stop = true,
                    Some(ref info) => {
                        sp = info.call_site;
                        cx.tcx.sess.span_note(sp, "continues");
                    }
                }
            })
        }
    }
}
