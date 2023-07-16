rust
pub enum Slot<'a, 'b: 'a, T: 'a, Tr: Transaction<'b> + 'b> {
    Nothing,
    Ref(&'a SlotFn<'a, 'b, T, Tr>),
    SRef(Rc<SlotFn<'a, 'b, T, Tr>>),
    WRef(Weak<SlotFn<'a, 'b, T, Tr>>),
    Mut(&'a mut SlotFnMut<'a, 'b, T, Tr>),
    Boxed(Box<SlotFnMut<'a, 'b, T, Tr>>),

    #[cfg(feature="nightly")]
    Once(Box<SlotFnBox<'a, 'b, T, Tr>>),
}

impl<'a, T, Tr: Transaction<'a> + 'a> Stream<'a, Tr> for Variable<'a, T, Tr>
    where T: Clone
{
    type Output = T;
    type Listen = <Event<'a, T, Tr> as Stream<'a, Tr>>::Listen;

    fn listen<F>(&self, f: F) -> Self::Listen
        where F: Into<Slot<'a, 'a, T, Tr>>
    {
        self.updates.listen(f)
    }
}
