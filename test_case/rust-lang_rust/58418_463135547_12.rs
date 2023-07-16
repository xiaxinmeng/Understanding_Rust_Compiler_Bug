\n\nwhere `P1, ..., Pm` are the type parameters of the `impl` and `T0, ..., Tn`\nare types. One of the types `T0, ..., Tn` must be a local type (this is another\norphan rule, see the explanation for E0117). Let `i` be the smallest integer\nsuch that `Ti` is a local type. Then no type parameter can appear in any of the\n`Tj` for `j < i`.\n\nFor information on the design of the orphan rules, see [RFC 1023].\n\n[RFC 1023]: https://github.com/rust-lang/rfcs/blob/master/text/1023-rebalancing-coherence.md\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-41974.rs","byte_start":51,"byte_end":80,"line_start":7,"line_end":7,"column_start":1,"column_end":30,"is_primary":true,"text":[{"text":"impl<T> Drop for T where T: A { //~ ERROR E0119","highlight_start":1,"highlight_end":30}],"label":"type parameter `T` must be used as the type parameter for some local type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"only traits defined in the current crate can be implemented for a type parameter","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)\n  --> /checkout/src/test/ui/issues/issue-41974.rs:7:1\n   |\nLL | impl<T> Drop for T where T: A { //~ ERROR E0119\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type parameter `T` must be used as the type parameter for some local type\n   |\n   = note: only traits defined in the current crate can be implemented for a type parameter\n\n"}
[01:01:00] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:01:00] {"message":"Some errors occurred: E0119, E0120, E0210.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0119, E0120, E0210.\n"}
[01:01:00] 
[01:01:00] ------------------------------------------
[01:01:00] 
[01:01:00] thread '[ui] ui/issues/issue-41974.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:01:00] thread '[ui] ui/issues/issue-41974.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:01:00] 
[01:01:00] ---- [ui] ui/nll/ty-outlives/projection-no-regions-closure.rs stdout ----
[01:01:00] diff of stderr:
[01:01:00] 
[01:01:00] 8                '_#1r,
[01:01:00] 9                T,
[01:01:00] 10                i32,
[01:01:00] -                extern "rust-call" fn((std::boxed::Box<T>,)) -> std::boxed::Box<(dyn Anything + '_#2r)>
[01:01:00] +                extern "rust-call" fn((std::boxed::Box<T, std::alloc::Global>,)) -> std::boxed::Box<(dyn Anything + '_#2r), std::alloc::Global>
[01:01:00] 12            ]
[01:01:00] 13    = note: number of external vids: 3
[01:01:00] 14    = note: where <T as std::iter::Iterator>::Item: '_#2r
[01:01:00] 48                '_#1r,
[01:01:00] 49                T,
[01:01:00] 50                i32,
[01:01:00] 50                i32,
[01:01:00] -                extern "rust-call" fn((std::boxed::Box<T>,)) -> std::boxed::Box<(dyn Anything + '_#2r)>
[01:01:00] +                extern "rust-call" fn((std::boxed::Box<T, std::alloc::Global>,)) -> std::boxed::Box<(dyn Anything + '_#2r), std::alloc::Global>
[01:01:00] 52            ]
[01:01:00] 53    = note: number of external vids: 3
[01:01:00] 54    = note: where <T as std::iter::Iterator>::Item: '_#2r
[01:01:00] 80                '_#2r,
[01:01:00] 81                T,
[01:01:00] 82                i32,
[01:01:00] 82                i32,
[01:01:00] -                extern "rust-call" fn((std::boxed::Box<T>,)) -> std::boxed::Box<(dyn Anything + '_#3r)>
[01:01:00] +                extern "rust-call" fn((std::boxed::Box<T, std::alloc::Global>,)) -> std::boxed::Box<(dyn Anything + '_#3r), std::alloc::Global>
[01:01:00] 84            ]
[01:01:00] 85    = note: number of external vids: 4
[01:01:00] 86    = note: where <T as std::iter::Iterator>::Item: '_#3r
[01:01:00] 122                '_#2r,
[01:01:00] 123                T,
[01:01:00] 124                i32,
[01:01:00] 124                i32,
[01:01:00] -                extern "rust-call" fn((std::boxed::Box<T>,)) -> std::boxed::Box<(dyn Anything + '_#3r)>
[01:01:00] +                extern "rust-call" fn((std::boxed::Box<T, std::alloc::Global>,)) -> std::boxed::Box<(dyn Anything + '_#3r), std::alloc::Global>
[01:01:00] 126            ]
[01:01:00] 127    = note: number of external vids: 4
[01:01:00] 128    = note: where <T as std::iter::Iterator>::Item: '_#3r
[01:01:00] 
[01:01:00] The actual stderr differed from the expected stderr.
[01:01:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-no-regions-closure/projection-no-regions-closure.stderr
[01:01:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-no-regions-closure/projection-no-regions-closure.stderr
[01:01:00] To update references, rerun the tests and pass the `--bless` flag
[01:01:00] To only update this specific test, also pass `--test-args nll/ty-outlives/projection-no-regions-closure.rs`
[01:01:00] error: 1 errors occurred comparing output.
[01:01:00] status: exit code: 1
[01:01:00] status: exit code: 1
[01:01:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-no-regions-closure/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-no-regions-closure/auxiliary" "-A" "unused"
[01:01:00] ------------------------------------------
[01:01:00] 
[01:01:00] ------------------------------------------
[01:01:00] stderr:
[01:01:00] stderr:
[01:01:00] ------------------------------------------
[01:01:00] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs","byte_start":571,"byte_end":597,"line_start":25,"line_end":25,"column_start":23,"column_end":49,"is_primary":true,"text":[{"text":"    with_signature(x, |mut y| Box::new(y.next()))","highlight_start":23,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:15 ~ projection_no_regions_closure[317d]::no_region[0]::{{closure}}[0]) with closure substs [\n    '_#1r,\n    T,\n    i32,\n    extern \"rust-call\" fn((std::boxed::Box<T, std::alloc::Global>,)) -> std::boxed::Box<(dyn Anything + '_#2r), std::alloc::Global>\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 3","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where <T as std::iter::Iterator>::Item: '_#2r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs:25:23\n   |\nLL |     with_signature(x, |mut y| Box::new(y.next()))\n   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: defining type: DefId(0/1:15 ~ projection_no_regions_closure[317d]::no_region[0]::{{closure}}[0]) with closure substs [\n               '_#1r,\n               T,\n               i32,\n               extern \"rust-call\" fn((std::boxed::Box<T, std::alloc::Global>,)) -> std::boxed::Box<(dyn Anything + '_#2r), std::alloc::Global>\n           ]\n   = note: number of external vids: 3\n   = note: where <T as std::iter::Iterator>::Item: '_#2r\n\n"}
[01:01:00] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs","byte_start":467,"byte_end":695,"line_start":21,"line_end":27,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn no_region<'a, T>(x: Box<T>) -> Box<dyn Anything + 'a>","highlight_start":1,"highlight_end":57},{"text":"where","highlight_start":1,"highlight_end":6},{"text":"    T: Iterator,","highlight_start":1,"highlight_end":17},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"    with_signature(x, |mut y| Box::new(y.next()))","highlight_start":1,"highlight_end":50},{"text":"    //~^ ERROR the associated type `<T as std::iter::Iterator>::Item` may not live long enough","highlight_start":1,"highlight_end":95},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:6 ~ projection_no_regions_closure[317d]::no_region[0]) with substs [\n    '_#1r,\n    T\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs:21:1\n   |\nLL | / fn no_region<'a, T>(x: Box<T>) -> Box<dyn Anything + 'a>\nLL | | where\nLL | |     T: Iterator,\nLL | | {\nLL | |     with_signature(x, |mut y| Box::new(y.next()))\nLL | |     //~^ ERROR the associated type `<T as std::iter::Iterator>::Item` may not live long enough\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:6 ~ projection_no_regions_closure[317d]::no_region[0]) with substs [\n               '_#1r,\n               T\n           ]\n\n"}
[01:01:00] {"message":"the associated type `<T as std::iter::Iterator>::Item` may not live long enough","code":{"code":"E0309","explanation":"\nThe type definition contains some field whose type\nrequires an outlives annotation. Outlives annotations\n(e.g., `T: 'a`) are used to guarantee that all the data in T is valid\nfor at least the lifetime `'a`. This scenario most commonly\narises when the type contains an associated type reference\nlike `<T as SomeTrait<'a>>::Output`, as shown in this example:\n\n