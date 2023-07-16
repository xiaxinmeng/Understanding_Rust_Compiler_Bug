rust
extern {
    #[unsafe_impl_all_auto_traits_by_default]
    type Foo;
}
impl !Send for Foo {}
