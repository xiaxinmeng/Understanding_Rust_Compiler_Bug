plain
2019-09-19T02:47:04.6401217Z ---- [ui] ui/async-await/issues/issue-62009-1.rs stdout ----
2019-09-19T02:47:04.6401510Z diff of stderr:
2019-09-19T02:47:04.6401646Z 
2019-09-19T02:47:04.6401806Z 32    |
2019-09-19T02:47:04.6401954Z 33 LL |     (|_| 2333).await;
2019-09-19T02:47:04.6402498Z 34    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@$DIR/issue-62009-1.rs:14:5: 14:15]`
2019-09-19T02:47:04.6402890Z -    | 
2019-09-19T02:47:04.6403293Z -   ::: $SRC_DIR/libstd/future.rs:LL:COL
2019-09-19T02:47:04.6403978Z - LL |     F: Future
2019-09-19T02:47:04.6403978Z - LL |     F: Future
2019-09-19T02:47:04.6404413Z -    |        ------ required by this bound in `std::future::poll_with_tls_context`
2019-09-19T02:47:04.6405182Z 41 error: aborting due to 4 previous errors
2019-09-19T02:47:04.6405346Z 42 
2019-09-19T02:47:04.6405481Z 
2019-09-19T02:47:04.6405594Z 
2019-09-19T02:47:04.6405594Z 
2019-09-19T02:47:04.6405760Z The actual stderr differed from the expected stderr.
2019-09-19T02:47:04.6406274Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/issue-62009-1.stderr
2019-09-19T02:47:04.6406714Z To update references, rerun the tests and pass the `--bless` flag
2019-09-19T02:47:04.6407150Z To only update this specific test, also pass `--test-args async-await/issues/issue-62009-1.rs`
2019-09-19T02:47:04.6407988Z error: 1 errors occurred comparing output.
2019-09-19T02:47:04.6408190Z status: exit code: 1
2019-09-19T02:47:04.6408190Z status: exit code: 1
2019-09-19T02:47:04.6409327Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62009-1.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/auxiliary" "-A" "unused"
2019-09-19T02:47:04.6410124Z ------------------------------------------
2019-09-19T02:47:04.6410305Z 
2019-09-19T02:47:04.6410852Z ------------------------------------------
2019-09-19T02:47:04.6411031Z stderr:
2019-09-19T02:47:04.6411031Z stderr:
2019-09-19T02:47:04.6411371Z ------------------------------------------
2019-09-19T02:47:04.6411571Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-09-19T02:47:04.6411945Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:8:5
2019-09-19T02:47:04.6412302Z LL | fn main() {
2019-09-19T02:47:04.6412675Z    |    ---- this is not `async`
2019-09-19T02:47:04.6412675Z    |    ---- this is not `async`
2019-09-19T02:47:04.6412926Z LL |     async { let (); }.await;
2019-09-19T02:47:04.6413407Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-09-19T02:47:04.6413858Z 
2019-09-19T02:47:04.6414113Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-09-19T02:47:04.6418374Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:10:5
2019-09-19T02:47:04.6418550Z LL |   fn main() {
2019-09-19T02:47:04.6419075Z    |      ---- this is not `async`
2019-09-19T02:47:04.6419165Z ...
2019-09-19T02:47:04.6419227Z LL | /     async {
2019-09-19T02:47:04.6419227Z LL | /     async {
2019-09-19T02:47:04.6419323Z LL | |     //~^ ERROR `await` is only allowed inside `async` functions and blocks
2019-09-19T02:47:04.6419412Z LL | |         let task1 = print_dur().await;
2019-09-19T02:47:04.6419500Z LL | |     }.await;
2019-09-19T02:47:04.6419693Z    | |___________^ only allowed inside `async` functions and blocks
2019-09-19T02:47:04.6419756Z 
2019-09-19T02:47:04.6419848Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-09-19T02:47:04.6420170Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5
2019-09-19T02:47:04.6420325Z LL | fn main() {
2019-09-19T02:47:04.6420570Z    |    ---- this is not `async`
2019-09-19T02:47:04.6420640Z ...
2019-09-19T02:47:04.6420640Z ...
2019-09-19T02:47:04.6420716Z LL |     (|_| 2333).await;
2019-09-19T02:47:04.6420793Z    |     ^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-09-19T02:47:04.6420878Z 
2019-09-19T02:47:04.6421245Z error[E0277]: the trait bound `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5: 14:15]: std::future::Future` is not satisfied
2019-09-19T02:47:04.6421618Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5
2019-09-19T02:47:04.6421704Z    |
2019-09-19T02:47:04.6421795Z LL |     (|_| 2333).await;
2019-09-19T02:47:04.6422205Z    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5: 14:15]`
2019-09-19T02:47:04.6422373Z error: aborting due to 4 previous errors
2019-09-19T02:47:04.6422439Z 
2019-09-19T02:47:04.6422729Z For more information about this error, try `rustc --explain E0277`.
2019-09-19T02:47:04.6422786Z 
2019-09-19T02:47:04.6422786Z 
2019-09-19T02:47:04.6423363Z ------------------------------------------
2019-09-19T02:47:04.6423571Z 
2019-09-19T02:47:04.6423602Z 
2019-09-19T02:47:04.6423845Z ---- [ui] ui/closures/closure-move-sync.rs stdout ----
2019-09-19T02:47:04.6423920Z diff of stderr:
2019-09-19T02:47:04.6423974Z 
2019-09-19T02:47:04.6424026Z 3    |
2019-09-19T02:47:04.6424098Z 4 LL |     let t = thread::spawn(|| {
2019-09-19T02:47:04.6424179Z 5    |             ^^^^^^^^^^^^^ `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
2019-09-19T02:47:04.6424409Z -    | 
2019-09-19T02:47:04.6424626Z -   ::: $SRC_DIR/libstd/thread/mod.rs:LL:COL
2019-09-19T02:47:04.6424707Z 8    |
2019-09-19T02:47:04.6424939Z - LL |     F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
2019-09-19T02:47:04.6425220Z -    |                          ---- required by this bound in `std::thread::spawn`
2019-09-19T02:47:04.6425503Z 12    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Receiver<()>`
2019-09-19T02:47:04.6425603Z 13    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
2019-09-19T02:47:04.6425603Z 13    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
2019-09-19T02:47:04.6425979Z 14    = note: required because it appears within the type `[closure@$DIR/closure-move-sync.rs:8:27: 11:6 recv:&std::sync::mpsc::Receiver<()>]`
2019-09-19T02:47:04.6426116Z 18    |
2019-09-19T02:47:04.6426116Z 18    |
2019-09-19T02:47:04.6426175Z 19 LL |     thread::spawn(|| tx.send(()).unwrap());
2019-09-19T02:47:04.6426281Z 20    |     ^^^^^^^^^^^^^ `std::sync::mpsc::Sender<()>` cannot be shared between threads safely
2019-09-19T02:47:04.6426484Z -    | 
2019-09-19T02:47:04.6426721Z -   ::: $SRC_DIR/libstd/thread/mod.rs:LL:COL
2019-09-19T02:47:04.6426904Z -    |
2019-09-19T02:47:04.6427153Z - LL |     F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
2019-09-19T02:47:04.6427411Z -    |                          ---- required by this bound in `std::thread::spawn`
2019-09-19T02:47:04.6427954Z 27    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Sender<()>`
2019-09-19T02:47:04.6428086Z 28    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Sender<()>`
2019-09-19T02:47:04.6428279Z 
2019-09-19T02:47:04.6428332Z 
2019-09-19T02:47:04.6428332Z 
2019-09-19T02:47:04.6428399Z The actual stderr differed from the expected stderr.
2019-09-19T02:47:04.6428839Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-move-sync/closure-move-sync.stderr
2019-09-19T02:47:04.6429217Z To update references, rerun the tests and pass the `--bless` flag
2019-09-19T02:47:04.6429576Z To only update this specific test, also pass `--test-args closures/closure-move-sync.rs`
2019-09-19T02:47:04.6429715Z error: 1 errors occurred comparing output.
2019-09-19T02:47:04.6429785Z status: exit code: 1
2019-09-19T02:47:04.6429785Z status: exit code: 1
2019-09-19T02:47:04.6430702Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-move-sync.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-move-sync" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-move-sync/auxiliary" "-A" "unused"
2019-09-19T02:47:04.6431209Z ------------------------------------------
2019-09-19T02:47:04.6431259Z 
2019-09-19T02:47:04.6431515Z ------------------------------------------
2019-09-19T02:47:04.6431585Z stderr:
2019-09-19T02:47:04.6431585Z stderr:
2019-09-19T02:47:04.6431994Z ------------------------------------------
2019-09-19T02:47:04.6432078Z error[E0277]: `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
2019-09-19T02:47:04.6432603Z    |
2019-09-19T02:47:04.6432603Z    |
2019-09-19T02:47:04.6432659Z LL |     let t = thread::spawn(|| {
2019-09-19T02:47:04.6432758Z    |             ^^^^^^^^^^^^^ `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
2019-09-19T02:47:04.6432934Z    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Receiver<()>`
2019-09-19T02:47:04.6433036Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
2019-09-19T02:47:04.6433036Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
2019-09-19T02:47:04.6433768Z    = note: required because it appears within the type `[closure@/checkout/src/test/ui/closures/closure-move-sync.rs:8:27: 11:6 recv:&std::sync::mpsc::Receiver<()>]`
2019-09-19T02:47:04.6433843Z 
2019-09-19T02:47:04.6433926Z error[E0277]: `std::sync::mpsc::Sender<()>` cannot be shared between threads safely
2019-09-19T02:47:04.6434254Z    |
2019-09-19T02:47:04.6434254Z    |
2019-09-19T02:47:04.6434310Z LL |     thread::spawn(|| tx.send(()).unwrap());
2019-09-19T02:47:04.6434405Z    |     ^^^^^^^^^^^^^ `std::sync::mpsc::Sender<()>` cannot be shared between threads safely
2019-09-19T02:47:04.6434568Z    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Sender<()>`
2019-09-19T02:47:04.6434680Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Sender<()>`
2019-09-19T02:47:04.6434680Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Sender<()>`
2019-09-19T02:47:04.6435052Z    = note: required because it appears within the type `[closure@/checkout/src/test/ui/closures/closure-move-sync.rs:20:19: 20:42 tx:&std::sync::mpsc::Sender<()>]`
2019-09-19T02:47:04.6435198Z error: aborting due to 2 previous errors
2019-09-19T02:47:04.6435253Z 
2019-09-19T02:47:04.6435486Z For more information about this error, try `rustc --explain E0277`.
2019-09-19T02:47:04.6435532Z 
---
2019-09-19T02:47:04.6436331Z 3    |
2019-09-19T02:47:04.6436400Z 4 LL |      x: Error
2019-09-19T02:47:04.6436468Z 5    |      ^^^^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-09-19T02:47:04.6436708Z -    | 
2019-09-19T02:47:04.6436909Z -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
2019-09-19T02:47:04.6437174Z -    |
2019-09-19T02:47:04.6437840Z - LL |     fn hash<H: Hasher>(&self, state: &mut H);
2019-09-19T02:47:04.6438219Z 11 
2019-09-19T02:47:04.6438299Z 12 error: aborting due to previous error
2019-09-19T02:47:04.6438364Z 13 
2019-09-19T02:47:04.6438418Z 
2019-09-19T02:47:04.6438418Z 
2019-09-19T02:47:04.6438452Z 
2019-09-19T02:47:04.6438518Z The actual stderr differed from the expected stderr.
2019-09-19T02:47:04.6438928Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum-struct-variant/derives-span-Hash-enum-struct-variant.stderr
2019-09-19T02:47:04.6439246Z To update references, rerun the tests and pass the `--bless` flag
2019-09-19T02:47:04.6439588Z To only update this specific test, also pass `--test-args derives/derives-span-Hash-enum-struct-variant.rs`
2019-09-19T02:47:04.6439734Z error: 1 errors occurred comparing output.
2019-09-19T02:47:04.6439810Z status: exit code: 1
2019-09-19T02:47:04.6439810Z status: exit code: 1
2019-09-19T02:47:04.6449535Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-Hash-enum-struct-variant.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum-struct-variant" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum-struct-variant/auxiliary" "-A" "unused"
2019-09-19T02:47:04.6450321Z ------------------------------------------
2019-09-19T02:47:04.6450393Z 
2019-09-19T02:47:04.6450637Z ------------------------------------------
2019-09-19T02:47:04.6450727Z stderr:
2019-09-19T02:47:04.6450727Z stderr:
2019-09-19T02:47:04.6450957Z ------------------------------------------
2019-09-19T02:47:04.6451067Z error[E0277]: the trait bound `Error: std::hash::Hash` is not satisfied
2019-09-19T02:47:04.6451845Z   --> /checkout/src/test/ui/derives/derives-span-Hash-enum-struct-variant.rs:11:6
2019-09-19T02:47:04.6451935Z    |
2019-09-19T02:47:04.6451988Z LL |      x: Error //~ ERROR
2019-09-19T02:47:04.6452074Z    |      ^^^^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-09-19T02:47:04.6452191Z error: aborting due to previous error
2019-09-19T02:47:04.6452229Z 
2019-09-19T02:47:04.6452480Z For more information about this error, try `rustc --explain E0277`.
2019-09-19T02:47:04.6452537Z 
---
2019-09-19T02:47:04.6453234Z 3    |
2019-09-19T02:47:04.6453293Z 4 LL |      Error
2019-09-19T02:47:04.6453377Z 5    |      ^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-09-19T02:47:04.6453568Z -    | 
2019-09-19T02:47:04.6453787Z -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
2019-09-19T02:47:04.6453967Z -    |
2019-09-19T02:47:04.6454592Z - LL |     fn hash<H: Hasher>(&self, state: &mut H);
2019-09-19T02:47:04.6454943Z 11 
2019-09-19T02:47:04.6455198Z 12 error: aborting due to previous error
2019-09-19T02:47:04.6455279Z 13 
2019-09-19T02:47:04.6455312Z 
2019-09-19T02:47:04.6455312Z 
2019-09-19T02:47:04.6455529Z 
2019-09-19T02:47:04.6455607Z The actual stderr differed from the expected stderr.
2019-09-19T02:47:04.6456258Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum/derives-span-Hash-enum.stderr
2019-09-19T02:47:04.6456604Z To update references, rerun the tests and pass the `--bless` flag
2019-09-19T02:47:04.6457293Z To only update this specific test, also pass `--test-args derives/derives-span-Hash-enum.rs`
2019-09-19T02:47:04.6457489Z error: 1 errors occurred comparing output.
2019-09-19T02:47:04.6458211Z status: exit code: 1
2019-09-19T02:47:04.6458211Z status: exit code: 1
2019-09-19T02:47:04.6459565Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-Hash-enum.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum/auxiliary" "-A" "unused"
2019-09-19T02:47:04.6461389Z ------------------------------------------
2019-09-19T02:47:04.6461466Z 
2019-09-19T02:47:04.6462543Z ------------------------------------------
2019-09-19T02:47:04.6462784Z stderr:
---
2019-09-19T02:47:04.6466993Z 3    |
2019-09-19T02:47:04.6467222Z 4 LL |     x: Error
2019-09-19T02:47:04.6467293Z 5    |     ^^^^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-09-19T02:47:04.6467808Z -    | 
2019-09-19T02:47:04.6468794Z -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
2019-09-19T02:47:04.6469032Z -    |
2019-09-19T02:47:04.6469278Z - LL |     fn hash<H: Hasher>(&self, state: &mut H);
2019-09-19T02:47:04.6469660Z 11 
2019-09-19T02:47:04.6469739Z 12 error: aborting due to previous error
2019-09-19T02:47:04.6469805Z 13 
2019-09-19T02:47:04.6469840Z 
2019-09-19T02:47:04.6469840Z 
2019-09-19T02:47:04.6469892Z 
2019-09-19T02:47:04.6469956Z The actual stderr differed from the expected stderr.
2019-09-19T02:47:04.6470354Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-struct/derives-span-Hash-struct.stderr
2019-09-19T02:47:04.6470657Z To update references, rerun the tests and pass the `--bless` flag
2019-09-19T02:47:04.6470983Z To only update this specific test, also pass `--test-args derives/derives-span-Hash-struct.rs`
2019-09-19T02:47:04.6471123Z error: 1 errors occurred comparing output.
2019-09-19T02:47:04.6471192Z status: exit code: 1
2019-09-19T02:47:04.6471192Z status: exit code: 1
2019-09-19T02:47:04.6472493Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-Hash-struct.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-struct/auxiliary" "-A" "unused"
2019-09-19T02:47:04.6473204Z ------------------------------------------
2019-09-19T02:47:04.6473273Z 
2019-09-19T02:47:04.6473672Z ------------------------------------------
2019-09-19T02:47:04.6473757Z stderr:
2019-09-19T02:47:04.6473757Z stderr:
2019-09-19T02:47:04.6473979Z ------------------------------------------
2019-09-19T02:47:04.6474075Z error[E0277]: the trait bound `Error: std::hash::Hash` is not satisfied
2019-09-19T02:47:04.6475275Z   --> /checkout/src/test/ui/derives/derives-span-Hash-struct.rs:10:5
2019-09-19T02:47:04.6475371Z    |
2019-09-19T02:47:04.6475424Z LL |     x: Error //~ ERROR
2019-09-19T02:47:04.6475521Z    |     ^^^^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-09-19T02:47:04.6475639Z error: aborting due to previous error
2019-09-19T02:47:04.6475677Z 
2019-09-19T02:47:04.6475923Z For more information about this error, try `rustc --explain E0277`.
2019-09-19T02:47:04.6475987Z 
---
2019-09-19T02:47:04.6477057Z 3    |
2019-09-19T02:47:04.6477146Z 4 LL |     Error
2019-09-19T02:47:04.6477239Z 5    |     ^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-09-19T02:47:04.6478551Z -    | 
2019-09-19T02:47:04.6478838Z -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
2019-09-19T02:47:04.6479267Z -    |
2019-09-19T02:47:04.6479573Z - LL |     fn hash<H: Hasher>(&self, state: &mut H);
2019-09-19T02:47:04.6479969Z 11 
2019-09-19T02:47:04.6480031Z 12 error: aborting due to previous error
2019-09-19T02:47:04.6480113Z 13 
2019-09-19T02:47:04.6480150Z 
2019-09-19T02:47:04.6480150Z 
2019-09-19T02:47:04.6480185Z 
2019-09-19T02:47:04.6480277Z The actual stderr differed from the expected stderr.
2019-09-19T02:47:04.6480661Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-tuple-struct/derives-span-Hash-tuple-struct.stderr
2019-09-19T02:47:04.6481175Z To update references, rerun the tests and pass the `--bless` flag
2019-09-19T02:47:04.6481466Z To only update this specific test, also pass `--test-args derives/derives-span-Hash-tuple-struct.rs`
2019-09-19T02:47:04.6481594Z error: 1 errors occurred comparing output.
2019-09-19T02:47:04.6481673Z status: exit code: 1
2019-09-19T02:47:04.6481673Z status: exit code: 1
2019-09-19T02:47:04.6482525Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-Hash-tuple-struct.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-tuple-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-tuple-struct/auxiliary" "-A" "unused"
2019-09-19T02:47:04.6483019Z ------------------------------------------
2019-09-19T02:47:04.6483064Z 
2019-09-19T02:47:04.6483292Z ------------------------------------------
2019-09-19T02:47:04.6483354Z stderr:
---
2019-09-19T02:47:04.6486162Z ---- [ui] ui/interior-mutability/interior-mutability.rs stdout ----
2019-09-19T02:47:04.6486232Z diff of stderr:
2019-09-19T02:47:04.6486285Z 
2019-09-19T02:47:04.6486337Z 3    |
2019-09-19T02:47:04.6486414Z 4 LL |     catch_unwind(|| { x.set(23); });
2019-09-19T02:47:04.6486514Z 5    |     ^^^^^^^^^^^^ `std::cell::UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
2019-09-19T02:47:04.6486772Z -    | 
2019-09-19T02:47:04.6486981Z -   ::: $SRC_DIR/libstd/panic.rs:LL:COL
2019-09-19T02:47:04.6487182Z -    |
2019-09-19T02:47:04.6487431Z - LL | pub fn catch_unwind<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Result<R> {
2019-09-19T02:47:04.6488285Z -    |                                        ---------- required by this bound in `std::panic::catch_unwind`
2019-09-19T02:47:04.6488383Z 11    |
2019-09-19T02:47:04.6488494Z 12    = help: within `std::cell::Cell<i32>`, the trait `std::panic::RefUnwindSafe` is not implemented for `std::cell::UnsafeCell<i32>`
2019-09-19T02:47:04.6488601Z 13    = note: required because it appears within the type `std::cell::Cell<i32>`
2019-09-19T02:47:04.6488708Z 
2019-09-19T02:47:04.6488776Z The actual stderr differed from the expected stderr.
2019-09-19T02:47:04.6489170Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/interior-mutability/interior-mutability/interior-mutability.stderr
2019-09-19T02:47:04.6489170Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/interior-mutability/interior-mutability/interior-mutability.stderr
2019-09-19T02:47:04.6489496Z To update references, rerun the tests and pass the `--bless` flag
2019-09-19T02:47:04.6489811Z To only update this specific test, also pass `--test-args interior-mutability/interior-mutability.rs`
2019-09-19T02:47:04.6489961Z error: 1 errors occurred comparing output.
2019-09-19T02:47:04.6490029Z status: exit code: 1
2019-09-19T02:47:04.6490029Z status: exit code: 1
2019-09-19T02:47:04.6490977Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/interior-mutability/interior-mutability.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/interior-mutability/interior-mutability" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/interior-mutability/interior-mutability/auxiliary" "-A" "unused"
2019-09-19T02:47:04.6491641Z ------------------------------------------
2019-09-19T02:47:04.6491686Z 
2019-09-19T02:47:04.6491898Z ------------------------------------------
2019-09-19T02:47:04.6491979Z stderr:
2019-09-19T02:47:04.6491979Z stderr:
2019-09-19T02:47:04.6492193Z ------------------------------------------
2019-09-19T02:47:04.6492311Z error[E0277]: the type `std::cell::UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
2019-09-19T02:47:04.6492684Z    |
2019-09-19T02:47:04.6492684Z    |
2019-09-19T02:47:04.6492757Z LL |     catch_unwind(|| { x.set(23); });
2019-09-19T02:47:04.6492853Z    |     ^^^^^^^^^^^^ `std::cell::UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
2019-09-19T02:47:04.6493097Z    |
2019-09-19T02:47:04.6493179Z    = help: within `std::cell::Cell<i32>`, the trait `std::panic::RefUnwindSafe` is not implemented for `std::cell::UnsafeCell<i32>`
2019-09-19T02:47:04.6493292Z    = note: required because it appears within the type `std::cell::Cell<i32>`
2019-09-19T02:47:04.6493448Z    = note: required because of the requirements on the impl of `std::panic::UnwindSafe` for `&std::cell::Cell<i32>`
2019-09-19T02:47:04.6493885Z    = note: required because it appears within the type `[closure@/checkout/src/test/ui/interior-mutability/interior-mutability.rs:7:18: 7:35 x:&std::cell::Cell<i32>]`
2019-09-19T02:47:04.6494031Z error: aborting due to previous error
2019-09-19T02:47:04.6494071Z 
2019-09-19T02:47:04.6494329Z For more information about this error, try `rustc --explain E0277`.
2019-09-19T02:47:04.6494379Z 
2019-09-19T02:47:04.6494379Z 
2019-09-19T02:47:04.6494604Z ------------------------------------------
2019-09-19T02:47:04.6494658Z 
2019-09-19T02:47:04.6494689Z 
2019-09-19T02:47:04.6494929Z ---- [ui] ui/issues/issue-21160.rs stdout ----
2019-09-19T02:47:04.6494995Z diff of stderr:
2019-09-19T02:47:04.6495030Z 
2019-09-19T02:47:04.6495100Z 3    |
2019-09-19T02:47:04.6495154Z 4 LL | struct Foo(Bar);
2019-09-19T02:47:04.6495243Z 5    |            ^^^ the trait `std::hash::Hash` is not implemented for `Bar`
2019-09-19T02:47:04.6495474Z -    | 
2019-09-19T02:47:04.6495725Z -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
2019-09-19T02:47:04.6495927Z -    |
2019-09-19T02:47:04.6496184Z - LL |     fn hash<H: Hasher>(&self, state: &mut H);
2019-09-19T02:47:04.6496541Z 11 
2019-09-19T02:47:04.6496599Z 12 error: aborting due to previous error
2019-09-19T02:47:04.6496679Z 13 
2019-09-19T02:47:04.6496712Z 
2019-09-19T02:47:04.6496712Z 
2019-09-19T02:47:04.6496745Z 
2019-09-19T02:47:04.6496994Z The actual stderr differed from the expected stderr.
2019-09-19T02:47:04.6497331Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21160/issue-21160.stderr
2019-09-19T02:47:04.6498031Z To update references, rerun the tests and pass the `--bless` flag
2019-09-19T02:47:04.6498362Z To only update this specific test, also pass `--test-args issues/issue-21160.rs`
2019-09-19T02:47:04.6498496Z error: 1 errors occurred comparing output.
2019-09-19T02:47:04.6498585Z status: exit code: 1
2019-09-19T02:47:04.6498585Z status: exit code: 1
2019-09-19T02:47:04.6499468Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21160.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21160" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21160/auxiliary" "-A" "unused"
2019-09-19T02:47:04.6499961Z ------------------------------------------
2019-09-19T02:47:04.6500010Z 
2019-09-19T02:47:04.6500259Z ------------------------------------------
2019-09-19T02:47:04.6500330Z stderr:
2019-09-19T02:47:04.6500330Z stderr:
2019-09-19T02:47:04.6500576Z ------------------------------------------
2019-09-19T02:47:04.6500667Z error[E0277]: the trait bound `Bar: std::hash::Hash` is not satisfied
2019-09-19T02:47:04.6500959Z   --> /checkout/src/test/ui/issues/issue-21160.rs:10:12
2019-09-19T02:47:04.6501190Z    |
2019-09-19T02:47:04.6501264Z LL | struct Foo(Bar);
2019-09-19T02:47:04.6501336Z    |            ^^^ the trait `std::hash::Hash` is not implemented for `Bar`
2019-09-19T02:47:04.6501466Z error: aborting due to previous error
2019-09-19T02:47:04.6501527Z 
2019-09-19T02:47:04.6501799Z For more information about this error, try `rustc --explain E0277`.
2019-09-19T02:47:04.6501854Z 
2019-09-19T02:47:04.6501854Z 
2019-09-19T02:47:04.6502105Z ------------------------------------------
2019-09-19T02:47:04.6502273Z 
2019-09-19T02:47:04.6502307Z 
2019-09-19T02:47:04.6502601Z ---- [ui] ui/no-send-res-ports.rs stdout ----
2019-09-19T02:47:04.6502672Z diff of stderr:
2019-09-19T02:47:04.6502727Z 
2019-09-19T02:47:04.6502782Z 3    |
2019-09-19T02:47:04.6502860Z 4 LL |     thread::spawn(move|| {
2019-09-19T02:47:04.6503018Z 5    |     ^^^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-09-19T02:47:04.6503293Z -    | 
2019-09-19T02:47:04.6503528Z -   ::: $SRC_DIR/libstd/thread/mod.rs:LL:COL
2019-09-19T02:47:04.6503752Z -    |
2019-09-19T02:47:04.6504010Z - LL |     F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
2019-09-19T02:47:04.6504321Z -    |                          ---- required by this bound in `std::thread::spawn`
2019-09-19T02:47:04.6504401Z 11    |
2019-09-19T02:47:04.6504774Z 12    = help: within `[closure@$DIR/no-send-res-ports.rs:27:19: 31:6 x:main::Foo]`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-09-19T02:47:04.6504895Z 13    = note: required because it appears within the type `Port<()>`
2019-09-19T02:47:04.6505005Z 
2019-09-19T02:47:04.6505087Z The actual stderr differed from the expected stderr.
2019-09-19T02:47:04.6505424Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-send-res-ports/no-send-res-ports.stderr
2019-09-19T02:47:04.6505424Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-send-res-ports/no-send-res-ports.stderr
2019-09-19T02:47:04.6505736Z To update references, rerun the tests and pass the `--bless` flag
2019-09-19T02:47:04.6506026Z To only update this specific test, also pass `--test-args no-send-res-ports.rs`
2019-09-19T02:47:04.6506163Z error: 1 errors occurred comparing output.
2019-09-19T02:47:04.6506247Z status: exit code: 1
2019-09-19T02:47:04.6506247Z status: exit code: 1
2019-09-19T02:47:04.6507049Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no-send-res-ports.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-send-res-ports" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-send-res-ports/auxiliary" "-A" "unused"
2019-09-19T02:47:04.6507625Z ------------------------------------------
2019-09-19T02:47:04.6507674Z 
2019-09-19T02:47:04.6508416Z ------------------------------------------
2019-09-19T02:47:04.6508514Z stderr:
2019-09-19T02:47:04.6508514Z stderr:
2019-09-19T02:47:04.6508745Z ------------------------------------------
2019-09-19T02:47:04.6508845Z error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
2019-09-19T02:47:04.6509207Z    |
2019-09-19T02:47:04.6509207Z    |
2019-09-19T02:47:04.6509268Z LL |     thread::spawn(move|| {
2019-09-19T02:47:04.6509369Z    |     ^^^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-09-19T02:47:04.6509476Z    |
2019-09-19T02:47:04.6509869Z    = help: within `[closure@/checkout/src/test/ui/no-send-res-ports.rs:27:19: 31:6 x:main::Foo]`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-09-19T02:47:04.6510005Z    = note: required because it appears within the type `Port<()>`
2019-09-19T02:47:04.6510100Z    = note: required because it appears within the type `main::Foo`
2019-09-19T02:47:04.6510483Z    = note: required because it appears within the type `[closure@/checkout/src/test/ui/no-send-res-ports.rs:27:19: 31:6 x:main::Foo]`
2019-09-19T02:47:04.6510634Z error: aborting due to previous error
2019-09-19T02:47:04.6510678Z 
2019-09-19T02:47:04.6510944Z For more information about this error, try `rustc --explain E0277`.
2019-09-19T02:47:04.6511014Z 
---
2019-09-19T02:47:04.6512168Z diff of stderr:
2019-09-19T02:47:04.6512206Z 
2019-09-19T02:47:04.6512437Z 5 LL | |     "0".parse()
2019-09-19T02:47:04.6512497Z 6 LL | | }
2019-09-19T02:47:04.6512581Z 7    | |_^ `main` can only return types that implement `std::process::Termination`
2019-09-19T02:47:04.6512812Z -    | 
2019-09-19T02:47:04.6513110Z -   ::: $SRC_DIR/libtest/lib.rs:LL:COL
2019-09-19T02:47:04.6513331Z -    |
2019-09-19T02:47:04.6513580Z - LL |   pub fn assert_test_result<T: Termination>(result: T) {
2019-09-19T02:47:04.6513864Z -    |                                ----------- required by this bound in `test::assert_test_result`
2019-09-19T02:47:04.6513958Z 13    |
2019-09-19T02:47:04.6514038Z 14    = help: the trait `std::process::Termination` is not implemented for `std::result::Result<f32, std::num::ParseFloatError>`
2019-09-19T02:47:04.6514172Z 
2019-09-19T02:47:04.6514203Z 
2019-09-19T02:47:04.6514281Z The actual stderr differed from the expected stderr.
2019-09-19T02:47:04.6514657Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/termination-trait-test-wrong-type.stderr
2019-09-19T02:47:04.6514657Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/termination-trait-test-wrong-type.stderr
2019-09-19T02:47:04.6514948Z To update references, rerun the tests and pass the `--bless` flag
2019-09-19T02:47:04.6515258Z To only update this specific test, also pass `--test-args rfc-1937-termination-trait/termination-trait-test-wrong-type.rs`
2019-09-19T02:47:04.6515393Z error: 1 errors occurred comparing output.
2019-09-19T02:47:04.6515472Z status: exit code: 1
2019-09-19T02:47:04.6515472Z status: exit code: 1
2019-09-19T02:47:04.6516390Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/auxiliary" "-A" "unused"
2019-09-19T02:47:04.6516897Z ------------------------------------------
2019-09-19T02:47:04.6516941Z 
2019-09-19T02:47:04.6517170Z ------------------------------------------
2019-09-19T02:47:04.6517234Z stderr:
2019-09-19T02:47:04.6517234Z stderr:
2019-09-19T02:47:04.6517457Z ------------------------------------------
2019-09-19T02:47:04.6517540Z error[E0277]: `main` has invalid return type `std::result::Result<f32, std::num::ParseFloatError>`
2019-09-19T02:47:04.6518804Z    |
2019-09-19T02:47:04.6518804Z    |
2019-09-19T02:47:04.6519108Z LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseFloatError> { //~ ERROR
2019-09-19T02:47:04.6519205Z LL | |     "0".parse()
2019-09-19T02:47:04.6519289Z LL | | }
2019-09-19T02:47:04.6519361Z    | |_^ `main` can only return types that implement `std::process::Termination`
2019-09-19T02:47:04.6519456Z    |
2019-09-19T02:47:04.6519570Z    = help: the trait `std::process::Termination` is not implemented for `std::result::Result<f32, std::num::ParseFloatError>`
2019-09-19T02:47:04.6519704Z error: aborting due to previous error
2019-09-19T02:47:04.6519767Z 
2019-09-19T02:47:04.6520043Z For more information about this error, try `rustc --explain E0277`.
2019-09-19T02:47:04.6520115Z 
2019-09-19T02:47:04.6520115Z 
2019-09-19T02:47:04.6520345Z ------------------------------------------
2019-09-19T02:47:04.6520393Z 
2019-09-19T02:47:04.6520443Z 
2019-09-19T02:47:04.6520697Z ---- [ui] ui/traits/trait-suggest-where-clause.rs stdout ----
2019-09-19T02:47:04.6520790Z diff of stderr:
2019-09-19T02:47:04.6520831Z 
2019-09-19T02:47:04.6520889Z 3    |
2019-09-19T02:47:04.6521104Z 4 LL |     mem::size_of::<U>();
2019-09-19T02:47:04.6521548Z 5    |     ^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-19T02:47:04.6521779Z -    | 
2019-09-19T02:47:04.6522002Z -   ::: $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-09-19T02:47:04.6522098Z 8    |
2019-09-19T02:47:04.6522323Z - LL | pub const fn size_of<T>() -> usize {
2019-09-19T02:47:04.6522706Z -    |                      - required by this bound in `std::mem::size_of`
2019-09-19T02:47:04.6523040Z 12    = help: the trait `std::marker::Sized` is not implemented for `U`
2019-09-19T02:47:04.6523040Z 12    = help: the trait `std::marker::Sized` is not implemented for `U`
2019-09-19T02:47:04.6523403Z 13    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-19T02:47:04.6523507Z 14    = help: consider adding a `where U: std::marker::Sized` bound
2019-09-19T02:47:04.6523629Z 18    |
2019-09-19T02:47:04.6523629Z 18    |
2019-09-19T02:47:04.6523686Z 19 LL |     mem::size_of::<Misc<U>>();
2019-09-19T02:47:04.6523992Z 20    |     ^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-19T02:47:04.6524203Z -    | 
2019-09-19T02:47:04.6524444Z -   ::: $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-09-19T02:47:04.6524511Z 23    |
2019-09-19T02:47:04.6524946Z - LL | pub const fn size_of<T>() -> usize {
2019-09-19T02:47:04.6525229Z -    |                      - required by this bound in `std::mem::size_of`
2019-09-19T02:47:04.6525465Z -    |
2019-09-19T02:47:04.6525540Z 27    = help: within `Misc<U>`, the trait `std::marker::Sized` is not implemented for `U`
2019-09-19T02:47:04.6525907Z 28    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-19T02:47:04.6526270Z 29    = help: consider adding a `where U: std::marker::Sized` bound
2019-09-19T02:47:04.6526382Z 60    |
2019-09-19T02:47:04.6526382Z 60    |
2019-09-19T02:47:04.6526460Z 61 LL |     mem::size_of::<[T]>();
2019-09-19T02:47:04.6526748Z 62    |     ^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-19T02:47:04.6526995Z -    | 
2019-09-19T02:47:04.6527223Z -   ::: $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-09-19T02:47:04.6527311Z 65    |
2019-09-19T02:47:04.6528257Z - LL | pub const fn size_of<T>() -> usize {
2019-09-19T02:47:04.6533698Z -    |                      - required by this bound in `std::mem::size_of`
2019-09-19T02:47:04.6534048Z 69    = help: the trait `std::marker::Sized` is not implemented for `[T]`
2019-09-19T02:47:04.6534048Z 69    = help: the trait `std::marker::Sized` is not implemented for `[T]`
2019-09-19T02:47:04.6534410Z 70    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-19T02:47:04.6534535Z 
2019-09-19T02:47:04.6534609Z 74    |
2019-09-19T02:47:04.6534609Z 74    |
2019-09-19T02:47:04.6534663Z 75 LL |     mem::size_of::<[&U]>();
2019-09-19T02:47:04.6534938Z 76    |     ^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-19T02:47:04.6535152Z -    | 
2019-09-19T02:47:04.6536039Z -   ::: $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-09-19T02:47:04.6536332Z -    |
2019-09-19T02:47:04.6536586Z - LL | pub const fn size_of<T>() -> usize {
2019-09-19T02:47:04.6537097Z -    |                      - required by this bound in `std::mem::size_of`
2019-09-19T02:47:04.6537413Z 82    |
2019-09-19T02:47:04.6537665Z 83    = help: the trait `std::marker::Sized` is not implemented for `[&U]`
2019-09-19T02:47:04.6538193Z 84    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-19T02:47:04.6538323Z 
2019-09-19T02:47:04.6538392Z The actual stderr differed from the expected stderr.
2019-09-19T02:47:04.6539057Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause/trait-suggest-where-clause.stderr
2019-09-19T02:47:04.6539057Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause/trait-suggest-where-clause.stderr
2019-09-19T02:47:04.6539615Z To update references, rerun the tests and pass the `--bless` flag
2019-09-19T02:47:04.6540129Z To only update this specific test, also pass `--test-args traits/trait-suggest-where-clause.rs`
2019-09-19T02:47:04.6540500Z error: 1 errors occurred comparing output.
2019-09-19T02:47:04.6540570Z status: exit code: 1
2019-09-19T02:47:04.6540570Z status: exit code: 1
2019-09-19T02:47:04.6541973Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-suggest-where-clause.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause/auxiliary" "-A" "unused"
2019-09-19T02:47:04.6542749Z ------------------------------------------
2019-09-19T02:47:04.6542800Z 
2019-09-19T02:47:04.6543388Z ------------------------------------------
2019-09-19T02:47:04.6543452Z stderr:
2019-09-19T02:47:04.6543452Z stderr:
2019-09-19T02:47:04.6543676Z ------------------------------------------
2019-09-19T02:47:04.6543755Z error[E0277]: the size for values of type `U` cannot be known at compilation time
2019-09-19T02:47:04.6544038Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:9:5
2019-09-19T02:47:04.6544126Z    |
2019-09-19T02:47:04.6544189Z LL |     mem::size_of::<U>();
2019-09-19T02:47:04.6544450Z    |     ^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-19T02:47:04.6544607Z    = help: the trait `std::marker::Sized` is not implemented for `U`
2019-09-19T02:47:04.6544607Z    = help: the trait `std::marker::Sized` is not implemented for `U`
2019-09-19T02:47:04.6544949Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-19T02:47:04.6545068Z    = help: consider adding a `where U: std::marker::Sized` bound
2019-09-19T02:47:04.6545206Z error[E0277]: the size for values of type `U` cannot be known at compilation time
2019-09-19T02:47:04.6545496Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:12:5
2019-09-19T02:47:04.6545585Z    |
2019-09-19T02:47:04.6545585Z    |
2019-09-19T02:47:04.6545644Z LL |     mem::size_of::<Misc<U>>();
2019-09-19T02:47:04.6545925Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-19T02:47:04.6546008Z    |
2019-09-19T02:47:04.6546095Z    = help: within `Misc<U>`, the trait `std::marker::Sized` is not implemented for `U`
2019-09-19T02:47:04.6546443Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-19T02:47:04.6546559Z    = help: consider adding a `where U: std::marker::Sized` bound
2019-09-19T02:47:04.6546638Z    = note: required because it appears within the type `Misc<U>`
2019-09-19T02:47:04.6546707Z 
2019-09-19T02:47:04.6546775Z error[E0277]: the trait bound `u64: std::convert::From<T>` is not satisfied
2019-09-19T02:47:04.6547148Z    |
2019-09-19T02:47:04.6547148Z    |
2019-09-19T02:47:04.6547778Z LL |     <u64 as From<T>>::from;
2019-09-19T02:47:04.6547868Z    |     ^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<T>` is not implemented for `u64`
2019-09-19T02:47:04.6547968Z    |
2019-09-19T02:47:04.6548051Z    = help: consider adding a `where u64: std::convert::From<T>` bound
2019-09-19T02:47:04.6548151Z    = note: required by `std::convert::From::from`
2019-09-19T02:47:04.6548199Z 
2019-09-19T02:47:04.6548297Z error[E0277]: the trait bound `u64: std::convert::From<<T as std::iter::Iterator>::Item>` is not satisfied
2019-09-19T02:47:04.6548738Z    |
2019-09-19T02:47:04.6548738Z    |
2019-09-19T02:47:04.6548808Z LL |     <u64 as From<<T as Iterator>::Item>>::from;
2019-09-19T02:47:04.6549206Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<<T as std::iter::Iterator>::Item>` is not implemented for `u64`
2019-09-19T02:47:04.6549456Z    |
2019-09-19T02:47:04.6549556Z    = help: consider adding a `where u64: std::convert::From<<T as std::iter::Iterator>::Item>` bound
2019-09-19T02:47:04.6549649Z    = note: required by `std::convert::From::from`
2019-09-19T02:47:04.6550586Z 
2019-09-19T02:47:04.6550996Z error[E0277]: the trait bound `Misc<_>: std::convert::From<T>` is not satisfied
2019-09-19T02:47:04.6551476Z    |
2019-09-19T02:47:04.6551476Z    |
2019-09-19T02:47:04.6551545Z LL |     <Misc<_> as From<T>>::from;
2019-09-19T02:47:04.6551622Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<T>` is not implemented for `Misc<_>`
2019-09-19T02:47:04.6551776Z    = note: required by `std::convert::From::from`
2019-09-19T02:47:04.6551836Z 
2019-09-19T02:47:04.6551904Z error[E0277]: the size for values of type `[T]` cannot be known at compilation time
2019-09-19T02:47:04.6552209Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:30:5
2019-09-19T02:47:04.6552209Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:30:5
2019-09-19T02:47:04.6552280Z    |
2019-09-19T02:47:04.6552352Z LL |     mem::size_of::<[T]>();
2019-09-19T02:47:04.6552604Z    |     ^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-19T02:47:04.6552761Z    = help: the trait `std::marker::Sized` is not implemented for `[T]`
2019-09-19T02:47:04.6552761Z    = help: the trait `std::marker::Sized` is not implemented for `[T]`
2019-09-19T02:47:04.6553112Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-19T02:47:04.6553179Z 
2019-09-19T02:47:04.6553267Z error[E0277]: the size for values of type `[&U]` cannot be known at compilation time
2019-09-19T02:47:04.6553624Z    |
2019-09-19T02:47:04.6553624Z    |
2019-09-19T02:47:04.6553679Z LL |     mem::size_of::<[&U]>();
2019-09-19T02:47:04.6553948Z    |     ^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-19T02:47:04.6554027Z    |
2019-09-19T02:47:04.6554111Z    = help: the trait `std::marker::Sized` is not implemented for `[&U]`
2019-09-19T02:47:04.6554441Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-19T02:47:04.6554841Z error: aborting due to 7 previous errors
2019-09-19T02:47:04.6554883Z 
2019-09-19T02:47:04.6555213Z For more information about this error, try `rustc --explain E0277`.
2019-09-19T02:47:04.6555262Z 
---
2019-09-19T02:47:04.6561373Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-19T02:47:04.6561476Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-19T02:47:04.6561891Z 
2019-09-19T02:47:04.6561922Z 
2019-09-19T02:47:04.6563939Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-19T02:47:04.6571358Z 
2019-09-19T02:47:04.6571398Z 
2019-09-19T02:47:04.6571910Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-09-19T02:47:04.6572006Z Build completed unsuccessfully in 1:18:23
2019-09-19T02:47:04.6572006Z Build completed unsuccessfully in 1:18:23
2019-09-19T02:47:04.6572094Z == clock drift check ==
2019-09-19T02:47:04.6572177Z   local time: Thu Sep 19 02:47:04 UTC 2019
2019-09-19T02:47:04.6927157Z   network time: Thu, 19 Sep 2019 02:47:04 GMT
2019-09-19T02:47:04.6927359Z == end clock drift check ==
2019-09-19T02:47:05.5474319Z ##[error]Bash exited with code '1'.
2019-09-19T02:47:05.5510706Z ##[section]Starting: Upload CPU usage statistics
2019-09-19T02:47:05.5519180Z ==============================================================================
2019-09-19T02:47:05.5519293Z Task         : Bash
2019-09-19T02:47:05.5519362Z Description  : Run a Bash script on macOS, Linux, or Windows
