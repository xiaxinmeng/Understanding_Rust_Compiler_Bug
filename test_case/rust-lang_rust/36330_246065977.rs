
error[E0559]: variant `Field::Fool` has no field named `joke`
 --> src/test/compile-fail/E0559.rs:16:27
   | 
16 |     let s = Field::Fool { joke: 0 }; 
   |                           ^^^^ did you mean `x`?    
