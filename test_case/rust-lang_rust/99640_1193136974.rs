plain
 finished in 3.602 seconds
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 42 tests
Some tests failed in compiletest suite=codegen-units mode=codegen-units host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
ii...................i.........FFF.F......

---- [codegen-units] src/test/codegen-units/partitioning/local-inlining-but-not-all.rs stdout ----

These items should have been contained but were not:
These items should have been contained but were not:

MONO_ITEM fn inline::inlined_function @@ local_inlining_but_not_all-inline[External]

thread '[codegen-units] src/test/codegen-units/partitioning/local-inlining-but-not-all.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2771:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


---- [codegen-units] src/test/codegen-units/partitioning/local-transitive-inlining.rs stdout ----

These items should have been contained but were not:

MONO_ITEM fn direct_user::foo @@ local_transitive_inlining-indirect_user[Internal]
MONO_ITEM fn inline::inlined_function @@ local_transitive_inlining-indirect_user[Internal]

thread '[codegen-units] src/test/codegen-units/partitioning/local-transitive-inlining.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2771:13

---- [codegen-units] src/test/codegen-units/partitioning/local-inlining.rs stdout ----
---- [codegen-units] src/test/codegen-units/partitioning/local-inlining.rs stdout ----

These items should have been contained but were not:

MONO_ITEM fn inline::inlined_function @@ local_inlining-user1[Internal] local_inlining-user2[Internal]

thread '[codegen-units] src/test/codegen-units/partitioning/local-inlining.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2771:13

---- [codegen-units] src/test/codegen-units/partitioning/inlining-from-extern-crate.rs stdout ----
---- [codegen-units] src/test/codegen-units/partitioning/inlining-from-extern-crate.rs stdout ----

These items should have been contained but were not:

MONO_ITEM fn cgu_explicit_inlining::always_inlined @@ inlining_from_extern_crate[Internal] inlining_from_extern_crate-mod2[Internal]
MONO_ITEM fn cgu_explicit_inlining::inlined @@ inlining_from_extern_crate[Internal] inlining_from_extern_crate-mod1[Internal]

thread '[codegen-units] src/test/codegen-units/partitioning/inlining-from-extern-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2771:13


