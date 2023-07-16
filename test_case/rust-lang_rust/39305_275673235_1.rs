rust
error[E0106]: missing lifetime specifier
 --> <anon>:5:15
  |
5 | impl Quux for Foo { } // error: expected 1, found 0
  |               ^^^ expected lifetime parameter

thread 'rustc' panicked at 'Some expected error codes were not found: ["E0107"]
