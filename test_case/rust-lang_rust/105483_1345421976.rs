diff
impl<'a, T: ?Sized> Decodable for Cow<'a, T>
    where T: ToOwned, T::Owned: Decodable
{
    #[inline]
-    fn decode<D: Decoder>(d: &mut D) -> Result<Cow<'static, T>, D::Error> {
+    fn decode<D: Decoder>(d: &mut D) -> Result<Cow<'a, T>, D::Error> {
        Ok(Cow::Owned(try!(Decodable::decode(d))))
    }
}
