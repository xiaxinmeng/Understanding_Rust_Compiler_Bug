
<anon>:2:5: 2:13 error: a value named `Bar` has already been imported in this module [E0252]
<anon>:2 use bar::Bar;
             ^~~~~~~~
<anon>:2:5: 2:13 help: see the detailed explanation for E0252
<anon>:1:5: 1:13 note: previous import of `Bar` here
<anon>:1 use foo::Bar;
             ^~~~~~~~
<anon>:2:5: 2:13 error: a type named `Bar` has already been imported in this module [E0252]
<anon>:2 use bar::Bar;
             ^~~~~~~~
<anon>:2:5: 2:13 help: see the detailed explanation for E0252
<anon>:1:5: 1:13 note: previous import of `Bar` here
<anon>:1 use foo::Bar;
             ^~~~~~~~
error: aborting due to 2 previous errors
