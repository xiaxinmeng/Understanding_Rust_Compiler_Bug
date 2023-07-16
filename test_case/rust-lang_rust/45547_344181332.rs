diff
  pub struct IntoInnerError<W>(W, Error);

  impl<W> IntoInnerError<W> {
      pub fn error(&self) -> &Error;
+     pub fn into_error(self) -> Error;
      pub fn into_inner(self) -> W;
  }
