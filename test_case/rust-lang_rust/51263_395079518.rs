
[01:10:15] error[E0432]: unresolved import `arc`
[01:10:15] --> liballoc/task.rs:13:5
[01:10:15]    |
[01:10:15]    | use arc::Arc;
[01:10:15] 
[01:10:15] error[E0207]: the type parameter `T` is not constrained by the impl trait, self type, or predicates[0m
[01:10:15] --> liballoc/task.rs:74:6
[01:10:15]    | impl<T> From<Arc<T>> for Waker

