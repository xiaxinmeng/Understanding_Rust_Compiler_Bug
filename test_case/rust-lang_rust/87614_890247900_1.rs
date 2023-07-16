
error[E0599]: the method `count` exists for array `[{integer}; 4]`, but its trait bounds were not satisfied
  --> $DIR/count2len.rs:5:11
   |
LL |     slice.count();
   |           ^^^^^ method cannot be called on `[{integer}; 4]` due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `[{integer}; 4]: Iterator`
           which is required by `&mut [{integer}; 4]: Iterator`
           `[{integer}]: Iterator`
           which is required by `&mut [{integer}]: Iterator`
