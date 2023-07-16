
pub fn link_name<F>(check_name: F, attrs: &[ast::Attribute]) -> Option<Symbol>
where
    F: Fn(&ast::Attribute, Symbol) -> bool
