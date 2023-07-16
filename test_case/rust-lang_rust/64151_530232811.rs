plain
2019-09-11T05:57:10.0526613Z ---- [ui] ui/async-await/issues/issue-62009-1.rs stdout ----
2019-09-11T05:57:10.0526855Z diff of stderr:
2019-09-11T05:57:10.0526999Z 
2019-09-11T05:57:10.0555857Z 32    |
2019-09-11T05:57:10.0556409Z 33 LL |     (|_| 2333).await;
2019-09-11T05:57:10.0564722Z 34    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@$DIR/issue-62009-1.rs:12:5: 12:15]`
2019-09-11T05:57:10.0565042Z -    | 
2019-09-11T05:57:10.0565582Z -   ::: $SRC_DIR/libstd/future.rs:LL:COL
2019-09-11T05:57:10.0565956Z - LL |     F: Future
2019-09-11T05:57:10.0565956Z - LL |     F: Future
2019-09-11T05:57:10.0566223Z -    |        ------ required by this bound in `std::future::poll_with_tls_context`
2019-09-11T05:57:10.0566367Z 41 error: aborting due to 4 previous errors
2019-09-11T05:57:10.0566446Z 42 
2019-09-11T05:57:10.0566496Z 
2019-09-11T05:57:10.0566526Z 
2019-09-11T05:57:10.0566526Z 
2019-09-11T05:57:10.0566582Z The actual stderr differed from the expected stderr.
2019-09-11T05:57:10.0566917Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/issue-62009-1.stderr
2019-09-11T05:57:10.0567171Z To update references, rerun the tests and pass the `--bless` flag
2019-09-11T05:57:10.0567471Z To only update this specific test, also pass `--test-args async-await/issues/issue-62009-1.rs`
2019-09-11T05:57:10.0567595Z error: 1 errors occurred comparing output.
2019-09-11T05:57:10.0567654Z status: exit code: 1
2019-09-11T05:57:10.0567654Z status: exit code: 1
2019-09-11T05:57:10.0568493Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62009-1.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/auxiliary" "-A" "unused"
2019-09-11T05:57:10.0568938Z ------------------------------------------
2019-09-11T05:57:10.0569003Z 
2019-09-11T05:57:10.0569209Z ------------------------------------------
2019-09-11T05:57:10.0569285Z stderr:
2019-09-11T05:57:10.0569285Z stderr:
2019-09-11T05:57:10.0569481Z ------------------------------------------
2019-09-11T05:57:10.0569569Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-09-11T05:57:10.0569815Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:6:5
2019-09-11T05:57:10.0569950Z LL | fn main() {
2019-09-11T05:57:10.0571771Z    |    ---- this is not `async`
2019-09-11T05:57:10.0571771Z    |    ---- this is not `async`
2019-09-11T05:57:10.0572378Z LL |     async { let (); }.await;
2019-09-11T05:57:10.0572488Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-09-11T05:57:10.0572545Z 
2019-09-11T05:57:10.0572632Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-09-11T05:57:10.0572994Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:8:5
2019-09-11T05:57:10.0573162Z LL |   fn main() {
2019-09-11T05:57:10.0573414Z    |      ---- this is not `async`
2019-09-11T05:57:10.0573481Z ...
2019-09-11T05:57:10.0573555Z LL | /     async {
2019-09-11T05:57:10.0573555Z LL | /     async {
2019-09-11T05:57:10.0573791Z LL | |     //~^ ERROR `await` is only allowed inside `async` functions and blocks
2019-09-11T05:57:10.0573887Z LL | |         let task1 = print_dur().await;
2019-09-11T05:57:10.0573952Z LL | |     }.await;
2019-09-11T05:57:10.0574040Z    | |___________^ only allowed inside `async` functions and blocks
2019-09-11T05:57:10.0574088Z 
2019-09-11T05:57:10.0574182Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-09-11T05:57:10.0574460Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:12:5
2019-09-11T05:57:10.0574604Z LL | fn main() {
2019-09-11T05:57:10.0574838Z    |    ---- this is not `async`
2019-09-11T05:57:10.0574904Z ...
2019-09-11T05:57:10.0574904Z ...
2019-09-11T05:57:10.0574977Z LL |     (|_| 2333).await;
2019-09-11T05:57:10.0575315Z    |     ^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-09-11T05:57:10.0575376Z 
2019-09-11T05:57:10.0575906Z error[E0277]: the trait bound `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:12:5: 12:15]: std::future::Future` is not satisfied
2019-09-11T05:57:10.0576242Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:12:5
2019-09-11T05:57:10.0576319Z    |
2019-09-11T05:57:10.0576390Z LL |     (|_| 2333).await;
2019-09-11T05:57:10.0576761Z    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:12:5: 12:15]`
2019-09-11T05:57:10.0576921Z error: aborting due to 4 previous errors
2019-09-11T05:57:10.0576962Z 
2019-09-11T05:57:10.0577237Z For more information about this error, try `rustc --explain E0277`.
2019-09-11T05:57:10.0577287Z 
2019-09-11T05:57:10.0577287Z 
2019-09-11T05:57:10.0577522Z ------------------------------------------
2019-09-11T05:57:10.0577565Z 
2019-09-11T05:57:10.0577596Z 
2019-09-11T05:57:10.0577857Z ---- [ui] ui/closures/closure-move-sync.rs stdout ----
2019-09-11T05:57:10.0577926Z diff of stderr:
2019-09-11T05:57:10.0577961Z 
2019-09-11T05:57:10.0578033Z 3    |
2019-09-11T05:57:10.0578088Z 4 LL |     let t = thread::spawn(|| {
2019-09-11T05:57:10.0578187Z 5    |             ^^^^^^^^^^^^^ `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
2019-09-11T05:57:10.0578423Z -    | 
2019-09-11T05:57:10.0578651Z -   ::: $SRC_DIR/libstd/thread/mod.rs:LL:COL
2019-09-11T05:57:10.0578729Z 8    |
2019-09-11T05:57:10.0578988Z - LL |     F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
2019-09-11T05:57:10.0579284Z -    |                          ---- required by this bound in `std::thread::spawn`
2019-09-11T05:57:10.0579585Z 12    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Receiver<()>`
2019-09-11T05:57:10.0579697Z 13    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
2019-09-11T05:57:10.0579697Z 13    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
2019-09-11T05:57:10.0580690Z 14    = note: required because it appears within the type `[closure@$DIR/closure-move-sync.rs:6:27: 9:6 recv:&std::sync::mpsc::Receiver<()>]`
2019-09-11T05:57:10.0581115Z 18    |
2019-09-11T05:57:10.0581115Z 18    |
2019-09-11T05:57:10.0581181Z 19 LL |     thread::spawn(|| tx.send(()).unwrap());
2019-09-11T05:57:10.0581286Z 20    |     ^^^^^^^^^^^^^ `std::sync::mpsc::Sender<()>` cannot be shared between threads safely
2019-09-11T05:57:10.0581555Z -    | 
2019-09-11T05:57:10.0581808Z -   ::: $SRC_DIR/libstd/thread/mod.rs:LL:COL
2019-09-11T05:57:10.0582172Z -    |
2019-09-11T05:57:10.0582447Z - LL |     F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
2019-09-11T05:57:10.0582735Z -    |                          ---- required by this bound in `std::thread::spawn`
2019-09-11T05:57:10.0582908Z 27    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Sender<()>`
2019-09-11T05:57:10.0583046Z 28    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Sender<()>`
2019-09-11T05:57:10.0583115Z 
2019-09-11T05:57:10.0583149Z 
2019-09-11T05:57:10.0583149Z 
2019-09-11T05:57:10.0583232Z The actual stderr differed from the expected stderr.
2019-09-11T05:57:10.0583744Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-move-sync/closure-move-sync.stderr
2019-09-11T05:57:10.0584045Z To update references, rerun the tests and pass the `--bless` flag
2019-09-11T05:57:10.0584343Z To only update this specific test, also pass `--test-args closures/closure-move-sync.rs`
2019-09-11T05:57:10.0584467Z error: 1 errors occurred comparing output.
2019-09-11T05:57:10.0584550Z status: exit code: 1
2019-09-11T05:57:10.0584550Z status: exit code: 1
2019-09-11T05:57:10.0586927Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-move-sync.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-move-sync" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-move-sync/auxiliary" "-A" "unused"
2019-09-11T05:57:10.0587497Z ------------------------------------------
2019-09-11T05:57:10.0587543Z 
2019-09-11T05:57:10.0587768Z ------------------------------------------
2019-09-11T05:57:10.0587842Z stderr:
2019-09-11T05:57:10.0587842Z stderr:
2019-09-11T05:57:10.0588061Z ------------------------------------------
2019-09-11T05:57:10.0588137Z error[E0277]: `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
2019-09-11T05:57:10.0588478Z    |
2019-09-11T05:57:10.0588478Z    |
2019-09-11T05:57:10.0588548Z LL |     let t = thread::spawn(|| {
2019-09-11T05:57:10.0588634Z    |             ^^^^^^^^^^^^^ `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
2019-09-11T05:57:10.0588794Z    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Receiver<()>`
2019-09-11T05:57:10.0588908Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
2019-09-11T05:57:10.0588908Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
2019-09-11T05:57:10.0589305Z    = note: required because it appears within the type `[closure@/checkout/src/test/ui/closures/closure-move-sync.rs:6:27: 9:6 recv:&std::sync::mpsc::Receiver<()>]`
2019-09-11T05:57:10.0589391Z 
2019-09-11T05:57:10.0589473Z error[E0277]: `std::sync::mpsc::Sender<()>` cannot be shared between threads safely
2019-09-11T05:57:10.0589809Z    |
2019-09-11T05:57:10.0589809Z    |
2019-09-11T05:57:10.0589867Z LL |     thread::spawn(|| tx.send(()).unwrap());
2019-09-11T05:57:10.0589968Z    |     ^^^^^^^^^^^^^ `std::sync::mpsc::Sender<()>` cannot be shared between threads safely
2019-09-11T05:57:10.0590507Z    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Sender<()>`
2019-09-11T05:57:10.0590612Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Sender<()>`
2019-09-11T05:57:10.0590612Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Sender<()>`
2019-09-11T05:57:10.0591081Z    = note: required because it appears within the type `[closure@/checkout/src/test/ui/closures/closure-move-sync.rs:18:19: 18:42 tx:&std::sync::mpsc::Sender<()>]`
2019-09-11T05:57:10.0591378Z error: aborting due to 2 previous errors
2019-09-11T05:57:10.0591420Z 
2019-09-11T05:57:10.0591733Z For more information about this error, try `rustc --explain E0277`.
2019-09-11T05:57:10.0591787Z 
---
2019-09-11T05:57:10.0592593Z 3    |
2019-09-11T05:57:10.0592652Z 4 LL |      x: Error
2019-09-11T05:57:10.0592746Z 5    |      ^^^^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-09-11T05:57:10.0592964Z -    | 
2019-09-11T05:57:10.0593214Z -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
2019-09-11T05:57:10.0593414Z -    |
2019-09-11T05:57:10.0593836Z - LL |     fn hash<H: Hasher>(&self, state: &mut H);
2019-09-11T05:57:10.0594201Z 11 
2019-09-11T05:57:10.0594276Z 12 error: aborting due to previous error
2019-09-11T05:57:10.0594339Z 13 
2019-09-11T05:57:10.0594372Z 
2019-09-11T05:57:10.0594372Z 
2019-09-11T05:57:10.0594421Z 
2019-09-11T05:57:10.0594483Z The actual stderr differed from the expected stderr.
2019-09-11T05:57:10.0594965Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum-struct-variant/derives-span-Hash-enum-struct-variant.stderr
2019-09-11T05:57:10.0595607Z To update references, rerun the tests and pass the `--bless` flag
2019-09-11T05:57:10.0595913Z To only update this specific test, also pass `--test-args derives/derives-span-Hash-enum-struct-variant.rs`
2019-09-11T05:57:10.0596043Z error: 1 errors occurred comparing output.
2019-09-11T05:57:10.0596104Z status: exit code: 1
2019-09-11T05:57:10.0596104Z status: exit code: 1
2019-09-11T05:57:10.0596988Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-Hash-enum-struct-variant.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum-struct-variant" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum-struct-variant/auxiliary" "-A" "unused"
2019-09-11T05:57:10.0597452Z ------------------------------------------
2019-09-11T05:57:10.0597494Z 
2019-09-11T05:57:10.0597720Z ------------------------------------------
2019-09-11T05:57:10.0597781Z stderr:
2019-09-11T05:57:10.0597781Z stderr:
2019-09-11T05:57:10.0598001Z ------------------------------------------
2019-09-11T05:57:10.0598073Z error[E0277]: the trait bound `Error: std::hash::Hash` is not satisfied
2019-09-11T05:57:10.0598365Z   --> /checkout/src/test/ui/derives/derives-span-Hash-enum-struct-variant.rs:9:6
2019-09-11T05:57:10.0598455Z    |
2019-09-11T05:57:10.0598508Z LL |      x: Error //~ ERROR
2019-09-11T05:57:10.0598595Z    |      ^^^^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-09-11T05:57:10.0598696Z error: aborting due to previous error
2019-09-11T05:57:10.0598751Z 
2019-09-11T05:57:10.0598994Z For more information about this error, try `rustc --explain E0277`.
2019-09-11T05:57:10.0599059Z 
---
2019-09-11T05:57:10.0599931Z 3    |
2019-09-11T05:57:10.0600471Z 4 LL |      Error
2019-09-11T05:57:10.0600549Z 5    |      ^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-09-11T05:57:10.0600957Z -    | 
2019-09-11T05:57:10.0601185Z -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
2019-09-11T05:57:10.0601404Z -    |
2019-09-11T05:57:10.0601643Z - LL |     fn hash<H: Hasher>(&self, state: &mut H);
2019-09-11T05:57:10.0602007Z 11 
2019-09-11T05:57:10.0602084Z 12 error: aborting due to previous error
2019-09-11T05:57:10.0602149Z 13 
2019-09-11T05:57:10.0602211Z 
2019-09-11T05:57:10.0602211Z 
2019-09-11T05:57:10.0602245Z 
2019-09-11T05:57:10.0602308Z The actual stderr differed from the expected stderr.
2019-09-11T05:57:10.0602689Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum/derives-span-Hash-enum.stderr
2019-09-11T05:57:10.0603000Z To update references, rerun the tests and pass the `--bless` flag
2019-09-11T05:57:10.0603306Z To only update this specific test, also pass `--test-args derives/derives-span-Hash-enum.rs`
2019-09-11T05:57:10.0603453Z error: 1 errors occurred comparing output.
2019-09-11T05:57:10.0603535Z status: exit code: 1
2019-09-11T05:57:10.0603535Z status: exit code: 1
2019-09-11T05:57:10.0604683Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-Hash-enum.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum/auxiliary" "-A" "unused"
2019-09-11T05:57:10.0605389Z ------------------------------------------
2019-09-11T05:57:10.0605594Z 
2019-09-11T05:57:10.0605802Z ------------------------------------------
2019-09-11T05:57:10.0605879Z stderr:
---
2019-09-11T05:57:10.0607841Z 3    |
2019-09-11T05:57:10.0607901Z 4 LL |     x: Error
2019-09-11T05:57:10.0607986Z 5    |     ^^^^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-09-11T05:57:10.0608182Z -    | 
2019-09-11T05:57:10.0608404Z -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
2019-09-11T05:57:10.0608583Z -    |
2019-09-11T05:57:10.0608815Z - LL |     fn hash<H: Hasher>(&self, state: &mut H);
2019-09-11T05:57:10.0609150Z 11 
2019-09-11T05:57:10.0609206Z 12 error: aborting due to previous error
2019-09-11T05:57:10.0609284Z 13 
2019-09-11T05:57:10.0609315Z 
2019-09-11T05:57:10.0609315Z 
2019-09-11T05:57:10.0609345Z 
2019-09-11T05:57:10.0609420Z The actual stderr differed from the expected stderr.
2019-09-11T05:57:10.0609745Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-struct/derives-span-Hash-struct.stderr
2019-09-11T05:57:10.0610181Z To update references, rerun the tests and pass the `--bless` flag
2019-09-11T05:57:10.0610866Z To only update this specific test, also pass `--test-args derives/derives-span-Hash-struct.rs`
2019-09-11T05:57:10.0610988Z error: 1 errors occurred comparing output.
2019-09-11T05:57:10.0611074Z status: exit code: 1
2019-09-11T05:57:10.0611074Z status: exit code: 1
2019-09-11T05:57:10.0612014Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-Hash-struct.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-struct/auxiliary" "-A" "unused"
2019-09-11T05:57:10.0612506Z ------------------------------------------
2019-09-11T05:57:10.0612563Z 
2019-09-11T05:57:10.0612811Z ------------------------------------------
2019-09-11T05:57:10.0612881Z stderr:
2019-09-11T05:57:10.0612881Z stderr:
2019-09-11T05:57:10.0613123Z ------------------------------------------
2019-09-11T05:57:10.0613203Z error[E0277]: the trait bound `Error: std::hash::Hash` is not satisfied
2019-09-11T05:57:10.0613494Z   --> /checkout/src/test/ui/derives/derives-span-Hash-struct.rs:8:5
2019-09-11T05:57:10.0613570Z    |
2019-09-11T05:57:10.0613720Z LL |     x: Error //~ ERROR
2019-09-11T05:57:10.0613819Z    |     ^^^^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-09-11T05:57:10.0613933Z error: aborting due to previous error
2019-09-11T05:57:10.0614147Z 
2019-09-11T05:57:10.0614425Z For more information about this error, try `rustc --explain E0277`.
2019-09-11T05:57:10.0614476Z 
---
2019-09-11T05:57:10.0615523Z 3    |
2019-09-11T05:57:10.0615586Z 4 LL |     Error
2019-09-11T05:57:10.0615654Z 5    |     ^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-09-11T05:57:10.0615865Z -    | 
2019-09-11T05:57:10.0616078Z -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
2019-09-11T05:57:10.0616274Z -    |
2019-09-11T05:57:10.0616504Z - LL |     fn hash<H: Hasher>(&self, state: &mut H);
2019-09-11T05:57:10.0616840Z 11 
2019-09-11T05:57:10.0616904Z 12 error: aborting due to previous error
2019-09-11T05:57:10.0616963Z 13 
2019-09-11T05:57:10.0617007Z 
2019-09-11T05:57:10.0617007Z 
2019-09-11T05:57:10.0617038Z 
2019-09-11T05:57:10.0617097Z The actual stderr differed from the expected stderr.
2019-09-11T05:57:10.0617458Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-tuple-struct/derives-span-Hash-tuple-struct.stderr
2019-09-11T05:57:10.0617784Z To update references, rerun the tests and pass the `--bless` flag
2019-09-11T05:57:10.0618101Z To only update this specific test, also pass `--test-args derives/derives-span-Hash-tuple-struct.rs`
2019-09-11T05:57:10.0618234Z error: 1 errors occurred comparing output.
2019-09-11T05:57:10.0618299Z status: exit code: 1
2019-09-11T05:57:10.0618299Z status: exit code: 1
2019-09-11T05:57:10.0619488Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-Hash-tuple-struct.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-tuple-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-tuple-struct/auxiliary" "-A" "unused"
2019-09-11T05:57:10.0620040Z ------------------------------------------
2019-09-11T05:57:10.0620085Z 
2019-09-11T05:57:10.0620711Z ------------------------------------------
2019-09-11T05:57:10.0620799Z stderr:
---
2019-09-11T05:57:10.0622757Z ---- [ui] ui/interior-mutability/interior-mutability.rs stdout ----
2019-09-11T05:57:10.0622845Z diff of stderr:
2019-09-11T05:57:10.0622884Z 
2019-09-11T05:57:10.0622950Z 3    |
2019-09-11T05:57:10.0623014Z 4 LL |     catch_unwind(|| { x.set(23); });
2019-09-11T05:57:10.0623231Z 5    |     ^^^^^^^^^^^^ `std::cell::UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
2019-09-11T05:57:10.0623514Z -    | 
2019-09-11T05:57:10.0623756Z -   ::: $SRC_DIR/libstd/panic.rs:LL:COL
2019-09-11T05:57:10.0624106Z -    |
2019-09-11T05:57:10.0624370Z - LL | pub fn catch_unwind<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Result<R> {
2019-09-11T05:57:10.0624675Z -    |                                        ---------- required by this bound in `std::panic::catch_unwind`
2019-09-11T05:57:10.0624777Z 11    |
2019-09-11T05:57:10.0624873Z 12    = help: within `std::cell::Cell<i32>`, the trait `std::panic::RefUnwindSafe` is not implemented for `std::cell::UnsafeCell<i32>`
2019-09-11T05:57:10.0624973Z 13    = note: required because it appears within the type `std::cell::Cell<i32>`
2019-09-11T05:57:10.0625063Z 
2019-09-11T05:57:10.0625126Z The actual stderr differed from the expected stderr.
2019-09-11T05:57:10.0625946Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/interior-mutability/interior-mutability/interior-mutability.stderr
2019-09-11T05:57:10.0625946Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/interior-mutability/interior-mutability/interior-mutability.stderr
2019-09-11T05:57:10.0626211Z To update references, rerun the tests and pass the `--bless` flag
2019-09-11T05:57:10.0626497Z To only update this specific test, also pass `--test-args interior-mutability/interior-mutability.rs`
2019-09-11T05:57:10.0626618Z error: 1 errors occurred comparing output.
2019-09-11T05:57:10.0626676Z status: exit code: 1
2019-09-11T05:57:10.0626676Z status: exit code: 1
2019-09-11T05:57:10.0628548Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/interior-mutability/interior-mutability.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/interior-mutability/interior-mutability" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/interior-mutability/interior-mutability/auxiliary" "-A" "unused"
2019-09-11T05:57:10.0629340Z ------------------------------------------
2019-09-11T05:57:10.0629403Z 
2019-09-11T05:57:10.0629613Z ------------------------------------------
2019-09-11T05:57:10.0629684Z stderr:
2019-09-11T05:57:10.0629684Z stderr:
2019-09-11T05:57:10.0629881Z ------------------------------------------
2019-09-11T05:57:10.0630147Z error[E0277]: the type `std::cell::UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
2019-09-11T05:57:10.0631651Z    |
2019-09-11T05:57:10.0631651Z    |
2019-09-11T05:57:10.0631711Z LL |     catch_unwind(|| { x.set(23); });
2019-09-11T05:57:10.0631843Z    |     ^^^^^^^^^^^^ `std::cell::UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
2019-09-11T05:57:10.0631945Z    |
2019-09-11T05:57:10.0632047Z    = help: within `std::cell::Cell<i32>`, the trait `std::panic::RefUnwindSafe` is not implemented for `std::cell::UnsafeCell<i32>`
2019-09-11T05:57:10.0632154Z    = note: required because it appears within the type `std::cell::Cell<i32>`
2019-09-11T05:57:10.0632267Z    = note: required because of the requirements on the impl of `std::panic::UnwindSafe` for `&std::cell::Cell<i32>`
2019-09-11T05:57:10.0632699Z    = note: required because it appears within the type `[closure@/checkout/src/test/ui/interior-mutability/interior-mutability.rs:5:18: 5:35 x:&std::cell::Cell<i32>]`
2019-09-11T05:57:10.0633103Z error: aborting due to previous error
2019-09-11T05:57:10.0633147Z 
2019-09-11T05:57:10.0633989Z For more information about this error, try `rustc --explain E0277`.
2019-09-11T05:57:10.0634072Z 
2019-09-11T05:57:10.0634072Z 
2019-09-11T05:57:10.0634516Z ------------------------------------------
2019-09-11T05:57:10.0634710Z 
2019-09-11T05:57:10.0634756Z 
2019-09-11T05:57:10.0635018Z ---- [ui] ui/issues/issue-21160.rs stdout ----
2019-09-11T05:57:10.0635097Z diff of stderr:
2019-09-11T05:57:10.0635133Z 
2019-09-11T05:57:10.0635357Z 3    |
2019-09-11T05:57:10.0635570Z 4 LL | struct Foo(Bar);
2019-09-11T05:57:10.0635653Z 5    |            ^^^ the trait `std::hash::Hash` is not implemented for `Bar`
2019-09-11T05:57:10.0635842Z -    | 
2019-09-11T05:57:10.0636049Z -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
2019-09-11T05:57:10.0636222Z -    |
2019-09-11T05:57:10.0636454Z - LL |     fn hash<H: Hasher>(&self, state: &mut H);
2019-09-11T05:57:10.0636766Z 11 
2019-09-11T05:57:10.0636820Z 12 error: aborting due to previous error
2019-09-11T05:57:10.0636884Z 13 
2019-09-11T05:57:10.0636915Z 
2019-09-11T05:57:10.0636915Z 
2019-09-11T05:57:10.0636944Z 
2019-09-11T05:57:10.0637008Z The actual stderr differed from the expected stderr.
2019-09-11T05:57:10.0637484Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21160/issue-21160.stderr
2019-09-11T05:57:10.0637761Z To update references, rerun the tests and pass the `--bless` flag
2019-09-11T05:57:10.0638232Z To only update this specific test, also pass `--test-args issues/issue-21160.rs`
2019-09-11T05:57:10.0638353Z error: 1 errors occurred comparing output.
2019-09-11T05:57:10.0638427Z status: exit code: 1
2019-09-11T05:57:10.0638427Z status: exit code: 1
2019-09-11T05:57:10.0639476Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21160.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21160" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21160/auxiliary" "-A" "unused"
2019-09-11T05:57:10.0639975Z ------------------------------------------
2019-09-11T05:57:10.0640022Z 
2019-09-11T05:57:10.0640781Z ------------------------------------------
2019-09-11T05:57:10.0640856Z stderr:
2019-09-11T05:57:10.0640856Z stderr:
2019-09-11T05:57:10.0641099Z ------------------------------------------
2019-09-11T05:57:10.0641179Z error[E0277]: the trait bound `Bar: std::hash::Hash` is not satisfied
2019-09-11T05:57:10.0641461Z   --> /checkout/src/test/ui/issues/issue-21160.rs:8:12
2019-09-11T05:57:10.0641666Z    |
2019-09-11T05:57:10.0641736Z LL | struct Foo(Bar);
2019-09-11T05:57:10.0641814Z    |            ^^^ the trait `std::hash::Hash` is not implemented for `Bar`
2019-09-11T05:57:10.0641949Z error: aborting due to previous error
2019-09-11T05:57:10.0641994Z 
2019-09-11T05:57:10.0642329Z For more information about this error, try `rustc --explain E0277`.
2019-09-11T05:57:10.0642385Z 
2019-09-11T05:57:10.0642385Z 
2019-09-11T05:57:10.0642651Z ------------------------------------------
2019-09-11T05:57:10.0642700Z 
2019-09-11T05:57:10.0642734Z 
2019-09-11T05:57:10.0642999Z ---- [ui] ui/no-send-res-ports.rs stdout ----
2019-09-11T05:57:10.0643074Z diff of stderr:
2019-09-11T05:57:10.0643114Z 
2019-09-11T05:57:10.0643182Z 3    |
2019-09-11T05:57:10.0643243Z 4 LL |     thread::spawn(move|| {
2019-09-11T05:57:10.0643338Z 5    |     ^^^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-09-11T05:57:10.0643570Z -    | 
2019-09-11T05:57:10.0643832Z -   ::: $SRC_DIR/libstd/thread/mod.rs:LL:COL
2019-09-11T05:57:10.0644208Z -    |
2019-09-11T05:57:10.0644469Z - LL |     F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
2019-09-11T05:57:10.0644755Z -    |                          ---- required by this bound in `std::thread::spawn`
2019-09-11T05:57:10.0644832Z 11    |
2019-09-11T05:57:10.0645435Z 12    = help: within `[closure@$DIR/no-send-res-ports.rs:25:19: 29:6 x:main::Foo]`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-09-11T05:57:10.0645711Z 13    = note: required because it appears within the type `Port<()>`
2019-09-11T05:57:10.0645801Z 
2019-09-11T05:57:10.0645859Z The actual stderr differed from the expected stderr.
2019-09-11T05:57:10.0646350Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-send-res-ports/no-send-res-ports.stderr
2019-09-11T05:57:10.0646350Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-send-res-ports/no-send-res-ports.stderr
2019-09-11T05:57:10.0646622Z To update references, rerun the tests and pass the `--bless` flag
2019-09-11T05:57:10.0646900Z To only update this specific test, also pass `--test-args no-send-res-ports.rs`
2019-09-11T05:57:10.0647029Z error: 1 errors occurred comparing output.
2019-09-11T05:57:10.0647088Z status: exit code: 1
2019-09-11T05:57:10.0647088Z status: exit code: 1
2019-09-11T05:57:10.0647869Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no-send-res-ports.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-send-res-ports" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-send-res-ports/auxiliary" "-A" "unused"
2019-09-11T05:57:10.0648304Z ------------------------------------------
2019-09-11T05:57:10.0648359Z 
2019-09-11T05:57:10.0648575Z ------------------------------------------
2019-09-11T05:57:10.0648657Z stderr:
2019-09-11T05:57:10.0648657Z stderr:
2019-09-11T05:57:10.0648868Z ------------------------------------------
2019-09-11T05:57:10.0648953Z error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
2019-09-11T05:57:10.0649577Z    |
2019-09-11T05:57:10.0649577Z    |
2019-09-11T05:57:10.0649631Z LL |     thread::spawn(move|| {
2019-09-11T05:57:10.0649865Z    |     ^^^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-09-11T05:57:10.0650969Z    |
2019-09-11T05:57:10.0651523Z    = help: within `[closure@/checkout/src/test/ui/no-send-res-ports.rs:25:19: 29:6 x:main::Foo]`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-09-11T05:57:10.0651642Z    = note: required because it appears within the type `Port<()>`
2019-09-11T05:57:10.0651740Z    = note: required because it appears within the type `main::Foo`
2019-09-11T05:57:10.0652120Z    = note: required because it appears within the type `[closure@/checkout/src/test/ui/no-send-res-ports.rs:25:19: 29:6 x:main::Foo]`
2019-09-11T05:57:10.0652404Z error: aborting due to previous error
2019-09-11T05:57:10.0652460Z 
2019-09-11T05:57:10.0653335Z For more information about this error, try `rustc --explain E0277`.
2019-09-11T05:57:10.0653413Z 
---
2019-09-11T05:57:10.0654524Z diff of stderr:
2019-09-11T05:57:10.0654561Z 
2019-09-11T05:57:10.0654614Z 5 LL | |     "0".parse()
2019-09-11T05:57:10.0654683Z 6 LL | | }
2019-09-11T05:57:10.0654751Z 7    | |_^ `main` can only return types that implement `std::process::Termination`
2019-09-11T05:57:10.0654972Z -    | 
2019-09-11T05:57:10.0655179Z -   ::: $SRC_DIR/libtest/lib.rs:LL:COL
2019-09-11T05:57:10.0655694Z -    |
2019-09-11T05:57:10.0655916Z - LL |   pub fn assert_test_result<T: Termination>(result: T) {
2019-09-11T05:57:10.0656210Z -    |                                ----------- required by this bound in `test::assert_test_result`
2019-09-11T05:57:10.0656283Z 13    |
2019-09-11T05:57:10.0656374Z 14    = help: the trait `std::process::Termination` is not implemented for `std::result::Result<f32, std::num::ParseFloatError>`
2019-09-11T05:57:10.0656494Z 
2019-09-11T05:57:10.0656522Z 
2019-09-11T05:57:10.0656691Z The actual stderr differed from the expected stderr.
2019-09-11T05:57:10.0657084Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/termination-trait-test-wrong-type.stderr
2019-09-11T05:57:10.0657084Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/termination-trait-test-wrong-type.stderr
2019-09-11T05:57:10.0657361Z To update references, rerun the tests and pass the `--bless` flag
2019-09-11T05:57:10.0657656Z To only update this specific test, also pass `--test-args rfc-1937-termination-trait/termination-trait-test-wrong-type.rs`
2019-09-11T05:57:10.0657791Z error: 1 errors occurred comparing output.
2019-09-11T05:57:10.0657861Z status: exit code: 1
2019-09-11T05:57:10.0657861Z status: exit code: 1
2019-09-11T05:57:10.0658757Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/auxiliary" "-A" "unused"
2019-09-11T05:57:10.0659220Z ------------------------------------------
2019-09-11T05:57:10.0659263Z 
2019-09-11T05:57:10.0659477Z ------------------------------------------
2019-09-11T05:57:10.0659547Z stderr:
2019-09-11T05:57:10.0659547Z stderr:
2019-09-11T05:57:10.0660300Z ------------------------------------------
2019-09-11T05:57:10.0660396Z error[E0277]: `main` has invalid return type `std::result::Result<f32, std::num::ParseFloatError>`
2019-09-11T05:57:10.0660827Z    |
2019-09-11T05:57:10.0660827Z    |
2019-09-11T05:57:10.0661125Z LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseFloatError> { //~ ERROR
2019-09-11T05:57:10.0661206Z LL | |     "0".parse()
2019-09-11T05:57:10.0661285Z LL | | }
2019-09-11T05:57:10.0661355Z    | |_^ `main` can only return types that implement `std::process::Termination`
2019-09-11T05:57:10.0661444Z    |
2019-09-11T05:57:10.0661530Z    = help: the trait `std::process::Termination` is not implemented for `std::result::Result<f32, std::num::ParseFloatError>`
2019-09-11T05:57:10.0661674Z error: aborting due to previous error
2019-09-11T05:57:10.0661832Z 
2019-09-11T05:57:10.0662138Z For more information about this error, try `rustc --explain E0277`.
2019-09-11T05:57:10.0662191Z 
2019-09-11T05:57:10.0662191Z 
2019-09-11T05:57:10.0662431Z ------------------------------------------
2019-09-11T05:57:10.0662476Z 
2019-09-11T05:57:10.0662509Z 
2019-09-11T05:57:10.0662772Z ---- [ui] ui/traits/trait-suggest-where-clause.rs stdout ----
2019-09-11T05:57:10.0662846Z diff of stderr:
2019-09-11T05:57:10.0662896Z 
2019-09-11T05:57:10.0662962Z 3    |
2019-09-11T05:57:10.0663034Z 4 LL |     mem::size_of::<U>();
2019-09-11T05:57:10.0663309Z 5    |     ^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-11T05:57:10.0663688Z -    | 
2019-09-11T05:57:10.0663907Z -   ::: $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-09-11T05:57:10.0663985Z 8    |
2019-09-11T05:57:10.0664204Z - LL | pub const fn size_of<T>() -> usize {
2019-09-11T05:57:10.0664479Z -    |                      - required by this bound in `std::mem::size_of`
2019-09-11T05:57:10.0664774Z 12    = help: the trait `std::marker::Sized` is not implemented for `U`
2019-09-11T05:57:10.0664774Z 12    = help: the trait `std::marker::Sized` is not implemented for `U`
2019-09-11T05:57:10.0665130Z 13    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-11T05:57:10.0668358Z 14    = help: consider adding a `where U: std::marker::Sized` bound
2019-09-11T05:57:10.0668483Z 18    |
2019-09-11T05:57:10.0668483Z 18    |
2019-09-11T05:57:10.0668694Z 19 LL |     mem::size_of::<Misc<U>>();
2019-09-11T05:57:10.0669096Z 20    |     ^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-11T05:57:10.0669299Z -    | 
2019-09-11T05:57:10.0669528Z -   ::: $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-09-11T05:57:10.0669592Z 23    |
2019-09-11T05:57:10.0670233Z - LL | pub const fn size_of<T>() -> usize {
2019-09-11T05:57:10.0670516Z -    |                      - required by this bound in `std::mem::size_of`
2019-09-11T05:57:10.0670738Z -    |
2019-09-11T05:57:10.0670816Z 27    = help: within `Misc<U>`, the trait `std::marker::Sized` is not implemented for `U`
2019-09-11T05:57:10.0671213Z 28    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-11T05:57:10.0671321Z 29    = help: consider adding a `where U: std::marker::Sized` bound
2019-09-11T05:57:10.0671438Z 60    |
2019-09-11T05:57:10.0671438Z 60    |
2019-09-11T05:57:10.0671508Z 61 LL |     mem::size_of::<[T]>();
2019-09-11T05:57:10.0671792Z 62    |     ^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-11T05:57:10.0672015Z -    | 
2019-09-11T05:57:10.0672241Z -   ::: $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-09-11T05:57:10.0672321Z 65    |
2019-09-11T05:57:10.0672549Z - LL | pub const fn size_of<T>() -> usize {
2019-09-11T05:57:10.0672830Z -    |                      - required by this bound in `std::mem::size_of`
2019-09-11T05:57:10.0673122Z 69    = help: the trait `std::marker::Sized` is not implemented for `[T]`
2019-09-11T05:57:10.0673122Z 69    = help: the trait `std::marker::Sized` is not implemented for `[T]`
2019-09-11T05:57:10.0673639Z 70    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-11T05:57:10.0673771Z 
2019-09-11T05:57:10.0673835Z 74    |
2019-09-11T05:57:10.0673835Z 74    |
2019-09-11T05:57:10.0673890Z 75 LL |     mem::size_of::<[&U]>();
2019-09-11T05:57:10.0674157Z 76    |     ^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-11T05:57:10.0674379Z -    | 
2019-09-11T05:57:10.0674624Z -   ::: $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-09-11T05:57:10.0674833Z -    |
2019-09-11T05:57:10.0675072Z - LL | pub const fn size_of<T>() -> usize {
2019-09-11T05:57:10.0675646Z -    |                      - required by this bound in `std::mem::size_of`
2019-09-11T05:57:10.0675728Z 82    |
2019-09-11T05:57:10.0675788Z 83    = help: the trait `std::marker::Sized` is not implemented for `[&U]`
2019-09-11T05:57:10.0676118Z 84    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-11T05:57:10.0676333Z 
2019-09-11T05:57:10.0676402Z The actual stderr differed from the expected stderr.
2019-09-11T05:57:10.0676745Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause/trait-suggest-where-clause.stderr
2019-09-11T05:57:10.0676745Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause/trait-suggest-where-clause.stderr
2019-09-11T05:57:10.0677050Z To update references, rerun the tests and pass the `--bless` flag
2019-09-11T05:57:10.0677322Z To only update this specific test, also pass `--test-args traits/trait-suggest-where-clause.rs`
2019-09-11T05:57:10.0677444Z error: 1 errors occurred comparing output.
2019-09-11T05:57:10.0677516Z status: exit code: 1
2019-09-11T05:57:10.0677516Z status: exit code: 1
2019-09-11T05:57:10.0678509Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-suggest-where-clause.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause/auxiliary" "-A" "unused"
2019-09-11T05:57:10.0678969Z ------------------------------------------
2019-09-11T05:57:10.0679012Z 
2019-09-11T05:57:10.0679316Z ------------------------------------------
2019-09-11T05:57:10.0679387Z stderr:
2019-09-11T05:57:10.0679387Z stderr:
2019-09-11T05:57:10.0679627Z ------------------------------------------
2019-09-11T05:57:10.0680040Z error[E0277]: the size for values of type `U` cannot be known at compilation time
2019-09-11T05:57:10.0680381Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:7:5
2019-09-11T05:57:10.0680461Z    |
2019-09-11T05:57:10.0680532Z LL |     mem::size_of::<U>();
2019-09-11T05:57:10.0680799Z    |     ^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-11T05:57:10.0680979Z    = help: the trait `std::marker::Sized` is not implemented for `U`
2019-09-11T05:57:10.0680979Z    = help: the trait `std::marker::Sized` is not implemented for `U`
2019-09-11T05:57:10.0681384Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-11T05:57:10.0681511Z    = help: consider adding a `where U: std::marker::Sized` bound
2019-09-11T05:57:10.0681653Z error[E0277]: the size for values of type `U` cannot be known at compilation time
2019-09-11T05:57:10.0681985Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:10:5
2019-09-11T05:57:10.0682063Z    |
2019-09-11T05:57:10.0682063Z    |
2019-09-11T05:57:10.0682138Z LL |     mem::size_of::<Misc<U>>();
2019-09-11T05:57:10.0682432Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-11T05:57:10.0682528Z    |
2019-09-11T05:57:10.0682603Z    = help: within `Misc<U>`, the trait `std::marker::Sized` is not implemented for `U`
2019-09-11T05:57:10.0683019Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-11T05:57:10.0683143Z    = help: consider adding a `where U: std::marker::Sized` bound
2019-09-11T05:57:10.0683228Z    = note: required because it appears within the type `Misc<U>`
2019-09-11T05:57:10.0683281Z 
2019-09-11T05:57:10.0683367Z error[E0277]: the trait bound `u64: std::convert::From<T>` is not satisfied
2019-09-11T05:57:10.0683906Z    |
2019-09-11T05:57:10.0683906Z    |
2019-09-11T05:57:10.0683962Z LL |     <u64 as From<T>>::from;
2019-09-11T05:57:10.0684054Z    |     ^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<T>` is not implemented for `u64`
2019-09-11T05:57:10.0684140Z    |
2019-09-11T05:57:10.0684207Z    = help: consider adding a `where u64: std::convert::From<T>` bound
2019-09-11T05:57:10.0684298Z    = note: required by `std::convert::From::from`
2019-09-11T05:57:10.0684456Z 
2019-09-11T05:57:10.0684541Z error[E0277]: the trait bound `u64: std::convert::From<<T as std::iter::Iterator>::Item>` is not satisfied
2019-09-11T05:57:10.0684937Z    |
2019-09-11T05:57:10.0684937Z    |
2019-09-11T05:57:10.0685016Z LL |     <u64 as From<<T as Iterator>::Item>>::from;
2019-09-11T05:57:10.0685127Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<<T as std::iter::Iterator>::Item>` is not implemented for `u64`
2019-09-11T05:57:10.0685392Z    |
2019-09-11T05:57:10.0685637Z    = help: consider adding a `where u64: std::convert::From<<T as std::iter::Iterator>::Item>` bound
2019-09-11T05:57:10.0685719Z    = note: required by `std::convert::From::from`
2019-09-11T05:57:10.0685760Z 
2019-09-11T05:57:10.0686001Z error[E0277]: the trait bound `Misc<_>: std::convert::From<T>` is not satisfied
2019-09-11T05:57:10.0686333Z    |
2019-09-11T05:57:10.0686333Z    |
2019-09-11T05:57:10.0686384Z LL |     <Misc<_> as From<T>>::from;
2019-09-11T05:57:10.0686470Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<T>` is not implemented for `Misc<_>`
2019-09-11T05:57:10.0686614Z    = note: required by `std::convert::From::from`
2019-09-11T05:57:10.0686653Z 
2019-09-11T05:57:10.0686810Z error[E0277]: the size for values of type `[T]` cannot be known at compilation time
2019-09-11T05:57:10.0687303Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:28:5
2019-09-11T05:57:10.0687303Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:28:5
2019-09-11T05:57:10.0687382Z    |
2019-09-11T05:57:10.0687434Z LL |     mem::size_of::<[T]>();
2019-09-11T05:57:10.0687864Z    |     ^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-11T05:57:10.0688016Z    = help: the trait `std::marker::Sized` is not implemented for `[T]`
2019-09-11T05:57:10.0688016Z    = help: the trait `std::marker::Sized` is not implemented for `[T]`
2019-09-11T05:57:10.0688373Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-11T05:57:10.0688468Z 
2019-09-11T05:57:10.0688540Z error[E0277]: the size for values of type `[&U]` cannot be known at compilation time
2019-09-11T05:57:10.0688911Z    |
2019-09-11T05:57:10.0688911Z    |
2019-09-11T05:57:10.0688980Z LL |     mem::size_of::<[&U]>();
2019-09-11T05:57:10.0689257Z    |     ^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-11T05:57:10.0689345Z    |
2019-09-11T05:57:10.0689411Z    = help: the trait `std::marker::Sized` is not implemented for `[&U]`
2019-09-11T05:57:10.0690118Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-11T05:57:10.0690271Z error: aborting due to 7 previous errors
2019-09-11T05:57:10.0690315Z 
2019-09-11T05:57:10.0690615Z For more information about this error, try `rustc --explain E0277`.
2019-09-11T05:57:10.0690683Z 
---
2019-09-11T05:57:10.0694900Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-11T05:57:10.0695011Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-11T05:57:10.0695063Z 
2019-09-11T05:57:10.0695094Z 
2019-09-11T05:57:10.0697361Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-11T05:57:10.0697932Z 
2019-09-11T05:57:10.0697966Z 
2019-09-11T05:57:10.0698411Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-09-11T05:57:10.0698498Z Build completed unsuccessfully in 1:13:10
2019-09-11T05:57:10.0698498Z Build completed unsuccessfully in 1:13:10
2019-09-11T05:57:10.0698733Z == clock drift check ==
2019-09-11T05:57:10.0698790Z   local time: Wed Sep 11 05:57:10 UTC 2019
2019-09-11T05:57:10.2277123Z   network time: Wed, 11 Sep 2019 05:57:10 GMT
2019-09-11T05:57:10.2282654Z == end clock drift check ==
2019-09-11T05:57:10.9608358Z ##[error]Bash exited with code '1'.
2019-09-11T05:57:10.9646823Z ##[section]Starting: Upload CPU usage statistics
2019-09-11T05:57:10.9652885Z ==============================================================================
2019-09-11T05:57:10.9652978Z Task         : Bash
2019-09-11T05:57:10.9653064Z Description  : Run a Bash script on macOS, Linux, or Windows
