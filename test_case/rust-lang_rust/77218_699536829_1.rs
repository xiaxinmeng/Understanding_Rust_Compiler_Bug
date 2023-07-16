
error[E0308]: mismatched types
  --> $DIR/if-let-typo.rs:6:8
   |
LL |     if 3 = foo {} //~ ERROR mismatched types
   |        ^^^^^^^ expected `bool`, found `()`
