
impl<S, K: Serializable, V: Serializable> LinearMap<K, V>: Serializable<S> {
    fn serialize(&self, s: &S) {
        let mut i = 0;
        do s.emit_owned_vec(self.len()) {
            for self.each |key, value| {
                do s.emit_vec_elt(i) {
                    do s.emit_tup(2) {
                        s.emit_tup_elt(0, || key.serialize(s));
                        s.emit_tup_elt(1, || value.serialize(s));
                    }
                }
                i += 1;
            }
        }
    }
}
