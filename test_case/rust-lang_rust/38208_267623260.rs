
failures:

---- hash::BuildHasherDefault_0 stdout ----
	error[E0412]: type name `SomeHasher` is undefined or not in scope
  --> <anon>:22:41
   |
22 | type MyBuildHasher = BuildHasherDefault<SomeHasher>;
   |                                         ^^^^^^^^^^ undefined or not in scope
   |
   = help: no candidates by the name of `SomeHasher` found in your project; maybe you misspelled the name or forgot to import an external crate?

error[E0412]: type name `SomeBuidHasher` is undefined or not in scope
  --> <anon>:24:36
   |
24 | let hash_map = HashMap::<u32, u32, SomeBuidHasher>::default();
   |                                    ^^^^^^^^^^^^^^ undefined or not in scope
   |
   = help: no candidates by the name of `SomeBuidHasher` found in your project; maybe you misspelled the name or forgot to import an external crate?

error: aborting due to previous error(s)

thread 'hash::BuildHasherDefault_0' panicked at 'Box<Any>', C:\bot\slave\auto-win-msvc-32-opt\build\src\librustc\session/mod.rs:201
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    hash::BuildHasherDefault_0

test result: FAILED. 1074 passed; 1 failed; 12 ignored; 0 measured
