
error[E0282]: type annotations needed
  --> $DIR/issue-12187-1.rs:16:10
   |
LL |     let &v = new();
   |         -^
   |         ||
   |         |cannot infer type
   |         consider giving the pattern a type
