rust
error[E0053]: method `b` has an incompatible type for trait
 --> <anon>:6:13
  |
2 |     fn b(c: i32);
  |             --- type in trait
...
6 |     fn b(c: ()) {  }
  |             ^^ expected i32, found ()
  |
  = note: expected type `fn(i32)`
             found type `fn(())`
