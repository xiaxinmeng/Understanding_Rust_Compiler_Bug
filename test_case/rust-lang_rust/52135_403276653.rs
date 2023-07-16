rust
impl ::failure::Fail for ErrorKind {
   #[allow(unreachable_code)]
   fn cause(&self) -> ::std::option::Option<&::failure::Fail> {
       match *self {
           ErrorKind::NotAFile => { return None }
           ErrorKind::UnicodeError => { return None }
       }
       None
   }
   ...
}

impl Fail for MyError {
   fn cause(&self) -> Option<&dyn Fail> { self.inner.cause() }
   fn backtrace(&self) -> Option<&Backtrace> {
       self.inner.backtrace()
   }
}
