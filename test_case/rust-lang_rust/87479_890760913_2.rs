
trait IsStored<'x>: Sized {
    type Carrier<'a>;
    fn read<'a, S: Store<'a>>(self, storage: &mut S) -> Self::Carrier<'a>
    where 'x: 'a ;
    fn alloc<'a, S: Store<'a>>(self, storage: &mut S) -> Ptr<'a, Self::Carrier<'a>>
    where 'x: 'a;
}
