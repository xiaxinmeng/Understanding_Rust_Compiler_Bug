
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
 --> <anon>:3:1
  |
3 | impl fmt::Display for Vec<Bar> { } 
  |                                     ^^^^ not defined in the current crate
  |
  = note: define and implement a trait or new type instead
