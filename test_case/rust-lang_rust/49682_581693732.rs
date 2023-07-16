
error[E0712]: thread-local variable borrowed past end of function
  --> src/main.rs:10:5
   |
10 |     &FOO
   |     ^^^^ thread-local variables cannot be borrowed beyond the end of the function
11 | }
   | - end of enclosing function is here
