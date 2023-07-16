
impl PartialEq<MyType> for Ptr<MyType, ()> {
    fn eq(&self, other: &MyType) -> bool {
        unimplemented!()
    }
}
