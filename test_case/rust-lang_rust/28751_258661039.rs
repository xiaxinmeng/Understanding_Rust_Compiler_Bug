
error[E0017]: references in statics may only refer to immutable values
  --> /tmp/t.rs:10:20
   |
10 |     data: unsafe { &mut EXTRA_DATA }
   |                    ^^^^^^^^^^^^^^^ statics require immutable values

error[E0017]: references in statics may only refer to immutable values
  --> /tmp/t.rs:10:20
   |
10 |     data: unsafe { &mut EXTRA_DATA }
   |                    ^^^^^^^^^^^^^^^ statics require immutable values

error: aborting due to 2 previous errors
