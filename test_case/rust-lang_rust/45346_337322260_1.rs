
error[E0597]: borrowed value does not live long enough
 --> src/main.rs:4:19
  |
4 |     let _ = |&Foo(ref name)| {
  |                   ^^^^^^^^ does not live long enough
5 |         name.len()
  |                  - borrowed value needs to live until here
  |
  = note: ...but borrowed value is only valid for the empty lifetime
