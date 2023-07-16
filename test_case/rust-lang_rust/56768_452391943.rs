
fn check_attribute(&mut self, cx: &LateContext, attr: &ast::Attribute) {
    if attr.check_name("allow_internal_unsafe") {
        if let Some(items) = attr.meta_item_list() {
            for item in items {
                self.report_unsafe(cx, item.span, "description");
            }
        }
    }
}
