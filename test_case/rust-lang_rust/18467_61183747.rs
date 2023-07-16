 rust
impl <'a> ::std::cmp::PartialEq for Test<'a> {
    #[inline]
    fn eq(&self, __arg_0: &Test<'a>) -> ::bool {
        match (&*self, &*__arg_0) {
            (&Slice(ref __self_0), &Slice(ref __arg_1_0)) =>
            true && (*__self_0) == (*__arg_1_0),
        }
    }
    // ne omitted
}
