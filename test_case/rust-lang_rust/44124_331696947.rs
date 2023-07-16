
[00:42:51]  error[E0623]: lifetime mismatch
[00:42:51]    --> $DIR/ex3-both-anon-regions-both-are-structs-4.rs:16:11
[00:42:51]     |
[00:42:51]  15 | fn foo(mut x: Ref) {
[00:42:51]     |               ---
[00:42:51]     |               |
[00:42:51] -   |               this type was declared with multiple lifetimes...
[00:42:51] +   |               this type is declared with multiple lifetimes...
[00:42:51]  16 |     x.a = x.b;
