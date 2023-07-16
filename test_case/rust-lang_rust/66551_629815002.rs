rust
pub trait Captures<U: ?Sized> {}
impl<U: ?Sized, T: ?Sized> Captures<U> for T {}

impl<'t> S<'t> {
    pub fn foo(&mut self) -> impl FnMut() + Captures<&'t str> + '_ {
        move || self.0 += 1
    }
}
