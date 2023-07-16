 rust
impl LintPass for ShadowingMacrosPass {
    fn get_lints(&self) -> LintArray {
        lint_array!(SHADOWING_MACROS)
    }

    #[allow(unused_must_use)]
    fn check_mac(&mut self, cx: &Context, mac: &ast::Mac) {
        // Pull out the macro identifier out of the definition
        let ast::MacInvocTT(ref path, _, _) = mac.node;
        let id: ast::Ident = path.segments.last().unwrap().identifier;

        let id_str = id.name.as_str();
        io::stderr().write(format!("ShadowingMacrosPass#check_mac: {}\n", id_str).as_bytes());
    }
}
