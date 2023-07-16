
error[E0593]: function is expected to take 1 argument, but it takes 2 arguments
  --> src/main.rs:12:5
   |
1  | fn takes_callback<F,T,R>(callback: F, t: T)
   |                          -------- expected function that takes 1 argument here...
2  |     where F: FnOnce(T) -> R
   |           ----------------- ...because of this binding
...
12 |     takes_callback(callback, "rust!");
   |     ^^^^^^^^^^^^^^ -------- this function takes 2 arguments
   |     |
   |     expected function that takes 1 argument
   |
   = note: required by `takes_callback`
