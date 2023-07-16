rust
impl<'a, I> TryFrom<I> for CStringVec
   where I: Iterator<Item = &'a str>
{
   type Error = Error;

   /// Creates a new `CStringVec` from an iterator.
   #[inline]
   fn try_from(it: I) -> Result<Self, Self::Error>
   {
      // ...
   }
}
