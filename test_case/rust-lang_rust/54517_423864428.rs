plain
[01:00:24] ....................................................................................................
[01:00:27] ....................................................i...............................................
[01:00:30] ....................................................................................................
[01:00:33] ....................................................................................................
[01:00:36] .iiiiiiiii..........................................................................................
[01:00:42] ....................................................................................................
[01:00:45] .....................................................................................i..............
[01:00:48] ....................................................................................................
[01:00:51] ........................................i.i..ii.....................................................
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:35] 
[01:09:35] running 107 tests
[01:09:38] i..ii...iii....i...i............iii...........i.....i....ii...i.i.ii..............i...i.i.ii.i....ii
[01:09:38] test result: ok. 78 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out
[01:09:38] 
[01:09:38]  finished in 3.640
[01:09:38] travis_fold:end:test_codegen
---
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:05] 
[01:10:05] running 40 tests
[01:10:40] ...............FF...F............F.F....
[01:10:40] 
[01:10:40] ---- [ui] ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test.rs stdout ----
[01:10:40] diff of stderr:
[01:10:40] 
[01:10:40] 
[01:10:40] + error[E0428]: the name `Test` is defined multiple times
[01:10:40] +   --> $DIR/ambiguous-builtin-attrs-test.rs:13:1
[01:10:40] +    |
[01:10:40] + LL | #[test] // OK, shadowed
[01:10:40] +    | ------- previous definition of the type `Test` here
[01:10:40] + ...
[01:10:40] + LL | #[bench] // OK, shadowed
[01:10:40] +    | ^^^^^^^^ `Test` redefined here
[01:10:40] +    |
[01:10:40] +    = note: `Test` must be defined only once in the type namespace of this module
[01:10:40] + 
[01:10:40] + error[E0425]: cannot find value `Bench` in this scope
[01:10:40] +   --> $DIR/ambiguous-builtin-attrs-test.rs:18:5
[01:10:40] +    |
[01:10:40] + LL |     Bench;
[01:10:40] + 
[01:10:40] 1 error[E0425]: cannot find value `NonExistent` in this scope
[01:10:40] 2   --> $DIR/ambiguous-builtin-attrs-test.rs:19:5
[01:10:40] 3    |
[01:10:40] 3    |
[01:10:40] 
[01:10:40] 4 LL |     NonExistent; //~ ERROR cannot find value `NonExisten:40] ------------------------------------------
[01:10:40] ------------------------------------------
[01:10:40] stderr:
[01:10:40] ------------------------------------------
[01:10:40] ------------------------------------------
[01:10:40] {"message":"the name `Test` is defined multiple times","code":{"code":"E0428","explanation":"\nA type or module has been defined more than once.\n\nErroneous code example:\n\n