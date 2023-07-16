
<anon>:11:1: 11:2 error: the trait `serialize::json::ToJson` is not implemented for the type `&str`
<anon>:11 f("foo") //
          ^
<anon>:11:1: 11:2 note: the trait `serialize::json::ToJson` must be implemented because it is required by `f`
<anon>:11 f("foo") //
          ^
error: aborting due to previous error
