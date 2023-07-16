console
$ echo 'extern crate b;' | rustc - --extern b=libb.rlib
error[E0463]: can't find crate for `a` which `b` depends on
 --> <anon>:1:1
  |
1 | extern crate b;
  | ^^^^^^^^^^^^^^^ can't find crate
