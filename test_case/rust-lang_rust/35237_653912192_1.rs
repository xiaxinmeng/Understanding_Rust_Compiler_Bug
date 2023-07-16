
error[E0034]: multiple applicable items in scope
  --> fil12.rs:16:25
   |
16 |     fn bar(&self) {self.foo()}
   |                         ^^^ multiple `foo` found
   |
note: candidate #1 is defined in the trait `Foo`
  --> fil12.rs:9:5
   |
9  |     fn foo(&self);
   |     ^^^^^^^^^^^^^^
note: candidate #2 is defined in the trait `Baz`
  --> fil12.rs:12:5
   |
12 |     fn foo(&self);
   |     ^^^^^^^^^^^^^^
help: disambiguate the associated function for candidate #1
   |
16 |     fn bar(&self) {Foo::foo(&self)}
   |                    ^^^^^^^^^^^^^^^
help: disambiguate the associated function for candidate #2
   |
16 |     fn bar(&self) {Baz::foo(&self)}
   |                    ^^^^^^^^^^^^^^^
