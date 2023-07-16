plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:43:20] 
[00:43:20] running 1380 tests
[00:43:25] ...........................................................F.FFF..................i.................
[00:43:34] ....................................................................................................
[00:43:38] ....................................................................................................
[00:43:41] ....................................................................................................
[00:43:45] ....................................................................................................
[00:43:45] ....................................................................................................
[00:43:51] ....................................................................................................
[00:43:57] ....................................................................................................
[00:44:02] ....................................................................................................
[00:44:09] .....................................i..............................................................
[00:44:14] .............i......................................................................................
[00:44:20] ...............................ii...................................................................
[00:44:27] ....................................................................................................
lemented(Self: Bar) :- FromEnv(Self: Bar).
[00:44:32] 20    = note: Implemented(Self: Foo) :- FromEnv(Self: Foo).
[00:44:32] 21    = note: Implemented(Self: std::marker::Sized) :- FromEnv(Self: std::marker::Sized).
[00:44:32] +    = note: WellFormed(Self: Bar) :- Implemented(Self: Bar), WellFormed(Self: Bar).
[00:44:32] +    = note: WellFormed(Self: Bar) :- Implemented(Self: Bar), WellFormed(Self: Foo).
[00:44:32] 23 error: aborting due to 2 previous errors
[00:44:32] 24 
[00:44:32] 
[00:44:32] 
[00:44:32] 
[00:44:32] The actual stderr differed from the expected stderr.
[00:44:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_env1.stderr
[00:44:32] To update references, run this command from build directory:
[00:44:32] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'chalkify/lower_env1.rs'
[00:44:32] error: 1 errors occurred comparing output.
[00:44:32] status: exit code: 101
[00:44:32] status: exit code: 101
[00:44:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/lower_env1.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_env1.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_env1.stage2-x86_64-unknown-linux-gnu.aux" "-Ao) :- FromEnv(Self: Bar).\n   = note: Implemented(Self: Bar) :- FromEnv(Self: Bar).\n   = note: WellFormed(Self: Bar) :- Implemented(Self: Bar), WellFormed(Self: Bar).\n   = note: WellFormed(Self: Bar) :- Implemented(Self: Bar), WellFormed(Self: Foo).\n\n"}
[00:44:32] {"message":"program clause dump","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/chalkify/lower_env1.rs","byte_start":620,"byte_end":653,"line_start":19,"line_end":19,"column_start":1,"column_end":34,"is_primary":true,"text":[{"text":"#[rustc_dump_env_program_clauses] //~ ERROR program clause dump","highlight_start":1,"highlight_end":34}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"FromEnv(Self: Bar) :- FromEnv(Self: Bar).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(Self: Foo) :- FromEnv(Self: Bar).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Implemented(Self: Bar) :- FromEnv(Self: Bar).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Implemented(Self: Foo) :- FromEnv(Self: Foo).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Implemented(Self: std::marker::Sized) :- FromEnv(Self: std::marker::Sized).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Bar) :- Implemented(Self: Bar), WellFormed(Self: Bar).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Bar) :- Implemented(Self: Bar), WellFormed(Self: Foo).","code":null,"level":"note","spans":[],"chil----------------------
[00:44:32] ------------------------------------------
[00:44:32] ------------------------------------------
[00:44:32] {"message":"program clause dump","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/chalkify/lower_trait.rs","byte_start":493,"byte_end":522,"line_start":13,"line_end":13,"column_start":1,"column_end":30,"is_primary":true,"text":[{"text":"#[rustc_dump_program_clauses] //~ ERROR program clause dump","highlight_start":1,"highlight_end":30}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"FromEnv(S: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(T: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(U: std::marker::Sized) :- FromEnv(Self: Foo<S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Implemented(Self: Foo<S, T, U>) :- FromEnv(Self: Foo<S, T, U>).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo<S, T, U>) :- Implemented(Self: Foo<S, T, U>), WellFormed(S: std::marker::Sized).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo<S, T, U>) :- Implemented(Self: Foo<S, T, U>), WellFormed(T: std::marker::Sized).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo<S, T, U>) :- Implemented(Self: Foo<S, T, U>), WellFormed(U: std::marker::Sized).","code":null,"level":"note","spans":[],"chiltput == &'a u8) }.","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo<F>) :- Implemented(Self: Foo<F>), forall<> { WellFormed(F: std::ops::Fn<(&'a (u8, u16),)>) }.","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: program clause dump\n  --> /checkout/src/test/ui/chalkify/lower_trait_higher_rank.rs:13:1\n   |\nLL | #[rustc_dump_program_clauses] //~ ERROR program clause dump\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: FromEnv(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) :- FromEnv(Self: Foo<F>).\n   = note: FromEnv(F: std::marker::Sized) :- FromEnv(Self: Foo<F>).\n   = note: FromEnv(F: std::ops::Fn<(&'a (u8, u16),)>) :- FromEnv(Self: Foo<F>).\n   = note: Implemented(Self: Foo<F>) :- FromEnv(Self: Foo<F>).\n   = note: WellFormed(Self: Foo<F>) :- Implemented(Self: Foo<F>), WellFormed(F: std::marker::Sized).\n   = note: WellFormed(Self: Foo<F>) :- Implemented(Self: Foo<F>), forall<> { WellFormed(<F as std::ops::FnOnce<(&'a (u8, u16),)>>::Output == &'a u8) }.\n   = note: WellFormed(Self: Foo<F>) :- Implemented(Self: Foo<F>), forall<> { WellFormed(F: std::ops::Fn<(&'a (u8, u16),)>) }.\n\n"}
[00:44:32] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:44:32] ------------------------------------------
[00:44:32] 
[00:44:32] thread '[ui] ui/chalkify/lower_trait_higher_rank.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[00:44:32] 
[00:44:32] 
[00:44:32] ---- [ui] ui/chalkify/lower_trait_where_clause.rs stdout ----
[00:44:32]  diff of stderr:
[00:44:32] 
[00:44:32] 11    = note: Implemented(Self: Foo<'a, 'b, S, T, U>) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).
[00:44:32] 12    = note: RegionOutlives('a : 'b) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).
[00:44:32] 13    = note: TypeOutlives(U : 'b) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).
[00:44:32] +    = note: WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), RegionOutlives('a : 'b).
[00:44:32] +    = note: WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), TypeOutlives(U : 'b).
[00:44:32] +    = note: WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), WellFormed(S: std::fmt::Debug).
[00:44:32] +    = note: WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), WellFormed(S: std::marker::Sized).
[00:44:32] +    = note: WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), WellFormed(T: std::borrow::Borrow<U>).
[00:44:32] +    = note: WellFormed(Self: Foo<'a, 'b, S, T, U>) :- Implemented(Self: Foo<'a, 'b, S, T, U>), WellFormed(T: std::marker::Sized).
[00:44:32] 15 error: aborting due to previous error
[00:44:32] 16 
[00:44:32] 
[00:44:32] 
[00:44:32] 
[00:44:32] The actual stderr differed from the expected stderr.
[00:44:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_trait_where_clause.stderr
[00:44:32] To update references, run this command from build directory:
[00:44:32] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'chalkify/lower_trait_where_clause.rs'
[00:44:32] 
[00:44:32] error: 1 errors occurred comparing output.
[00:44:32] status: exit code: 101
[00:44:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/lower_trait_where_clause.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_trait_where_clause.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_trait_where_clause.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:44:32] ------------------------------------------
[00:44:32] 
[00:44:32] ------------------------------------------
[00:44:32] stderr:
[00:44:32] stderr:
[00:44:32] ------------------------------------------
[00:44:32] {"message":"program clause dump","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/chalkify/lower_trait_where_clause.rs","byte_start":551,"byte_end":580,"line_start":16,"line_end":16,"column_start":1,"column_end":30,"is_primary":true,"text":[{"text":"#[rustc_dump_program_clauses] //~ ERROR program clause dump","highlight_start":1,"highlight_end":30}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"FromEnv(S: std::fmt::Debug) :- FromEnv(Self: Foo<'a, 'b, S, T, U>).","code":null,"level":"note","spaead https://google.com | grep ^Date: | sed 's/Date: //g' || true)
travis_fold:start:after_failure.1
travis_time:start:0173fd24
