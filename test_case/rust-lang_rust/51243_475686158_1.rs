
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements
  --> src/lib.rs:13:9
   |
13 |         C { a: self.a } 
   |         ^
   |
note: first, the lifetime cannot outlive the lifetime 'a as defined on the impl at 11:6...
  --> src/lib.rs:11:6
   |
11 | impl<'a> B<'a> {
   |      ^^
note: ...so that reference does not outlive borrowed content
  --> src/lib.rs:13:16
   |
13 |         C { a: self.a } 
   |                ^^^^^^
note: but, the lifetime must be valid for the lifetime 'b as defined on the method body at 12:18...
  --> src/lib.rs:12:18
   |
12 |     pub fn get_c<'b: 'a>(&self) -> C<'b> { 
   |                  ^^
   = note: ...so that the expression is assignable:
           expected C<'b>
              found C<'_>
