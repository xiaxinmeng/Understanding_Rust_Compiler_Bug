plain
  python3-oauthlib python3-pyparsing python3-secretstorage python3-six
  python3-software-properties python3-wadllib python3-zipp systemd
  systemd-sysv systemd-timesyncd unattended-upgrades
Suggested packages:
  default-dbus-session-bus | dbus-session-bus dbus-user-session
  pinentry-gnome3 tor parcimonie xloadimage scdaemon isoquery
  gstreamer1.0-tools iw | wireless-tools appstream pinentry-doc
  python3-apt-dbg python-apt-doc python-blinker-doc python-cryptography-doc
  python3-cryptography-vectors python-dbus-doc python3-crypto gir1.2-secret-1
  gnome-keyring libkf5wallet-bin python3-keyrings.alt python3-testresources
  python-pyparsing-doc python-secretstorage-doc systemd-container libfido2-1
  libtss2-esys-3.0.2-0 libtss2-mu0 libtss2-rc0 bsd-mailx default-mta
  | mail-transport-agent needrestart powermgmt-base
  apt-transport-https dbus dirmngr distro-info-data dmsetup gir1.2-glib-2.0
  gir1.2-packagekitglib-1.0 gnupg gnupg-l10n gnupg-utils gpg gpg-agent
  gpg-wks-client gpg-wks-server gpgconf gpgsm iso-codes libapparmor1
  libappstream4 libargon2-1 libassuan0 libcap2-bin libcryptsetup12 libdbus-1-3
---
Successfully built a366be08301d
Successfully tagged rust-ci:latest
Built container sha256:a366be08301d2ab29a7e5d504d3367f6879d7df7bbbcf41a25cc36ccc54ab873
Uploading finished image to https://ci-caches.rust-lang.org/docker/c5b89d39ad489cccff774ea7e5f2ba4bac05c17cec4e3fc9bc723a692a9f276f24875116b84f5c924781dbf77a3fb5eb823154191e0d213687b94a5b21600b78
upload failed: - to s3://rust-lang-ci-sccache2/docker/c5b89d39ad489cccff774ea7e5f2ba4bac05c17cec4e3fc9bc723a692a9f276f24875116b84f5c924781dbf77a3fb5eb823154191e0d213687b94a5b21600b78 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
........................................................................................ 968/13448
........................................................................................ 1056/13448
........................................................................................ 1144/13448
........................................................................................ 1232/13448
.......F......................................i..............F.......................... 1320/13448
.................i...................................................................... 1496/13448
........................................................................................ 1584/13448
........................................................................................ 1672/13448
.......................................................i......ii........................ 1760/13448
---
........................................i............................................... 6600/13448
........................................................................................ 6688/13448
.................i.......................................................ii.ii........i. 6776/13448
...i...............................................................i.................... 6864/13448
........................................................................F.....F......... 6952/13448
......................F.F.......F.F.FFFi.F...i........................................i. 7040/13448
.i...................................................................................... 7216/13448
....................i................................................................... 7304/13448
........................................................................................ 7392/13448
........................................................................................ 7480/13448
---
........................................................................................ 10648/13448
...................iiiii...i...i.i...................................................... 10736/13448
...........................................................................i............ 10824/13448
.....................................................................................iii 10912/13448
iii.i..ii.iiiii............................F..F...................F.F............FFFFF.F 11000/13448
.FF..................................................................................... 11088/13448
........................................................................................ 11264/13448
........................................................................................ 11352/13448
........................................................................................ 11440/13448
........................................................................................ 11528/13448
---
diff of stderr:

