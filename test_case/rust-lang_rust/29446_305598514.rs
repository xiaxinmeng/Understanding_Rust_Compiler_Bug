
error: reached the recursion limit while instantiating `<T as A><X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<X<i32>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>::foo`
  --> test.rs:10:5
   |
10 | /     fn foo(self) {
11 | |         X(self).foo() // <-- error points here
12 | |     }
   | |_____^
