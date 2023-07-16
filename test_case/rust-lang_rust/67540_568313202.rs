plain
2019-12-22T23:40:42.7086920Z ---- [ui] ui/async-await/issues/issue-62009-1.rs stdout ----
2019-12-22T23:40:42.7087330Z diff of stderr:
2019-12-22T23:40:42.7087920Z 
2019-12-22T23:40:42.7088090Z 35    | 
2019-12-22T23:40:42.7088850Z 36   ::: $SRC_DIR/libstd/future.rs:LL:COL
2019-12-22T23:40:42.7090040Z - LL |     F: Future
2019-12-22T23:40:42.7090520Z + LL |     F: Future,
2019-12-22T23:40:42.7090520Z + LL |     F: Future,
2019-12-22T23:40:42.7091490Z 39    |        ------ required by this bound in `std::future::poll_with_tls_context`
2019-12-22T23:40:42.7092170Z 41 error: aborting due to 4 previous errors
2019-12-22T23:40:42.7092290Z 
2019-12-22T23:40:42.7092400Z 
2019-12-22T23:40:42.7092580Z The actual stderr differed from the expected stderr.
2019-12-22T23:40:42.7092580Z The actual stderr differed from the expected stderr.
2019-12-22T23:40:42.7093610Z Actual stderr saved to /Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/test/ui/async-await/issues/issue-62009-1/issue-62009-1.stderr
2019-12-22T23:40:42.7094800Z To update references, rerun the tests and pass the `--bless` flag
2019-12-22T23:40:42.7096120Z To only update this specific test, also pass `--test-args async-await/issues/issue-62009-1.rs`
2019-12-22T23:40:42.7096890Z error: 1 errors occurred comparing output.
2019-12-22T23:40:42.7097090Z status: exit code: 1
2019-12-22T23:40:42.7097090Z status: exit code: 1
2019-12-22T23:40:42.7099180Z command: "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui/async-await/issues/issue-62009-1.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/test/ui/async-await/issues/issue-62009-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--edition=2018" "-L" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/test/ui/async-await/issues/issue-62009-1/auxiliary" "-A" "unused"
2019-12-22T23:40:42.7100900Z ------------------------------------------
2019-12-22T23:40:42.7101290Z 
2019-12-22T23:40:42.7102230Z ------------------------------------------
2019-12-22T23:40:42.7102700Z stderr:
2019-12-22T23:40:42.7102700Z stderr:
2019-12-22T23:40:42.7103570Z ------------------------------------------
2019-12-22T23:40:42.7103700Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-12-22T23:40:42.7104870Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/async-await/issues/issue-62009-1.rs:7:5
2019-12-22T23:40:42.7105580Z LL | fn main() {
2019-12-22T23:40:42.7106360Z    |    ---- this is not `async`
2019-12-22T23:40:42.7106360Z    |    ---- this is not `async`
2019-12-22T23:40:42.7106830Z LL |     async { let (); }.await;
2019-12-22T23:40:42.7107070Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-12-22T23:40:42.7107200Z 
2019-12-22T23:40:42.7107370Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-12-22T23:40:42.7108440Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/async-await/issues/issue-62009-1.rs:9:5
2019-12-22T23:40:42.7109480Z LL |   fn main() {
2019-12-22T23:40:42.7110420Z    |      ---- this is not `async`
2019-12-22T23:40:42.7110880Z ...
2019-12-22T23:40:42.7111090Z LL | /     async {
2019-12-22T23:40:42.7111090Z LL | /     async {
2019-12-22T23:40:42.7111290Z LL | |     //~^ ERROR `await` is only allowed inside `async` functions and blocks
2019-12-22T23:40:42.7111470Z LL | |         let task1 = print_dur().await;
2019-12-22T23:40:42.7111600Z LL | |     }.await;
2019-12-22T23:40:42.7111780Z    | |___________^ only allowed inside `async` functions and blocks
2019-12-22T23:40:42.7111920Z 
2019-12-22T23:40:42.7112090Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-12-22T23:40:42.7113070Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/async-await/issues/issue-62009-1.rs:13:5
2019-12-22T23:40:42.7113620Z LL | fn main() {
2019-12-22T23:40:42.7114500Z    |    ---- this is not `async`
2019-12-22T23:40:42.7114950Z ...
2019-12-22T23:40:42.7114950Z ...
2019-12-22T23:40:42.7115170Z LL |     (|_| 2333).await;
2019-12-22T23:40:42.7115380Z    |     ^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-12-22T23:40:42.7115530Z 
2019-12-22T23:40:42.7116560Z error[E0277]: the trait bound `[closure@/Users/runner/runners/2.163.1/work/1/s/src/test/ui/async-await/issues/issue-62009-1.rs:13:5: 13:15]: std::future::Future` is not satisfied
2019-12-22T23:40:42.7117800Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/async-await/issues/issue-62009-1.rs:13:5
2019-12-22T23:40:42.7118350Z    |
2019-12-22T23:40:42.7118560Z LL |     (|_| 2333).await;
2019-12-22T23:40:42.7119630Z    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@/Users/runner/runners/2.163.1/work/1/s/src/test/ui/async-await/issues/issue-62009-1.rs:13:5: 13:15]`
2019-12-22T23:40:42.7120240Z   ::: /Users/runner/runners/2.163.1/work/1/s/src/libstd/future.rs:78:8
2019-12-22T23:40:42.7120470Z    |
2019-12-22T23:40:42.7120630Z LL |     F: Future,
2019-12-22T23:40:42.7120630Z LL |     F: Future,
2019-12-22T23:40:42.7121750Z    |        ------ required by this bound in `std::future::poll_with_tls_context`
2019-12-22T23:40:42.7122320Z error: aborting due to 4 previous errors
2019-12-22T23:40:42.7122460Z 
2019-12-22T23:40:42.7122650Z Some errors have detailed explanations: E0277, E0728.
2019-12-22T23:40:42.7123530Z For more information about an error, try `rustc --explain E0277`.
---
2019-12-22T23:40:42.7126190Z ---- [ui] ui/closures/closure-move-sync.rs stdout ----
2019-12-22T23:40:42.7126650Z diff of stderr:
2019-12-22T23:40:42.7126820Z 
2019-12-22T23:40:42.7126980Z 6    | 
2019-12-22T23:40:42.7127150Z 7   ::: $SRC_DIR/libstd/thread/mod.rs:LL:COL
2019-12-22T23:40:42.7127270Z 8    |
2019-12-22T23:40:42.7128140Z - LL |     F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
2019-12-22T23:40:42.7129300Z -    |                          ---- required by this bound in `std::thread::spawn`
2019-12-22T23:40:42.7130480Z + LL |     F: Send + 'static,
2019-12-22T23:40:42.7131630Z +    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:40:42.7132320Z 12    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Receiver<()>`
2019-12-22T23:40:42.7132550Z 13    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
2019-12-22T23:40:42.7132710Z 
2019-12-22T23:40:42.7132820Z 21    | 
2019-12-22T23:40:42.7132820Z 21    | 
2019-12-22T23:40:42.7133000Z 22   ::: $SRC_DIR/libstd/thread/mod.rs:LL:COL
2019-12-22T23:40:42.7133190Z 23    |
2019-12-22T23:40:42.7134010Z - LL |     F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
2019-12-22T23:40:42.7135340Z -    |                          ---- required by this bound in `std::thread::spawn`
2019-12-22T23:40:42.7136450Z + LL |     F: Send + 'static,
2019-12-22T23:40:42.7137590Z +    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:40:42.7138500Z 27    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Sender<()>`
2019-12-22T23:40:42.7138690Z 28    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Sender<()>`
2019-12-22T23:40:42.7139180Z 
2019-12-22T23:40:42.7139360Z 
2019-12-22T23:40:42.7139360Z 
2019-12-22T23:40:42.7139530Z The actual stderr differed from the expected stderr.
2019-12-22T23:40:42.7140540Z Actual stderr saved to /Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/test/ui/closures/closure-move-sync/closure-move-sync.stderr
2019-12-22T23:40:42.7141710Z To update references, rerun the tests and pass the `--bless` flag
2019-12-22T23:40:42.7142900Z To only update this specific test, also pass `--test-args closures/closure-move-sync.rs`
2019-12-22T23:40:42.7143530Z error: 1 errors occurred comparing output.
2019-12-22T23:40:42.7143740Z status: exit code: 1
2019-12-22T23:40:42.7143740Z status: exit code: 1
2019-12-22T23:40:42.7145470Z command: "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui/closures/closure-move-sync.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/test/ui/closures/closure-move-sync" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/test/ui/closures/closure-move-sync/auxiliary" "-A" "unused"
2019-12-22T23:40:42.7147060Z ------------------------------------------
2019-12-22T23:40:42.7147120Z 
2019-12-22T23:40:42.7148230Z ------------------------------------------
2019-12-22T23:40:42.7148720Z stderr:
2019-12-22T23:40:42.7148720Z stderr:
2019-12-22T23:40:42.7149750Z ------------------------------------------
2019-12-22T23:40:42.7149910Z error[E0277]: `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
2019-12-22T23:40:42.7151590Z    |
2019-12-22T23:40:42.7151590Z    |
2019-12-22T23:40:42.7151810Z LL |     let t = thread::spawn(|| {
2019-12-22T23:40:42.7152000Z    |             ^^^^^^^^^^^^^ `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
2019-12-22T23:40:42.7152320Z   ::: /Users/runner/runners/2.163.1/work/1/s/src/libstd/thread/mod.rs:616:8
2019-12-22T23:40:42.7152530Z    |
2019-12-22T23:40:42.7153370Z LL |     F: Send + 'static,
2019-12-22T23:40:42.7154510Z    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:40:42.7154510Z    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:40:42.7154990Z    |
2019-12-22T23:40:42.7155200Z    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Receiver<()>`
2019-12-22T23:40:42.7155470Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
2019-12-22T23:40:42.7156570Z    = note: required because it appears within the type `[closure@/Users/runner/runners/2.163.1/work/1/s/src/test/ui/closures/closure-move-sync.rs:7:27: 10:6 recv:&std::sync::mpsc::Receiver<()>]`
2019-12-22T23:40:42.7156690Z 
2019-12-22T23:40:42.7157150Z error[E0277]: `std::sync::mpsc::Sender<()>` cannot be shared between threads safely
2019-12-22T23:40:42.7158590Z    |
2019-12-22T23:40:42.7158590Z    |
2019-12-22T23:40:42.7158810Z LL |     thread::spawn(|| tx.send(()).unwrap());
2019-12-22T23:40:42.7159030Z    |     ^^^^^^^^^^^^^ `std::sync::mpsc::Sender<()>` cannot be shared between threads safely
2019-12-22T23:40:42.7159300Z   ::: /Users/runner/runners/2.163.1/work/1/s/src/libstd/thread/mod.rs:616:8
2019-12-22T23:40:42.7159730Z    |
2019-12-22T23:40:42.7160280Z LL |     F: Send + 'static,
2019-12-22T23:40:42.7160810Z    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:40:42.7160810Z    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:40:42.7161280Z    |
2019-12-22T23:40:42.7161490Z    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Sender<()>`
2019-12-22T23:40:42.7161720Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Sender<()>`
2019-12-22T23:40:42.7162810Z    = note: required because it appears within the type `[closure@/Users/runner/runners/2.163.1/work/1/s/src/test/ui/closures/closure-move-sync.rs:19:19: 19:42 tx:&std::sync::mpsc::Sender<()>]`
2019-12-22T23:40:42.7163390Z error: aborting due to 2 previous errors
2019-12-22T23:40:42.7163530Z 
2019-12-22T23:40:42.7164510Z For more information about this error, try `rustc --explain E0277`.
2019-12-22T23:40:42.7164580Z 
2019-12-22T23:40:42.7164580Z 
2019-12-22T23:40:42.7165690Z ------------------------------------------
2019-12-22T23:40:42.7166090Z 
2019-12-22T23:40:42.7166260Z 
2019-12-22T23:40:42.7167140Z ---- [ui] ui/no-send-res-ports.rs stdout ----
2019-12-22T23:40:42.7167610Z diff of stderr:
2019-12-22T23:40:42.7167760Z 
2019-12-22T23:40:42.7167860Z 6    | 
2019-12-22T23:40:42.7168040Z 7   ::: $SRC_DIR/libstd/thread/mod.rs:LL:COL
2019-12-22T23:40:42.7168220Z 8    |
2019-12-22T23:40:42.7169920Z - LL |     F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
2019-12-22T23:40:42.7171500Z -    |                          ---- required by this bound in `std::thread::spawn`
2019-12-22T23:40:42.7172760Z + LL |     F: Send + 'static,
2019-12-22T23:40:42.7173810Z +    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:40:42.7174350Z 11    |
2019-12-22T23:40:42.7175620Z 12    = help: within `[closure@$DIR/no-send-res-ports.rs:26:19: 30:6 x:main::Foo]`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-12-22T23:40:42.7176610Z 13    = note: required because it appears within the type `Port<()>`
2019-12-22T23:40:42.7177350Z 
2019-12-22T23:40:42.7177490Z The actual stderr differed from the expected stderr.
2019-12-22T23:40:42.7178590Z Actual stderr saved to /Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/test/ui/no-send-res-ports/no-send-res-ports.stderr
2019-12-22T23:40:42.7178590Z Actual stderr saved to /Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/test/ui/no-send-res-ports/no-send-res-ports.stderr
2019-12-22T23:40:42.8281120Z To update references, rerun the tests and pass the `--bless` flag
2019-12-22T23:40:42.8281760Z To only update this specific test, also pass `--test-args no-send-res-ports.rs`
2019-12-22T23:40:42.8282270Z error: 1 errors occurred comparing output.
2019-12-22T23:40:42.8282390Z status: exit code: 1
2019-12-22T23:40:42.8282390Z status: exit code: 1
2019-12-22T23:40:42.8283790Z command: "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui/no-send-res-ports.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/test/ui/no-send-res-ports" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/test/ui/no-send-res-ports/auxiliary" "-A" "unused"
2019-12-22T23:40:42.8284910Z ------------------------------------------
2019-12-22T23:40:42.8285190Z 
2019-12-22T23:40:42.8285810Z ------------------------------------------
2019-12-22T23:40:42.8286130Z stderr:
2019-12-22T23:40:42.8286130Z stderr:
2019-12-22T23:40:42.8286710Z ------------------------------------------
2019-12-22T23:40:42.8287060Z error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
2019-12-22T23:40:42.8288430Z    |
2019-12-22T23:40:42.8288430Z    |
2019-12-22T23:40:42.8288570Z LL |     thread::spawn(move|| {
2019-12-22T23:40:42.8288690Z    |     ^^^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-12-22T23:40:42.8288910Z   ::: /Users/runner/runners/2.163.1/work/1/s/src/libstd/thread/mod.rs:616:8
2019-12-22T23:40:42.8289030Z    |
2019-12-22T23:40:42.8289610Z LL |     F: Send + 'static,
2019-12-22T23:40:42.8290400Z    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:40:42.8290400Z    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:40:42.8290740Z    |
2019-12-22T23:40:42.8291490Z    = help: within `[closure@/Users/runner/runners/2.163.1/work/1/s/src/test/ui/no-send-res-ports.rs:26:19: 30:6 x:main::Foo]`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-12-22T23:40:42.8291880Z    = note: required because it appears within the type `Port<()>`
2019-12-22T23:40:42.8292040Z    = note: required because it appears within the type `main::Foo`
2019-12-22T23:40:42.8292790Z    = note: required because it appears within the type `[closure@/Users/runner/runners/2.163.1/work/1/s/src/test/ui/no-send-res-ports.rs:26:19: 30:6 x:main::Foo]`
2019-12-22T23:40:42.8293250Z error: aborting due to previous error
2019-12-22T23:40:42.8293330Z 
2019-12-22T23:40:42.8293920Z For more information about this error, try `rustc --explain E0277`.
2019-12-22T23:40:42.8294000Z 
2019-12-22T23:40:42.8294000Z 
2019-12-22T23:40:42.8294730Z ------------------------------------------
2019-12-22T23:40:42.8295030Z 
2019-12-22T23:40:42.8295130Z 
2019-12-22T23:40:42.8295710Z ---- [ui] ui/type_length_limit.rs stdout ----
2019-12-22T23:40:42.8296400Z diff of stderr:
2019-12-22T23:40:42.8296450Z 
2019-12-22T23:40:42.8297080Z 1 error: reached the type-length limit while instantiating `std::mem::drop::<std::option::Op... G), (G, G, G), (G, G, G))))))>>`
2019-12-22T23:40:42.8297640Z 2   --> $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-12-22T23:40:42.8297720Z 3    |
2019-12-22T23:40:42.8298210Z - LL | pub fn drop<T>(_x: T) { }
2019-12-22T23:40:42.8298730Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-22T23:40:42.8299560Z + LL | pub fn drop<T>(_x: T) {}
2019-12-22T23:40:42.8299790Z 6    |
2019-12-22T23:40:42.8299790Z 6    |
2019-12-22T23:40:42.8299870Z 7    = note: consider adding a `#![type_length_limit="1094"]` attribute to your crate
2019-12-22T23:40:42.8300000Z 
2019-12-22T23:40:42.8300040Z 
2019-12-22T23:40:42.8300110Z The actual stderr differed from the expected stderr.
2019-12-22T23:40:42.8300800Z Actual stderr saved to /Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/test/ui/type_length_limit/type_length_limit.stderr
2019-12-22T23:40:42.8300800Z Actual stderr saved to /Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/test/ui/type_length_limit/type_length_limit.stderr
2019-12-22T23:40:42.8301380Z To update references, rerun the tests and pass the `--bless` flag
2019-12-22T23:40:42.8301930Z To only update this specific test, also pass `--test-args type_length_limit.rs`
2019-12-22T23:40:42.8302090Z error: 1 errors occurred comparing output.
2019-12-22T23:40:42.8302160Z status: exit code: 1
2019-12-22T23:40:42.8302160Z status: exit code: 1
2019-12-22T23:40:42.8303470Z command: "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui/type_length_limit.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/test/ui/type_length_limit" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/test/ui/type_length_limit/auxiliary" "-A" "unused"
2019-12-22T23:40:42.8304320Z ------------------------------------------
2019-12-22T23:40:42.8304380Z 
2019-12-22T23:40:42.8304890Z ------------------------------------------
2019-12-22T23:40:42.8304980Z stderr:
2019-12-22T23:40:42.8304980Z stderr:
2019-12-22T23:40:42.8305730Z ------------------------------------------
2019-12-22T23:40:42.8306340Z error: reached the type-length limit while instantiating `std::mem::drop::<std::option::Op... G), (G, G, G), (G, G, G))))))>>`
2019-12-22T23:40:42.8307030Z    |
2019-12-22T23:40:42.8307030Z    |
2019-12-22T23:40:42.8307110Z LL | pub fn drop<T>(_x: T) {}
2019-12-22T23:40:42.8307250Z    |
2019-12-22T23:40:42.8307250Z    |
2019-12-22T23:40:42.8307340Z    = note: consider adding a `#![type_length_limit="1094"]` attribute to your crate
2019-12-22T23:40:42.8307460Z error: aborting due to previous error
2019-12-22T23:40:42.8307520Z 
2019-12-22T23:40:42.8307550Z 
2019-12-22T23:40:42.8308080Z ------------------------------------------
---
2019-12-22T23:40:42.8311180Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-22T23:40:42.8311290Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-22T23:40:42.8311370Z 
2019-12-22T23:40:42.8311400Z 
2019-12-22T23:40:42.8314190Z command did not execute successfully: "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/test/ui" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "ui" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-22T23:40:42.8315080Z 
2019-12-22T23:40:42.8315120Z 
2019-12-22T23:40:42.8315210Z failed to run: /Users/runner/runners/2.163.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-12-22T23:40:42.8315310Z Build completed unsuccessfully in 0:49:49
2019-12-22T23:40:42.8315310Z Build completed unsuccessfully in 0:49:49
2019-12-22T23:40:42.8315380Z == clock drift check ==
2019-12-22T23:40:42.8315460Z   local time: Sun Dec 22 23:40:42 UTC 2019
2019-12-22T23:40:42.8315530Z   network time: Sun, 22 Dec 2019 23:40:42 GMT
2019-12-22T23:40:42.8315610Z == end clock drift check ==
2019-12-22T23:40:42.8315850Z 
2019-12-22T23:40:42.8397590Z ##[error]Bash exited with code '1'.
2019-12-22T23:40:42.8439950Z ##[section]Starting: Checkout
2019-12-22T23:40:42.8442610Z ==============================================================================
2019-12-22T23:40:42.8442700Z Task         : Get sources
2019-12-22T23:40:42.8442790Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
