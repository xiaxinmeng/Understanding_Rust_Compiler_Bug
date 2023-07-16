
error[E0XXX]: expected lifetime generic closure, found specific lifetime closure
 --> <source>:7:5
  |
7 |     x(y);
  |       ^ expected a lifetime generic closure, but `y` is bound to a specific `'_` lifetime
  |
  = note: expected trait `for<'x> FnOnce<(LtGen<'x>,)>`
             found trait `FnOnce<(LtGen<'_>,)>`
  = help: `for<'x>` means that `FnOnce<(LtGen<'x>,)>` can be called for any given lifetime, but `y` can only be called for a specific bound lifetime `'_` (FIXME)
note: the lifetime requirement is introduced here
 --> <source>:4:41
  |
4 | fn takes_ltgen_forall_lifetimes(_: impl for<'x> FnOnce(LtGen<'x>)) {}
  |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^
