
  1 error[E0618]: expected function, found `i32`
  2   --> $DIR/issue-10969.rs:12:5
  3    |
  4 12 |     i(); //~ERROR expected function, found `i32`
  5    |     ^^^
  6    |
  7 note: defined here
  8    |
  9 11 | fn func(i: i32) {
 10    |         ^
 11 
 12 error[E0618]: expected function, found `i32`
 13   --> $DIR/issue-10969.rs:16:5
 14    |
 15 16 |     i(); //~ERROR expected a function, I found `i32`!!
 16    |     ^^^
 17    |
 18 note: defined here
 19   --> $DIR/issue-10969.rs:15:9
 20 i is `i32`, not a function just like I mentioned before
 21    |
 22 15 |     let i = 0i32;
 23    |         ^
 24 
 25 error: aborting due to 2 previous errors
