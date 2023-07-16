
<anon>:56:5: 56:34 error: type mismatch resolving `for<'a> <collections::vec::Drain<'a, &'a str> as core::iter::Iterator>::Item == &str`:
 expected bound lifetime parameter 'a,
    found concrete lifetime [E0271]
<anon>:56     expects_polymorphic_function1(Const("hi"));
              ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:56:5: 56:34 help: see the detailed explanation for E0271
<anon>:56:5: 56:34 note: required by `expects_polymorphic_function1`
