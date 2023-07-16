
fn check_attribute(&mut self, cx: &LateContext, attr: &ast::Attribute) {
    if attr.check_name("allow_internal_unsafe") {
        self.report_unsafe(cx, attr.span, "description");
    }
}
