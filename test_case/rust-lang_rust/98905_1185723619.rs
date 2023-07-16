rust
// As lifts over &
/* … */
impl<T: ?Sized, U: ?Sized> const AsRef<U> for &T
where
    T: ~const AsRef<U>,
{
    /* … */
}

// As lifts over &mut
/* … */
impl<T: ?Sized, U: ?Sized> const AsRef<U> for &mut T
where
    T: ~const AsRef<U>,
{
    /* … */
}

// FIXME (#45742): replace the above impls for &/&mut with the following more general one:
// // As lifts over Deref
// impl<D: ?Sized + Deref<Target: AsRef<U>>, U: ?Sized> AsRef<U> for D {
//     fn as_ref(&self) -> &U {
//         self.deref().as_ref()
//     }
// }
