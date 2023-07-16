plain
[00:44:30] ....................................................................................................
[00:44:33] ....................................................................................................
[00:44:36] ....................................................................................................
[00:44:40] ....................................................................................................
[00:44:44] .....................................................................F.............................F
[00:44:53] ....................................................................................................
[00:44:53] ....................................................................................................
[00:44:58] ............................................................iF......................................
[00:45:03] .........................F.....F.....i..............................................................
[00:45:12] ....................................................................................................
[00:45:19] ..........................................................i.........................................
this error, try `rustc --explain E0308`.\n"}
[00:45:20] 
[00:45:20] 
[00:45:20] ------------------------------------------
[00:45:20] 
[00:45:20] thread '[ui] ui/mismatched_types/trait-bounds-cant-coerce.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3042:9
[00:45:20] 
[00:45:20] ---- [ui] ui/nll/ty-outlives/projection-no-regions-closure.rs stdout ----
[00:45:20] diff of stderr:
[00:45:20] 
[00:45:20] 92                '_#2r,
[00:45:20] 93                T,
[00:45:20] 94                i32,
[00:45:20] -                extern "rust-call" fn((std::boxed::Box<T>,)) -> std::boxed::Box<Anything + '_#3r>
[00:45:20] +                extern "rust-call" fn((std::boxed::Box<T, std::heap::Global>,)) -> std::boxed::Box<Anything + '_#3r, std::heap::Global>
[00:45:20] 96            ]
[00:45:20] 97    = note: number of external vids: 4
[00:45:20] 98    = note: where <T as std::iter::Iterator>::Item: '_#3r
[00:45:20] 134                '_#2r,
[00:45:20] 135                T,
[00:45:20] 136                i32,
[00:45:20] 136                i32,
[00:45:20] -                extern "rust-call" fn((std::boxed::Box<T>,)) -> std::boxed::Box<Anything + '_#3r>
[00:45:20] +                extern "rust-call" fn((std::boxed::Box<T, std::heap::Global>,)) -> std::boxed::Box<Anything + '_#3r, std::heap::Global>
[00:45:20] 138            ]
[00:45:20] 139    = note: number of external vids: 4
[00:45:20] 140    = note: where <T as std::iter::Iterator>::Item: '_#3r
[00:45:20] 
[00:45:20] The actual stderr differed from the expected stderr.
[00:45:20] The actual stderr differed from the expected stderr.
[00:45:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-oue: DefId(0/1:26 ~ projection_no_regions_closure[317d]::outlives_region[0]::{{closure}}[0]) with closure substs [\n    '_#1r,\n    '_#2r,\n    T,\n    i32,\n    extern \"rust-call\" fn((std::boxed::Box<T, std::heap::Global>,)) -> std::boxed::Box<Anything + '_#3r, std::heap::Global>\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 4","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where <T as std::iter::Iterator>::Item: '_#3r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs:64:23\n   |\nLL |     with_signature(x, |mut y| Box::new(y.next()))\n   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: defining type: DefId(0/1:26 ~ projection_no_regions_closure[317d]::outlives_region[0]::{{closure}}[0]) with closure substs [\n               '_#1r,\n               '_#2r,\n               T,\n               i32,\n               extern \"rust-call\" fn((std::boxed::Box<T, std::heap::Global>,)) -> std::boxed::Box<Anything + '_#3r, std::heap::Global>\n           ]\n   = note: number of external vids: 4\n   = note: where <T as std::iter::Iterator>::Item: '_#3r\n\n"}
[00:45:20] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs","byte_start":1712,"byte_end":1872,"line_start":59,"line_end":65,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn outlives_region<'a, 'b, T>(x:Iterator<Item=&T> + 'static, _>
[00:45:20] +               found std::boxed::Box<std::iter::Iterator<Item=&T>, _>
[00:45:20] 25 error: aborting due to previous error
[00:45:20] 26 
[00:45:20] 
[00:45:20] 
[00:45:20] 
[00:45:20] The actual stderr differed from the expected stderr.
[00:45:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/dyn-trait-underscore/dyn-trait-underscore.stderr
[00:45:20] To update references, rerun the tests and pass the `--bless` flag
[00:45:20] To only update this specific test, also pass `--test-args underscore-lifetime/dyn-trait-underscore.rs`
[00:45:20] error: 1 errors occurred comparing output.
[00:45:20] status: exit code: 101
[00:45:20] status: exit code: 101
[00:45:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/dyn-trait-underscore.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/dyn-trait-underscore/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/dyn-trait-underscore/auxiliary" "-A" "unused"
[00:45:20] ------------------------------------------
[00:45:20] 
[00:45:20] ------------------------------------------
[00:45:20] stderr:
[00:45:20] stderr:
[00:45:20] ------------------------------------------
[00:45:20] {"message":"cannot infer an appropriate lifetime for autoref due to conflicting requ for the static lifetime...\n   = note: ...so that the expression is assignable:\n           expected std::boxed::Box<std::iter::Iterator<Item=&T> + 'static, _>\n              found std::boxed::Box<std::iter::Iterator<Item=&T>, _>\n\n"}
[00:45:20] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:20] {"message":"For more information about this error, try `rustc --explain E0495`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0495`.\n"}
[00:45:20] ------------------------------------------
[00:45:20] 
[00:45:20] thread '[ui] ui/underscore-lifetime/dyn-trait-underscore.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3042:9
[00:45:20] 
