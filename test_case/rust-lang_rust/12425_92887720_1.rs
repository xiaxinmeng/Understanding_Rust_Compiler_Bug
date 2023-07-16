
<anon>:3:21: 3:26 error: chained comparison operators require parentheses
<anon>:3     let x = MyStruct<i32> { my_field: 42 };
                             ^~~~~
<anon>:3:26: 3:26 help: use `::<...>` instead of `<...>` if you meant to specify type arguments
<anon>:3:37: 3:38 error: expected one of `!`, `.`, `::`, `;`, `{`, `}`, or an operator, found `:`
<anon>:3     let x = MyStruct<i32> { my_field: 42 };
                                             ^
