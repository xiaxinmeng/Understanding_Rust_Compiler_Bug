
error[E0593]: function is expected to take 1 argument, but it takes 2 arguments
  --> $DIR/closure-arg-count.rs:40:41
   |
40 |     let _it = vec![1, 2, 3].into_iter().map(usize::checked_add);
   |                                         ^^^ ------------------ takes 2 arguments
   |                                         |
   |                                         expected function that takes 1 argument

