diff
  pub fn check<'a>(r: &'a &'a ()) {
-     pub type Borrow<'a, T : 'a> = &'a T;
+     pub type Borrow<'a, T> = &'a T;
      type DynSend<'a> = Borrow<'a, dyn Send>;
      let _: DynSend<'a> = r;
  }
