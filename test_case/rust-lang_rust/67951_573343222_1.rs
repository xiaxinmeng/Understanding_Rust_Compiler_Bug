
error[E0603]: module import `bar` is private
  --> $DIR/shadowed-use-visibility.rs:9:14
   |
LL |     use foo::bar::f as g;
   |              ^^^ this import is private
   |
note: the module import `bar` is declared here
  --> $DIR/shadowed-use-visibility.rs:4:9
   |
LL |     use foo as bar;
   |         ^^^^^^^^^^
