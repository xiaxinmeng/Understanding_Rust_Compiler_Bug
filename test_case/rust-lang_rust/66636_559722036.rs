rust
warning: unused variable `bark` in the function new_dog()
--> src/main.rs:2:9
1 |  / fn new_dog() {
...  |
2 |  |    let bark = Bark::from("bark"); 
  |  |        ^^^^ help: you probably wanted to use it somehow, check the code around to see if it's the case
  |  |             help: if the variable is meant to be unused, consider prefixing with an underscore: `_bark`
  |
  = note: `#[warn(unused_variables)]` on by default
