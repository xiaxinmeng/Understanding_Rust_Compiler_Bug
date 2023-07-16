
error[E0423]: expected function, found struct `Box`
 --> test.rs:5:18
  |
5 |     U{ wtf: Some(Box(U{ wtf: None })) };
  |                  ^^^ help: did you mean to use the `new` method?: `Box::new`

error: aborting due to previous error
