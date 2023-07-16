 rust
impl<Data, P> Placer<Data> for P where P: InPlace<Data> {
    type Place = P;
    fn make_place(self) -> P { self }
}
