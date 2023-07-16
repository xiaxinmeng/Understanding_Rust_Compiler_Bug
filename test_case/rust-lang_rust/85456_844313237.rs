rust
struct TemporaryStruct<'a>(PhantomData<(&'a mut &'a mut i32)>);

impl<'a> Query<'a> for TemporaryStruct<'a> {
    fn query() {
        let foo: TemporaryStruct<'a> = TemporaryStruct(PhantomData);
    }
}
