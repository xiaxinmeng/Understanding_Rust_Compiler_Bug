
pub trait Error: Drop + Debug + Display {
    fn description(&self) -> &str { "description() is deprecated; use Display" }
    fn cause(&self) -> Option<&dyn Error> { self.source() }
    fn source(&self) -> Option<&(dyn Error + 'static)> { None }
    fn type_id(&self, _: private::Internal) -> TypeId where Self: 'static { TypeId::of::<Self>() }
    #[cfg(feature = "alloc")]
    fn backtrace(&self) -> Option<&Backtrace> { None }
}
