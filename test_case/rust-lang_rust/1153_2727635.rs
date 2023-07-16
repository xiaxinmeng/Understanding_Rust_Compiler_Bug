
use std;
use rustc;

import rustc::syntax::ast;
import rustc::syntax::codemap;
import rustc::syntax::parse::parser;
import rustc::syntax::visit;

fn doc_item(item: @ast::item) {
    log item.ident;
    for attr: ast::attribute in item.attrs {
        alt attr.node.value.node {
            ast::meta_name_value("doc", {node: ast::lit_str(value), span: _}) {
                log #fmt["  doc %s", value];
            }
        }
    }
}

fn main() {
    let sess = @{cm: codemap::new_codemap(), mutable next_id: 0};
    let p = parser::parse_crate_from_source_file("rustdoc.rs", [], sess);
    let v = visit::mk_simple_visitor(@{
        visit_item: doc_item
        with *visit::default_simple_visitor()});
    visit::visit_crate(*p, (), v);
}
