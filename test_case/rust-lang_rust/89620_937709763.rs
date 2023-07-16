
error[E0308]: mismatched types
 --> <anon>:5:13
  |
1 | fn bar<Input>() -> impl Fn(Input) {
  |                    --------------
  |                    |
  |                    the expected opaque type
  |                    the found opaque type
...
5 | fn foo() -> impl Fn(&str) {
  |             ^^^^^^^^^^^^^ one type is more general than the other
  |
  = note: expected associated type `<Opaque(DefId(0:5 ~ rust_out[8787]::bar::{opaque#0}), [&'_#0r str]) as FnOnce<(&'_#0r str,)>>::Output`
             found associated type `<Opaque(DefId(0:5 ~ rust_out[8787]::bar::{opaque#0}), [&'_#0r str]) as FnOnce<(&RePlaceholder(Placeholder { universe: U3, name: BrAnon(0) }) str,)>>::Output`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
