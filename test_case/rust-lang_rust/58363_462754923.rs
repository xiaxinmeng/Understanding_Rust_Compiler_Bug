rust
pub struct Frozen<T>;

impl<T> Frozen<T> {
    pub fn new(t: T) -> Frozen<T>;
    pub fn into_inner(this: Frozen<T>) -> T;
}

impl<T: ?Sized> Frozen<T> {
    pub fn from_mut(t: &mut T) -> &mut Frozen<T>;
}

impl<T: ?Sized> Deref for Frozen<T> {
    type Target = T;
}
impl<T: ?Sized> DerefMut for Frozen<T> {}
