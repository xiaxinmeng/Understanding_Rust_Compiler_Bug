rust
impl MacroExpansionContext {
    fn expand(binder: Ident) -> TokenStream {
        if (binder == kw::crate) {
            #[cfg(current-behavior)] {
                Ident::new(kw::crate, Span::def_site_at(binder))
            }
            #[cfg(expander-model)] {
                Ident::new(kw::crate, Span::def_site_at(self))
            }
        } else {
            self.resolve_binding(binder)
        }
    }
}
