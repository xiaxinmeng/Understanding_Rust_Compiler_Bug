compile_fail,E0017\nstatic X: i32 = 1;\nco-----------------------
[01:00:57] thread '[ui] ui/issues/issue-17718-const-bad-values.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:00:57] 
[01:00:57] ---- [ui] ui/issues/issue-17718-const-borrow.rs stdout ----
[01:00:57] diff of stderr:
[01:00:57] diff of stderr:
[01:00:57] 
[01:00:57] 1 error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
[01:00:57] -    |
[01:00:57] -    |
[01:00:57] - LL | const B: &'static UnsafeCell<usize> = &A;
[01:00:57] - 
[01:00:57] - 
[01:00:57] - error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
[01:00:57] 9    |
[01:00:57] 9    |
[01:00:57] 10 LL | const E: &'static UnsafeCell<usize> = &D.a;
[01:00:57] 15    |
[01:00:57] 15    |
[01:00:57] 16 LL | const F: &'static C = &D;
[01:00:57] + 
[01:00:57] + 
[01:00:57] + error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
[01:00:57] +   --> $DIR/issue-17718-const-borrow.rs:14:39
[01:00:57] +    |
[01:00:57] + LL | const B: &'static UnsafeCell<usize> = &A;
[01:00:57] 18 
[01:00:57] 19 error: aborting due to 3 previous errors
[01:00:57] 20 
[01:00:57] 
[01:00:57] 
[01:00:57] 
[01:00:57] The actual stderr differed from the expected stderr.
[01:00:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-const-borrow/issue-1er this solution is unsafe! You will have to ensure that accesses to the\ncell are synchronized.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-17718-const-borrow.rs","byte_start":776,"byte_end":780,"line_start":19,"line_end":19,"column_start":39,"column_end":43,"is_primary":true,"text":[{"text":"const E: &'static UnsafeCell<usize> = &D.a;","highlight_start":39,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead\n  --> /checkout/src/test/ui/issues/issue-17718-const-borrow.rs:19:39\n   |\nLL | const E: &'static UnsafeCell<usize> = &D.a;\n   |                                       ^^^^\n\n"}
[01:00:57] {"message":"cannot borrow a constant which may contain interior mutability, create a static instead","code":{"code":"E0492","explanation":"\nA borrow of a constant containing interior mutability was attempted. Erroneous\ncode example:\n\n