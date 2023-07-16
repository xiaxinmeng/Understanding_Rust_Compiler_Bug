
<anon>:2:12: 2:25 error: mismatched types: expected `&str`, found `&&'static str` (expected str, found &-ptr)
<anon>:2 struct Foo(&'static str);
                    ^~~~~~~~~~~~~
note: in expansion of #[deriving]
<anon>:1:1: 1:44 note: expansion site
