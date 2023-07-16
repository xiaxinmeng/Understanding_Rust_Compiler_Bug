rust
struct HasVec<T>(Vec<T>);

impl <T> From<HasVec<T>> for Vec<T> {
    fn from(HasVec(vec): HasVec<T>) -> Self {
        vec
    }
}
