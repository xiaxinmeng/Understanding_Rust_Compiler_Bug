
<anon>:6:14: 6:16 error: `xs` does not live long enough
<anon>:6     for x in xs.iter() {
                      ^~
note: in expansion of for loop expansion
<anon>:6:5: 8:6 note: expansion site
<anon>:4:49: 10:2 note: reference must be valid for the block suffix following statement 0 at 4:48...
<anon>:4     let mut set: HashSet<&i32> = HashSet::new();
<anon>:5     let xs = vec![0, 1, 2, 3, 4];
<anon>:6     for x in xs.iter() {
<anon>:7         set.insert(x);
<anon>:8     }
<anon>:9     xs
         ...
<anon>:5:34: 10:2 note: ...but borrowed value is only valid for the block suffix following statement 1 at 5:33
<anon>: 5     let xs = vec![0, 1, 2, 3, 4];
<anon>: 6     for x in xs.iter() {
<anon>: 7         set.insert(x);
<anon>: 8     }
<anon>: 9     xs
<anon>:10 }
error: aborting due to previous error
playpen: application terminated with error code 101
