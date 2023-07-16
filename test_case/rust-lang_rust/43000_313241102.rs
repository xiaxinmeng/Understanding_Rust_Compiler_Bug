
[01:09:26] error[E0232]: this attribute must have a value
[01:09:26]   --> $DIR/bad-annotation.rs:26:1
[01:09:26]    |
[01:09:26] 26 | #[rustc_on_unimplemented] //~ ERROR this attribute must have a value
[01:09:26]    | ^^^^^^^^^^^^^^^^^^^^^^^^^ attribute requires a value
[01:09:26]    |
[01:09:26]    = note: eg `#[rustc_on_unimplemented = "foo"]`
[01:09:26] 
[01:09:26] error[E0230]: there is no type parameter C on trait BadAnnotation2
[01:09:26]   --> $DIR/bad-annotation.rs:30:1
[01:09:26]    |
[01:09:26] 30 | #[rustc_on_unimplemented = "Unimplemented trait error on `{Self}` with params `<{A},{B},{C}>`"]
[01:09:26]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:09:26] 
[01:09:26] error[E0231]: only named substitution parameters are allowed
[01:09:26]   --> $DIR/bad-annotation.rs:35:1
[01:09:26]    |
[01:09:26] 35 | #[rustc_on_unimplemented = "Unimplemented trait error on `{Self}` with params `<{A},{B},{}>`"]
[01:09:26]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:09:26] 
[01:09:26] error: aborting due to 3 previous errors
