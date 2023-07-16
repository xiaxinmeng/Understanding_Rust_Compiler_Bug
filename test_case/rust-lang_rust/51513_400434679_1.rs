
warning: unnecessary lifetime parameter `'a`
 --> file.rs:1:8
  |
1 | fn foo<'a: 'static>(x: for<'a> fn(&'a u32)) { }
  |        ^^^^^^^^^^^
help: you can use the `'static` lifetime directly, in place of `'a`
  |
1 | fn foo(x: for<'a> fn(&'static u32)) { }
  |      --               ^^^^^^^

error[E0496]: lifetime name `'a` shadows a lifetime name that is already in scope
 --> file.rs:1:28
  |
1 | fn foo<'a: 'static>(x: for<'a> fn(&'a u32)) { }
  |        --                  ^^ lifetime 'a already in scope
  |        |
  |        first declared here
