plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> src/librustdoc/html/render/print_item.rs:288:25
    |
288 |                         cx.tcx().sess().diagnostic(),
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Session`, found struct `Handler`
    = note: expected reference `&Session`
               found reference `&Handler`

error: aborting due to previous error
