plain
[00:51:35] ...................................i................................................................ 4200/4932
[00:51:40] .................................................................................................... 4300/4932
[00:51:44] .................................................................................................... 4400/4932
[00:51:48] .................................................................................................... 4500/4932
[00:51:51] .......i...................F..F..................................................................... 4600/4932
[00:51:57] .................................................................................................... 4800/4932
[00:51:59] .......................................................................i............................ 4900/4932
[00:52:00] ................................
[00:52:00] failures:
[00:52:00] failures:
[00:52:00] 
[00:52:00] ---- [ui] ui/trait-alias-fail.rs stdout ----
[00:52:00] diff of stderr:
[00:52:00] 
[00:52:00] 22 LL | impl Alias1 for () { //~ERROR expected trait, found trait alias
[00:52:00] 24 
[00:52:00] 24 
[00:52:00] - error[E0658]: trait aliases are not yet fully implemented (see issue #41517)
[00:52:00] + error[E0658]: trait aliases are experimental (see issue #41517)
[00:52:00] 27    |
[00:52:00] 27    |
[00:52:00] 28 LL | trait Alias1<T> = Default where T: Clone; // ok
[00:52:00] 30    |
[00:52:00] 31    = help: add #![feature(trait_alias)] to the crate attributes to enable
[00:52:00] 32 
[00:52:00] 32 
[00:52:00] - error[E0658]: trait aliases are not yet fully implemented (see issue #41517)
[00:52:00] + error[E0658]: trait aliases are experimental (see issue #41517)
[00:52:00] 35    |
[00:52:00] 35    |
[00:52:00] 36 LL | trait Alias2<T: Clone = ()> = Default;
[00:52:00] 
[00:52:00] The actual stderr differed from the expected stderr.
[00:52:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-alias-fail/trait-alias-fail.stderr
[00:52:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-alias-fail/trait-alias-fail.stderr
[00:52:00] To \nenum Foo {\n    Bar(u64),\n}\n