Rust
   pub struct RefPrintContext {
       inner: RefCell<PrintContext>
   }

   impl RefPrintContext {
       pub fn new(p: PrintContext) -> Self { /* .. */ }
       pub fn into_inner(self) -> PrintContext { self.inner.into_inner() }
       pub fn with<T: Print>(&self, data: T) -> RefPrintContextWith<'a, T> {
           RefPrintContextWith { cx: self, data }
       }
   }

   pub struct RefPrintContextWith<'a, T: fmt::Print> {
       cx: &'a RefPrintContext,
       data: T
   }

   impl<'a, T: Print> Display for RefPrintContextWith<'a, T> {
       fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
           Print::print_display(&self.data, f, &self.cx.borrow_mut())
       }
   }
2. Change the `span_err!` and `span_warn!` macros in rustc to create a new `PrintContext`, turn it to a `RefPrintContext`, and wrap `rpcx.with(arg)` around the format args.
   So `span_err!(sess, span, E0222, "{}", x)` becomes
   