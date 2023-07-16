plain
[00:49:14] ....................................................................................................
[00:49:17] .....................................................i..............................................
[00:49:19] ....................................................................................................
[00:49:22] ....................................................................................................
[00:49:25] ..iiiiiiiii.........................................................................................
[00:49:31] ....................................................................................................
[00:49:34] ......................................................................................i.............
[00:49:37] ....................................................................................................
[00:49:40] .........................................i.i..ii....................................................
---
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:50:21] 
[00:50:21] running 3088 tests
[00:50:31] ............F...F.........F..............F.....F..F....F.....................................F......
[00:50:42] ........F.FF..F.......F...FFFF..FF.F.F....F....FF......................F.....i..........F...........
[00:50:49] ....F..F..F...F....FF..F.F..F.F.FFF..F............FF........FFF..F..............F.F.................
[00:50:58] FF..F..F.F.FF.F....F..F.F.........................................F......FF...............F.F..F....
[00:51:06] ..F...FFFFFFF.....F..........F..F.F.F..........FFFFFFFFFFF...F..F..F...FF....F.FFF..FFF.F...F.......
[00:51:16] ............F...F....F..F.FF.FF..FF..........................FF.F................FFF.FFFFFFF........
[00:51:30] .....FFFF..F...F...FF.F.F.FF...................................................F.....F..F..F........
[00:51:40] .F..........F...FF.F........F...F.....F.............FF...F.F...F...FF.......F...F............FF.....
[00:51:48] ..FF.....F...FF....F.......F..F......F.F.....FF.........F......F.F..F..F....FF....FFFF.F..F..FF.F...
[00:52:01] F......................F.............FF.F.F...........................F......FFF.FF.......F...FF..FF
[00:52:11] .F.............FFF.F......F...F..F.....FFF...F..F.F....FFFF.F....F.....F....F.F...FF....F.......FF.F
[00:52:19] ....FF..F.F.F....F.F...F.FF...F.F...F..F.F.F.FF...F.....FFF..FF.FFFF..FF...F..F.F..F......F...F.....
[00:52:26] .F.FF..FF.FF.F...FF..F..FFFF.......FF.FFFF.FF.F..F..FF.....FFF.FF....F..FFFFF.F.F...F....F.F.....FF.
[00:52:34] FF.F.FF......F.F........F..F..FF...F.......F.....FFF....F.FFFF.F..F......FF.....F.FFF...FF.....FF.FF
[00:52:45] ...FF.........FF.F..F.F.FF...F..F..F..FF....F.F..FF.....F...FF.FF.F...FF.F...F....F.F....F....F.F.FF
[00:52:55] .......FF....iFFFFF.............F.F.F..F.F...F..F...F.F.F.....FFF...FF..FFF.....F..........F...FF..F
[00:53:04] .F.F..F.....F.FFF...F.F..F.F..F...FFF..FF.F.FFF..F.F.FFF.FF........FF.FF.F..FF....F.....FFF....F.F.F
[00:53:15] .F..F..FF..F........F......FF..FFFFFFFi.F..F.FFFF.F........F...F..F...F...F..FFF..FF.FFFFFF...F.FFF.
[00:53:27] ..FF.........FFF.FFF..F.F.FF.F.FF..F..FFFF.F....FF.FFFFF.F.F.FF.FF..FF.F.............F......F.......
[00:53:38] ..........................................F.F....F....F............F..F...F...F................F....
[00:53:47] .....FF.F........FF.F..F.F.F.............F.......F..F..F......F..F.....F.........F....i......FFFFFF.
[00:53:58] ...F..F....................................FF...........i...........................................
[00:54:17] ..................F........F.....F.........F.F.F.FFF...F..F..FF...F.F........F...F.F.FF..F..........
[00:54:25] ................F...FF...FF.FF..FF....F....F.FFF...F..F.F..F..FFF.FFFFFFF...F...F.F..........FF.....
[00:54:39] ...F...F.F....F..ii.FFFF.F.F..F...F..F.............F.F.F..FF.F...F........F..F..F.......i....i..F...
[00:54:51] F............F...F.F..F....F.F.i........F..............FFFF.F.FFF....F.FFFF..F.F..F..FFFFF.FF.......
[00:55:00] ..F....F.F....FF..F.FFF...F..FF......FF..F..F.....F.F..F..F...FF.F.FFFFF....F...F...................
[00:55:28] ...........F.FF..FF...F..F.....FFF.FFFFFFF.FFFFFF...F.F...........F.F..F...F..FF..FFF..F.F.F.FF..F.F
[00:55:36] ..F.F..FFF.FFFF.FFFFFF...F....F....F...F.....F.F....F...F....F..F...................................
[00:55:44] .....FF....F...FF..F.....................F....F...F.F.....FF...FFFFFF...F...F.F.F..FFF...F........FF
/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/array-slice-vec/array_const_index-1/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"constant item is never used: `ARR`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/array-slice-vec/array_const_index-1.rs","byte_start":552,"byte_end":599,"line_start":17,"line_end":17,"column_start":5,"column_end":52,"is_primary":true,"text":[{"text":"    const ARR: [i32; 6] = [42, 43, 44, 45, 46, 47];","highlight_start":5,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: constant item is never used: `ARR`\n  --> /checkout/src/test/run-pass/array-slice-vec/array_const_index-1.rs:17:5\n   |\nLL |     const ARR: [i32; 6] = [42, 43, 44, 45, 46, 47];\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] {"message":"constant item is never used: `IDX`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/array-slice-vec/array_const_index-1.rs","byte_start":604,"byte_end":625,"line_start":18,"line_end":18,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    const IDX: usize = 3;","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: constant item is never used: `IDX`\n  --> /checkout/src/test/run-pass/array-slice-vec/array_const_index-1.rs:18:5\n   |\nLL |     const IDX: usize = 3;\n   |     ^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:55:55] {"message":"constant item is never used: `VAL`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/array-slice-vec/array_const_index-1.rs","byte_start":630,"byte_end":656,"line_start":19,"line_end":19,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    const VAL: i32 = ARR[IDX];","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: constant item is never used: `VAL`\n  --> /checkout/src/test/run-pass/array-slice-vec/array_const_index-1.rs:19:5\n   |\nLL |     const VAL: i32 = ARR[IDX];\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:55:55] {"message":"constant item is never used: `BLUB`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/array-slice-vec/array_const_index-1.rs","byte_start":661,"byte_end":709,"line_start":20,"line_end":20,"column_start":5,"column_end":53,"is_primary":true,"text":[{"text":"    const BLUB: [i32; (ARR[0] - 41) as usize] = [5];","highlight_start":5,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: constant item is never used: `BLUB`\n  --> /checkout/src/test/run-pass/array-slice-vec/array_const_index-1.rs:20:5\n   |\nLL |     const BLUB: [i32; (ARR[0] - 41) as usize] = [5];\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/array-slice-vec/array_const_index-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:55:55] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/array-slice-vec/check-static-mut-slices.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: static item is never used: `EMPTY`
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL | static mut EMPTY: &'static mut [isize] = &mut [];
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/array-slice-vec/check-static-mut-slices/check-static-mut-slices.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args array-slice-vec/check-static-mut-slices.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/array-slice-vec/evec-slice/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"value assigned to `z` is never read","code":{"code":"unused_assignments","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/array-slice-vec/evec-slice.rs","byte_start":545,"byte_end":546,"line_start":15,"line_end":15,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"    let mut z : &[isize] = &[1,2,3,4,5];","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_assignments)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: value assigned to `z` is never read\n  --> /checkout/src/test/run-pass/array-slice-vec/evec-slice.rs:15:13\n   |\nLL |     let mut z : &[isize] = &[1,2,3,4,5];\n   |             ^\n   |\n   = note: #[warn(unused_assignments)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/array-slice-vec/evec-slice.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/array-slicsted_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_index` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/array-slice-vec/slice.rs","byte_start":874,"byte_end":879,"line_start":31,"line_end":31,"column_start":21,"column_end":26,"is_primary":true,"text":[{"text":"    fn index(&self, index: RangeTo<Foo>) -> &Foo {","highlight_start":21,"highlight_end":26}],"label":null,"suggested_replacement":"_index","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `index`\n  --> /checkout/src/test/run-pass/array-slice-vec/slice.rs:31:21\n   |\nLL |     fn index(&self, index: RangeTo<Foo>) -> &Foo {\n   |                     ^^^^^ help: consider using `_index` instead\n\n"}
[00:55:55] {"message":"unused variable: `index`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/array-slice-vec/slice.rs","byte_start":1037,"byte_end":1042,"line_start":38,"line_end":38,"column_start":21,"column_end":26,"is_primary":true,"text":[{"text":"    fn index(&self, index: RangeFrom<Foo>) -> &Foo {","highlight_start":21,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_index` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/array-slice-vec/slice.rs","byte_start":1037,"byte_end":1042,"line_start":38,"line_end":38,"column_start":21,"column_end":26,"is_primary":true,"text":[{"text":"    fn index(&self, index: RangeFrom<Foo>) -> &Foo {","highlight_start":21,"highlight_end":26}],"label":null,"suggested_replacement":"_index","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `index`\n  --> /checkout/src/test/run-pass/array-slice-vec/slice.rs:38:21\n   |\nLL |     fn index(&self, index: RangeFrom<Foo>) -> &Foo {\n   |                     ^^^^^ help: consider using `_index` instead\n\n"}
[00:55:55] {"message":"unused variable: `index`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/array-slice-vec/slice.rs","byte_start":1343,"byte_end":1348,"line_start":52,"line_end":52,"column_start":29,"column_end":34,"is_primary":true,"text":[{"text":"    fn index_mut(&mut self, index: Range<Foo>) -> &mut Foo {","highlight_start":29,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_index` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/array-slice-vec/slice.rs","byte_start":1343,"byte_end":1348,"line_start":52,"line_end":52,"column_start":29,"column_end":34,"is_primary":true,"text":[{"text":"    fn index_mut(&mut self, index: Range<Foo>) -> &mut Foo {","highlight_start":29,"highlight_end":34}],"label":null,"suggested_replacement":"_index","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `index`\n  --> /checkout/src/test/run-pass/array-slice-vec/slice.rs:52:29\n   |\nLL |     fn index_mut(&mut self, index: Range<Foo>) -> &mut Foo {\n   |                             ^^^^^ help: consider using `_index` instead\n\n"}
[00:55:55] {"message":"unused variable: `index`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/array-slice-vec/slice.rs","byte_start":1494,"byte_end":1499,"line_start":58,"line_end":58,"column_start":29,"column_end":34,"is_primary":true,"text":[{"text":"    fn index_mut(&mut self, index: RangeTo<Foo>) -> &mut Foo {","highlight_start":29,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_index` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/array-slice-vec/slice.rs","byte_start":1494,"byte_end":1499,"line_start":58,"line_end":58,"column_start":29,"column_end":34,"is_primary":true,"text":[{"text":"    fn index_mut(&mut self, index: RangeTo<Foo>) -> &mut Foo {","highlight_start":29,"highlight_end":34}],"label":null,"suggested_replacement":"_index","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `index`\n  --> /checkout/src/test/run-pass/array-slice-vec/slice.rs:58:29\n   |\nLL |     fn index_mut(&mut self, index: RangeTo<Foo>) -> &mut Foo {\n   |                             ^^^^^ help: consider using `_index` instead\n\n"}
[00:55:55] {"message":"unused variable: `index`","code":{"code":"unused_variables","explanation":null},"level]   --> $DIR/vec-late-init.rs:15:9
[00:55:55]    |
[00:55:55] LL |     let mut later: Vec<isize> ;
[00:55:55]    |         |
[00:55:55]    |         help: remove this `mut`
[00:55:55]    |
[00:55:55]    = note: #[warn(unused_mut)] on by default
[00:55:55]    = note: #[warn(unused_mut)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/array-slice-vec/vec-late-init/vec-late-init.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args array-slice-vec/vec-late-init.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/array-slice-vec/vec-late-init.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/array-slice-vec/vec-late-init/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/array-slice-vec/vec-late-init/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"variable does not need to be mutable","code":{"code":"unused_mut","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/array-slice-vec/vec-late-init.rs","byte_start":505,"byte_end":514,"line_start":15,"line_end":15,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"    let mut later: Vec<isize> ;","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_mut)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove this `mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/array-slice-vec/vec-late-init.rs","byte_start":505,"byte_end":509,"line_start":15,"line_end":15,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"    let mut later: Vec<isize> ;","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: variable does not need to be mutable\n  --> /checkout/src/test/run-pass/array-slice-vec/vec-late-init.rs:15:9\n   |\nLL |     let mut later: Vec<isize> ;\n   |         ----^^^^^\n   |         |\n   |         help: remove this `mut`\n   |\n   = note: #[warn(unused_mut)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/array-slice-vec/vec-late-init.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/array-slice-vec/vec-macro-with-brackets.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `my_vec`
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     let my_vec = vec![1, 2, 3, 4, 5];
[00:55:55]    |         ^^^^^^ help: consider using `_my_vec` instead
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/array-slice-vec/vec-macro-with-brackets/vec-macro-with-brackets.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args array-slice-vec/vec-macro-with-brackets.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/array-slice-vec/vec-macro-with-brackets.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/array-slice-vec/vec-macro-with-brackets/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/array-slice-vec/vec-macro-with-brackets/auxiliary"
[00:55:55] stdout:
[00:55:55] ---------------------rray-slice-vec/vec-matching-legal-tail-element-borrow/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/array-slice-vec/vec-matching-legal-tail-element-borrow/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused variable: `x`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/array-slice-vec/vec-matching-legal-tail-element-borrow.rs","byte_start":533,"byte_end":534,"line_start":16,"line_end":16,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let x = &[1, 2, 3, 4, 5];","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_x` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/array-slice-vec/vec-matching-legal-tail-element-borrow.rs","byte_start":533,"byte_end":534,"line_start":16,"line_end":16,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let x = &[1, 2, 3, 4, 5];","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"_x","suggestion_applicability":"MachineApplicable","expnu/test/run-pass/associated-types/associated-types-cc/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-cc/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused variable: `b`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-cc.rs","byte_start":686,"byte_end":687,"line_start":21,"line_end":21,"column_start":15,"column_end":16,"is_primary":true,"text":[{"text":"fn foo<B:Bar>(b: B) -> <B as Bar>::T {","highlight_start":15,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_b` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-cc.rs","byte_start":686,"byte_end":687,"line_start":21,"line_end":21,"column_start":15,"column_end":16,"is_primary":true,"text":[{"text":"fn foo<B:Bar>(b: B) -> <B as Bar>::T {","highlight_start":15,"highlight_end":16}],"label":null,"suggested_replacement":"_b","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `b`\n  --> /checkout/src/test/run-pass/associated-types/associated-types-cc.rs:21:15\n   |\nLL | fn foo<B:Bar>(b: B) -> <B as Bar>::T {\n   |               ^ help: consider using `_b` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/associated-types/associated-types-cc.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/associated-types/associated-types-duplicate-binding-in-env-hrtb.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: function is never used: `foo`
[00:55:55]   --> $DIR/associated-types-duplicate-binding-in-env-hrtb.rs:18:1
[00:55:55]    |
[00:55:55] LL | / fn foo<T>(t: T) -> i32
[00:55:55] LL | |     where T : for<'a> Fn(&'a u8) -> i32,
[00:55:55] LL | |           T : for<'b> Fn(&'b u8) -> i32,
[00:55:55] LL | | {
[00:55:55] LL | |     t(&3)
[00:55:55] LL | | }
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-duplicate-binding-in-env-hrtb/associated-types-duplicate-binding-in-env-hrtb.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args associated-types/associated-types-duplicate-binding-in-env-hrtb.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/associated-types/associated-types-duplicate-binding-in-env-hrtb.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-duplicate-binding-in-env-hrtb/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-duplicate-binding-in-env-hrtb/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"function is never used: `foo`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-duplicate-binding-in-env-hrtb.rs","byte_start":667,"byte_end":785,"line_start":18,"line_end":23,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn foo<T>(t: T) -> i32","highlight_start":1,"highlight_end":23},{"text":"    where T : for<'a> Fn(&'a u8) -> i32,","highlight_start":1,"highlight_end":41},{"text":"          T : for<'b> Fn(&'b u8) -> i32,","highlight_start":1,"highlight_end":41},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"    t(&3)","highlight_start":1,"highlight_end":10},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: function is never used: `foo`\n  --> /checkout/src/test/run-pass/associated-types/associated-types-duplicate-binding-in-env-hrtb.rs:18:1\n   |\nLL | / fn foo<T>(t: T) -> i32\nLL | |     where T : for<'a> Fn(&'a u8) -> i32,\nLL | |           T : for<'b> Fn(&'b u8) -> i32,\nLL | | {\nLL | |     t(&3)\nLL | | }\n   | |_^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/associated-types/associated-types-duplicate-binding-in-env-hrtb.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/associated-types/associated-types-duplicate-binding-in-env.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: function is never used: `foo`
[00:55:55]   --> $DIR/associated-types-duplicate-binding-in-env.rs:23:1
[00:55:55]    |
[00:55:55] LL | / fn foo<T>() -> ()
[00:55:55] LL | |     where T : Foo<B=()>, T : Foo<B=()>
[00:55:55] LL | | {
[00:55:55] LL | |     <T as Foo>::get()
[00:55:55] LL | | }
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00-types/associated-types-duplicate-binding-in-env.rs","byte_start":684,"byte_end":766,"line_start":23,"line_end":27,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn foo<T>() -> ()","highlight_start":1,"highlight_end":18},{"text":"    where T : Foo<B=()>, T : Foo<B=()>","highlight_start":1,"highlight_end":39},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"    <T as Foo>::get()","highlight_start":1,"highlight_end":22},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: function is never used: `foo`\n  --> /checkout/src/test/run-pass/associated-types/associated-types-duplicate-binding-in-env.rs:23:1\n   |\nLL | / fn foo<T>() -> ()\nLL | |     where T : Foo<B=()>, T : Foo<B=()>\nLL | | {\nLL | |     <T as Foo>::get()\nLL | | }\n   | |_^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/associated-types/associated-types-duplicate-binding-in-env.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/associated-types/associated-types-impl-redirect.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused imports: `None`, `Some`
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL | use std::option::Option::{None,5] 
[00:55:55]   --> $DIR/associated-types-impl-redirect.rs:54:1
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL | fn test2<A, I1: Iterator<Item=A>, I2: Iterator<Item=I1::Item>>(mut it: I2) {
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-impl-redirect/associated-types-impl-redirect.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args associated-types/associated-types-impl-redirect.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/associated-types/associated-types-impl-redirect.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-impl-redirect/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-impl-redirect/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused imports: `None`, `Some`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-impl-redirect.rs","byte_start":903,"byte_end":907,"line_start":22,"line_end":22,"column_start":27,"column_end":31,"is_primary":true,"text":[{"text":"use std::option::Option::{None, Some, self};","highlight_start":27,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-impl-redirect.rs","byte_start":909,"byte_end":913,"line_start":22,"line_end":22,"column_start":33,"column_end":37,"is_primary":true,"text":[{"text":"use std::option::Option::{None, Some, self};","highlight_start":33,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused imports: `None`, `Some`\n  --> /checkout/src/test/run-pass/associated-types/associated-types-impl-redirect.rs:22:27\n   |\nLL | use std::option::Option::{None, Some, self};\n   |                           ^^^^  ^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[00:55:55] {"message":"variable does not need to be mutable","code":{"code":"unused_mut","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-impl-redirect.rs","byte_start":1583,"byte_end":1589,"line_start":54,"line_end":54,"column_start":64,"column_end":70,"is_primary":true,"text":[{"text":"fn test2<A, I1: Iterator<Item=A>, I2: Iterator<Item=I1::Item>>(mut it: I2) {","highlight_start":64,"highlight_end":70}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_mut)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove this `mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-impl-redirect.rs","byte_start":1583,"byte_end":1587,"line_start":54,"line_end":54,"column_start":64,"column_end":68,"is_primary":true,"text":[{"text":"fn test2<A, I1: Iterator<Item=A>, I2: Iterator<Item=I1::Item>>(mut it: I2) {","highlight_start":64,"highlight_end":68}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: variable does not need to be mutable\n  --> /checkout/src/test/run-pass/associated-types/associated-types-impl-redirect.rs:54:64\n   |\nLL | fn test2<A, I1: Iterator<Item=A>, I2: Iterator<Item=I1::Item>>(mut it: I2) {\n   |                                                                ----^^\n   |                                                                |\n   |                                                                help: remove this `mut`\n   |\n   = note: #[warn(unused_mut)] on by default\n\n"}
[00:55:55] {"message":"struct is never constructed: `ByRef`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-impl-redirect.rs","byte_start":1162,"byte_end":1208,"line_start":38,"line_end":38,"column_start":1,"column_end":47,"is_primary":true,"text":[{"text":"struct ByRef<'a, I: 'a + Iterator>(&'a mut I);","highlight_start":1,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: struct is never constructed: `ByRef`\n  --> /checkout/src/test/run-pass/associated-types/associated-types-impl-redirect.rs:38:1\n   |\nLL | struct ByRef<'a, I: 'a + Iterator>(&'a mut I);\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] {"message":"function is never used: `is_iterator_of`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-impl-redirect.rs","byte_start":1377,"byte_end":1425,"line_start":48,"line_end":48,"column_start":1,"column_end":49,"is_primary":true,"text":[{"text":"fn is_iterator_of<A, I: Iterator<Item=A>>(_: &I) {}","highlight_start":1,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: function is never used: `is_iterator_of`\n  --> /checkout/src/test/run-pass/associated-types/assoiated-types/associated-types-issue-21212.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-issue-21212/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-issue-21212/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused variable: `input`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-issue-21212.rs","byte_start":751,"byte_end":756,"line_start":21,"line_end":21,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"    fn parse(input: <Self as Parser>::Input) {","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_input` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-issue-21212.rs","byte_start":751,"byte_end":756,"line_start":21,"line_end":21,"column_start":14,"column_end":19,null,"expansion":null}],"children":[{"message":"consider using `_x` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-nested-projections.rs","byte_start":1009,"byte_end":1010,"line_start":42,"line_end":42,"column_start":17,"column_end":18,"is_primary":true,"text":[{"text":"fn bar<T, I, X>(x: X) where","highlight_start":17,"highlight_end":18}],"label":null,"suggested_replacement":"_x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `x`\n  --> /checkout/src/test/run-pass/associated-types/associated-types-nested-projections.rs:42:17\n   |\nLL | fn bar<T, I, X>(x: X) where\n   |                 ^ help: consider using `_x` instead\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/associated-types/associated-types-nested-projections.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/associated-types/associated-types-normalize-in-bounds-binding.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `a`
[00:55:55]   --> $DIR/associated-types-normalize-in-bounds-binding.rs:34:21
[00:55:55]    |
[00:55:55] LL |     fn dummy(&self, a: A) -> Self::R { loop { } }
[00:55:55]    |
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-normalize-in-bounds-binding/associated-types-normalize-in-bounds-binding.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args associated-types/associated-types-normalize-in-bounds-binding.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/associated-types/associated-types-normalize-in-bounds-binding.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-normalize-in-bounds-binding/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-normalize-in-bounds-binding/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused variable: `a`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-normalize-in-bounds-binding.rs","byte_start":845,"byte_end":846,"line_start":34,"line_end":34,"column_start":21,"column_end":22,"is_primary":true,"text":[{"text":"    fn dummy(&self, a: A) -> Self::R { loop { } }","highlight_start":21,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_a` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-normalize-in-bounds-binding.rs","byte_start":845,"byte_end":846,"line_start":34,"line_end":34,"column_start":21,"column_end":22,"is_primary":true,"text":[{"text":"    fn dummy(&self, a: A) -> Self::R { loop { } }","highlight_start":21,"highlight_end":22}],"label":null,"suggested_replacement":"_a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `a`\n  --> /checkout/src/test/run-pass/associated-types/associated-types-normalize-in-bounds-binding.rs:34:21\n   |\nLL |     fn dummy(&self, a: A) -> Self::R { loop { } }\n   |                     ^ help: consider using `_a` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/associated-types/associated-types-normalize-in-bounds-binding.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/associated-types/associated-types-normalize-in-bounds-ufcs.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `pred`
[00:55:55]   --> $DIR/associated-types-normalize-in-bounds-ufcs.rs:34:25
[00:55:55]    |
[00:55:55] LL |     fn split2<P>(&self, pred: P) -> Splits<T, P> where P: FnMut(&T) -> bool {
[00:55:55]    |                         ^^^^ help: consider using `_pred` instead
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] warning: unused variable: `n`
[00:55:55]   --> $DIR/associated-types-normalize-in-bounds-ufcs.rs:38:26
[00:55:55]   --> $DIR/associated-types-normalize-in-bounds-ufcs.rs:38:26
[00:55:55]    |
[00:55:55] LL |     fn splitn2<P>(&self, n: u32, pred: P) -> SplitsN<Splits<T, P>> where P: FnMut(&T) -> bool {
[00:55:55]    |                          ^ help: consider using `_n` instead
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-normalize-in-bounds-ufcs/associated-types-normalize-in-bounds-ufcs.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args associated-types/associated-types-normalize-in-bounds-ufcs.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/associated-types/associated-types-normalize-in-bounds-ufcs.rs" "--targetlumn_start":25,"column_end":29,"is_primary":true,"text":[{"text":"    fn split2<P>(&self, pred: P) -> Splits<T, P> where P: FnMut(&T) -> bool {","highlight_start":25,"highlight_end":29}],"label":null,"suggested_replacement":"_pred","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `pred`\n  --> /checkout/src/test/run-pass/associated-types/associated-types-normalize-in-bounds-ufcs.rs:34:25\n   |\nLL |     fn split2<P>(&self, pred: P) -> Splits<T, P> where P: FnMut(&T) -> bool {\n   |                         ^^^^ help: consider using `_pred` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:55:55] {"message":"unused variable: `n`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-normalize-in-bounds-ufcs.rs","byte_start":1222,"byte_end":1223,"line_start":38,"line_end":38,"column_start":26,"column_end":27,"is_primary":true,"text":[{"text":"    fn splitn2<P>(&self, n: u32, pred: P) -> SplitsN<Splits<T, P>> where P: FnMut(&T) -> bool {","highlight_start":26,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_n` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-normalize-in-bounds-ufcs.rs","byte_start":1222,"byte_end":1223,"line_start":38,"line_end":38,"column_start":26,"column_end":27,"is_primary":true,"text":[{"text":"    fn splitn2<P>(&self,                   ^ help: consider using `_n` instead
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-normalize-in-bounds/associated-types-normalize-in-bounds.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args associated-types/associated-types-normalize-in-bounds.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/associated-types/associated-types-normalize-in-bounds.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-normalize-in-bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-normalize-in-bounds/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused variable: `pred`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-normalize-in-bounds.rs","byte_start":1120,"byte_end":1124,"line_start":34,"line_end":34,"column_start":25,"column_end":29,"is_primary":true,"text":[{"text":"    fn split2<P>(&self, pred: P) -> Splits<T, P> where P: FnMut(&T) -> bool {","highlight_start":25,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_pred` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-normalize-in-bounds.rs","byte_start":1120,"byte_end":1124,"line_start":34,"line_end":34,"column_start":25,"column_end":29,"is_primary":true,"text":[{"text":"    fn split2<P>(&self, pred: P) -> Splits<T, P> where P: FnMut(&T) -> bool {","highlight_start":25,"highlight_end":29}],"label":null,"suggested_replacement":"_pred","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `pred`\n  --> /checkout/src/test/run-pass/associated-types/associated-types-normalize-in-bounds.rs:34:25\n   |\nLL |     fn split2<P>(&self, pred: P) -> Splits<T, P> where P: FnMut(&T) -> bool {\n   |                         ^^^^ help: consider using `_pred` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:55:55] {"message":"unused variable: `n`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-normalize-in-bounds.rs","byte_start":1222,"byte_end":1223,"line_start":38,"line_end":38,"column_start":26,"column_end":27,"is_primary":true,"text":[{"text":"    fn splitn2<P>(&self, n: usize, pred: P) -> SplitsN<Splits<T, P>> where P: FnMut(&T) -> bool {","highlight_start":26,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_n` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-normalize-in-bounds.rs","byte_start":1222,"byte_end":1223,"line_start":38,"line_end":38,"column_start":26,"column_end":27,"is_primary":true,"text":[{"text":"    fn splitn2<P>(&self, n: usize, pred: P) -> SplitsN<Splits<T, P>> where P: FnMut(&T) -> bool {","highlight_start":26,"highlight_end":27}],"label":null,"suggested_replacement":"_n","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `n`\n  --> /checkout/src/test/run-pass/associated-types/associated-types-normalize-in-bounds.rs:38:26\n   |\nLL |     fn splitn2<P>(&self, n: usize, pred: P) -> SplitsN<Splits<T, P>> where P: FnMut(&T) -> bool {\n   |                          ^ help: consider using `_n` instead\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/associated-types/associated-types-normalize-in-bounds.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/associated-types/associated-types-projection-bound-in-supertraits.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `x`
[00:55:55]   --> $DIR/associated-types-projection-bound-in-supertraits.rs:27:13
[00:55:55]    |
[00:55:55] LL |         let x: Self = self.not();
[00:55:55]    |             ^ help: consider using `_x` instead
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-projection-bound-in-supertraits/associated-types-projection-bound-in-supertraits.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args associated-types/associated-types-projection-bound-in-supertraits.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/associated-types/associated-types-projection-bound-in-supertraits.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-projection-bound-in-supertraits/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-projection-bound-in-supertraits/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused variable: `x`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-projection-bound-in-supertraits.rs","byte_start":927,"byte_end":928,"line_start":27,"line_end":27,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"        let x: Self = self.not();","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_x` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-projection-bound-in-supertraits.rs","byte_start":927,"byte_end":928,"line_start":27,"line_end":27,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"        let x: Self = self.not();","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":"_x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `x`\n  --> /checkout/src/test/run-pass/associated-types/associated-types-projection-bound-in-supertraits.rs:27:13\n   |\nLL |         let x: Self = self.not();\n   |             ^ help: consider using `_x` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/associated-types/associated-types-projection-bound-in-supertraits.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/associated-types/associated-types-projection-in-object-type.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused imports: `Shl`, `Shr`
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL | use std::ops::{Shl, Shr};
[00:55:55]    |
[00:55:55]    = note: #[warn(unused_imports)] on by default
[00:55:55] 
[00:55:55] warning: unused import: `std::cell::RefCell`
[00:55:55] warning: unused import: `std::cell::RefCell`
[00:55:55]   --> $DIR/associated-types-projection-in-object-type.rs:20:5
[00:55:55]    |
[00:55:55] LL | use std::cell::RefCell;
[00:55:55] 
[00:55:55] 
[00:55:55] warning: struct is never constructed: `MyStruct`
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL | struct MyStruct<'a> {
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-projection-in-object-type/associated-types-projection-in-object-type.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args associated-types/associated-types-projection-in-object-type.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/associated-types/associated-types-projection-in-object-type.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-projection-in-object-type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-projection-in-object-type/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused imports: `Shl`, `Shr`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-projection-in-object-type.rs","byte_start":767,"byte_end":770,"line_start":19,"line_end":19,"column_start":16,"column_end":19,"is_primary":true,"text":[{"text":"use std::ops::{Shl, Shr};","highlight_start":16,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-projection-in-object-type.rs","byte_start":772,"byte_end":775,"line_start":19,"line_end":19,"column_start":21,"column_end":24,"is_primary":true,"text":[{"text":"use std::ops::{Shl, Shr};","highlight_start":21,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused imports: `Shl`, `Shr`\n  --> /checkout/src/test/run-pass/associated-types/associated-types-projection-in-object-type.rs:19:16\n   |\nLL | use std::ops::{Shl, Shr};\n   |                ^^^  ^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[00:55:55] {"message":"unused import: `std::cell::RefCell`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-projection-in-object-type.rs","byte_start":782,"byte_end":800,"line_start":20,"line_end":20,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"use std::cell::RefCell;","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: u5:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/associated-types/associated-types-projection-in-where-clause.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-projection-in-where-clause/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-projection-in-where-clause/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused variable: `t`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-projection-in-where-clause.rs","byte_start":703,"byte_end":704,"line_start":28,"line_end":28,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"fn foo<I:Int<T=J>,J>(t: I) -> bool","highlight_start":22,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_t` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ru5:55] ------------------------------------------
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] ------------------------------------------
[00:55:55] {"message":"struct is never constructed: `X`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/associated-types/associated-types-projection-in-supertrait.rs","byte_start":745,"byte_end":754,"line_start":30,"line_end":30,"column_start":1,"column_end":10,"is_primary":true,"text":[{"text":"struct X;","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: struct is never constructed: `X`\n  --> /checkout/src/test/run-pass/associated-types/associated-types-projection-in-supertrait.rs:30:1\n   |\nLL | struct X;\n   | ^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/associated-types/associated-types-projection-in-supertrait.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/associated-types/associated-types-region-erasure-issue-20582.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: struct is never constructed: `Foo`
[00:55:55]   --> $DIR/associated-types-region-erasure-issue-20582.rs:17:1
[00:55:55]    |
[00:55:55] LL | struct Foo<'a> {
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-region-erasure-issue-20582/associated-types-region-erasure-issue-20582.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args associated-types/associated-types-region-erasure-issue-20582.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/associated-types/associated-types-region-erasure-issue-20582.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-region-erasure-issue-20582/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-region-erasure-issue-20582/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55]   --> $DIR/associated-types-sugar-path.rs:47:9
[00:55:55]    |
[00:55:55] LL |     let z: usize = bar(2, 4);
[00:55:55]    |         ^ help: consider using `_z` instead
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] warning: unused variable: `x`
[00:55:55]   --> $DIR/associated-types-sugar-path.rs:42:13
[00:55:55]   --> $DIR/associated-types-sugar-path.rs:42:13
[00:55:55]    |
[00:55:55] LL |         let x: T::A = panic!();
[00:55:55]    |             ^ help: consider using `_x` instead
[00:55:55] warning: struct is never constructed: `B`
[00:55:55]   --> $DIR/associated-types-sugar-path.rs:39:1
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL | struct B<X>(X);
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-sugar-path/associated-types-sugar-path.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args associated-types/associated-types-sugar-path.rs`
---
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-clone-enum"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variant is never constructed: `B`\n  --> /checkout/src/test/run-pass/deriving/deriving-clone-enum.rs:17:5\n   |\nLL |     B(()),\n   |     ^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] {"message":"variant is never constructed: `C`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/deriving/deriving-clone-enum.rs","byte_start":560,"byte_end":561,"line_start":18,"line_end":18,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    C","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: variant is never constructed: `C`\n  --> /checkout/src/test/run-pass/deriving/deriving-clone-enum.rs:18:5\n   |\nLL |     C\n   |     ^\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/deriving/deriving-clone-enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/deriving/deriving-clone-array.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: field is never used: `arr`
[00:55:55]   --> $DIR/deriving-clone-array.rs:16:5
[00:55:55]    |
[00:55:55] LL |     arr: [[u8; 256]; 4]
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expe_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: field is never used: `arr`\n  --> /checkout/src/test/run-pass/deriving/deriving-clone-array.rs:16:5\n   |\nLL |     arr: [[u8; 256]; 4]\n   |     ^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/deriving/deriving-clone-array.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/deriving/deriving-clone-generic-enum.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: variant is never constructed: `B`
[00:55:55]   --> $DIR/deriving-clone-generic-enum.rs:17:5
[00:55:55]    |
[00:55:55] LL |     B(T,U),
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] warning: variant is never constructed: `C`
---
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-clone-generic-enum/deriving-clone-generic-enum.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args deriving/deriving-clone-generic-enum.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/deriving/deriving-clone-generic-enum.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-clone-generic-enum/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-clone-generic-enum/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"variant is never constructed: `B`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/deriving/deriving-clone-generic-enum.rs","byte_start":557,"byte_end":563,"line_start":17,"line_end":17,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    B(T,U),","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variant is never constructed: `B`\n  --> /checkout/src/test/run-pass/deriving/deriving-clone-generic-enum.rs:17:5\n   |\nLL |     B(T,U),\n   |     ^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] {"message":"variant is never constructed: `C`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/deriving/deriving-clone-generic-enum.rs","byte_start":569,"byte_end":570,"line_start":18,"line_end":18,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    C","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: variant is never constructed: `C`\n  --> /checkout/src/test/run-pass/deriving/deriving-clone-generic-enum.rs:18:5\n   |\nLL |     C\n   |     ^\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/deriving/deriving-clone-generic-enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/deriving/deriving-meta-multiple.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused import: `SipHasher`
[00:55:55]   --> $DIR/deriving-meta-multiple.rs:15:23
[00:55:55]    |
[00:55:55] LL | use std::hash::{Hash, SipHasher};
[00:55:55]    |
[00:55:55]    = note: #[warn(unused_imports)] on by default
[00:55:55] 
[00:55:55] warning: unused comparison which must be used
[00:55:55] warning: unused comparison which must be used
[00:55:55]   --> $DIR/deriving-meta-multiple.rs:31:5
[00:55:55]    |
[00:55:55] LL |     a == a;    // check for PartialEq impl w/o testing its correctness
[00:55:55]    |
[00:55:55]    = note: #[warn(unused_must_use)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] warning: unused return value of `std::clone::Clone::clone` which must be used
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     a.clone(); // check for Clone impl w/o testing its correctness
[00:55:55]    |
[00:55:55]    |
[00:55:55]    = note: cloning is often expensive and is not expected to have side effects
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-meta-multiple/deriving-meta-multiple.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args deriving/deriving-meta-multiple.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/deriving/deriving-meta-multiple.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-meta-multiple/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-meta-multiple/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused import: `SipHasher`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/deriving/deriving-meta-multiple.rs","byte_start":561,"byte_end":570,"line_start":15,"line_end":15,"column_start":23,"column_end":32,"is_primary":true,"text":[{"text":"use std::hash::{Hash, SipHasher};","highlight_start":23,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `SipHasher`\n  --> /checkout/src/test/run-pass/deriving/deriving-meta-multiple.rs:15:23\n   |\nLL | use std::hash::{Hash, SipHasher};\n   |                       ^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[00:55:55] {"message":"unused comparison which must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/deriving/deriving-meta-multiple.rs","byte_start":809,"byte_end":815,"line_start":31,"line_end":31,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    a == a;    // check for PartialEq impl w/o testing its correctness","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_must_use)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused comparison which must be used\n  --> /checkout/src/test/run-pass/deriving/deriving-meta-multiple.rs:31:5\n   |\nLL |     a == a;    // check for PartialEq impl w/o testing its correctness\n   |     ^^^^^^\n   |\n   = note: #[warn(unused_must_use)] on by default\n\n"}
[00:55:55] {"message":"unused return value of `std::clone::Clone::clone` which must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/deriving/deriving-meta-multiple.rs","byte_start":880,"byte_end":890,"line_start":32,"line_end":32,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a.clone(); // check for Clone impl w/o testing its correctness","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"cloning is often expensive and is not expected to have side effects","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused return value of `std::clone::Clone::clone` which must be used\n  --> /checkout/src/test/run-pass/deriving/deriving-meta-multiple.rs:32:5\n   |\nLL |     a.clone(); // check for Clone impl w/o testing its correctness\n   |     ^^^^^^^^^^\n   |\n   = note: cloning is often expensive and is not expected to have side effects\n\n"}
[00:55:55] 
[00:55:at" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-hash/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-hash/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused import: `std::mem::size_of`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/deriving/deriving-hash.rs","byte_start":638,"byte_end":655,"line_start":18,"line_end":18,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"use std::mem::size_of;","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `std::mem::size_of`\n  --> /checkout/src/test/run-pass/deriving/deriving-hash.rs:18:5\n   |\nLL | use std::mem::size_of;\n   |     ^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[00:55:55] {"message":"enum is never used: `Collision`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/deriving/deriving-hash.rs","bytte: #[warn(unused_must_use)] on by default
[00:55:55] 
[00:55:55] warning: unused return value of `std::clone::Clone::clone` which must be used
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     a.clone(); // check for Clone impl w/o testing its correctness
[00:55:55]    |
[00:55:55]    |
[00:55:55]    = note: cloning is often expensive and is not expected to have side effects
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-meta/deriving-meta.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args deriving/deriving-meta.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/deriving/deriving-meta.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-meta/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-meta/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused import: `SipHasher`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/deriving/deriving-meta.rs","byte_start":561,"byte_end":570,"line_start":15,"line_end":15,"column_start":23,"column_end":32,"is_primary":true,"text":[{"text":"use std::hash::{Hash, SipHasher};","highlight_start":23,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `SipHasher`\n  --> /checkout/src/test/run-pass/deriving/deriving-meta.rs:15:23\n   |\nLL | use std::hash::{Hash, SipHasher};\n   |                       ^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[00:55:55] {"message":"unused comparison which must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/deriving/deriving-meta.rs","byte_start":740,"byte_end":746,"line_start":28,"line_end":28,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    a == a;    // check for PartialEq impl w/o testing its correctness","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_must_use)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused comparison which must be used\n  --> /checkout/src/test/run-pass/deriving/deriving-meta.rs:28:5\n   |\nLL |     a == a;    // check for PartialEq impl w/o testing its correctness\n   |     ^^^^^^\n   |\n   = note: #[warn(unused_must_use)] on by default\n\n"}
[00:55:55] {"message":"unused return value of `std::clone::Clone::clone` which must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/deriving/deriving-meta.rs","byte_start":811,"byte_end":821,"line_start":29,"line_end":29,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a.clone(); // check for Clone impl w/o testing its correctness","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"cloning is often expensive and is not expected to have side effects","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused return value of `std::clone::Clone::clone` which must be used\n  --> /checkout/src/test/run-pass/deriving/deriving-meta.rs:29:5\n   |\nLL |     a.clone(); // check for Clone impl w/o testing its correctness\n   |     ^^^^^^^^^^\n   |\n   = note: cloning is often expensive and is not expected to have side effects\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/deriving/deriving-meta.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/deriving/deriving-via-extension-c-enum.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: variant is never constructed: `Baz`
[00:55:55]   --> $DIR/deriving-via-extension-c-enum.rs:15:5
[00:55:55]    |
[00:55:55] LL |     Baz,
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] warning: variant is never constructed: `Boo`
---
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-via-extension-c-enum/deriving-via-extension-c-enum.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args deriving/deriving-via-extension-c-enum.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/deriving/deriving-via-extension-c-enum.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-via-extension-c-enum/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-via-extension-c-enum/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"variant is never constructed: `Baz`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/deriving/deriving-via-extension-c-enum.rs","byte_start":531,"byte_end":534,"line_start":15,"line_end":15,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    Baz,","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variant is never constructed: `Baz`\n  --> /checkout/src/test/run-pass/deriving/deriving-via-extension-c-enum.rs:15:5\n   |\nLL |     Baz,\n   |     ^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] {"message":"variant is never constructed: `Boo`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/deriving/deriving-via-extension-c-enum.rs","byte_start":540,"byte_end":543,"line_start":16,"line_end":16,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    Boo","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: variant is never constructed: `Boo`\n  --> /checkout/src/test/run-pass/deriving/deriving-via-extension-c-enum.rs:16:5\n   |\nLL |     Boo\n   |     ^^^\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/deriving/deriving-via-extension-c-enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/deriving/deriving-show.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: struct is never constructed: `Pointers`
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL | struct Pointers(*const Send, *mut Sync);
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-show/deriving-show.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args deriving/deriving-show.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/deriving/deriving-show.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving//deriving-via-extension-enum.rs stdout ----
[00:55:55] warning: variant is never constructed: `Baz`
[00:55:55]   --> $DIR/deriving-via-extension-enum.rs:15:5
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     Baz(f64, f64)
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-via-extension-enum/deriving-via-extension-enum.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args deriving/deriving-via-extension-enum.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/deriving/deriving-via-extension-enum.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-via-extension-enum/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-via-extension-enum/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"variant is never constructed: `Baz`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/deriving/deriving-via-extension-enum.rs","byte_start":545,"byte_end":558,"line_start":15,"line_end":15,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    Baz(f64, f64)","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variant is never constructed: `Baz`\n  --> /checkout/src/test/run-pass/deriving/deriving-via-extension-enum.rs:15:5\n   |\nLL |     Baz(f64, f64)\n   |     ^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/deriving/deriving-via-extension-enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/deriving/deriving-show-2.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: enum is never used: `A`
[00:55:55]   --> $DIR/deriving-show-2.rs:15:1
[00:55:55]    |
[00:55:55] LL | enum A {}
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] warning: variant is never constructed: `B3`
[00:55:55] warning: variant is never constructed: `B3`
[00:55:55]   --> $DIR/deriving-show-2.rs:17:18
[00:55:55]    |
[00:55:55] LL | enum B { B1, B2, B3 }
[00:55:55] 
[00:55:55] warning: variant is never constructed: `C3`
[00:55:55]   --> $DIR/deriving-show-2.rs:19:28
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL | enum C { C1(isize), C2(B), C3(String) }
[00:55:55] 
[00:55:55] warning: struct is never constructed: `H`
[00:55:55]   --> $DIR/deriving-show-2.rs:29:1
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL | struct H { a: isize }
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-show-2/deriving-show-2.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args deriving/deriving-show-2.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/deriving/deriving-show-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-show-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-show-2/auxiliary"
[00:55s/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/deriving/deriving-via-extension-hash-enum.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] normalized stderr:
[00:55:55] warning: enum is never used: `Foo`
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL | enum Foo {
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] warning: enum is never used: `A`
[00:55:55] warning: enum is never used: `A`
[00:55:55]   --> $DIR/deriving-via-extension-hash-enum.rs:19:1
[00:55:55]    |
[00:55:55] LL | enum A {
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-via-extension-hash-enum/deriving-via-extension-hash-enum.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args deriving/deriving-via-extension-hash-enum.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/deriving/deriving-via-extension-hash-enum.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-via-extension-hash-enum/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-via-extension-hash-struct.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] ---- [run-pass] run-pass/deriving/deriving-via-extension-struct-like-enum-variant.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: variant is never constructed: `Y`
[00:55:55]   --> $DIR/deriving-via-extension-struct-like-enum-variant.rs:15:5
---
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-via-extension-struct-like-enum-variant/deriving-via-extension-struct-like-enum-variant.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args deriving/deriving-via-extension-struct-like-enum-variant.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/deriving/deriving-via-extension-struct-like-enum-variant.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-via-extension-struct-like-enum-variant/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/deriving/deriving-via-extension-struct-like-enum-variant/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"variant is never constructed: `Y`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/deriving/deriving-via-extension-struct-like-enum-variant.rs","byte_start":550,"byte_end":551,"line_start":15,"line_end":15,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    Y","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variant is never constructed: `Y`\n  --> /checkout/src/test/run-pass/deriving/deriving-via-extension-struct-like-enum-variant.rs:15:5\n   |\nLL |     Y\n   |     ^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/deriving/deriving-via-extension-struct-like-enum-variant.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/drop/drop-struct-as-object.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `nyan`
[00:55:55]   --> $DIR/drop-struct-as-object.rs:42:13
[00:55:55]    |
[00:55:55] LL |         let nyan: Box<Dummy> = x as Box<Dummy>;
[00:55:55]    |             ^^^^ help: consider using `_nyan` instead
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/drop-struct-as-object/drop-struct-as-object.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args drop/drop-struct-as-object.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/drop/drop-struct-as-object.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/drop-struct-as-object/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/drop-struct-as-object/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused va238:9
[00:55:55] ---- [run-pass] run-pass/drop/drop-trait-generic.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: field is never used: `x`
[00:55:55]   --> $DIR/drop-trait-generic.rs:13:5
[00:55:55]   --> $DIR/drop-trait-generic.rs:13:5
[00:55:55]    |
[00:55:55] LL |     x: T
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/drop-trait-generic/drop-trait-generic.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args drop/drop-trait-generic.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/drop/drop-trait-generic.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/drop-trait-generic/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/drop-trait-generic/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ----------------------------ferences, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args drop/drop-trait.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/drop/drop-trait.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/drop-trait/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/drop-trait/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"field is never used: `x`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/drop-trait.rs","byte_start":496,"byte_end":504,"line_start":13,"line_end":13,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    x: isize","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: field is never used: `x`\n  --> /checkout/src/test/run-pass/drop/drop-trait.rs:13:5\n   |\nLL |     x: isize\n   |     ^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/drop/drop-trait.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/drop/drop-uninhabited-enum.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `x`
[00:55:55]   --> $DIR/drop-uninhabited-enum.rs:20:8
[00:55:55]    |
[00:55:55] LL | fn foo(x: Foo) { }
[00:55:55]    |        ^ help: consider using `_x` instead
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] warning: enum is never used: `Foo`
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL | enum Foo { }
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] warning: function is never used: `foo`
[00:55:55] warning: function is never used: `foo`
[00:55:55]   --> $DIR/drop-uninhabited-enum.rs:20:1
[00:55:55]    |
[00:55:55] LL | fn foo(x: Foo) { }
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/drop-uninhabited-enum/drop-uninhabited-enum.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args drop/drop-uninhabited-enum.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/drop/drop-uninhabited-enum.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/drop-uninhabited-enum/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/drop-uninhabited-enum/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused variable: `x`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/drop-uninhabited-enum.rs","byte_start":583,"byte_end":584,"line_start":20,"line_end":20,"column_start":8,"column_end":9,"is_primary":true,"text":[{"text":"fn foo(x: Foo) { }","highlight_start":8,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_x` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/drop/drop-uninhabited-enum.rs","byte_start":583,"byte_end":584,"line_start":20,"line_end":20,"column_start":8,"column_end":9,"is_primary":true,"text":[{"text":"fn foo(x: Foo) { }","highlight_start":8,"highlight_end":9}],"label":null,"suggested_replacement":"_x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `x`\n  --> /checkout/src/test/run-pass/drop/drop-uninhabited-enum.rs:20:8\n   |\nLL | fn foo(x: Foo) { }\n   |        ^ help: consider using `_x` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:55:55] {"message":"enum is never used: `Foo`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/drop-uninhabited-enum.rs","byte_start":512,"byte_end":520,"line_start":14,"line_end":14,"column_start":1,"column_end":9,"is_primary":true,"text":[{"text":"enum Foo { }","highlight_start":1,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: enum is never used: `Foo`\n  --> /checkout/src/test/run-pass/drop/drop-uninhabited-enum.rs:14:1\n   |\nLL | enum Foo { }\n   | ^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] {"message":"function is never used: `foo`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/drop-uninhabited-enum.rs","byte_start":576,"byte_end":590,"line_start":20,"line_end":20,"column_start":1,"column_end":15,"is_primary":true,"text":[{"text":"fn foo(x: Foo) { }","highlight_start":1,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: function is never used: `foo`\n  --> /checkout/src/test/run-pass/drop/drop-uninhabited-enum.rs:20:1\n   |\nLL | fn foo(x: Foo) { }\n   | ^^^^^^^^^^^^^^\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/drop/drop-uninhabited-enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/drop/drop-trait-enum.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `v`
[00:55:55]   --> $DIR/drop-trait-enum.rs:75:13
[00:55:55]    |
[00:55:55] LL |         let v = Foo::FailingVariant { on_drop: SendOnDrop { sender: sender } };
[00:55:55]    |             ^ help: consider using `_v` instead
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] warning: variable `v` is assigned to, but never used
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |             let mut v = Foo::NestedVariant(box 42, SendOnDrop {
[00:55:55]    |
[00:55:55]    = note: consider using `_v` instead
[00:55:55] 
[00:55:55] 
[00:55:55] warning: value assigned to `v` is never read
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |             v = Foo::NestedVariant(box 42,
[00:55:55]    |
[00:55:55]    = note: #[warn(unused_assignments)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] warning: value assigned to `v` is never read
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |             v = Foo::SimpleVariant(sender.clone());
[00:55:55] 
[00:55:55] 
[00:55:55] warning: value assigned to `v` is never read
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |             v = Foo::FailingVariant { on_drop: SendOnDrop { sender: sender } };
[00:55:55] 
[00:55:55] warning: unused variable: `v`
[00:55:55]   --> $DIR/drop-trait-enum.rs:60:13
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |         let v = Foo::SimpleVariant(sender);
[00:55:55]    |             ^ help: consider using `_v` instead
[00:55:55] warning: unused variable: `v`
[00:55:55]   --> $DIR/drop-trait-enum.rs:67:13
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |         let v = Foo::NestedVariant(box 42, SendOnDrop { sender: sender.clone() }, sender);
[00:55:55]    |             ^ help: consider using `_v` instead
[00:55:55] 
[00:55:55] warning: field is never used: `on_drop`
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     FailingVariant { on_drop: SendOnDrop }
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_v` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/drop/drop-trait-enum.rs","byte_start":2076,"byte_end":2077,"line_start":75,"line_end":75,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"        let v = Foo::FailingVariant { on_drop: SendOnDrop { sender: sender } };","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":"_v","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `v`\n  --> /checkout/src/test/run-pass/drop/drop-trait-enum.rs:75:13\n   |\nLL |         let v = Foo::FailingVariant { on_drop: SendOnDrop { sender: sender } };\n   |             ^ help: consider using `_v` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:55:55] {"message":"variable `v` is assigned to, but never used","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/drop-trait-enum.rs","byte_start":2382,"byte_end":2383,"line_start":84,"line_end":84,"column_start":21,"column_end":22,"is_primary":true,"text":[{"text":"            let mut v = Foo::NestedVariant(box 42, SendOnDrop {","highlight_start":21,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_v` instead","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variable `v` is assigned to, but never used\n  --> /checkout/src/test/run-pass/drop/drop-trait-enum.rs:84:21\n   |\nLL |             let mut v = Foo::NestedVariant(box 42, SendOnDrop {\n   |                     ^\n   |\n   = note: consider using `_v` instead\n\n"}
[00:55:55] {"message":"value assigned to `v` is never read","code":{"code":"unused_assignments","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/drop-trait-enum.rs","byte_start":2509,"byte_end":2510,"line_start":87,"line_end":87,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"            v = Foo::NestedVariant(box 42,","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_assignments)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: value assigned to `v` is never read\n  --> /checkout/src/test/run-pass/drop/drop-trait-enum.rs:87:13\n   |\nLL |             v = Foo::NestedVariant(box 42,\n   |             ^\n   |\n   = note: #[warn(unused_assignments)] on by default\n\n"}
[00:55:55] {"message":"value assigned to `v` is never read","code":{"code":"unused_assignments","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/drop-trait-enum.rs","byte_start":2678,"byte_end":2679,"line_start":90,"line_end":90,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"            v = Foo::SimpleVariant(sender.clone());","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: value assigned to `v` is never read\n  --> /checkout/src/test/run-pass/drop/drop-trait-enum.rs:90:13\n   |\nLL |             v = Foo::SimpleVariant(sender.clone());\n   |             ^\n\n"}
[00:55:55] {"message":"value assigned to `v` is never read","code":{"code":"unused_assignments","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/drop-trait-enum.rs","byte_start":2730,"byte_end":2731,"line_start":91,"line_end":91,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"            v = Foo::FailingVariant { on_drop: SendOnDrop { sender: sender } };","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: value assigned to `v` is never read\n  --> /checkout/src/test/run-pass/drop/drop-trait-enum.rs:91:13\n   |\nLL |             v = Foo::FailingVariant { on_drop: SendOnDrop { sender: sender } };\n   |             ^\n\n"}
[00:55:55] {"message":"unused variable: `v`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/drop-trait-enum.rs","byte_start":1526,"byte_end":1527,"line_start":60,"line_end":60,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"        let v = Foo::SimpleVariant(sender);","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_v` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/drop/drop-trait-enum.rs","byte_start":1526,"byte_end":1527,"line_start":60,"line_end":60,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"        let v = Foo::SimpleVariant(sender);","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":"_v","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `v`\n  --> /checkout/src/test/run-pass/drop/drop-trait-enum.rs:60:13\n   |\nLL |         let v = Foo::SimpleVariant(sender);\n   |             ^ help: consider using `_v` instead\n\n"}
[00:55:55] {"message":"unused variable: `v`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/drop-trait-enum.rs","byte_start":1733,"byte_end":1734,"line_start":67,"line_end":67,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"        let v = Foo::NestedVariant(box 42, SendOnDrop { sender: sender.clone() }, sender);","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_v` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/drop/drop-trait-enum.rs","byte_start":1733,"byte_end":1734,"line_start":67,"line_end":67,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"        let v = Foo::NestedVariant(box 42, SendOnDrop { sender: sender.clone() }, sender);","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":"_v","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `v`\n  --> /checkout/src/test/run-pass/drop/drop-trait-enum.rs:67:13\n   |\nLL |         let v = Foo::NestedVariant(box 42, SendOnDrop { sender: sender.clone() }, sender);\n   |             ^ help: consider using `_v` instead\n\n"}
[00:55:55] {"message":"field is never used: `on_drop`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/drop-trait-enum.rs","byte_start":973,"byte_end":992,"line_start":38,"line_end":38,"column_start":22,"column_end":41,"is_primary":true,"text":[{"text":"    FailingVariant { on_drop: SendOnDrop }","highlight_start":22,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: field is never used: `on_drop`\n  --> /checkout/src/test/run-pass/drop/drop-trait-enum.rs:38:22\n   |\nLL |     FailingVariant { on_drop: SendOnDrop }\n   |                      ^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/drop/drop-trait-enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/drop/no-drop-flag-size.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: field is never used: `a`
[00:55:55]   --> $DIR/no-drop-flag-size.rs:15:5
[00:55:55]    |
[00:55:55] LL |     a: T
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/no-drop-flag-size/no-drop-flag-size.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args drop/no-drop-flag-size.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/drop/no-drop-flag-size.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/no-drop-flag-size/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/no-drop-flag-size/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"field is never used: `a`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/no-drop-flag-size.rs","byte_start":524,"byte_end":528,"line_start":15,"line_end":15,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    a: T","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: field is never used: `a`\n  --> /checkout/src/test/run-pass/drop/no-drop-flag-size.rs:15:5\n   |\nLL |     a: T\n   |     ^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/drop/no-drop-flag-size.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/dynamically-sized-types/dst-coerce-rc.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `d`
[00:55:55]   --> $DIR/dst-coerce-rc.rs:42:9
[00:55:55]    |
[00:55:55] LL |     let d: Weak<Baz> = c.clone();
[00:55:55]    |         ^ help: consider using `_d` instead
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] warning: unused variable: `c`
[00:55:55]   --> $DIR/dst-coerce-rc.rs:50:9
[00:55:55]   --> $DIR/dst-coerce-rc.rs:50:9
[00:55:55]    |
[00:55:55] LL |     let c: Weak<RefCell<Baz>> = Rc::downgrade(&a) as Weak<_>;
[00:55:55]    |         ^ help: consider using `_c` instead
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/dynamically-sized-types/dst-coerce-rc/dst-coerce-rc.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args dynamically-sized-types/dst-coerce-rc.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/dynamically-sized-types/dst-coerce-rc.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/dynamically-sized-types/dst-coerce-rc/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/dynamically-sized-types/dst-coerce-rc/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused variable: `d`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/dynamically-sized-types/dst-coerce-rc.rs","byte_start":1044,"byte_end":1045,"line_start":42,"line_end":42,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let d: Weak<Baz> = c.clone();","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_d` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/dynamically-sized-types/dst-coerce-rc.rs","byte_start":1044,"byte_end":1045,"line_start":42,"line_end":42,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let d: Weak<Baz> = c.clone();","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"_d","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `d`\n  --> /checkout/src/test/run-pass/dynamically-sized-types/dst-coerce-rc.rs:42:9\n   |\nLL |     let d: Weak<Baz> = c.clone();\n   |         ^ help: consider using `_d` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:55:55] {"message":"unused variable: `c`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/dynamically-sized-types/dst-coerce-rc.rs","byte_start":1253,"byte_end":1254,"line_start":50,"line_end":50,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let c: Weak<RefCell<Baz>> = Rc::downgrade(&a) as Weak<_>;","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_c` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/dynamically-sized-types/dst-coerce-rc.rs","byte_start":1253,"byte_end":1254,"line_start":50,"line_end":50,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let c: Weak<RefCell<Baz>> = Rc::downgrade(&a) as Weak<_>;","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"_c","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `c`\n  --> /checkout/src/test/run-pass/dynamically-sized-types/dst-coerce-rc.rs:50:9\n   |\nLL |     let c: Weak<RefCell<Baz>> = Rc::downgrade(&a) as Weak<_>;\n   |         ^ help: consider using `_c` instead\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/dynamically-sized-types/dst-coerce-rc.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/dynamically-sized-types/dst-coercions.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `x`
[00:55:55]   --> $DIR/dst-coercions.rs:21:9
[00:55:55]    |
[00:55:55] LL |     let x: &T = &S;
[00:55:55]    |         ^ help: consider using `_x` instead
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] warning: unused variable: `x`
[00:55:55]   --> $DIR/dst-coercions.rs:23:9
[00:55:55]   --> $DIR/dst-coercions.rs:23:9
[00:55:55]    |
[00:55:55] LL |     let x: *const T = &S;
[00:55:55]    |         ^ help"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_x` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/dynamically-sized-types/dst-coercions.rs","byte_start":648,"byte_end":649,"line_start":21,"line_end":21,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let x: &T = &S;","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"_x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `x`\n  --> /checkout/src/test/run-pass/dynamically-sized-types/dst-coercions.rs:21:9\n   |\nLL |     let x: &T = &S;\n   |         ^ help: consider using `_x` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:55:55] {"message":"unused variable: `x`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/dynamically-sized-types/dst-coercions.rs","byte_start":732,"byte_end":733,"line_start":23,"line_end":23,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let x: *const T = &S;","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_x` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/dynamically-sized-types/dst-coercions.rs","byte_start":732,"byte_end":733,"line_start":23,"line_end":23,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let x: *const T = &S;","hlight_end":10}],"label":null,"suggested_replacement":"_x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `x`\n  --> /checkout/src/test/run-pass/dynamically-sized-types/dst-coercions.rs:32:9\n   |\nLL |     let x: *mut S = &mut S;\n   |         ^ help: consider using `_x` instead\n\n"}
[00:55:55] {"message":"unused variable: `x`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/dynamically-sized-types/dst-coercions.rs","byte_start":1032,"byte_end":1033,"line_start":35,"line_end":35,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let x: &T = &mut S;","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_x` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/dynamically-sized-types/dst-coercions.rs","byte_start":1032,"byte_end":1033,"line_start":35,"line_end":35,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let x: &T = &mut S;","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"_x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `x`\n  --> /checkout/src/test/run-pass/dynamically-sized-types/dst-coercions.rs:35:9\n   |\nLL |     let x: &T = &mut S;\n   |         ^ help: consider using `_x` instead\n\n"}
[00:55:55] {"message":"unused variable: `x`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/dynamically-sized-types/dst-coercions.rs","byte_start":1056,"byte_end":1057,"line_start":36,"line_end":36,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let x: *const T = &mut S;","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_x` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/dynamically-sized-types/dst-coercions.rs","byte_start":1056,"byte_end":1057,"line_start":36,"line_end":36,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let x: *const T = &mut S;","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"_x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `x`\n  --> /checkout/src/test/run-pass/dynamically-sized-types/dst-coercions.rs:36:9\n   |\nLL |     let x: *const T = &mut S;\n   |         ^ help: consider using `_x` instead\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/dynamically-sized-types/dst-coercions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/drop/dynamic-drop.rs#nll stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: variable `x` is assigned to, but never used
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     let (x, y, z);
[00:55:55]    |
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55]    = note: consider using `_x` instead
[00:55:55] 
[00:55:55] warning: variable `y` is assigned to, but never used
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     let (x, y, z);
[00:55:55]    |
[00:55:55]    = note: consider using `_y` instead
[00:55:55] 
[00:55:55] 
[00:55:55] warning: variable `z` is assigned to, but never used
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     let (x, y, z);
[00:55:55]    |
[00:55:55]    |
[00:55:55]    = note: consider using `_z` instead
[00:55:55] warning: value assigned to `x` is never read
[00:55:55]   --> $DIR/dynamic-drop.rs:193:5
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     x = a.alloc();
[00:55:55]    |
[00:55:55]    = note: #[warn(unused_assignments)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] warning: value assigned to `y` is never read
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     y = 5;
[00:55:55] 
[00:55:55] 
[00:55:55] warning: value assigned to `z` is never read
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     z = a.alloc();
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dynamic-drop.nll/dode":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_x` instead","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variable `x` is assigned to, but never used\n  --> /checkout/src/test/run-pass/drop/dynamic-drop.rs:192:10\n   |\nLL |     let (x, y, z);\n   |          ^\n   |\n   = note: #[warn(unused_variables)] on by default\n   = note: consider using `_x` instead\n\n"}
[00:55:55] {"message":"variable `y` is assigned to, but never used","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dynamic-drop.rs","byte_start":4247,"byte_end":4248,"line_start":192,"line_end":192,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"    let (x, y, z);","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_y` instead","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variable `y` is assigned to, but never used\n  --> /checkout/src/test/run-pass/drop/dynamic-drop.rs:192:13\n   |\nLL |     let (x, y, z);\n   |             ^\n   |\n   = note: consider using `_y` instead\n\n"}
[00:55:55] {"message":"variable `z` is assigned to, but never used","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dynamic-drop.rs","byte_start":4250,"byte_end":4251,"line_start":192,"line_end":192,"column_start":16,"column_end":17,"is_primary":true,"text194,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    y = 5;","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: value assigned to `y` is never read\n  --> /checkout/src/test/run-pass/drop/dynamic-drop.rs:194:5\n   |\nLL |     y = 5;\n   |     ^\n\n"}
[00:55:55] {"message":"value assigned to `z` is never read","code":{"code":"unused_assignments","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dynamic-drop.rs","byte_start":4288,"byte_end":4289,"line_start":195,"line_end":195,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    z = a.alloc();","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: value assigned to `z` is never read\n  --> /checkout/src/test/run-pass/drop/dynamic-drop.rs:195:5\n   |\nLL |     z = a.alloc();\n   |     ^\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/drop/dynamic-drop.rs#nll' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] ---- [run-pass] run-pass/drop/dynamic-drop.rs#lexical stdout ----
[00:55:55] normalized stderr:
[00:55:55] normalized stderr:
[00:55:55] warning: variable `x` is assigned to, but never used
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     let (x, y, z);
[00:55:55]    |
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55]    = note: consider using `_x` instead
[00:55:55] 
[00:55:55] warning: variable `y` is assigned to, but never used
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     let (x, y, z);
[00:55:55]    |
[00:55:55]    = note: consider using `_y` instead
[00:55:55] 
[00:55:55] 
[00:55:55] warning: variable `z` is assigned to, but never used
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     let (x, y, z);
[00:55:55]    |
[00:55:55]    |
[00:55:55]    = note: consider using `_z` instead
[00:55:55] warning: value assigned to `x` is never read
[00:55:55]   --> $DIR/dynamic-drop.rs:193:5
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     x = a.alloc();
[00:55:55]    |
[00:55:55]    = note: #[warn(unused_assignments)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] warning: value assigned to `y` is never read
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     y = 5;
[00:55:55] 
[00:55:55] 
[00:55:55] warning: value assigned to `z` is never read
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     z = a.alloc();
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dynamic-drop.lexical/dynamic-drop.lexical.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args drop/dynamic-drop.rs`
[00:55:55] 
[00:55:55] error in revision `lexical`: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/drop/dynamic-drop.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "lexical" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dynamic-drop.lexical/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dynamic-drop.lexical/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"variable `x` is assigned to, but never used","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dynamic-drop.rs","byte_start":4244,"byte_end":4245,"line_start":192,"line_end":192,"column_start":10,"column_end":11,"is_primary":true,"text":[{"text":"    let (x, y, z);","highlight_start":10,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_x` instead","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variable `x` is assigned to, but never used\n  --> /checkout/src/test/run-pass/drop/dynamic-drop.rs:192:10\n   |\nLL |     let (x, y, z);\n   |          ^\n   |\n   = note: #[warn(unused_variables)] on by default\n   = note: consider using `_x` instead\n\n"}
[00:55:55] {"message":"variable `y` is assigned to, but never used","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dynamic-drop.rs","byte_start":4247,"byte_end":4248,"line_start":192,"line_end":192,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"    let (x, y, z);","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_y` instead","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variable `y` is assigned to, but never used\n  --> /checkout/src/test/run-pass/drop/dynamic-drop.rs:192:13\n   |\nLL |     let (x, y, z);\n   |             ^\n   |\n   = note: consider using `_y` instead\n\n"}
[00:55:55] {"message":"variable `z` is assigned to, but never used","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dynamic-drop.rs","byte_start":4250,"byte_end":4251,"line_start":192,"line_end":192,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"    let (x, y, z);","highlight_start":16,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_z` instead","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variable `z` is assigned to, but never used\n  --> /checkout/src/test/run-pass/drop/dynamic-drop.rs:192:16\n   |\nLL |     let (x, y, z);\n   |                ^\n   |\n   = note: consider using `_z` instead\n\n"}
[00:55:55] {"message":"value assigned to `x` is never read","code":{"code":"unused_assignments","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dynamic-drop.rs","byte_start":4258,"byte_end":4259,"line_start":193,"line_end":193,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    x = a.alloc();","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_assignments)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: value assigned to `x` is never read\n  --> /checkout/src/test/run-pass/drop/dynamic-drop.rs:193:5\n   |\nLL |     x = a.alloc();\n   |     ^\n   |\n   = note: #[warn(unused_assignments)] on by default\n\n"}
[00:55:55] {"message":"value assigned to `y` is never read","code":{"code":"unused_assignments","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dynamic-drop.rs","byte_start":4277,"byte_end":4278,"line_start":194,"line_end":194,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    y = 5;","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: value assigned to `y` is never read\n  --> /checkout/src/test/run-pass/drop/dynamic-drop.rs:194:5\n   |\nLL |     y = 5;\n   |     ^\n\n"}
[00:55:55] {"message":"value assigned to `z` is never read","code":{"code":"unused_assignments","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dynamic-drop.rs","byte_start":4288,"byte_end":4289,"line_start":195,"line_end":195,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    z = a.alloc();","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: value assigned to `z` is never read\n  --> /checkout/src/test/run-pass/drop/dynamic-drop.rs:195:5\n   |\nLL |     z = a.alloc();\n   |     ^\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/drop/dynamic-drop.rs#lexical' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] ---- [run-pass] run-pass/dynamically-sized-types/dst-index.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `idx`
[00:55:55]   --> $DIR/dst-index.rs:33:28
[00:55:55]   --> $DIR/dst-index.rs:33:28
[00:55:55]    |
[00:55:55] LL |     fn index<'a>(&'a self, idx: usize) -> &'a (Debug + 'static) {
[00:55:55]    |                            ^^^ help: consider using `_idx` instead
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00mary":true,"text":[{"text":"    fn index<'a>(&'a self, idx: usize) -> &'a (Debug + 'static) {","highlight_start":28,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_idx` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/dynamically-sized-types/dst-index.rs","byte_start":846,"byte_end":849,"line_start":33,"line_end":33,"column_start":28,"column_end":31,"is_primary":true,"text":[{"text":"    fn index<'a>(&'a self, idx: usize) -> &'a (Debug + 'static) {","highlight_start":28,"highlight_end":31}],"label":null,"suggested_replacement":"_idx","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `idx`\n  --> /checkout/src/test/run-pass/dynamically-sized-types/dst-index.rs:33:28\n   |\nLL |     fn index<'a>(&'a self, idx: usize) -> &'a (Debug + 'static) {\n   |                            ^^^ help: consider using `_idx` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/dynamically-sized-types/dst-index.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/dynamically-sized-types/dst-field-align.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: field is never used: `a`
[00:55:55]   --> $DIR/dst-field-align.rs:13:5
[00:55:55]    |
[00:55:55] LL |     a: u16,
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] warning: field is never used: `ptr`
[00:55:55] warning: field is never used: `ptr`
[00:55:55]   --> $DIR/dst-field-align.rs:30:5
[00:55:55]    |
[00:55:55] LL |     ptr: Box<usize>,
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/dynamically-sized-types/dst-field-align/dst-field-align.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args dynamically-sized-types/dst-field-align.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/dynamically-sized-types/dst-field-align.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/dynamically-sized-types/dst-field-align/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/dynamically-sized-types/dst-field-align/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"field is never used: `a`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/dynamically-sized-types/dst-field-align.rs","byte_start":507,"byte_end":513,"line_start":13,"line_end":13,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    a: u16,","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: field is never used: `a`\n  --> /checkout/src/test/run-pass/dynamically-sized-types/dst-field-align.rs:13:5\n   |\nLL |     a: u16,\n   |     ^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
---
[00:55:55] normalized stderr:
[00:55:55] warning: field is never used: `f`
[00:55:55]   --> $DIR/issue-18353.rs:18:5
[00:55:55]    |
[00:55:55] LL |     f: [u8]
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-18353/issue-18353.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args issues/issue-18353.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-18353.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-18353/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-18353/auxiliary"
[00:55:55] tion is never used: `rust_task_is_unwinding`
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |             pub fn rust_task_is_unwinding(rt: *const rust_task) -> bool;
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-1866/issue-1866.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args issues/issue-1866.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-1866.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-1866/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-1866/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"type alias is never used: `rust_task`","code":{"code":"dead_code","explanationkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-18738/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"variant is never constructed: `Int`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-18738.rs","byte_start":541,"byte_end":555,"line_start":14,"line_end":14,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    Int(&'a isize),","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variant is never constructed: `Int`\n  --> /checkout/src/test/run-pass/issues/issue-18738.rs:14:5\n   |\nLL |     Int(&'a isize),\n   |     ^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] {"message":"variant is never constructed: `Slice`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-18738.rs","byte_start":561,"byte_end":576,"line_start":15,"line_end":15,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    Slice(&'a [u8]),","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replaceme To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args issues/issue-18906.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-18906.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-18906/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-18906/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"function is never used: `bar`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-18906.rs","byte_start":719,"byte_end":772,"line_start":26,"line_end":26,"column_start":1,"column_end":54,"is_primary":true,"text":[{"text":"fn bar<K, Q>(k: &K, q: &Q) where K: Borrow<Q>, Q: Foo {","highlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: function is never used: `bar`\n  --> /checkout/src/test/run-pass/issues/issue-18906.rs:26:1\n   |\nLL | fn bar<K, Q>(k: &K, q: &Q) where K: Borrow<Q>, Q: Foo {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] {"message":"struct is never constructed: `MyTree`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-18906.rs","byte_start":800,"byte_end":820,"line_start":30,"line_end":30,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"struct MyTree<K>(K);","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: struct is never constructed: `MyTree`\n  --> /checkout/src/test/run-pass/issues/issue-18906.rs:30:1\n   |\nLL | struct MyTree<K>(K);\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:55:55] {"message":"method is never used: `bar`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-18906.rs","byte_start":885,"byte_end":935,"line_start":34,"line_end":34,"column_start":5,"column_end":55,"is_primary":true,"text":[{"text":"    fn bar<Q>(k: &K, q: &Q) where K: Borrow<Q>, Q: Foo {","highlight_start":5,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: method is never used: `bar`\n  --> /checkout/src/test/run-pass/issues/issue-18906.rs:34:5\n   |\nLL |     fn bar<Q>(k: &K, q: &Q) where K: Borrow<Q>, Q: Foo {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/issues/issue-18906.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/issues/issue-19001.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: field is never used: `ptr`
[00:55:55]   --> $DIR/issue-19001.rs:15:5
[00:55:55]    |
[00:55:55] LL |     ptr: *mut [Loopy; 1]
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19001/issue-19001.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args issues/issue-19001.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-19001.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19001/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19001/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"field is never used: `ptr`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-19001.rs","byte_start":563,"byte_end":583,"line_start":15,"line_end":15,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    ptr: *mut [Loopy; 1]","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: field is never used: `ptr`\n  --> /checkout/src/test/run-pass/issues/issue-19001.rs:15:5\n   |\nLL |     ptr: *mut [Loopy; 1]\n   |     ^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/issues/issue-19001.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/issues/issue-18988.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: field is never used: `children`
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     children: Vec<Box<Foo>>,
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-18988/issue-18988.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args issues/issue-18988.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-18988.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-18988/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-18988/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"field is never used: `children`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-18988.rs","byte_start":528,"byte_end":551,"line_start":15,"line_end":15,"cerror: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-19037.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19037/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19037/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"function is never used: `clone`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-19037.rs","byte_start":689,"byte_end":714,"line_start":25,"line_end":25,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"fn clone(s: &Str) -> &Str {","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: function is never used: `clone`\n  --> /checkout/src/test/run-pass/issues/issue-19037.rs:25:1\n   |\nLL | fn clone(s: &Str) -> &Str {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[wa                              ^ help: consider using `_f` instead
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19127/issue-19127.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args issues/issue-19127.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-19127.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19127/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19127/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused variable: `f`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-19127.rs","byte_start":541,"byte_end":542,"line_start:5,"column_end":15,"is_primary":true,"text":[{"text":"use self::A::B;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `self::A::B`\n  --> /checkout/src/test/run-pass/issues/issue-19102.rs:14:5\n   |\nLL | use self::A::B;\n   |     ^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/issues/issue-19102.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/issues/issue-19129-2.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `i`
[00:55:55]   --> $DIR/issue-19129-2.rs:17:22
[00:55:55]    |
[00:55:55] LL |     fn method(&self, i: Input) -> bool { false }
[00:55:55]    |                      ^ help: consider using `_i` instead
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19129-2/issue-19129-2.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args issues/issue-19129-2.rs`
[00:55:55]byte_end":573,"line_start":17,"line_end":17,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    fn method(&self, i: Input) -> bool { false }","highlight_start":22,"highlight_end":23}],"label":null,"suggested_replacement":"_i","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `i`\n  --> /checkout/src/test/run-pass/issues/issue-19129-2.rs:17:22\n   |\nLL |     fn method(&self, i: Input) -> bool { false }\n   |                      ^ help: consider using `_i` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/issues/issue-19129-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/issues/issue-19340-2.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `name`
[00:55:55]   --> $DIR/issue-19340-2.rs:29:13
[00:55:55]    |
[00:55:55] LL |             name,
[00:55:55]    |             ^^^^ help: try ignoring the field: `name: _`
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] warning: unused variable: `age`
[00:55:55]   --> $DIR/issue-19340-2.rs:30:13
[00:55:55]   --> $DIR/issue-19340-2.rs:30:13
[00:55:55]    |
[00:55:55] LL |             age,
[00:55:55]    |             ^^^ help: try ignoring the field: `age: _`
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checles)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try ignoring the field","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-19340-2.rs","byte_start":751,"byte_end":755,"line_start":29,"line_end":29,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"            name,","highlight_start":13,"highlight_end":17}],"label":null,"suggested_replacement":"name: _","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `name`\n  --> /checkout/src/test/run-pass/issues/issue-19340-2.rs:29:13\n   |\nLL |             name,\n   |             ^^^^ help: try ignoring the field: `name: _`\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:55:55] {"message":"unused variable: `age`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-19340-2.rs","byte_start":769,"byte_end":772,"line_start":30,"line_end":30,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"            age,","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try ignoring the field","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-19340-2.rs","byte_start":769,"byte_end":772,"line_start":30,"line_end":30,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"            age,","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":"age: _","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `age`\n  --> /checkout/src/test/run-pass/issues/issue-19340-2.rs:30:13\n   |\nLL |             age,\n   |             ^^^ help: try ignoring the field: `age: _`\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/issues/issue-19340-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/issues/issue-19340-1.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `name`
[00:55:55]   --> $DIR/issue-19340-1.rs:24:26
[00:55:55]    |
[00:55:55] LL |         Homura::Madoka { name } => (),
[00:55:55]    |                          ^^^^ help: try ignoring the field: `name: _`
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19340-1/issue-19340-1.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args issues/issue-19340-1.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-19340-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19340-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19340-1/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused variable: `name`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-19340-1.rs","byte_start":718,"byte_end":722,"line_start":24,"line_end":24,"column_start":26,"column_end":30,"is_primary":true,"text":[{"text":"        Homura::Madoka { name } => (),","highlight_start":26,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try ignoring the field","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-19340-1.rs","byte_start":718,"byte_end":722,"line_start":24,"line_end":24,"column_start":26,"column_end":30,"is_primary":true,"text":[{"text":"        Homura::Madoka { name } => (),","highlight_start":26,"highlight_end":30}],"label":null,"suggested_replacement":"name: _","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `name`\n  --> /checkout/src/test/run-pass/issues/issue-19340-1.rs:24:26\n   |\nLL |         Homura::Madoka { name } => (),\n   |                          ^^^^ help: try ignoring the field: `name: _`\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/issues/issue-19340-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/issues/issue-19404.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `x`
[00:55:55]   --> $DIR/issue-19404.rs:28:13
[00:55:55]    |
[00:55:55] LL |         let x = self.get_component_type_id(TypeId::of::<T>());
[00:55:55]    |             ^ help: consider using `_x` instead
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] warning: unused variable: `engine`
[00:55:55]   --> $DIR/issue-19404.rs:41:13
[00:55:55]   --> $DIR/issue-19404.rs:41:13
[00:55:55]    |
[00:55:55] LL |         let engine = env.get_component::<Engine>();
[00:55:55]    |             ^^^^^^ help: consider using `_engine` instead
[00:55:55] 
[00:55:55] warning: struct is never constructed: `MyFigment`
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL | struct MyFigment;
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19404/issue-19404.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args issues/issue-19404.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-19404.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19404/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19404/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"unused variable: `x`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-19404.rs","byte_start":799,"byte_end":800,"line_start":28,"line_end":28,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"        let x = self.get_component_type_id(TypeId::of::<T>());","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_x` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-19404.rs","byte_start":799,"byte_end":800,"line_start":28,"line_end":28,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"        let x = self.get_component_type_id(TypeId::of::<T>());","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":"_x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `x`\n  --> /checkout/src/test/run-pass/issues/issue-19404.rs:28:13\n   |\nLL |         let x = self.get_component_type_id(TypeId::of::<T>());\n   |             ^ help: consider using `_x` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:55:55] {"message":"unused variable: `engine`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-19404.rs","byte_start":1022,"byte_end":1028,"line_start":41,"line_end":41,"column_start":13,"column_end":19,"is_primary":true,"text":[{"text":"        let engine = env.get_component::<Engine>();","highlight_start":13,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_engine` instead","code":n thread '[run-pass] run-pass/issues/issue-19404.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] ---- [run-pass] run-pass/issues/issue-19499.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `it`
[00:55:55]   --> $DIR/issue-19499.rs:22:9
[00:55:55]   --> $DIR/issue-19499.rs:22:9
[00:55:55]    |
[00:55:55] LL |     let it = Some(1_usize).into_iter().inspect(|_| {n;});
[00:55:55]    |         ^^ help: consider using `_it` instead
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] warning: path statement with no effect
[00:55:55]   --> $DIR/issue-19499.rs:22:53
[00:55:55]   --> $DIR/issue-19499.rs:22:53
[00:55:55]    |
[00:55:55] LL |     let it = Some(1_usize).into_iter().inspect(|_| {n;});
[00:55:55]    |
[00:55:55]    |
[00:55:55]    = note: #[warn(path_statements)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19499/issue-19499.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args issues/issue-19499.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-19499.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefenull}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `it`\n  --> /checkout/src/test/run-pass/issues/issue-19499.rs:22:9\n   |\nLL |     let it = Some(1_usize).into_iter().inspect(|_| {n;});\n   |         ^^ help: consider using `_it` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:55:55] {"message":"path statement with no effect","code":{"code":"path_statements","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-19499.rs","byte_start":906,"byte_end":908,"line_start":22,"line_end":22,"column_start":53,"column_end":55,"is_primary":true,"text":[{"text":"    let it = Some(1_usize).into_iter().inspect(|_| {n;});","highlight_start":53,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(path_statements)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: path statement with no effect\n  --> /checkout/src/test/run-pass/issues/issue-19499.rs:22:53\n   |\nLL |     let it = Some(1_usize).into_iter().inspect(|_| {n;});\n   |                                                     ^^\n   |\n   = note: #[warn(path_statements)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/issues/issue-19499.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/issues/issue-19631.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: structpass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args issues/issue-19632.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-19632.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19632/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-19632/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"struct is never constructed: `InnerPool`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-19632.rs","byte_start":571,"byte_end":603,"line_start":19,"line_end":19,"column_start":1,"column_end":33,"is_primary":true,"text":[{"text":"struct InnerPool<M: PoolManager> {","highlight_start":1,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: struct is never constructed: ` help: consider using `_r` instead\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/issues/issue-19850.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/issues/issue-20186.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `b`
[00:55:55]   --> $DIR/issue-20186.rs:15:20
[00:55:55]    |
[00:55:55] LL |     fn putc(&self, b: u8) { }
[00:55:55]    |                    ^ help: consider using `_b` instead
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] warning: struct is never constructed: `Foo`
[00:55:55]   --> $DIR/issue-20186.rs:12:1
[00:55:55]   --> $DIR/issue-20186.rs:12:1
[00:55:55]    |
[00:55:55] LL | struct Foo;
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] warning: method is never used: `putc`
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     fn putc(&self, b: u8) { }
[00:55:55] 
[00:55:55] 
[00:55:55] warning: method is never used: `puts`
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL |     fn puts(&self, s: &str) {
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-20186/issue-20186.stderr
[00:55:55] To update references, rerun the ":[{"file_name":"/checkout/src/test/run-pass/issues/issue-20186.rs","byte_start":507,"byte_end":528,"line_start":15,"line_end":15,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    fn putc(&self, b: u8) { }","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: method is never used: `putc`\n  --> /checkout/src/test/run-pass/issues/issue-20186.rs:15:5\n   |\nLL |     fn putc(&self, b: u8) { }\n   |     ^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:55:55] {"message":"method is never used: `puts`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-20186.rs","byte_start":538,"byte_end":561,"line_start":17,"line_end":17,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    fn puts(&self, s: &str) {","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: method is never used: `puts`\n  --> /checkout/src/test/run-pass/issues/issue-20186.rs:17:5\n   |\nLL |     fn puts(&self, s: &str) {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/issues/issue-20186.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/issues/issue-20313.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: foreign function is never used: `sqrt`kout/src/test/run-pass/issues/issue-20313.rs","byte_start":595,"byte_end":618,"line_start":18,"line_end":18,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    fn sqrt(x: f32) -> f32;","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: foreign function is never used: `sqrt`\n  --> /checkout/src/test/run-pass/issues/issue-20313.rs:18:5\n   |\nLL |     fn sqrt(x: f32) -> f32;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/issues/issue-20313.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/issues/issue-20343.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused variable: `a`
[00:55:55]   --> $DIR/issue-20343.rs:22:30
[00:55:55]    |
[00:55:55] LL | trait T<A> { fn dummy(&self, a: A) { } }
[00:55:55]    |
[00:55:55]    = note: #[warn(unused_variables)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-20343/issue-20343.stderr
[00:55:55] To update references_a` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-20343.rs","byte_start":641,"byte_end":642,"line_start":22,"line_end":22,"column_start":30,"column_end":31,"is_primary":true,"text":[{"text":"trait T<A> { fn dummy(&self, a: A) { } }","highlight_start":30,"highlight_end":31}],"label":null,"suggested_replacement":"_a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `a`\n  --> /checkout/src/test/run-pass/issues/issue-20343.rs:22:30\n   |\nLL | trait T<A> { fn dummy(&self, a: A) { } }\n   |                              ^ help: consider using `_a` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/issues/issue-20343.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/issues/issue-20389.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: struct is never constructed: `Foo`
[00:55:55]   --> $DIR/issue-20389.rs:18:1
[00:55:55]    |
[00:55:55] LL | struct Foo;
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-20389/issue-20389.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args issues/issue-20389.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-20389.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-20389/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-20389/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"struct is never constructed: `Foo`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-20389.rs","byte_start":568,"byte_end":579,"line_start":18,"line_end":18,"column_start":1,"column_end":12,"is_primary":true,"text":[{"text":"struct Foo;","highlight_start":1,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: struct is never constructed: `Foo`\n  --> /checkout/src/test/run-pass/issues/issue-20389.rs:18:1\n   |\nLL | struct Foo;\n   | ^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/issues/issue-20389.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/issues/issue-20414.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: struct is never constructed: `Wrapper`
[00:55:55]   --> $DIR/issue-20414.rs:18:1
[00:55:55]    |
[00:55:55] LL | struct Wrapper<T> {
[00:55:55]    |
[00:55:55]    = note: #[warn(dead_code)] on by default
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] 
[00:55:55] The actual stderr differed from the expected stderr.
[00:55:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-20414/issue-20414.stderr
[00:55:55] To update references, rerun the tests and pass the `--bless` flag
[00:55:55] To only update this specific test, also pass `--test-args issues/issue-20414.rs`
[00:55:55] error: 1 errors occurred comparing output.
[00:55:55] status: exit code: 0
[00:55:55] status: exit code: 0
[00:55:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-20414.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-20414/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-20414/auxiliary"
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] ------------------------------------------
[00:55:55] stderr:
[00:55:55] stderr:
[00:55:55] ------------------------------------------
[00:55:55] {"message":"struct is never constructed: `Wrapper`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-20414.rs","byte_start":563,"byte_end":580,"line_start":18,"line_end":18,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"struct Wrapper<T> {","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: struct is never constructed: `Wrapper`\n  --> /checkout/src/test/run-pass/issues/issue-20414.rs:18:1\n   |\nLL | struct Wrapper<T> {\n   | ^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:55:55] ------------------------------------------
[00:55:55] 
[00:55:55] thread '[run-pass] run-pass/issues/issue-20414.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:55:55] 
[00:55:55] 
[00:55:55] ---- [run-pass] run-pass/issues/issue-20454.rs stdout ----
[00:55:55] normalized stderr:
[00:55:55] warning: unused `std::result::Result` which must be used
[00:55:55]    |
[00:55:55]    |
[00:55:55] LL | /     thread::spawn(move || { // no need for -> ()
[00:55:55] LL | |         loop {
[00:55:55] LL | |             println!("hello");
[00:55:55] LL | |         }
[00:55:55] LL | |     }).join();
[00:55:55]    |
[00:55:55]    = note: #[warn(unused_must_use)] on by default
[00:55:55]    = note: #[warn(unused_must_use)] on by default
[00:55:55]    = note: this `Result` may be an `Err` variant, which should be handled
