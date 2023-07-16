
error[E0658]: non-reference pattern used to match a reference (see issue #42640)
  --> <source>:11:9
   |
11 |         Opaque(_) => {}
   |         ^^^^^^^^^ help: consider using a reference: `&Opaque(_)`
