Rust
#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
#[macro_use]
extern crate rustc;
extern crate rustc_plugin;

use rustc::lint::{LateContext, LintPass, LateLintPass, LintArray, LintContext};
use rustc::hir;
use rustc::hir::intravisit::FnKind;
use rustc::middle::region::CodeExtent;
use rustc::util::nodemap::FnvHashMap;

use syntax::ast::{self, NodeId};
use syntax::codemap::Span;

declare_lint!(REGION_HIERARCHY, Warn, "warn about bogus region hierarchy");

struct Pass {
    map: FnvHashMap<CodeExtent, NodeId>
}

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray { lint_array!(REGION_HIERARCHY) }
}

impl LateLintPass for Pass {
    fn check_fn(&mut self, cx: &LateContext,
                fk: FnKind, _: &hir::FnDecl, expr: &hir::Block,
                span: Span, node: ast::NodeId)
    {
        if let FnKind::Closure(..) = fk { return }

        let mut extent = cx.tcx.region_maps.node_extent(expr.id);
        while let Some(parent) = cx.tcx.region_maps.opt_encl_scope(extent) {
            extent = parent;
        }
        if let Some(other) = self.map.insert(extent, node) {
            cx.span_lint(REGION_HIERARCHY, span, &format!(
                "different fns {:?}, {:?} with the same root extent {:?}",
                cx.tcx.map.local_def_id(other),
                cx.tcx.map.local_def_id(node),
                extent));
        }
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut ::rustc_plugin::Registry) {
    reg.register_late_lint_pass(Box::new(
        Pass { map: FnvHashMap() }
    ));
}
