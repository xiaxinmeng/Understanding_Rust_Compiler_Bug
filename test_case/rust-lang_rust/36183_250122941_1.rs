 rust
error[E0495]: the `Any` trait does not apply to references
 --> <anon>:4:14
  |
4 |     Box::new(value) as Box<Any>
  |              ^^^^^  ------------- being converted into a trait object
  |              value is a shared reference
  |
