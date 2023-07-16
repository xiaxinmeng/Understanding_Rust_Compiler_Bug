rust
macro foo($type_name: ident, $docs: expr) {
    #[allow(non_camel_case_types)]
    #[doc=$docs]
    #[derive(Debug, Clone, Copy)]
    pub struct $type_name;
}
