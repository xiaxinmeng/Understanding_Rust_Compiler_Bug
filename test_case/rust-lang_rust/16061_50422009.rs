
/// Returns the path as a string, if possible.
/// If the path is not representable in utf-8, this returns None.
#[inline]
fn as_str<'a>(&'a self) -> Option<&'a str> {
