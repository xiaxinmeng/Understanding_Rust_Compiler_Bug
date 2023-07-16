
[00:51:50] 
[00:51:50] 8    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:51:50] 9    = note: required by `std::iter::IntoIterator::into_iter`
[00:51:50] 10 
[00:51:50] - error: aborting due to previous error
[00:51:50] + error[E0277]: the size for values of type `dyn std::iter::Iterator<Item=&mut u8>` cannot be known at compilation time
[00:51:50] +   --> $DIR/issue-20605.rs:12:17
[00:51:50] +    |
[00:51:50] + LL |     for item in *things { *item = 0 }
[00:51:50] +    |
[00:51:50] +    |
[00:51:50] +    = help: the trait `std::marker::Sized` is not implemented for `dyn std::iter::Iterator<Item=&mut u8>`
[00:51:50] +    = note: all local variables must have a statically known size
[00:51:50] +    = help: unsized locals are gated as an unstable feature
[00:51:50] + 
[00:51:50] + error: aborting due to 2 previous errors
[00:51:50] + error: aborting due to 2 previous errors
[00:51:50] 12 
[00:51:50] 13 For more information about this error, try `rustc --explain E0277`.
[00:51:50] 14 
[00:51:50] 
