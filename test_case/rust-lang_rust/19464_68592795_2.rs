 cmd
<anon>:22:18: 22:25 error: mismatched types:
 expected `Bar`,
    found `<I as Foo>::A`
(expected struct Bar,
    found associated type)
<anon>:22     let _: Bar = x.boo();
                           ^~~~~~~
<anon>:33:5: 33:9 error: type mismatch resolving `<int as Foo>::A == Bar`:
 expected uint,
    found struct Bar
<anon>:33     foo1(a);
              ^~~~
<anon>:33:5: 33:9 note: required by `foo1`
<anon>:33     foo1(a);
              ^~~~
<anon>:34:9: 34:11 error: type mismatch resolving `<int as Foo>::A == Bar`:
 expected uint,
    found struct Bar
<anon>:34     baz(&a);
                  ^~
<anon>:34:9: 34:11 note: required for the cast to the object type `Foo`
<anon>:34     baz(&a);
                  ^~
error: aborting due to 3 previous errors
playpen: application terminated with error code 101
Program ended.
