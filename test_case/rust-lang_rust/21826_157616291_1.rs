
<anon>:18:13: 18:23 error: a trait named `Write` has already been imported in this module [E0252]
<anon>:18         use bar::Write;
                      ^~~~~~~~~~
<anon>:25:5: 25:17 note: in this expansion of write! (defined in <anon>)
<anon>:18:13: 18:23 help: see the detailed explanation for E0252
<anon>:17:9: 17:24 note: previous import of `Write` here
<anon>:17         use foo::Write;
                  ^~~~~~~~~~~~~~~
<anon>:25:5: 25:17 note: in this expansion of write! (defined in <anon>)
