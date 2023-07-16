
impl<S, V: Serializable> LinearMap<~str, V>: Serializable<S> {
    fn serialize(&self, s: &S) {
        let mut i = 0;
        do s.emit_rec {
            for self.each |key, value| {
                s.emit_field(key, i, || value.serialize(s));
                i += 1;
            }
        }
    }
}