2   --> $DIR/issue-76547.rs:20:13
3    |
4 LL | async fn fut(bufs: &mut [&mut [u8]]) {
-    |                    -     - let's call the lifetime of this reference `'2`
+    |                    -     - let's call the lifetime of this reference `'1`
6    |                    |
7    |                    let's call the lifetime of this reference `'1`
8 LL |     ListFut(bufs).await
17   --> $DIR/issue-76547.rs:34:14
18    |
18    |
19 LL | async fn fut2(bufs: &mut [&mut [u8]]) -> i32 {
-    |                     -     - let's call the lifetime of this reference `'2`
+    |                     -     - let's call the lifetime of this reference `'1`
21    |                     |
22    |                     let's call the lifetime of this reference `'1`
23 LL |     ListFut2(bufs).await

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-76547/issue-76547.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-76547.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-76547.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-76547" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-76547/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | async fn fut(bufs: &mut [&mut [u8]]) {
   |                    -     - let's call the lifetime of this reference `'1`
   |                    |
   |                    let's call the lifetime of this reference `'1`
LL |     ListFut(bufs).await
   |             ^^^^ this usage requires that `'1` must outlive `'2`
help: consider introducing a named lifetime parameter
   |
   |
LL | async fn fut<'a>(bufs: &'a mut [&'a mut [u8]]) {
   |             ++++        ++       ++
error: lifetime may not live long enough
  --> /checkout/src/test/ui/async-await/issue-76547.rs:34:14
   |
   |
LL | async fn fut2(bufs: &mut [&mut [u8]]) -> i32 {
   |                     -     - let's call the lifetime of this reference `'1`
   |                     |
   |                     let's call the lifetime of this reference `'1`
LL |     ListFut2(bufs).await
   |              ^^^^ this usage requires that `'1` must outlive `'2`
help: consider introducing a named lifetime parameter
   |
   |
LL | async fn fut2<'a>(bufs: &'a mut [&'a mut [u8]]) -> i32 {
   |              ++++        ++       ++
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/borrowck/issue-53432-nested-closure-outlives-borrowed-value.rs stdout ----
diff of stderr:

4 LL |     let _action = move || {
6    |                   |     |
6    |                   |     |
-    |                   |     return type of closure `[closure@$DIR/issue-53432-nested-closure-outlives-borrowed-value.rs:4:9: 4:11]` contains a lifetime `'2`
+    |                   |     return type of closure `[closure@$DIR/issue-53432-nested-closure-outlives-borrowed-value.rs:4:9: 4:11]` contains a lifetime `'1`
8    |                   lifetime `'1` represents this closure's body
9 LL |         || f() // The `nested` closure
10    |         ^^^^^^ returning this value requires that `'1` must outlive `'2`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-53432-nested-closure-outlives-borrowed-value/issue-53432-nested-closure-outlives-borrowed-value.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/issue-53432-nested-closure-outlives-borrowed-value.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-53432-nested-closure-outlives-borrowed-value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-53432-nested-closure-outlives-borrowed-value" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-53432-nested-closure-outlives-borrowed-value/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     let _action = move || {
   |                   |     |
   |                   |     |
   |                   |     return type of closure `[closure@/checkout/src/test/ui/borrowck/issue-53432-nested-closure-outlives-borrowed-value.rs:4:9: 4:11]` contains a lifetime `'1`
   |                   lifetime `'1` represents this closure's body
LL |         || f() // The `nested` closure
   |         ^^^^^^ returning this value requires that `'1` must outlive `'2`
   |
   = note: closure implements `Fn`, so references to captured variables can't escape the closure
help: consider adding 'move' keyword before the nested closure
   |
LL |         move || f() // The `nested` closure

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/borrowck/issue-95079-missing-move-in-nested-closure.rs stdout ----
diff of stderr:

24 LL |     move |()| s.chars().map(|c| format!("{}{}", c, s))
25    |     --------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
26    |     |       |
-    |     |       return type of closure `Map<Chars<'_>, [closure@$DIR/issue-95079-missing-move-in-nested-closure.rs:9:29: 9:32]>` contains a lifetime `'2`
+    |     |       return type of closure `Map<Chars<'_>, [closure@$DIR/issue-95079-missing-move-in-nested-closure.rs:9:29: 9:32]>` contains a lifetime `'1`
28    |     lifetime `'1` represents this closure's body
29    |
30    = note: closure implements `Fn`, so references to captured variables can't escape the closure

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-95079-missing-move-in-nested-closure/issue-95079-missing-move-in-nested-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/issue-95079-missing-move-in-nested-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-95079-missing-move-in-nested-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-95079-missing-move-in-nested-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-95079-missing-move-in-nested-closure/auxiliary"
stdout: none
--- stderr -------------------------------
error: captured variable cannot escape `FnMut` closure body
   |
   |
LL | fn foo1(s: &str) -> impl Iterator<Item = String> + '_ {
   |         - variable defined here
LL |     None.into_iter()
LL |         .flat_map(move |()| s.chars().map(|c| format!("{}{}", c, s)))
   |                           - -^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                           | |
   |                           | returns a reference to a captured variable which escapes the closure body
   |                           | variable captured here
   |                           inferred to be a `FnMut` closure
   |
   = note: `FnMut` closures only have access to their captured variables while they are executing...
   = note: ...therefore, they cannot allow references to captured variables to escape
help: consider adding 'move' keyword before the nested closure
   |
LL |         .flat_map(move |()| s.chars().map(move |c| format!("{}{}", c, s)))

error: lifetime may not live long enough
  --> /checkout/src/test/ui/borrowck/issue-95079-missing-move-in-nested-closure.rs:9:15
   |
   |
LL |     move |()| s.chars().map(|c| format!("{}{}", c, s))
   |     --------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
   |     |       |
   |     |       return type of closure `Map<Chars<'_>, [closure@/checkout/src/test/ui/borrowck/issue-95079-missing-move-in-nested-closure.rs:9:29: 9:32]>` contains a lifetime `'1`
   |     lifetime `'1` represents this closure's body
   |
   = note: closure implements `Fn`, so references to captured variables can't escape the closure
help: consider adding 'move' keyword before the nested closure
   |
LL |     move |()| s.chars().map(move |c| format!("{}{}", c, s))

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/lifetimes/issue-79187-2.rs stdout ----
diff of stderr:

13 LL |     take_foo(|a: &i32| -> &i32 { a });
14    |                  -        -      ^ returning this value requires that `'1` must outlive `'2`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |                  |        let's call the lifetime of this reference `'2`
+    |                  |        let's call the lifetime of this reference `'1`
17    |                  let's call the lifetime of this reference `'1`
18 
19 error: implementation of `FnOnce` is not general enough

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187-2/issue-79187-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/issue-79187-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-79187-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     take_foo(|a: &i32| a);
   |                  -   - ^ returning this value requires that `'1` must outlive `'2`
   |                  |   |
   |                  |   return type of closure is &'2 i32
   |                  let's call the lifetime of this reference `'1`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:14:34
   |
   |
LL |     take_foo(|a: &i32| -> &i32 { a });
   |                  -        -      ^ returning this value requires that `'1` must outlive `'2`
   |                  |        |
   |                  |        let's call the lifetime of this reference `'1`
   |                  let's call the lifetime of this reference `'1`

error: implementation of `FnOnce` is not general enough
   |
   |
LL |     take_foo(|a| a);
   |     ^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(&'2 i32) -> &i32` must implement `FnOnce<(&'1 i32,)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&'2 i32,)>`, for some specific lifetime `'2`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:8:5
   |
   |
LL |     take_foo(|a| a);
   |     ^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'r> Fn<(&'r i32,)>`
              found trait `Fn<(&i32,)>`
note: this closure does not fulfill the lifetime requirements
   |
   |
LL |     take_foo(|a| a);
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:5:21
   |
   |
LL | fn take_foo(_: impl Foo) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:11:5
   |
   |
LL |     take_foo(|a: &i32| a);
   |     ^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   = note: expected reference `&i32`
              found reference `&i32`
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:5:21
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:5:21
   |
LL | fn take_foo(_: impl Foo) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:14:5
   |
   |
LL |     take_foo(|a: &i32| -> &i32 { a });
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   = note: expected reference `&i32`
              found reference `&i32`
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:5:21
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:5:21
   |
LL | fn take_foo(_: impl Foo) {}

error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/lifetimes/issue-90170-elision-mismatch.rs stdout ----
diff of stderr:

5    |                        -        -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
6    |                        |        |
7    |                        |        let's call the lifetime of this reference `'1`
-    |                        let's call the lifetime of this reference `'2`
+    |                        let's call the lifetime of this reference `'1`
10 help: consider introducing a named lifetime parameter
11    |


19    |                         -           -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
20    |                         |           |
21    |                         |           let's call the lifetime of this reference `'1`
-    |                         let's call the lifetime of this reference `'2`
+    |                         let's call the lifetime of this reference `'1`
24 help: consider introducing a named lifetime parameter
25    |


33    |                                               -        -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
34    |                                               |        |
35    |                                               |        let's call the lifetime of this reference `'1`
-    |                                               let's call the lifetime of this reference `'2`
+    |                                               let's call the lifetime of this reference `'1`
38 help: consider introducing a named lifetime parameter
39    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90170-elision-mismatch/issue-90170-elision-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/issue-90170-elision-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-90170-elision-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90170-elision-mismatch" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90170-elision-mismatch/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | pub fn foo(x: &mut Vec<&u8>, y: &u8) { x.push(y); } //~ ERROR lifetime may not live long enough
   |                        -        -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
   |                        |        |
   |                        |        let's call the lifetime of this reference `'1`
   |                        let's call the lifetime of this reference `'1`
help: consider introducing a named lifetime parameter
   |
   |
LL | pub fn foo<'a>(x: &mut Vec<&'a u8>, y: &'a u8) { x.push(y); } //~ ERROR lifetime may not live long enough
   |           ++++              ++          ++
error: lifetime may not live long enough
  --> /checkout/src/test/ui/lifetimes/issue-90170-elision-mismatch.rs:5:44
   |
   |
LL | pub fn foo2(x: &mut Vec<&'_ u8>, y: &u8) { x.push(y); } //~ ERROR lifetime may not live long enough
   |                         -           -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
   |                         |           |
   |                         |           let's call the lifetime of this reference `'1`
   |                         let's call the lifetime of this reference `'1`
help: consider introducing a named lifetime parameter
   |
   |
LL | pub fn foo2<'a>(x: &mut Vec<&'a u8>, y: &'a u8) { x.push(y); } //~ ERROR lifetime may not live long enough

error: lifetime may not live long enough
  --> /checkout/src/test/ui/lifetimes/issue-90170-elision-mismatch.rs:7:63
   |
   |
LL | pub fn foo3<'a>(_other: &'a [u8], x: &mut Vec<&u8>, y: &u8) { x.push(y); } //~ ERROR lifetime may not live long enough
   |                                               -        -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
   |                                               |        |
   |                                               |        let's call the lifetime of this reference `'1`
   |                                               let's call the lifetime of this reference `'1`
help: consider introducing a named lifetime parameter
   |
   |
LL | pub fn foo3<'a>(_other: &'a [u8], x: &mut Vec<&'a u8>, y: &'a u8) { x.push(y); } //~ ERROR lifetime may not live long enough
   |                                                ++          ++
error: aborting due to 3 previous errors
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-2.rs stdout ----
diff of stderr:

4 LL | fn foo(&mut (ref mut v, w): &mut (&u8, &u8), x: &u8) {
5    |                                   -             - let's call the lifetime of this reference `'1`
6    |                                   |
-    |                                   let's call the lifetime of this reference `'2`
+    |                                   let's call the lifetime of this reference `'1`
8 LL |     *v = x;
9    |     ^^^^^^ assignment requires that `'1` must outlive `'2`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-2/ex3-both-anon-regions-2.stderr
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex3-both-anon-regions-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo(&mut (ref mut v, w): &mut (&u8, &u8), x: &u8) {
   |                                   -             - let's call the lifetime of this reference `'1`
   |                                   |
   |                                   let's call the lifetime of this reference `'1`
LL |     *v = x;
   |     ^^^^^^ assignment requires that `'1` must outlive `'2`
help: consider introducing a named lifetime parameter
   |
   |
LL | fn foo<'a>(&mut (ref mut v, w): &mut (&'a u8, &u8), x: &'a u8) {
   |       ++++                             ++               ++
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-3.rs stdout ----
diff of stderr:

4 LL | fn foo(z: &mut Vec<(&u8,&u8)>, (x, y): (&u8, &u8)) {
5    |                     -                   - let's call the lifetime of this reference `'1`
6    |                     |
-    |                     let's call the lifetime of this reference `'2`
+    |                     let's call the lifetime of this reference `'1`
8 LL |     z.push((x,y));
9    |     ^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`


19 LL | fn foo(z: &mut Vec<(&u8,&u8)>, (x, y): (&u8, &u8)) {
20    |                         -                    - let's call the lifetime of this reference `'3`
21    |                         |
-    |                         let's call the lifetime of this reference `'4`
+    |                         let's call the lifetime of this reference `'3`
23 LL |     z.push((x,y));
24    |     ^^^^^^^^^^^^^ argument requires that `'3` must outlive `'4`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-3/ex3-both-anon-regions-3.stderr
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex3-both-anon-regions-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-3/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo(z: &mut Vec<(&u8,&u8)>, (x, y): (&u8, &u8)) {
   |                     -                   - let's call the lifetime of this reference `'1`
   |                     |
   |                     let's call the lifetime of this reference `'1`
LL |     z.push((x,y));
   |     ^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
help: consider introducing a named lifetime parameter
   |
   |
LL | fn foo<'a>(z: &mut Vec<(&'a u8,&u8)>, (x, y): (&'a u8, &u8)) {
   |       ++++               ++                     ++
error: lifetime may not live long enough
  --> /checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-3.rs:2:5
   |
   |
LL | fn foo(z: &mut Vec<(&u8,&u8)>, (x, y): (&u8, &u8)) {
   |                         -                    - let's call the lifetime of this reference `'3`
   |                         |
   |                         let's call the lifetime of this reference `'3`
LL |     z.push((x,y));
   |     ^^^^^^^^^^^^^ argument requires that `'3` must outlive `'4`
help: consider introducing a named lifetime parameter
   |
   |
LL | fn foo<'a>(z: &mut Vec<(&u8,&'a u8)>, (x, y): (&u8, &'a u8)) {
   |       ++++                   ++                      ++
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-return-type-is-anon.rs stdout ----
diff of stderr:

4 LL |   fn foo<'a>(&self, x: &i32) -> &i32 {
5    |              -         - let's call the lifetime of this reference `'1`
6    |              |
-    |              let's call the lifetime of this reference `'2`
+    |              let's call the lifetime of this reference `'1`
8 LL |     x
9    |     ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-return-type-is-anon/ex3-both-anon-regions-return-type-is-anon.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-return-type-is-anon/ex3-both-anon-regions-return-type-is-anon.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex3-both-anon-regions-return-type-is-anon.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-return-type-is-anon.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-return-type-is-anon" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-return-type-is-anon/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |   fn foo<'a>(&self, x: &i32) -> &i32 {
   |              -         - let's call the lifetime of this reference `'1`
   |              |
   |              let's call the lifetime of this reference `'1`
