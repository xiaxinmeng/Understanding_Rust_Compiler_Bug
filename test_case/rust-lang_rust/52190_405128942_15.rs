\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/generator/yield-while-local-borrowed.rs","byte_start":1774,"byte_end":1781,"line_start":55,"line_end":55,"column_start":13,"column_end":20,"is_primary":false,"text":[{"text":"            yield();","highlight_start":13,"highlight_end":20}],"label":"possible yield occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/generator/yield-while-local-borrowed.rs","byte_start":1603,"byte_end":1604,"line_start":52,"line_end":52,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"            let b = &a;","highlight_start":22,"highlight_end":23}],"lab] ui/generator/yield-while-local-borrowed.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
[00:42:59] ---- [ui] ui/impl-trait/region-escape-via-bound-contravariant-closure.rs stdout ----
[00:42:59] 
[00:42:59] 
[00:42:59] error: test compilation failed although it shouldn't!
[00:42:59] status: exit code: 101
[00:42:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/region-escape-via-bound-contravariant-closure.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/region-escape-via-bound-contravariant-closure/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/region-escape-via-bound-contravariant-closure/auxiliary" "-A" "unused"
[00:42:59] ------------------------------------------
[00:42:59] 
[00:42:59] ------------------------------------------
[00:42:59] stderr:
[00:42:59] stderr:
[00:42:59] ------------------------------------------
[00:42:59] thread 'main' panicked at 'index out of bounds: the len is 6 but the index is 6', /checkout/src/libcore/slice/mod.rs:2085:10
[00:42:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:42:59] 
[00:42:59] error: internal compiler error: unexpected panic
[00:42:59] 
[00:42:59] note: the compiler unexpectedly panicked. this is a bug.
[00:42:59] 
[00:42:59] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:42:59] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:42:59] 
[00:42:59] 
[00:42:59] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:42:59] 
[00:42:59] ------------------------------------------
[00:42:59] 
[00:42:59] thread '[ui] ui/impl-trait/region-escape-via-bound-contravariant-closure.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
[00:42:59] thread '[ui] ui/impl-trait/region-escape-via-bound-contravariant-closure.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
[00:42:59] 
[00:42:59] ---- [ui] ui/issue-27282-move-match-input-into-guard.rs stdout ----
[00:42:59] diff of stderr:
[00:42:59] 
[00:42:59] - error[E0505]: cannot move out of `b` because it is borrowed
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |        match b {
[00:42:59] -    |   _____-
[00:42:59] -    |  |_____|
[00:42:59] -    | ||
[00:42:59] - LL | ||         &mut false => {},
[00:42:59] - LL | ||         _ if { (|| { let bar = b; *bar = false; })();
[00:42:59] -    | ||                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ move out of `b` occurs here
[00:42:59] - LL | ||                      //~^ ERROR cannot move out of `b` because it is borrowed [E0505]
[00:42:59] - ...  ||
[00:42:59] - LL | ||         _ => panic!("surely we could never get here, since rustc warns it is unreachable."),
[00:42:59] - LL | ||     }
[00:42:59] -    | ||     -
[00:42:59] -    | ||_____|
[00:42:59] -    | |______borrow of `b` occurs here
[00:42:59] -    |        borrow later used here
[00:42:59] - 
[00:42:59] - error[E0382]: use of moved value: `*b`
[00:42:59] -   --> $DIR/isr-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-27282-mutate-before-diverging-arm-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-27282-mutate-before-diverging-arm-1/auxiliary" "-A" "unused"
[00:42:59] ------------------------------------------
[00:42:59] 
[00:42:59] ------------------------------------------
[00:42:59] stderr:
[00:42:59] stderr:
[00:42:59] ------------------------------------------
[00:42:59] {"message":"librustc_mir/borrow_check/nll/region_infer/error_reporting/mod.rs:289: could not find any constraint to blame for '_#6r: bb11[3]","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc_mir/borrow_check/nll/region_infer/error_reporting/mod.rs:289: could not find any constraint to blame for '_#6r: bb11[3]\n\n"}
[00:42:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:42:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:42:59] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:42:59] 
[00:42:59] note: the compiler unexpectedly panicked. this is a bug.
[00:42:59] 
[00:42:59] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:42:59] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:42:59] 
[00:42:59] 
[00:42:59] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:42:59] 
[00:42:59] ------------------------------------------
[00:42:59] 
[00:42:59] thread '[ui] ui/issue-27282-mutate-before-diverging-arm-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
[00:42:59] thread '[ui] ui/issue-27282-mutate-before-diverging-arm-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
[00:42:59] 
[00:42:59] ---- [ui] ui/issue-27282-mutate-before-diverging-arm-2.rs stdout ----
[00:42:59] diff of stderr:
[00:42:59] 
[00:42:59] - error[E0500]: closure requires unique access to `x` but it is already borrowed
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |        match x {
[00:42:59] -    |   _____-
[00:42:59] -    |  |_____|
[00:42:59] -    | ||
[00:42:59] - LL | ||         &mut None => panic!("unreachable"),
[00:42:59] - LL | ||         &mut Some(&_)
[00:42:59] - LL | ||             if {
[00:42:59] - LL | ||                 // ForceFnOnce needed to exploit #27282
[00:42:59] - LL | ||                 (|| { *x = None; drop(force_fn_once); })();
[00:42:59] -    | ||                  ^^    - borrow occurs due to use of `x` in closure
[00:42:59] -    | ||                  |
[00:42:59] -    | ||                  closure construction occurs here
[00:42:59] - ...  ||
[00:42:59] - LL | ||         _ => panic!("unreachable"),
[00:42:59] - LL | ||     }
[00:42:59] -    | ||     -
[00:42:59] -    | ||_____|
[00:42:59] -    | |______borrow occurs here
[00:42:59] -    |        borrow later used here
[00:42:59] + error: internal compiler error: librustc_mir/borrow_check/nll/region_infer/error_reporting/mod.r9] {"message":"librustc_mir/borrow_check/nll/region_infer/error_reporting/mod.rs:289: could not find any constraint to blame for '_#6r: bb11[3]","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc_mir/borrow_check/nll/region_infer/error_reporting/mod.rs:289: could not find any constraint to blame for '_#6r: bb11[3]\n\n"}
[00:42:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:42:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:42:59] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:42:59] 
[00:42:59] note: the compiler unexpectedly panicked. this is a bug.
[00:42:59] 
[00:42:59] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:42:59] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:42:59] 
[00:42:59] 
[00:42:59] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:42:59] 
[00:42:59] ------------------------------------------
[00:42:59] 
[00:42:59] thread '[ui] ui/issue-27282-mutate-before-diverging-arm-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
[00:42:59] thread '[ui] ui/issue-27282-mutate-before-diverging-arm-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
[00:42:59] 
[00:42:59] ---- [ui] ui/issue-27282-reborrow-ref-mut-in-guard.rs stdout ----
[00:42:59] diff of stderr:
[00:42:59] 
[00:42:59] - error[E0596]: cannot borrow immutable item `*r` as mutable
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |         ref mut r if { (|| { let bar = &mut *r; **bar = false; })();
[00:42:59] -    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
[00:42:59] - error: aborting due to previous error
[00:42:59] - 
[00:42:59] - For more information about this error, try `rustc --explain E0596`.
[00:42:59] - 
[00:42:59] - 
[00:42:59] 
[00:42:59] 
[00:42:59] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-27282-reborrow-ref-mut-in-guard/issue-27282-reborrow-ref-mut-in-guard.stderr`: No such file or directory (os error 2)
[00:42:59] 
[00:42:59] ---- [ui] ui/issue-45157.rs stdout ----
[00:42:59] diff of stderr:
[00:42:59] 
[00:42:59] 
[00:42:59] - error[E0502]: cannot borrow `u.z.c` as immutable because it is also borrowed as mutable
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |         let mref = &mut u.s.a;
[00:42:59] -    |                    ---------- mutable borrow occurs here
[00:42:59] - ...
[00:42:59] - LL |         let nref = &u.z.c;
[00:42:59] -    |                    ^^^^^^ immutable borrow occurs here
[00:42:59] - LL |         //~^ ERROR cannot borrow `u.z.c` as immutable because it is also borrowed as mutable [E0502]
[00:42:59] - LL |         println!("{} {}", mref, nref)
[00:42:59] -    |                           ---- borrow later used here
[00:42:59] - error: aborting due to previous error
[00:42:59] - 
[00:42:59] - For more information about this error, try `rustc --explain E0502`.
[00:42:59] - 
[00:42:59] - 
[00:42:59] 
[00:42:59] 
[00:42:59] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-45157/issue-45157.stderr`: No such file or directory (os error 2)
[00:42:59] 
[00:42:59] ---- [ui] ui/issue-45697-1.rs stdout ----
[00:42:59] diff of stderr:
[00:42:59] 
[00:42:59] 
[00:42:59] 6 LL |         *y.pointer += 1;
[00:42:59] 7    |         ^^^^^^^^^^^^^^^ assignment to borrowed `*y.pointer` occurs here
[00:42:59] 8 
[00:42:59] - error[E0503]: cannot use `*y.pointer` because it was mutably borrowed (Mir)
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |         let z = copy_borrowed_ptr(&mut y);
[00:42:59] -    |                                   ------ borrow of `y` occurs here
[00:42:59] - LL |         *y.pointer += 1;
[00:42:59] -    |         ^^^^^^^^^^^^^^^ use of borrowed `y`
[00:42:59] - ...
[00:42:59] - LL |         *z.pointer += 1;
[00:42:59] -    |         --------------- borrow later used here
[00:42:59] + error: aborting due to previous error
[00:42:59] 19 
[00:42:59] - error[E0506]: cannot assign to `*y.pointer` because it is borrowed (Mir)
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |         let z = copy_borrowed_ptr(&mut y);
[00:42:59] -    |                                   ------ borrow of `*y.pointer` occurs here
[00:42:59] - LL |         *y.pointer += 1;
[00:42:59] -    |         ^^^^^^^^^^^^^^^ assignment to bo