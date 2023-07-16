
test.rs:27:0: 34:1 error: overly deep expansion of inlined function
test.rs:27 fn test<T:Dot> (n: int, i: int, first: T, second: T) ->int
test.rs:28 {
test.rs:29  match n
test.rs:30  {
test.rs:31      0 => {first.dot(second)}
test.rs:32      _ => {test (n-1, i+1, Cons {head:2*i+1, tail:first}, Cons{head:i*i, tail:second})}
