
[00:52:28] ---- [run-pass] run-pass/impl-trait/example-calendar.rs stdout ----
[00:52:28] 	
[00:52:28] error: compilation failed!
[00:52:28] status: exit code: 101
[00:52:28] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/run-pass/impl-trait/example-calendar.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass --target=x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/impl-trait/example-calendar.stage2-x86_64-unknown-linux-gnu.run-pass.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/impl-trait/example-calendar.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:52:28] stdout:
[00:52:28] ------------------------------------------
[00:52:28] 
[00:52:28] ------------------------------------------
[00:52:28] stderr:
[00:52:28] ------------------------------------------
[00:52:28] error[E0407]: method `step` is not a member of trait `std::iter::Step`
[00:52:28]    --> /checkout/src/test/run-pass/impl-trait/example-calendar.rs:165:5
[00:52:28]     |
[00:52:28] 165 | /     fn step(&self, by: &Self) -> Option<Self> {
[00:52:28] 166 | |         Some(self + by)
[00:52:28] 167 | |     }
[00:52:28]     | |_____^ not a member of trait `std::iter::Step`
[00:52:28] 
[00:52:28] error[E0407]: method `steps_between_by_one` is not a member of trait `std::iter::Step`
[00:52:28]    --> /checkout/src/test/run-pass/impl-trait/example-calendar.rs:173:5
[00:52:28]     |
[00:52:28] 173 | /     fn steps_between_by_one(_: &Self, _: &Self) -> Option<usize> {
[00:52:28] 174 | |         unimplemented!()
[00:52:28] 175 | |     }
[00:52:28]     | |_____^ not a member of trait `std::iter::Step`
[00:52:28] 
[00:52:28] error[E0407]: method `is_negative` is not a member of trait `std::iter::Step`
[00:52:28]    --> /checkout/src/test/run-pass/impl-trait/example-calendar.rs:177:5
[00:52:28]     |
[00:52:28] 177 | /     fn is_negative(&self) -> bool {
[00:52:28] 178 | |         false
[00:52:28] 179 | |     }
[00:52:28]     | |_____^ not a member of trait `std::iter::Step`
[00:52:28] 
[00:52:28] error[E0050]: method `steps_between` has 3 parameters but the declaration in trait `std::iter::Step::steps_between` has 2
[00:52:28]    --> /checkout/src/test/run-pass/impl-trait/example-calendar.rs:169:45
[00:52:28]     |
[00:52:28] 169 |     fn steps_between(_: &Self, _: &Self, _: &Self) -> Option<usize> {
[00:52:28]     |                                             ^^^^^ expected 2 parameters, found 3
[00:52:28]     |
[00:52:28]     = note: `steps_between` from trait: `fn(&Self, &Self) -> std::option::Option<usize>`
[00:52:28] 
[00:52:28] error[E0046]: not all trait items implemented, missing: `add_n`
[00:52:28]    --> /checkout/src/test/run-pass/impl-trait/example-calendar.rs:164:1
[00:52:28]     |
[00:52:28] 164 | / impl std::iter::Step for NaiveDate {
[00:52:28] 165 | |     fn step(&self, by: &Self) -> Option<Self> {
[00:52:28] 166 | |         Some(self + by)
[00:52:28] 167 | |     }
[00:52:28] ...   |
[00:52:28] 195 | |     }
[00:52:28] 196 | | }
[00:52:28]     | |_^ missing `add_n` in implementation
[00:52:28]     |
[00:52:28]     = note: `add_n` from trait: `fn(&Self, usize) -> std::option::Option<Self>`
[00:52:28] 
[00:52:28] error: aborting due to 5 previous errors
[00:52:28] 
[00:52:28] 
[00:52:28] ------------------------------------------
[00:52:28] 
[00:52:28] thread '[run-pass] run-pass/impl-trait/example-calendar.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2473:8
[00:52:28] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:52:28] 
[00:52:28] 
[00:52:28] failures:
[00:52:28]     [run-pass] run-pass/impl-trait/example-calendar.rs
