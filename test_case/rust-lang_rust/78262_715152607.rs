
error[E0308]: mismatched types
 --> <source>:8:28
  |
8 |     let f = |x: &dyn TT| x.func();
  |                            ^^^^ lifetime mismatch
  |
  = note: expected reference `&(dyn TT + 'static)`
             found reference `&dyn TT`
note: the anonymous lifetime #1 defined on the body at 8:13...
 --> <source>:8:13
  |
8 |     let f = |x: &dyn TT| x.func();
  |             ^^^^^^^^^^^^^^^^^^^^^
  = note: ...does not necessarily outlive the static lifetime

error: aborting due to previous error
