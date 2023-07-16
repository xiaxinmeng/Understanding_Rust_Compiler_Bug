
pub fn link_name<'a, F>(check_name: F, attrs: &'a [ast::Attribute]) -> Option<Symbol>
where
    F: Fn(&'a ast::Attribute, Symbol) -> bool
