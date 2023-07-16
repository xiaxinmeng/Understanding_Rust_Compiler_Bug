
impl ::std::hash::Hash for E {
    fn hash<__H: ::std::hash::Hasher>(&self, __arg_0: &mut __H) -> () {
        match (&*self,) {
            (&E::A,) => { ::std::hash::Hash::hash(&1, __arg_0); }
            (&E::B,) => { ::std::hash::Hash::hash(&1usize, __arg_0); }
        }
    }
}
