
[...]

impl<T> Container<T> where T: 'static, for<'a> T: TraitA<'a>, for<'a> <<T as TraitA<'a>>::ReturnA as TraitB>::ReturnB: 'static {
    async fn do_something() -> impl TraitC {
        ()
    }
}
