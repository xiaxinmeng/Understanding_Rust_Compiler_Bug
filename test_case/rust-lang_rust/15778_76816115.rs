 rust
#![feature(plugin_registrar)]
#![feature(box_syntax)]

extern crate syntax;
#[macro_use] extern crate rustc;

use syntax::{ast, attr};
use rustc::lint::{Context, LintPass, LintPassObject, LintArray};
use rustc::plugin::Registry;

declare_lint!(CRATE_NOT_OKAY, Warn, "crate not marked with #![crate_okay]");

struct Pass;

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(CRATE_NOT_OKAY)
    }

    fn check_crate(&mut self, cx: &Context, krate: &ast::Crate) {
        if !attr::contains_name(&krate.attrs, "crate_okay") {
            cx.span_lint(CRATE_NOT_OKAY, krate.span,
                         "crate is not marked with #![crate_okay]");
        }
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(box Pass as LintPassObject);
}
