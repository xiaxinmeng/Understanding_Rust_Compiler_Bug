
<anon>:14:22: 14:28 error: cannot borrow `x.data` as immutable because it is also borrowed as mutable
<anon>:14     println!("{:?}", x.data); 
                               ^~~~~~
note: in expansion of format_args!
<std macros>:2:43: 2:76 note: expansion site
<std macros>:1:1: 2:78 note: in expansion of println!
<anon>:14:5: 14:30 note: expansion site
<anon>:10:22: 10:28 note: previous borrow of `x.data` occurs here; the mutable borrow prevents subsequent moves, borrows, or modification of `x.data` until the borrow ends
<anon>:10     if let Some(y) = x.data.as_mut() {
                               ^~~~~~
<anon>:16:2: 16:2 note: previous borrow ends here
<anon>:9 fn foo(x: &mut Foo) -> Option<&mut i32> {
...
<anon>:16 }
          ^
error: aborting due to previous error
