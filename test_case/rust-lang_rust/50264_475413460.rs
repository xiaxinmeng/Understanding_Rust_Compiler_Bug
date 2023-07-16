
impl<T> Option<T> {
    as_as_ref<U>(&self) -> Option<&U> where T: AsRef<U>;
}
