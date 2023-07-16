 rust
fn project_vec<O>(v: Vec<O>) -> <Vec_ as TypeToType<O>>::Output {
    v
}

impl<T> Mappable for Vec<T> {
    type E = T;
    type HKT = Vec_;

    fn map<F, O>(self, mut f: F) -> <Self::HKT as TypeToType<O>>::Output
        where F: FnMut(Self::E) -> O,
              Self::HKT: TypeToType<O>
    {
        let r: Vec<O> = self.into_iter().map(&mut f).collect();
        project_vec::<O>(r)
    }

}
