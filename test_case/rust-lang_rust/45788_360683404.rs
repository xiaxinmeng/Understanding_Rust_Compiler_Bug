
error[E0593]: function is expected to take 1 argument, but it takes 2 arguments
  --> src/main.rs:12:5
   |
7  | fn callback(s1: &str, s2: &str) {
   | ------------------------------- takes 2 arguments
...
12 |     takes_callback(callback, "rust!");
   |     ^^^^^^^^^^^^^^ expected function that takes 1 argument
   |
note: required by `takes_callback`
  --> src/main.rs:1:1
   |
1  | / fn takes_callback<F,T,R>(callback: F, t: T)
2  | |     where F: FnOnce(T) -> R
3  | | {
4  | |     callback(t);
5  | | }
   | |_^
