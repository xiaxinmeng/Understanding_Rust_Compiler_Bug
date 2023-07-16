
<anon>:17:7: 17:22 error: no method named `destructuring` found for type `Foo` in the current scope
<anon>:17     x.destructuring(); // Explodes
                ^~~~~~~~~~~~~~~
<anon>:17:7: 17:22 note: found defined static methods, maybe a `self` is missing?
<anon>:8:5: 10:6 note: candidate #1 is defined in an impl for the type `Foo`
<anon>:8     fn destructuring(&Foo(u, v): &Self) {
<anon>:9         println!("{}, {}", u, v);
<anon>:10     }
error: aborting due to previous error
playpen: application terminated with error code 101
