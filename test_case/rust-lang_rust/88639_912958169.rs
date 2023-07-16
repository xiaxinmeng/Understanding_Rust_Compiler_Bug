rust
pub enum FooEnum {
    HiddenTupleItem(#[doc(hidden)] ()),
    MultipleHidden(#[doc(hidden)] (), #[doc(hidden)] ()),
    MixedHiddenTupleItemLast((), #[doc(hidden)] ()),
    MixedHiddenTupleItemFirst(#[doc(hidden)] (), ()),
    NotHiddenTupleItem1(()),
    NotHiddenTupleItem2((), ()),
}
