


rustc 1.16.0 (30cf806ef 2017-03-10)
error[E0277]: the trait bound `fn(&i32): std::clone::Clone` is not satisfied
 --> <anon>:5:9
  |
5 |     Bar(Foo)
  |         ^^^^ the trait `std::clone::Clone` is not implemented for `fn(&i32)`
  |
  = help: the following implementations were found:
            <fn(A) -> Ret as std::clone::Clone>
            <extern "C" fn(A) -> Ret as std::clone::Clone>
            <extern "C" fn(A, ...) -> Ret as std::clone::Clone>
            <unsafe fn(A) -> Ret as std::clone::Clone>
          and 2 others
  = note: required by `std::clone::AssertParamIsClone`
