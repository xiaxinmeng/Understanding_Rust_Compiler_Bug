plain
[TIMING] Std { target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 0.000
Build completed successfully in 0:20:46
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-type=lib ../library/core/src/lib.rs
error[E0501]: cannot borrow `self.iter` as mutable because previous closure requires unique access
   --> ../library/core/src/iter/adapters/mod.rs:182:9
182 | /         self.iter
182 | /         self.iter
183 | |             .try_fold(init, |acc, x| match Try::branch(x) {
    | |              --------       -------- closure construction occurs here
    | |              first borrow later used by call
    | |              first borrow later used by call
184 | |                 ControlFlow::Continue(x) => ControlFlow::from_try(f(acc, x)),
185 | |                 ControlFlow::Break(r) => {
186 | |                     *self.residual = Some(r);
    | |                     -------------- first borrow occurs due to use of `self` in closure
187 | |                     ControlFlow::Break(try { acc })
189 | |             })
189 | |             })
    | |______________^ second borrow occurs here

error[E0500]: closure requires unique access to `self` but it is already borrowed
   --> ../library/core/src/iter/adapters/mod.rs:183:29
182 | /         self.iter
182 | /         self.iter
183 | |             .try_fold(init, |acc, x| match Try::branch(x) {
    | |              --------       ^^^^^^^^ closure construction occurs here
    | |              first borrow later used by call
    | |              first borrow later used by call
184 | |                 ControlFlow::Continue(x) => ControlFlow::from_try(f(acc, x)),
185 | |                 ControlFlow::Break(r) => {
186 | |                     *self.residual = Some(r);
    | |                     -------------- second borrow occurs due to use of `self` in closure
187 | |                     ControlFlow::Break(try { acc })
189 | |             })
189 | |             })
    | |______________- borrow occurs here
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0500, E0501.
For more information about an error, try `rustc --explain E0500`.
