
[00:44:34] error: only foreign functions are allowed to be variadic
[00:44:34]  --> <anon>:5:15
[00:44:34]   |
[00:44:34] 5 | fn foo(x: u8, ...) {}
[00:44:34]   |               ^^^
[00:44:34] 
[00:44:34] error[E0178]: expected a path on the left-hand side of `+`, not `&'a Foo`
[00:44:34]  --> <anon>:4:8
[00:44:34]   |
[00:44:34] 4 |     w: &'a Foo + Copy,   // error, use &'a (Foo + Copy)
[00:44:34]   |        ^^^^^^^^^^^^^^ help: try adding parentheses: `&'a (Foo + Copy)`
[00:44:34] 
[00:44:34] error[E0178]: expected a path on the left-hand side of `+`, not `&'a Foo`
[00:44:34]  --> <anon>:5:8
[00:44:34]   |
[00:44:34] 5 |     x: &'a Foo + 'a,     // error, use &'a (Foo + 'a)
[00:44:34]   |        ^^^^^^^^^^^^ help: try adding parentheses: `&'a (Foo + 'a)`
[00:44:34] 
[00:44:34] error[E0178]: expected a path on the left-hand side of `+`, not `&'a mut Foo`
[00:44:34]  --> <anon>:6:8
[00:44:34]   |
[00:44:34] 6 |     y: &'a mut Foo + 'a, // error, use &'a mut (Foo + 'a)
[00:44:34]   |        ^^^^^^^^^^^^^^^^ help: try adding parentheses: `&'a mut (Foo + 'a)`
[00:44:34] 
[00:44:34] error[E0178]: expected a path on the left-hand side of `+`, not `fn() -> Foo`
[00:44:34]  --> <anon>:7:8
[00:44:34]   |
[00:44:34] 7 |     z: fn() -> Foo + 'a, // error, use fn() -> (Foo + 'a)
[00:44:34]   |        ^^^^^^^^^^^^^^^^ perhaps you forgot parentheses?
[00:44:34] 
[00:44:34] error: unexpected close delimiter: `}`
[00:44:34]  --> <anon>:5:1
[00:44:34]   |
[00:44:34] 5 | }
[00:44:34]   | ^ unexpected close delimiter
