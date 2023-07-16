
error[E0603]: module `bar` is private
  --> $DIR/shadowed-use-visibility.rs:9:14
   |
LL |     use foo::bar::f as g;
   |              ^^^ this module is private
   |
note: used restricted re-export of `bar`
  --> $DIR/shadowed-use-visibility.rs:4:9
   |
LL |     use foo as bar;
   |         ^^^^^^^^^^
note: re-exported module `bar` is declared here
  --> $DIR/shadowed-use-visibility.rs:1:1
   |
LL | / mod foo {
LL | |     pub fn f() {}
LL | |
LL | |     use foo as bar;
LL | |     pub use self::f as bar;
LL | | }
   | |_^
