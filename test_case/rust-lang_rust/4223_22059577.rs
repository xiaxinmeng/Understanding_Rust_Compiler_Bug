
    impl<'self,T:Eq> Eq for &'self [T] {
        fn eq(&self, other: & &'self [T]) -> bool {
            self.len() == other.len() &&
                self.iter().zip(other.iter()).all(|(s,o)| *s == *o)
        }
        #[inline]
        fn ne(&self, other: & &'self [T]) -> bool { !self.eq(other) }
    }

    impl<T:Eq> Eq for ~[T] {
        #[inline]
        fn eq(&self, other: &~[T]) -> bool { self.as_slice() == *other }
        #[inline]
        fn ne(&self, other: &~[T]) -> bool { !self.eq(other) }
    }

    impl<T:Eq> Eq for @[T] {
        #[inline]
        fn eq(&self, other: &@[T]) -> bool { self.as_slice() == *other }
        #[inline]
        fn ne(&self, other: &@[T]) -> bool { !self.eq(other) }
    }
