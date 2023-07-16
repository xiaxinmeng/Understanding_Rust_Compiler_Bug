
   |
11 | trait Foo<T> {
   |           - first `T` declared here
12 |     fn do_something(&self) -> T;
13 |     fn do_something_else<T: Clone>(&self, bar: T); //~ ERROR E0194
