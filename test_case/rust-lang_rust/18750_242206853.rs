
trait ToTObject<'a> {
    fn to_object(self) -> &'a T;
}

impl<'a> ToObject<'a> for &'a T {
    fn to_object(self) -> &'a T { self }
}

impl<'a,U: T> ToObject<'a> for &'a U {
    fn to_object(self) -> &'a T { self }
}
