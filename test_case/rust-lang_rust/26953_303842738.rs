rust
// Example #1
rustc 1.19.0-nightly (5b13bff52 2017-05-23)
error[E0423]: expected value, found type alias `A`
  --> <anon>:12:11
   |
12 |     let a=A;
   |           ^ did you mean `A { /* fields */ }`?

// Example #2
rustc 1.19.0-nightly (5b13bff52 2017-05-23)
error[E0423]: expected value, found type alias `A`
 --> <anon>:8:9
  |
8 |         A
  |         ^ did you mean `A { /* fields */ }`?

// Example #5
rustc 1.19.0-nightly (5b13bff52 2017-05-23)
error[E0423]: expected function, found type alias `A`
  --> <anon>:12:11
   |
12 |     let a=A(0);
   |           ^ did you mean `A { /* fields */ }`?
