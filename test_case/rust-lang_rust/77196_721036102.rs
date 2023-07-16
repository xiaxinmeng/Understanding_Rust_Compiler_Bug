rust
trait Trait<T> {
    type Assoc;
}
impl<T> Trait<T> for fn(&()) {
    type Assoc = T;
}
impl<'a, T> Trait<T> for fn(&'a ()) {
    type Assoc = ();
}
#[repr(C)]
struct Union<T, U, F: Trait<U>> {
    to: F::Assoc,
    from: T,
}
pub fn transmute<T, U>(x: T) -> U {
    Union::<Option<T>, Option<U>, fn(&())> { to: None, from: Some(x) }.to.unwrap()
}
