rust
impl<T: ?Sized> TypeName for T {
    #[cfg(rtti)]
    #[lang_item = "type_name_blanket_impl"]
    fn name(&self) -> &'static str;
    #[cfg(not(rtti))]
    fn name(&self) -> &'static str { "rtti-disabled" }
}
