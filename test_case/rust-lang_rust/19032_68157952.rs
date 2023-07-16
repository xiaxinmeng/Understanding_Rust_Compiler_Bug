
impl<'a> Fn for SomeType {
    type Arg = (&'a int,);
    type Ret = int;
    ...
}
