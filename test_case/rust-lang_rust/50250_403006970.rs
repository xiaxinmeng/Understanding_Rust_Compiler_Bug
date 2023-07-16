plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:37:50] 
[00:37:50] running 1548 tests
[00:37:55] ........................................................................F..FFF...................i..
[00:38:05] ....................................................................................................
[00:38:09] ....................................................................................................
[00:38:12] ....................................................................................................
[00:38:15] ....................................................................................................
---
[00:39:07] 
[00:39:07] ---- [ui] ui/chalkify/lower_env1.rs stdout ----
[00:39:07] diff of stderr:
[00:39:07] 
[00:39:07] 7    = note: FromEnv(Self: Foo) :- FromEnv(Self: Bar).
[00:39:07] 8    = note: FromEnv(Self: Foo) :- FromEnv(Self: Bar).
[00:39:07] 9    = note: Implemented(Self: Bar) :- FromEnv(Self: Bar).
[00:39:07] -    = note: WellFormed(Self: Bar) :- Implemented(Self: Bar), WellFormed(Self: Bar), WellFormed(Self: Foo).
[00:39:07] +    = note: WellFormed(Self: Foo) :- Implemented(Self: Foo), Implemented(Self: Foo).
[00:39:07] +    = note: WellFormed(Self: Foo) :- Implemented(Self: Foo), Implemented(Self: Foo).
[00:39:07] 11 
[00:39:07] 12 error: program clause dump
[00:39:07] 13   --> $DIR/lower_env1.rs:19:1
[00:39:07] 
[00:39:07] 20    = note: Implemented(Self: Bar) :- FromEnv(Self: Bar).
[00:39:07] 21    = note: Implemented(Self: Foo) :- FromEnv(Self: Foo).
[00:39:07] 22    = note: Implemented(Self: std::marker::Sized) :- FromEnv(Self: std::marker::Sized).
[00:39:07] -    = note: WellFormed(Self: Bar) :- Implemented(Self: Bar), WellFormed(Self: Bar), WellFormed(Self: Foo).
[00:39:07] -    = note: WellFormed(Self: Foo) :- Implemented(Self: Foo).
[00:39:07] -    = note: WellFormed(Self: std::marker::Sized) :- Implemented(Self: std::marker::Sized).
[00:39:07] +    = note: WellFormed(Self: Foo) :- Implemented(Self: Foo), Implemented(Self: Foo).
[00:39:07] +    = note: WellFormed(Self: Foo) :- Implemented(Self: Foo), Implemented(Self: Foo).
[00:39:07] 27 error: aborting due to 2 previous errors
[00:39:07] 28 
[00:39:07] 
[00:39:07] 
[00:39:07] 
[00:39:07] The actual stderr differed from the expected stderr.
[00:39:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_env1/lower_env1.stderr
[00:39:07] To update references, rerun the tests and pass the `--bless` flag
[00:39:07] To only update this specific test, also pass `--test-args chalkify/lower_env1.rs`
[00:39:07] error: 1 errors occurred comparing output.
[00:39:07] status: exit code: 101
[00:39:07] status: exit code: 101
[00:39:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/lower_env1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_env1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_env1/auxiliary" "-A" "unused"
[00:39:07] ------------------------------------------
[00:39:07] 
[00:39:07] ------------------------------------------
[00:39:07] stderr:
[00:39:07] stderr:
[00:39:07] ------------------------------------------
[00:39:07] {"message":"program clause dump","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/chalkify/lower_env1.rs","byte_start":529,"byte_end":558,"line_start":16,"line_end":16,"column_start":1,"column_end":30,"is_primary":true,"text":[{"text":"#[rustc_dump_program_clauses] //~ ERROR program clause dump","highlight_start":1,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"FromEnv(Self: Foo) :- FromEnv(Self: Bar).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(Self: Foo) :- FromEnv(Self: Bar).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Implemented(Self: Bar) :- FromEnv(Self: Bar).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo) :- Implemented(Self: Foo), Implemented(Self: Foo).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo) :- Implemented(Self: Foo), Implemented(Self: Foo).","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: program clause dump\n  --> /checkout/src/test/ui/chalkify/lower_env1.rs:16:1\n   |\nLL | #[rustc_dump_program_clauses] //~ ERROR program clause dump\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: FromEnv(Self: Foo) :- FromEnv(Self: Bar).\n ented(Self: Foo).","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: program clause dump\n  --> /checkout/src/test/ui/chalkify/lower_env1.rs:19:1\n   |\nLL | #[rustc_dump_env_program_clauses] //~ ERROR program clause dump\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: FromEnv(Self: Foo) :- FromEnv(Self: Bar).\n   = note: FromEnv(Self: Foo) :- FromEnv(Self: Bar).\n   = note: Implemented(Self: Bar) :- FromEnv(Self: Bar).\n   = note: Implemented(Self: Foo) :- FromEnv(Self: Foo).\n   = note: Implemented(Self: std::marker::Sized) :- FromEnv(Self: std::marker::Sized).\n   = note: WellFormed(Self: Foo) :- Implemented(Self: Foo), Implemented(Self: Foo).\n   = note: WellFormed(Self: Foo) :- Implemented(Self: Foo), Implemented(Self: Foo).\n\n"}
[00:39:07] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:39:07] ------------------------------------------
[00:39:07] 
[00:39:07] thread '[ui] ui/chalkify/lower_env1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:39:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:39:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:39:07] 
[00:39:07] ---- [ui] ui/chalkify/lower_trait.rs stdout ----
[00:39:07] diff of stderr:
[00:39:07] 
[00:39:07] 8    = note: FromEnv(T: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).
[00:39:07] 9    = note: FromEnv(U: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).
[00:39:07] 10    = note: Implemented(Self: Foo<S, T, U>) :- FromEnv(Self: Foo<S, T, U>).
[00:39:07] -    = note: WellFormed(Self: Foo<S, T, U>) :- Implemented(Self: Foo<S, T, U>), WellFormed(S: std::marker::Sized), WellFormed(T: std::marker::Sized), WellFormed(U: std::marker::Sized).
[00:39:07] +    = note: WellFormed(S: std::marker::Sized) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(U: std::marker::Sized).
[00:39:07] +    = note: WellFormed(T: std::marker::Sized) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(U: std::marker::Sized).
[00:39:07] +    = note: WellFormed(U: std::marker::Sized) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(U: std::marker::Sized).
[00:39:07] 13 error: aborting due to previous error
[00:39:07] 14 
[00:39:07] 
[00:39:07] 
[00:39:07] 
[00:39:07] The actual stderr differed from the expected stderr.
[00:39:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_trait/lower_trait.stderr
[00:39:07] To update references, rerun the tests and pass the `--bless` flag
[00:39:07] To only update this specific test, also pass `--test-args chalkify/lower_trait.rs`
[00:39:07] error: 1 errors occurred comparing output.
[00:39:07] status: exit code: 101
[00:39:07] status: exit code: 101
[00:39:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/lower_trait.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_trait/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_trait/auxiliary" "-A" "unused"
[00:39:07] ------------------------------------------
[00:39:07] 
[00:39:07] ------------------------------------------
[00:39:07] stderr:
[00:39:07] stderr:
[00:39:07] ------------------------------------------
[00:39:07] {"message":"program clause dump","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/chalkify/lower_trait.rs","byte_start":493,"byte_end":522,"line_start":13,"line_end":13,"column_start":1,"column_end":30,"is_primary":true,"text":[{"text":"#[rustc_dump_program_clauses] //~ ERROR program clause dump","highlight_start":1,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"FromEnv(S: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(T: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(U: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Implemented(Self: Foo<S, T, U>) :- FromEnv(Self: Foo<S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(S: std::marker::Sized) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(U: std::marker::Sized).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(T: std::marker::Sized) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(U: std::marker::Sized).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(U: std::marker::Sized) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(U: std::marker::Sized).","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: program clause dump\n  --> /checkout/src/test/ui/chalkify/lower_trait.rs:13:1\n   |\nLL | #[rustc_dump_program_clauses] //~ ERROR program clause dump\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: FromEnv(S: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).\n   = note: FromEnv(T: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).\n   = note: FromEnv(U: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).\n   = note: Implemented(Self: Foo<S, T, U>) :- FromEnv(Self: Foo<S, T, U>).\n   = note: WellFormed(S: std::marker::Sized) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(U: std::marker::Sized).\n   = note: WellFormed(T: std::marker::Sized) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(U: std::marker::Sized).\n   = note: WellFormed(U: std::marker::Sized) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(U: std::marker::Sized).\n\n"}
[00:39:07] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:39:07] 
[00:39:07] -------------------------------------ed(F: std::ops::Fn<(&'a (u8, u16),)>) }, forall<> { ProjectionEq(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) }.
[00:39:07] +    = note: WellFormed(F: std::ops::Fn<(&'a (u8, u16),)>) :- Implemented(F: std::marker::Sized), forall<> { Implemented(F: std::ops::Fn<(&'a (u8, u16),)>) }, forall<> { ProjectionEq(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) }.
[00:39:07] 16 error: aborting due to previous error
[00:39:07] 17 
[00:39:07] 
[00:39:07] 
[00:39:07] 
[00:39:07] The actual stderr differed from the expected stderr.
[00:39:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_trait_higher_rank/lower_trait_higher_rank.stderr
[00:39:07] To update references, rerun the tests and pass the `--bless` flag
[00:39:07] To only update this specific test, also pass `--test-args chalkify/lower_trait_higher_rank.rs`
[00:39:07] error: 1 errors occurred comparing output.
[00:39:07] status: exit code: 101
[00:39:07] status: exit code: 101
[00:39:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/lower_trait_higher_rank.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_trait_higher_rank/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_trait_higher_rank/auxiliary" "-A" "unused"
[00:39:07] ------------------------------------------
[00:39:07] 
[00:39:07] ------------------------------------------
[00:39:07] stderr:
[00:39:07] stderr:
[00:39:07] ------------------------------------------
[00:39:07] {"message":"program clause dump","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/chalkify/lower_trait_higher_rank.rs","byte_start":493,"byte_end":522,"line_start":13,"line_end":13,"column_start":1,"column_end":30,"is_primary":true,"text":[{"text":"#[rustc_dump_program_clauses] //~ ERROR program clause dump","highlight_start":1,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"FromEnv(F: std::marker::Sized) :- FromEnv(Self: Foo<F>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(F: std::ops::Fn<(&'a (u8, u16),)>) :- FromEnv(Self: Foo<F>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Implemented(Self: Foo<F>) :- FromEnv(Self: Foo<F>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"ProjectionEq(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) :- FromEnv(Self: Foo<F>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"ProjectionEq(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) :- Implemented(F: std::marker::Sized), forall<> { Implemented(F: std::ops::Fn<(&'a (u8, u16),)>) }, forall<> { ProjectionEq(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) }.","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(F: std::marker::Sized) :- Implemented(F: std::marker::Sized), forall<> { Implemented(F: std::ops::Fn<(&'a (u8, u16),)>) }, forall<> { ProjectionEq(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) }.","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(F: std::ops::Fn<(&'a (u8, u16),)>) :- Implemented(F: std::marker::Sized), forall<> { Implemented(F: std::ops::Fn<(&'a (u8, u16),)>) }, forall<> { ProjectionEq(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) }.","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: program clause dump\n  --> /checkout/src/test/ui/chalkify/lower_trait_higher_rank.rs:13:1\n   |\nLL | #[rustc_dump_program_clauses] //~ ERROR program clause dump\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: FromEnv(F: std::marker::Sized) :- FromEnv(Self: Foo<F>).\n   = note: FromEnv(F: std::ops::Fn<(&'a (u8, u16),)>) :- FromEnv(Self: Foo<F>).\n   = note: Implemented(Self: Foo<F>) :- FromEnv(Self: Foo<F>).\n   = note: ProjectionEq(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) :- FromEnv(Self: Foo<F>).\n   = note: ProjectionEq(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) :- Implemented(F: std::marker::Sized), forall<> { Implemented(F: std::ops::Fn<(&'a (u8, u16),)>) }, forall<> { ProjectionEq(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) }.\n   = note: WellFormed(F: std::marker::Sized) :- Implemented(F: std::marker::Sized), forall<> { Implemented(F: std::ops::Fn<(&'a (u8, u16),)>) }, forall<> { ProjectionEq(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) }.\n   = note: WellFormed(F: std::ops::Fn<(&'a (u8, u16),)>) :- Implemented(F: std::marker::Sized), forall<> { Implemented(F: std::ops::Fn<(&'a (u8, u16),)>) }, forall<> { ProjectionEq(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) }.\n\n"}
[00:39:07] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:39:07] ------------------------------------------
[00:39:07] 
[00:39:07] thread '[ui] ui/chalkify/lower_trait_higher_rank.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:39:07] 
[00:39:07] 
[00:39:07] ---- [ui] ui/chalkify/lower_trait_where_clause.rs stdout ----
[00:39:07] diff of stderr:
[00:39:07] 
[00:39:07] 10    = note: FromEnv(T: std::marker::Sized) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).
[00:39:07] 11    = note: Implemented(Self: Foo<'a, 'b, S, T, U>) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).
[00:39:07] 12    = note: RegionOutlives('a : 'b) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).
[00:39:07] +    = note: RegionOutlives('a : 'b) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(S: std::fmt::Debug), Implemented(T: std::borrow::Borrow<U>), RegionOutlives('a : 'b), TypeOutlives(U : 'b).
[00:39:07] 13    = note: TypeOutlives(U : 'b) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).
[00:39:07] -    = note: WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), WellFormed(S: std::marker::Sized), WellFormed(T: std::marker::Sized), WellFormed(S: std::fmt::Debug), WellFormed(T: std::borrow::Borrow<U>), RegionOutlives('a : 'b), TypeOutlives(U : 'b).
[00:39:07] +    = note: TypeOutlives(U : 'b) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(S: std::fmt::Debug), Implemented(T: std::borrow::Borrow<U>), RegionOutlives('a : 'b), TypeOutlives(U : 'b).
[00:39:07] +    = note: WellFormed(S: std::fmt::Debug) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(S: std::fmt::Debug), Implemented(T: std::borrow::Borrow<U>), RegionOutlives('a : 'b), TypeOutlives(U : 'b).
[00:39:07] +    = note: WellFormed(S: std::marker::Sized) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(S: std::fmt::Debug), Implemented(T: std::borrow::Borrow<U>), RegionOutlives('a : 'b), TypeOutlives(U : 'b).
[00:39:07] +    = note: WellFormed(T: std::borrow::Borrow<U>) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(S: std::fmt::Debug), Implemented(T: std::borrow::Borrow<U>), RegionOutlives('a : 'b), TypeOutlives(U : 'b).
[00:39:07] +    = note: WellFormed(T: std::marker::Sized) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(S: std::fmt::Debug), Implemented(T: std::borrow::Borrow<U>), RegionOutlives('a : 'b), TypeOutlives(U : 'b).
[00:39:07] 16 error: aborting due to previous error
[00:39:07] 17 
[00:39:07] 
[00:39:07] 
[00:39:07] 
[00:39:07] The actual stderr differed from the expected stderr.
[00:39:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_trait_where_clause/lower_trait_where_clause.stderr
[00:39:07] To update references, rerun the tests and pass the `--bless` flag
[00:39:07] To only update this specific test, also pass `--test-args chalkify/lower_trait_where_clause.rs`
[00:39:07] error: 1 errors occurred comparing output.
[00:39:07] status: exit code: 101
[00:39:07] status: exit code: 101
[00:39:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/lower_trait_where_clause.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_trait_where_clause/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_trait_where_clause/auxiliary" "-A" "unused"
[00:39:07] ------------------------------------------
[00:39:07] 
[00:39:07] ------------------------------------------
[00:39:07] stderr:
[00:39:07] stderr:
[00:39:07] ------------------------------------------
[00:39:07] {"message":"program clause dump","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/chalkify/lower_trait_where_clause.rs","byte_start":551,"byte_end":580,"line_start":16,"line_end":16,"column_start":1,"column_end":30,"is_primary":true,"text":[{"text":"#[rustc_dump_program_clauses] //~ ERROR program clause dump","highlight_start":1,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"FromEnv(S: std::fmt::Debug) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(S: std::marker::Sized) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(T: std::borrow::Borrow<U>) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(T: std::marker::Sized) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Implemented(Self: Foo<'a, 'b, S, T, U>) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"RegionOutlives('a : 'b) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"RegionOutlives('a : 'b) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(S: std::fmt::Debug), Implemented(T: std::borrow::Borrow<U>), RegionOutlives('a : 'b), TypeOutlives(U : 'b).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"TypeOutlives(U : 'b) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"TypeOutlives(U : 'b) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(S: std::fmt::Debug), Implemented(T: std::borrow::Borrow<U>), RegionOutlives('a : 'b), TypeOutlives(U : 'b).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(S: std::fmt::Debug) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(S: std::fmt::Debug), I  = note: Implemented(Self: Foo<'a, 'b, S, T, U>) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).\n   = note: RegionOutlives('a : 'b) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).\n   = note: RegionOutlives('a : 'b) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(S: std::fmt::Debug), Implemented(T: std::borrow::Borrow<U>), RegionOutlives('a : 'b), TypeOutlives(U : 'b).\n   = note: TypeOutlives(U : 'b) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).\n   = note: TypeOutlives(U : 'b) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(S: std::fmt::Debug), Implemented(T: std::borrow::Borrow<U>), RegionOutlives('a : 'b), TypeOutlives(U : 'b).\n   = note: WellFormed(S: std::fmt::Debug) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(S: std::fmt::Debug), Implemented(T: std::borrow::Borrow<U>), RegionOutlives('a : 'b), TypeOutlives(U : 'b).\n   = note: WellFormed(S: std::marker::Sized) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(S: std::fmt::Debug), Implemented(T: std::borrow::Borrow<U>), RegionOutlives('a : 'b), TypeOutlives(U : 'b).\n   = note: WellFormed(T: std::borrow::Borrow<U>) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(S: std::fmt::Debug), Implemented(T: std::borrow::Borrow<U>), RegionOutlives('a : 'b), TypeOutlives(U : 'b).\n   = note: WellFormed(T: std::marker::Sized) :- Implemented(S: std::marker::Sized), Implemented(T: std::marker::Sized), Implemented(S: std::fmt::Debug), Implemented(T: std::borrow::Borrow<U>), RegionOutlives('a : 'b), TypeOutlives(U : 'b).\n\n"}
[00:39:07] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:39:07] ------------------------------------------
[00:39:07] 
[00:39:07] thread '[ui] ui/chalkify/lower_trait_where_clause.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:39:07] 
---
[00:39:07] test result: FAILED. 1539 passed; 4 failed; 5 ignored; 0 measured; 0 filtered out
[00:39:07] 
[00:39:07] 
[00:39:07] 
[00:39:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:39:07] 
[00:39:07] 
[00:39:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:39:07] Build completed unsuccessfully in 0:01:50
[00:39:07] Build completed unsuccessfully in 0:01:50
[00:39:07] make: *** [check] Error 1
[00:39:07] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02c607b8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
