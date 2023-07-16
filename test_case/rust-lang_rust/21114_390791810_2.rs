
error[E0597]: `s` does not live long enough
  --> src/main.rs:17:15
   |
17 |         T{ s: &s }.s.n
   |         ------^^--
   |         |     |
   |         |     borrowed value does not live long enough
   |         borrow may end up in a temporary, created here
18 |     
19 | }
   | -
   | |
   | borrowed value only lives until here
   | temporary later dropped here, potentially using the reference
