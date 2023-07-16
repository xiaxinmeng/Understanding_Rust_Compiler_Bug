
error[E0423]: expected function, found struct `Box`
 --> test.rs:5:18
  |
5 |     U{ wtf: Some(Box(U{ wtf: None })) };
  |                  ^^^
  |                  |
  |                  did you mean `Box { /* fields */ }`?
  |                  constructor is not visible here due to private fields

error: aborting due to previous error
