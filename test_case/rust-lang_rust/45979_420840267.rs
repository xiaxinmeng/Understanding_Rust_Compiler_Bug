
error[E0621]: explicit lifetime required in the type of `lvalue`
  --> src/main.rs:26:69
   |
24 | fn describe<'cx>(lvalue: Lvalue, cx: &Cx<'cx>) -> String {
   |                          ------ help: add explicit lifetime `'cx` to the type of `lvalue`: `Lvalue<'cx>`
25 |     match lvalue {
26 |       Lvalue::Deref(base) => format!("deref of {} pointer", base.ty(cx)),
   |                                                                     ^^ lifetime `'cx` required
