
 #[derive(PartialEq)]
enum E<T> {
    V1(T, T),
    V2(T)
}

struct OwnedE<T>(Box<E<T>>);

impl<T> PartialEq for OwnedE<T> {
    fn eq(&self, other: &Self) -> bool {
        *self.0 == *other.0
    }
}
