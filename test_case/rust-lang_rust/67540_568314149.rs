plain
2019-12-22T22:43:42.7089584Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-22T22:43:42.7296125Z ##[command]git config gc.auto 0
2019-12-22T22:43:42.7380378Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-22T22:43:42.7443024Z ##[command]git config --get-all http.proxy
2019-12-22T22:43:42.7646925Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67540/merge:refs/remotes/pull/67540/merge
---
2019-12-22T23:46:19.2212529Z .................................................................................................... 1600/9427
2019-12-22T23:46:23.9458017Z .................................................................................................... 1700/9427
2019-12-22T23:46:34.6246296Z .....................................................................................i.............. 1800/9427
2019-12-22T23:46:42.4873739Z .................................................................................................... 1900/9427
2019-12-22T23:46:49.5826473Z ......................................................................iiiii......................... 2000/9427
2019-12-22T23:47:11.2089725Z .................................................................................................... 2200/9427
2019-12-22T23:47:13.6958924Z .................................................................................................... 2300/9427
2019-12-22T23:47:16.4854138Z .................................................................................................... 2400/9427
2019-12-22T23:47:30.0801003Z .................................................................................................... 2500/9427
---
2019-12-22T23:50:27.3219377Z .i...............i.................................................................................. 4900/9427
2019-12-22T23:50:37.6576453Z .................................................................................................... 5000/9427
2019-12-22T23:50:42.7029929Z .............................................i...................................................... 5100/9427
2019-12-22T23:50:52.8100811Z .................................................................................................... 5200/9427
2019-12-22T23:50:58.8045020Z ............ii.ii...........i....................................................................... 5300/9427
2019-12-22T23:51:08.3268074Z .................................................................................................... 5500/9427
2019-12-22T23:51:19.8494352Z ..............................................................................................i..... 5600/9427
2019-12-22T23:51:28.1000734Z .................................................................................................... 5700/9427
2019-12-22T23:51:33.4034417Z .................................................................................................... 5800/9427
2019-12-22T23:51:33.4034417Z .................................................................................................... 5800/9427
2019-12-22T23:51:43.5191882Z ..................................................................................ii...i..ii........ 5900/9427
2019-12-22T23:52:07.0199565Z .................................................................................................... 6100/9427
2019-12-22T23:52:15.0837191Z .................................................................................................... 6200/9427
2019-12-22T23:52:23.4463215Z .......................................................................................F............ 6300/9427
2019-12-22T23:52:23.4463215Z .......................................................................................F............ 6300/9427
2019-12-22T23:52:36.9264556Z .........i..ii...................................................................................... 6400/9427
2019-12-22T23:52:56.8813341Z .....................................................................................i.............. 6600/9427
2019-12-22T23:52:59.1030274Z .................................................................................................... 6700/9427
2019-12-22T23:53:01.4539538Z .....................................................................................i.............. 6800/9427
2019-12-22T23:53:04.3112257Z .................................................................................................... 6900/9427
---
2019-12-22T23:54:41.2518467Z .................................................................................................... 7400/9427
2019-12-22T23:54:46.2157252Z .................................................................................................... 7500/9427
2019-12-22T23:54:51.5350287Z .................................................................................................... 7600/9427
2019-12-22T23:54:58.6340405Z .................................................................................................... 7700/9427
2019-12-22T23:55:09.6326044Z ...................................................................................................i 7800/9427
2019-12-22T23:55:16.7937795Z iii................................................................................................. 7900/9427
2019-12-22T23:55:31.4510192Z .................................................................................................... 8100/9427
2019-12-22T23:55:43.5584456Z .................................................................................................... 8200/9427
2019-12-22T23:55:55.7950134Z .................................................................................................... 8300/9427
2019-12-22T23:56:01.7545345Z .................................................................................................... 8400/9427
---
2019-12-22T23:57:59.9105831Z ---- [ui] ui/async-await/issues/issue-62009-1.rs stdout ----
2019-12-22T23:57:59.9106213Z diff of stderr:
2019-12-22T23:57:59.9106482Z 
2019-12-22T23:57:59.9106745Z 35    | 
2019-12-22T23:57:59.9107005Z 36   ::: $SRC_DIR/libstd/future.rs:LL:COL
2019-12-22T23:57:59.9107816Z - LL |     F: Future
2019-12-22T23:57:59.9108172Z + LL |     F: Future,
2019-12-22T23:57:59.9108172Z + LL |     F: Future,
2019-12-22T23:57:59.9109335Z 39    |        ------ required by this bound in `std::future::poll_with_tls_context`
2019-12-22T23:57:59.9110400Z 41 error: aborting due to 4 previous errors
2019-12-22T23:57:59.9112204Z 
2019-12-22T23:57:59.9112754Z 
2019-12-22T23:57:59.9113060Z The actual stderr differed from the expected stderr.
2019-12-22T23:57:59.9113060Z The actual stderr differed from the expected stderr.
2019-12-22T23:57:59.9113813Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/issue-62009-1.stderr
2019-12-22T23:57:59.9114501Z To update references, rerun the tests and pass the `--bless` flag
2019-12-22T23:57:59.9115153Z To only update this specific test, also pass `--test-args async-await/issues/issue-62009-1.rs`
2019-12-22T23:57:59.9116489Z error: 1 errors occurred comparing output.
2019-12-22T23:57:59.9116848Z status: exit code: 1
2019-12-22T23:57:59.9116848Z status: exit code: 1
2019-12-22T23:57:59.9118468Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62009-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/auxiliary" "-A" "unused"
2019-12-22T23:57:59.9120361Z ------------------------------------------
2019-12-22T23:57:59.9120676Z 
2019-12-22T23:57:59.9126040Z ------------------------------------------
2019-12-22T23:57:59.9126351Z stderr:
2019-12-22T23:57:59.9126351Z stderr:
2019-12-22T23:57:59.9126680Z ------------------------------------------
2019-12-22T23:57:59.9126742Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-12-22T23:57:59.9127057Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:7:5
2019-12-22T23:57:59.9127184Z LL | fn main() {
2019-12-22T23:57:59.9127426Z    |    ---- this is not `async`
2019-12-22T23:57:59.9127426Z    |    ---- this is not `async`
2019-12-22T23:57:59.9129385Z LL |     async { let (); }.await;
2019-12-22T23:57:59.9129508Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-12-22T23:57:59.9129549Z 
2019-12-22T23:57:59.9129602Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-12-22T23:57:59.9130060Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:9:5
2019-12-22T23:57:59.9130164Z LL |   fn main() {
2019-12-22T23:57:59.9130430Z    |      ---- this is not `async`
2019-12-22T23:57:59.9130481Z ...
2019-12-22T23:57:59.9130528Z LL | /     async {
2019-12-22T23:57:59.9130528Z LL | /     async {
2019-12-22T23:57:59.9130607Z LL | |     //~^ ERROR `await` is only allowed inside `async` functions and blocks
2019-12-22T23:57:59.9130676Z LL | |         let task1 = print_dur().await;
2019-12-22T23:57:59.9130726Z LL | |     }.await;
2019-12-22T23:57:59.9130780Z    | |___________^ only allowed inside `async` functions and blocks
2019-12-22T23:57:59.9130834Z 
2019-12-22T23:57:59.9130887Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-12-22T23:57:59.9131188Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:13:5
2019-12-22T23:57:59.9131308Z LL | fn main() {
2019-12-22T23:57:59.9131547Z    |    ---- this is not `async`
2019-12-22T23:57:59.9131616Z ...
2019-12-22T23:57:59.9131616Z ...
2019-12-22T23:57:59.9131663Z LL |     (|_| 2333).await;
2019-12-22T23:57:59.9131716Z    |     ^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-12-22T23:57:59.9131750Z 
2019-12-22T23:57:59.9132191Z error[E0277]: the trait bound `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:13:5: 13:15]: std::future::Future` is not satisfied
2019-12-22T23:57:59.9132573Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:13:5
2019-12-22T23:57:59.9132641Z    |
2019-12-22T23:57:59.9132708Z LL |     (|_| 2333).await;
2019-12-22T23:57:59.9133104Z    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:13:5: 13:15]`
2019-12-22T23:57:59.9133242Z   ::: /checkout/src/libstd/future.rs:78:8
2019-12-22T23:57:59.9133291Z    |
2019-12-22T23:57:59.9133338Z LL |     F: Future,
2019-12-22T23:57:59.9133338Z LL |     F: Future,
2019-12-22T23:57:59.9133720Z    |        ------ required by this bound in `std::future::poll_with_tls_context`
2019-12-22T23:57:59.9133810Z error: aborting due to 4 previous errors
2019-12-22T23:57:59.9133842Z 
2019-12-22T23:57:59.9133910Z Some errors have detailed explanations: E0277, E0728.
2019-12-22T23:57:59.9134184Z For more information about an error, try `rustc --explain E0277`.
---
2019-12-22T23:57:59.9135031Z ---- [ui] ui/closures/closure-move-sync.rs stdout ----
2019-12-22T23:57:59.9135086Z diff of stderr:
2019-12-22T23:57:59.9135135Z 
2019-12-22T23:57:59.9135179Z 6    | 
2019-12-22T23:57:59.9135229Z 7   ::: $SRC_DIR/libstd/thread/mod.rs:LL:COL
2019-12-22T23:57:59.9135277Z 8    |
2019-12-22T23:57:59.9135668Z - LL |     F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
2019-12-22T23:57:59.9135999Z -    |                          ---- required by this bound in `std::thread::spawn`
2019-12-22T23:57:59.9136233Z + LL |     F: Send + 'static,
2019-12-22T23:57:59.9136521Z +    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:57:59.9142363Z 12    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Receiver<()>`
2019-12-22T23:57:59.9142740Z 13    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
2019-12-22T23:57:59.9142804Z 
2019-12-22T23:57:59.9142990Z 21    | 
2019-12-22T23:57:59.9142990Z 21    | 
2019-12-22T23:57:59.9143062Z 22   ::: $SRC_DIR/libstd/thread/mod.rs:LL:COL
2019-12-22T23:57:59.9143291Z 23    |
2019-12-22T23:57:59.9144224Z - LL |     F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
2019-12-22T23:57:59.9144548Z -    |                          ---- required by this bound in `std::thread::spawn`
2019-12-22T23:57:59.9144817Z + LL |     F: Send + 'static,
2019-12-22T23:57:59.9145083Z +    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:57:59.9145210Z 27    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Sender<()>`
2019-12-22T23:57:59.9145279Z 28    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Sender<()>`
2019-12-22T23:57:59.9145320Z 
2019-12-22T23:57:59.9145349Z 
2019-12-22T23:57:59.9145349Z 
2019-12-22T23:57:59.9145417Z The actual stderr differed from the expected stderr.
2019-12-22T23:57:59.9145776Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-move-sync/closure-move-sync.stderr
2019-12-22T23:57:59.9146051Z To update references, rerun the tests and pass the `--bless` flag
2019-12-22T23:57:59.9146369Z To only update this specific test, also pass `--test-args closures/closure-move-sync.rs`
2019-12-22T23:57:59.9146469Z error: 1 errors occurred comparing output.
2019-12-22T23:57:59.9146537Z status: exit code: 1
2019-12-22T23:57:59.9146537Z status: exit code: 1
2019-12-22T23:57:59.9147481Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-move-sync.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-move-sync" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-move-sync/auxiliary" "-A" "unused"
2019-12-22T23:57:59.9147868Z ------------------------------------------
2019-12-22T23:57:59.9147905Z 
2019-12-22T23:57:59.9148180Z ------------------------------------------
2019-12-22T23:57:59.9148232Z stderr:
2019-12-22T23:57:59.9148232Z stderr:
2019-12-22T23:57:59.9148467Z ------------------------------------------
2019-12-22T23:57:59.9148528Z error[E0277]: `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
2019-12-22T23:57:59.9149349Z    |
2019-12-22T23:57:59.9149349Z    |
2019-12-22T23:57:59.9149397Z LL |     let t = thread::spawn(|| {
2019-12-22T23:57:59.9149481Z    |             ^^^^^^^^^^^^^ `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
2019-12-22T23:57:59.9149866Z   ::: /checkout/src/libstd/thread/mod.rs:616:8
2019-12-22T23:57:59.9150114Z    |
2019-12-22T23:57:59.9150430Z LL |     F: Send + 'static,
2019-12-22T23:57:59.9150699Z    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:57:59.9150699Z    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:57:59.9150771Z    |
2019-12-22T23:57:59.9150827Z    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Receiver<()>`
2019-12-22T23:57:59.9150996Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
2019-12-22T23:57:59.9151457Z    = note: required because it appears within the type `[closure@/checkout/src/test/ui/closures/closure-move-sync.rs:7:27: 10:6 recv:&std::sync::mpsc::Receiver<()>]`
2019-12-22T23:57:59.9151512Z 
2019-12-22T23:57:59.9151568Z error[E0277]: `std::sync::mpsc::Sender<()>` cannot be shared between threads safely
2019-12-22T23:57:59.9151921Z    |
2019-12-22T23:57:59.9151921Z    |
2019-12-22T23:57:59.9151981Z LL |     thread::spawn(|| tx.send(()).unwrap());
2019-12-22T23:57:59.9152059Z    |     ^^^^^^^^^^^^^ `std::sync::mpsc::Sender<()>` cannot be shared between threads safely
2019-12-22T23:57:59.9152164Z   ::: /checkout/src/libstd/thread/mod.rs:616:8
2019-12-22T23:57:59.9152229Z    |
2019-12-22T23:57:59.9152455Z LL |     F: Send + 'static,
2019-12-22T23:57:59.9152726Z    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:57:59.9152726Z    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:57:59.9152798Z    |
2019-12-22T23:57:59.9152853Z    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Sender<()>`
2019-12-22T23:57:59.9152918Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Sender<()>`
2019-12-22T23:57:59.9153332Z    = note: required because it appears within the type `[closure@/checkout/src/test/ui/closures/closure-move-sync.rs:19:19: 19:42 tx:&std::sync::mpsc::Sender<()>]`
2019-12-22T23:57:59.9153432Z error: aborting due to 2 previous errors
2019-12-22T23:57:59.9153493Z 
2019-12-22T23:57:59.9153768Z For more information about this error, try `rustc --explain E0277`.
2019-12-22T23:57:59.9153806Z 
2019-12-22T23:57:59.9153806Z 
2019-12-22T23:57:59.9154042Z ------------------------------------------
2019-12-22T23:57:59.9154096Z 
2019-12-22T23:57:59.9154124Z 
2019-12-22T23:57:59.9154375Z ---- [ui] ui/no-send-res-ports.rs stdout ----
2019-12-22T23:57:59.9154427Z diff of stderr:
2019-12-22T23:57:59.9154477Z 
2019-12-22T23:57:59.9154521Z 6    | 
2019-12-22T23:57:59.9154571Z 7   ::: $SRC_DIR/libstd/thread/mod.rs:LL:COL
2019-12-22T23:57:59.9154618Z 8    |
2019-12-22T23:57:59.9154946Z - LL |     F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
2019-12-22T23:57:59.9155231Z -    |                          ---- required by this bound in `std::thread::spawn`
2019-12-22T23:57:59.9155488Z + LL |     F: Send + 'static,
2019-12-22T23:57:59.9155822Z +    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:57:59.9155875Z 11    |
2019-12-22T23:57:59.9182566Z 12    = help: within `[closure@$DIR/no-send-res-ports.rs:26:19: 30:6 x:main::Foo]`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-12-22T23:57:59.9182659Z 13    = note: required because it appears within the type `Port<()>`
2019-12-22T23:57:59.9182728Z 
2019-12-22T23:57:59.9182812Z The actual stderr differed from the expected stderr.
2019-12-22T23:57:59.9183175Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-send-res-ports/no-send-res-ports.stderr
2019-12-22T23:57:59.9183175Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-send-res-ports/no-send-res-ports.stderr
2019-12-22T23:57:59.9183456Z To update references, rerun the tests and pass the `--bless` flag
2019-12-22T23:57:59.9183801Z To only update this specific test, also pass `--test-args no-send-res-ports.rs`
2019-12-22T23:57:59.9183897Z error: 1 errors occurred comparing output.
2019-12-22T23:57:59.9183965Z status: exit code: 1
2019-12-22T23:57:59.9183965Z status: exit code: 1
2019-12-22T23:57:59.9185074Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no-send-res-ports.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-send-res-ports" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-send-res-ports/auxiliary" "-A" "unused"
2019-12-22T23:57:59.9185605Z ------------------------------------------
2019-12-22T23:57:59.9185648Z 
2019-12-22T23:57:59.9185932Z ------------------------------------------
2019-12-22T23:57:59.9185988Z stderr:
2019-12-22T23:57:59.9185988Z stderr:
2019-12-22T23:57:59.9186251Z ------------------------------------------
2019-12-22T23:57:59.9186314Z error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
2019-12-22T23:57:59.9186682Z    |
2019-12-22T23:57:59.9186682Z    |
2019-12-22T23:57:59.9186733Z LL |     thread::spawn(move|| {
2019-12-22T23:57:59.9186802Z    |     ^^^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-12-22T23:57:59.9186908Z   ::: /checkout/src/libstd/thread/mod.rs:616:8
2019-12-22T23:57:59.9186975Z    |
2019-12-22T23:57:59.9187235Z LL |     F: Send + 'static,
2019-12-22T23:57:59.9187531Z    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:57:59.9187531Z    |        ---- required by this bound in `std::thread::spawn`
2019-12-22T23:57:59.9187603Z    |
2019-12-22T23:57:59.9188018Z    = help: within `[closure@/checkout/src/test/ui/no-send-res-ports.rs:26:19: 30:6 x:main::Foo]`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-12-22T23:57:59.9188092Z    = note: required because it appears within the type `Port<()>`
2019-12-22T23:57:59.9188171Z    = note: required because it appears within the type `main::Foo`
2019-12-22T23:57:59.9188555Z    = note: required because it appears within the type `[closure@/checkout/src/test/ui/no-send-res-ports.rs:26:19: 30:6 x:main::Foo]`
2019-12-22T23:57:59.9188675Z error: aborting due to previous error
2019-12-22T23:57:59.9188744Z 
2019-12-22T23:57:59.9189051Z For more information about this error, try `rustc --explain E0277`.
2019-12-22T23:57:59.9189091Z 
2019-12-22T23:57:59.9189091Z 
2019-12-22T23:57:59.9189419Z ------------------------------------------
2019-12-22T23:57:59.9189457Z 
2019-12-22T23:57:59.9189488Z 
2019-12-22T23:57:59.9190291Z ---- [ui] ui/type_length_limit.rs stdout ----
2019-12-22T23:57:59.9190371Z diff of stderr:
2019-12-22T23:57:59.9190404Z 
2019-12-22T23:57:59.9190774Z 1 error: reached the type-length limit while instantiating `std::mem::drop::<std::option::Op... G), (G, G, G), (G, G, G))))))>>`
2019-12-22T23:57:59.9191049Z 2   --> $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-12-22T23:57:59.9191120Z 3    |
2019-12-22T23:57:59.9191369Z - LL | pub fn drop<T>(_x: T) { }
2019-12-22T23:57:59.9191619Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-22T23:57:59.9191696Z + LL | pub fn drop<T>(_x: T) {}
2019-12-22T23:57:59.9191798Z 6    |
2019-12-22T23:57:59.9191798Z 6    |
2019-12-22T23:57:59.9191856Z 7    = note: consider adding a `#![type_length_limit="1094"]` attribute to your crate
2019-12-22T23:57:59.9191949Z 
2019-12-22T23:57:59.9191979Z 
2019-12-22T23:57:59.9192040Z The actual stderr differed from the expected stderr.
2019-12-22T23:57:59.9192403Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.stderr
2019-12-22T23:57:59.9192403Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.stderr
2019-12-22T23:57:59.9192699Z To update references, rerun the tests and pass the `--bless` flag
2019-12-22T23:57:59.9193004Z To only update this specific test, also pass `--test-args type_length_limit.rs`
2019-12-22T23:57:59.9193109Z error: 1 errors occurred comparing output.
2019-12-22T23:57:59.9193161Z status: exit code: 1
2019-12-22T23:57:59.9193161Z status: exit code: 1
2019-12-22T23:57:59.9194211Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type_length_limit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/auxiliary" "-A" "unused"
2019-12-22T23:57:59.9194728Z ------------------------------------------
2019-12-22T23:57:59.9194769Z 
2019-12-22T23:57:59.9195055Z ------------------------------------------
2019-12-22T23:57:59.9195108Z stderr:
2019-12-22T23:57:59.9195108Z stderr:
2019-12-22T23:57:59.9195363Z ------------------------------------------
2019-12-22T23:57:59.9195741Z error: reached the type-length limit while instantiating `std::mem::drop::<std::option::Op... G), (G, G, G), (G, G, G))))))>>`
2019-12-22T23:57:59.9196095Z    |
2019-12-22T23:57:59.9196095Z    |
2019-12-22T23:57:59.9196159Z LL | pub fn drop<T>(_x: T) {}
2019-12-22T23:57:59.9196260Z    |
2019-12-22T23:57:59.9196260Z    |
2019-12-22T23:57:59.9196329Z    = note: consider adding a `#![type_length_limit="1094"]` attribute to your crate
2019-12-22T23:57:59.9196429Z error: aborting due to previous error
2019-12-22T23:57:59.9196462Z 
2019-12-22T23:57:59.9196491Z 
2019-12-22T23:57:59.9196769Z ------------------------------------------
---
2019-12-22T23:57:59.9200691Z test result: FAILED. 9376 passed; 4 failed; 47 ignored; 0 measured; 0 filtered out
2019-12-22T23:57:59.9200769Z 
2019-12-22T23:57:59.9200854Z 
2019-12-22T23:57:59.9200886Z 
2019-12-22T23:57:59.9202960Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-22T23:57:59.9203249Z 
2019-12-22T23:57:59.9203281Z 
2019-12-22T23:57:59.9203634Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-22T23:57:59.9203707Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-22T23:57:59.9203707Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-22T23:57:59.9203772Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-22T23:57:59.9203988Z Build completed unsuccessfully in 1:07:38
2019-12-22T23:57:59.9204039Z == clock drift check ==
2019-12-22T23:57:59.9225169Z   local time: Sun Dec 22 23:57:59 UTC 2019
2019-12-22T23:58:00.0169989Z   network time: Sun, 22 Dec 2019 23:58:00 GMT
2019-12-22T23:58:00.0170241Z == end clock drift check ==
2019-12-22T23:58:01.0870964Z 
2019-12-22T23:58:01.1007109Z ##[error]Bash exited with code '1'.
2019-12-22T23:58:01.1050919Z ##[section]Starting: Checkout
2019-12-22T23:58:01.1052916Z ==============================================================================
2019-12-22T23:58:01.1053001Z Task         : Get sources
2019-12-22T23:58:01.1053060Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
