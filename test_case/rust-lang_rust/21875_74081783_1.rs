
<anon>:4:21: 4:24 error: `bar` does not live long enough
<anon>:4     foo = Box::new(&bar);
                             ^~~
<anon>:2:23: 8:2 note: reference must be valid for the block suffix following statement 0 at 2:22...
<anon>:2     let foo: Box<&i32>;
<anon>:3     let bar = 1337;
<anon>:4     foo = Box::new(&bar);
<anon>:5     { foo; } // kill foo
<anon>:6     foo;
<anon>:7     bar;
         ...
<anon>:3:19: 8:2 note: ...but borrowed value is only valid for the block suffix following statement 1 at 3:18
<anon>:3     let bar = 1337;
<anon>:4     foo = Box::new(&bar);
<anon>:5     { foo; } // kill foo
<anon>:6     foo;
<anon>:7     bar;
<anon>:8 }
<anon>:6:5: 6:8 error: use of moved value: `foo`
<anon>:6     foo;
             ^~~
<anon>:5:7: 5:10 note: `foo` moved here because it has type `Box<&i32>`, which is non-copyable
<anon>:5     { foo; } // kill foo
               ^~~
error: aborting due to 2 previous errors
