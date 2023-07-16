rust
pub struct ArchivedReduceLink<'e>
where
	&'e Expr<'e>: ::rkyv::Archive,
{
	reduced: ::rkyv::Archived<&'e Expr<'e>>,
}
impl<'e> ArchivedReduceLink<'e>
where
	&'e Expr<'e>: ::rkyv::Archive,
{
	unsafe fn check_bytes<'__bytecheck, __C>(value: (), context: ()) -> Result<&'__bytecheck Self, ()> {
		unimplemented!()
	}
}
pub enum Expr<'e> {
	Lambda { bind: &'e () },
}
