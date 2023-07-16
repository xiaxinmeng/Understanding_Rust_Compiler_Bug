rust
  let mut err = self.sess.create_err(UseEqInstead { span: self.token.span });
  err.emit();
  return Err(err);
