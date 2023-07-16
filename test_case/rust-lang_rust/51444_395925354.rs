plain
[00:52:15] ....i..............................................................................i................
[00:52:20] ....................................................................................................
[00:52:26] ....................................................................................................
[00:52:32] ....................................................................................................
[00:52:38] ................i.................iiiiiiiii...................................................
[00:52:38] 
[00:52:38] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:52:53] ....................................................................................................
[00:52:57] ....................................................................................................
[00:53:01] ....................................................................................................
[00:53:04] ....................................................................................................
[00:53:09] ......................................................................F.............................
[00:53:20] ....................................................................................................
[00:53:26] ....................................................................................................
[00:53:31] ....i..............................................................................i................
[00:53:36] ....................................................................................................
[00:53:36] ....................................................................................................
[00:53:41] ....................................................................................................
[00:53:47] ....................................................................................................
[00:53:52] ................i.................iiiiiiiii...................................................
[00:53:52] 
[00:53:52] ---- [ui (nll)] ui/impl-trait/static-return-lifetime-infered.rs stdout ----
[00:53:52] diff of stderr:
[00:53:52] 
[00:53:52] 
[00:53:52] - error: cannot infer an appropriate lifetime
[00:53:52] + warning: not reporting region error due to nll
[00:53:52] 2   --> $DIR/static-return-lifetime-infered.rs:17:16
[00:53:52] 3    |
[00:53:52] - LL |     fn iter_values_anon(&self) -> impl Iterator<Item=u32> {
[00:53:52] -    |                                   ----------------------- this return type evaluates to the `'static` lifetime...
[00:53:52] 6 LL |         self.x.iter().map(|a| a.0)
[00:53:52] -    |         ------ ^^^^
[00:53:52] -    |         |
[00:53:52] -    |         ...but this borrow...
[00:53:52] -    |
[00:53:52] - note: ...can't outlive the anonymous lifetime #1 defined on the method body at 16:5
[00:53:52] -   --> $DIR/static-return-lifetime-infered.rs:16:5
[00:53:52] -    |
[00:53:52] - LL | /     fn iter_values_anon(&self) -> impl Iterator<Item=u32> {
[00:53:52] - LL | |         self.x.iter().map(|a| a.0)
[00:53:52] - LL | |     }
[00:53:52] -    | |_____^
[00:53:52] - help: you can add a constraint to the return type to make it last less than `'static` and match the anonymous lifetime #1 defined on the method body at 16:5
[00:53:52] -    |
[00:53:52] - LL |     fn iter_values_anon(&self) -> impl Iterator<Item=u32> + '_ {
[00:53:52] +    |                ^^^^
[00:53:52] 22 
[00:53:52] - error: cannot infer an appropriate lifetime
[00:53:52] - error: cannot infer an appropriate lifetime
[00:53:52] + warning: not reporting region error due to nll
[00:53:52] 24   --> $DIR/static-return-lifetime-infered.rs:21:16
[00:53:52] 25    |
[00:53:52] - LL |     fn iter_values<'a>(&'a self) -> impl Iterator<Item=u32> {
[00:53:52] -    |                                     ----------------------- this return type evaluates to the `'static` lifetime...
[00:53:52] 28 LL |         self.x.iter().map(|a| a.0)
[00:53:52] -    |         ------ ^^^^
[00:53:52] -    |         |
[00:53:52] -    |         ...but this borrow...
[00:53:52] + 
[00:53:52] + 
[00:53:52] + error: free region `` does not outlive free region `'static`
[00:53:52] +   --> $DIR/static-return-lifetime-infered.rs:17:9
[00:53:52] 32    |
[00:53:52] - note: ...can't outlive the lifetime 'a as defined on the method body at 20:5
[00:53:52] -   --> $DIR/static-return-lifetime-infered.rs:20:5
[00:53:52] + LL |         self.x.iter().map(|a| a.0)
[00:53:52] + 
[00:53:52] + 
[00:53:52] + error: free region `'a` does not outlive free region `'static`
[00:53:52] +   --> $DIR/static-return-lifetime-infered.rs:21:9
[00:53:52] 35    |
[00:53:52] - LL |     fn iter_values<'a>(&'a self) -> impl Iterator<Item=u32> {
[00:53:52] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:53:52] - help: you can add a constraint to the return type to make it last less than `'static` and match the lifetime 'a as defined on the method body at 20:5
[00:53:52] -    |
[00:53:52] - LL |     fn iter_values<'a>(&'a self) -> impl Iterator<Item=u32> + '_ {
[00:53:52] -    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:53:52] + LL |         self.x.iter().map(|a| a.0)
[00:53:52] 42 
[00:53:52] 43 error: aborting due to 2 previous errors
[00:53:52] 44 
[00:53:52] 
[00:53:52] 
[00:53:52] 
[00:53:52] The actual stderr differed from the expected stderr.
[00:53:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/static-return-lifetime-infered.nll/static-return-lifetime-infered.nll.stderr
[00:53:52] To update references, rerun the tests and pass the `--bless` flag
[00:53:52] To only update this specific test, also pass `--test-args impl-trait/static-return-lifetime-infered.rs`
[00:53:52] error: 1 errors occurred comparing output.
[00:53:52] status: exit code: 101
[00:53:52] status: exit code: 101
[00:53:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/static-return-lifetime-infered.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/static-return-lifetime-infered.nll/auxiliary" "-A" "unused"
[00:53:52] ------------------------------------------
[00:53:52] 
[00:53:52] ------------------------------------------
[00:53:52] stderr:
[00:53:52] stderr:
[00:53:52] ------------------------------------------
[00:53:52] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs","byte_start":589,"byte_end":593,"line_start":17,"line_end":17,"column_start":16,"column_end":20,"is_primary":true,"text":[{"text":"        self.x.iter().map(|a| a.0)","highlight_start":16,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs:17:16\n   |\nLL |         self.x.iter().map(|a| a.0)\n   |                ^^^^\n\n"}
[00:53:52] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs","byte_start":745,"byte_end":749,"line_start":21,"line_end":21,"column_start":16,"column_end":20,"is_primary":true,"text":[{"text":"        self.x.iter().map(|a| a.0)","highlight_start":16,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs:21:16\n   |\nLL |         self.x.iter().map(|a| a.0)\n   |                ^^^^\n\n"}
[00:53:52] {"message":"free region `` does not outlive free region `'static`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs","byte_start":582,"byte_end":595,"line_start":17,"line_end":17,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"        self.x.iter().map(|a| a.0)","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: free region `` does not outlive free region `'static`\n  --> /checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs:17:9\n   |\nLL |         self.x.iter().map(|a| a.0)\n   |         ^^^^^^^^^^^^^\n\n"}
[00:53:52] {"message":"free region `'a` does not outlive free region `'static`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs","byte_start":738,"byte_end":751,"line_start":21,"line_end":21,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"        self.x.iter().map(|a| a.0)","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: free region `'a` does not outlive free region `'static`\n  --> /checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs:21:9\n   |\nLL |         self.x.iter().map(|a| a.0)\n   |         ^^^^^^^^^^^^^\n\n"}
[00:53:52] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:53:52] ------------------------------------------
[00:53:52] 
[00:53:52] thread '[ui (nll)] ui/impl-trait/static-return-lifetime-infered.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:53:52] note: Run with `RUST_BACKTRACE=1` for a backtrace.
