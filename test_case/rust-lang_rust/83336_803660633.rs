
error[E0433]: failed to resolve: maybe a missing crate `clippy`?
  --> $DIR/tool-mod-child.rs:2:5
   |
LL | use clippy::a::b;
   |     ^^^^^^ maybe a missing crate `clippy`?

error[E0432]: unresolved import `clippy`
  --> $DIR/tool-mod-child.rs:1:5
   |
LL | use clippy::a;
   |     ^^^^^^ maybe a missing crate `clippy`?
