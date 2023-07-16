
#[stable(feature = "name", since = "version")]
struct S;
#[automatically_derived]
#[allow(unused_qualifications)]
#[stable(feature = "name", since = "version")]
impl ::core::clone::Clone for S {
    #[inline]
    fn clone(&self) -> S { match *self { S => S, } }
}
