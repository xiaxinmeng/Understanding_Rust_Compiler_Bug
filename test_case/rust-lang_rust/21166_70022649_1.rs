
<anon>:6:9: 6:10 error: use of moved value: `x`
<anon>:6     (x, x);
                 ^
<anon>:6:6: 6:7 note: `x` moved here because it has type `Foo`, which is non-copyable
<anon>:6     (x, x);
              ^
error: aborting due to previous error
