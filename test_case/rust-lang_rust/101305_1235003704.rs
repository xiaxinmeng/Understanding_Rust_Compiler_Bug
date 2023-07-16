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
Successfully built 8cd3f55f32ba
Successfully tagged rust-ci:latest
Built container sha256:8cd3f55f32ba779b9fe10887e0a48710e10937be18d9a42a6722bc43abc51724
Uploading finished image to https://ci-caches.rust-lang.org/docker/c5b89d39ad489cccff774ea7e5f2ba4bac05c17cec4e3fc9bc723a692a9f276f24875116b84f5c924781dbf77a3fb5eb823154191e0d213687b94a5b21600b78
upload failed: - to s3://rust-lang-ci-sccache2/docker/c5b89d39ad489cccff774ea7e5f2ba4bac05c17cec4e3fc9bc723a692a9f276f24875116b84f5c924781dbf77a3fb5eb823154191e0d213687b94a5b21600b78 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
........................................................................................ 352/13448
........................................................................................ 440/13448
........................................................................................ 528/13448
........................................................................................ 616/13448
...........................................................F..F......................... 704/13448
......................................................i................................. 880/13448
........................................................................................ 968/13448
........................................................................................ 1056/13448
........................................................................................ 1144/13448
........................................................................................ 1144/13448
........................................................................................ 1232/13448
.....F........................................i...........F............................. 1320/13448
.....................................................F.................................. 1408/13448
........................................................................................ 1584/13448
........................................................................................ 1672/13448
.......................................................i......ii........................ 1760/13448
........................................................................................ 1848/13448
---
........................................i............................................... 6600/13448
........................................................................................ 6688/13448
.................i.......................................................ii.ii........i. 6776/13448
...i...............................................................i.................... 6864/13448
.......................................................................F......F......... 6952/13448
...................FF..F..FF..FF...FFF.i....iF.F......................................i. 7040/13448
.i...................................................................................... 7216/13448
....................i................................................................... 7304/13448
........................................................................................ 7392/13448
........................................................................................ 7480/13448
---
........................................................................................ 7920/13448
........................................................................................ 8008/13448
...........................................................ii................i......i..i 8096/13448
i....................................................................................... 8184/13448
.....................................................................F..........F....... 8272/13448
..F..F..F......................................................................F....F... 8360/13448
.............................F.......................................................... 8448/13448
.................i..ii..............................................................ii.. 8624/13448
........................................................................................ 8712/13448
..iiii.................................................................................. 8800/13448
............................................i........................................i.. 8888/13448
---
........................................................................................ 9856/13448
...............................................................................ii....... 9944/13448
........i............................................................................... 10032/13448
........................................................................................ 10120/13448
....................................................................................F.F. 10208/13448
..............................F........................................................F 10296/13448
..F..................................................................................... 10384/13448
........................................................................................ 10560/13448
........................................................................................ 10648/13448
..................iiiii...i....i.i...................................................... 10736/13448
...........................................................................i............ 10824/13448
...........................................................................i............ 10824/13448
.....................................................................................iii 10912/13448
iii.i..iiiiii.i..........................F....F..................F..F.............FFFFF. 11000/13448
FF..F................................................................................... 11088/13448
........................................................................................ 11264/13448
........................................................................................ 11352/13448
........................................................................................ 11440/13448
........................................................................................ 11528/13448
---

---- [ui] src/test/ui/async-await/issue-74497-lifetime-in-opaque.rs stdout ----
diff of stderr:

5    |                  -- ^^^^^^ returning this value requires that `'1` must outlive `'2`
6    |                  ||
7    |                  |return type of closure `impl Future<Output = ()>` contains a lifetime `'2`
-    |                  has type `&'1 u8`
+    |                  has type `impl Future<Output = ()>`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74497-lifetime-in-opaque/issue-74497-lifetime-in-opaque.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-74497-lifetime-in-opaque.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-74497-lifetime-in-opaque.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74497-lifetime-in-opaque" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74497-lifetime-in-opaque/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     let _ = foo(|x| bar(x)); //~ ERROR lifetime may not live long enough
   |                  -- ^^^^^^ returning this value requires that `'1` must outlive `'2`
   |                  ||
   |                  |return type of closure `impl Future<Output = ()>` contains a lifetime `'2`
   |                  has type `impl Future<Output = ()>`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/async-await/issue-76547.rs stdout ----
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



Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---- [ui] src/test/ui/c-variadic/variadic-ffi-4.rs stdout ----
diff of stderr:

44 LL |     let _ = ap.with_copy(|ap| ap);
45    |                           --- ^^ returning this value requires that `'1` must outlive `'2`
46    |                           | |
-    |                           | return type of closure is VaList<'2, '_>
+    |                           | return type of closure is VaList<'1, '_>
48    |                           has type `VaList<'1, '_>`
50 error: lifetime may not live long enough

51   --> $DIR/variadic-ffi-4.rs:22:5
52    |
52    |
53 LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               -------                   ------- has type `&mut VaListImpl<'1>`
56    |                                               has type `&mut VaListImpl<'1>`
57 LL |     *ap0 = ap1;


67 LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
68    |                                               -------                   ------- has type `VaListImpl<'2>`
-    |                                               has type `&mut VaListImpl<'1>`
+    |                                               has type `VaListImpl<'2>`
71 LL |     *ap0 = ap1;
71 LL |     *ap0 = ap1;
72    |     ^^^^ assignment requires that `'2` must outlive `'1`

79   --> $DIR/variadic-ffi-4.rs:28:5
80    |
80    |
81 LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               -------                   ------- has type `&mut VaListImpl<'1>`
84    |                                               has type `&mut VaListImpl<'1>`
84    |                                               has type `&mut VaListImpl<'1>`
85 LL |     ap0 = &mut ap1;

95 LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
96    |                                               -------                   ------- has type `VaListImpl<'2>`
-    |                                               has type `&mut VaListImpl<'1>`
+    |                                               has type `VaListImpl<'2>`
+    |                                               has type `VaListImpl<'2>`
99 LL |     ap0 = &mut ap1;
100    |     ^^^^^^^^^^^^^^ assignment requires that `'2` must outlive `'1`

121   --> $DIR/variadic-ffi-4.rs:35:12
122    |
122    |
123 LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               -------                   ------- has type `&mut VaListImpl<'1>`
126    |                                               has type `&mut VaListImpl<'1>`
126    |                                               has type `&mut VaListImpl<'1>`
127 LL |     *ap0 = ap1.clone();

137 LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
138    |                                               -------                   ------- has type `VaListImpl<'2>`
-    |                                               has type `&mut VaListImpl<'1>`
+    |                                               has type `VaListImpl<'2>`
+    |                                               has type `VaListImpl<'2>`
141 LL |     *ap0 = ap1.clone();
142    |            ^^^^^^^^^^^ argument requires that `'2` must outlive `'1`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4/variadic-ffi-4.stderr
To only update this specific test, also pass `--test-args c-variadic/variadic-ffi-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | pub unsafe extern "C" fn no_escape0<'f>(_: usize, ap: ...) -> VaListImpl<'f> {
   |                                     --            -- has type `VaListImpl<'1>`
   |                                     |
   |                                     lifetime `'f` defined here
LL |     ap
   |     ^^ function was supposed to return data with lifetime `'1` but it is returning data with lifetime `'f`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:8:5
   |
   |
LL | pub unsafe extern "C" fn no_escape0<'f>(_: usize, ap: ...) -> VaListImpl<'f> {
   |                                     --            -- has type `VaListImpl<'1>`
   |                                     |
   |                                     lifetime `'f` defined here
LL |     ap
   |     ^^ function was supposed to return data with lifetime `'f` but it is returning data with lifetime `'1`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:14:5
   |
   |
LL | pub unsafe extern "C" fn no_escape1(_: usize, ap: ...) -> VaListImpl<'static> {
   |                                               -- has type `VaListImpl<'1>`
LL |     ap //~ ERROR: lifetime may not live long enough
   |     ^^ returning this value requires that `'1` must outlive `'static`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:18:31
   |
   |
LL |     let _ = ap.with_copy(|ap| ap); //~ ERROR: lifetime may not live long enough
   |                           --- ^^ returning this value requires that `'1` must outlive `'2`
   |                           | |
   |                           | return type of closure is VaList<'1, '_>
   |                           has type `VaList<'1, '_>`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:22:5
   |
   |
LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1;
LL |     *ap0 = ap1;
   |     ^^^^ assignment requires that `'1` must outlive `'2`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:22:5
   |
   |
LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `VaListImpl<'2>`
LL |     *ap0 = ap1;
LL |     *ap0 = ap1;
   |     ^^^^ assignment requires that `'2` must outlive `'1`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:28:5
   |
   |
LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     ap0 = &mut ap1;
   |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
   |
   = note: requirement occurs because of a mutable reference to `VaListImpl<'_>`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:28:5
   |
   |
LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `VaListImpl<'2>`
   |                                               has type `VaListImpl<'2>`
LL |     ap0 = &mut ap1;
   |     ^^^^^^^^^^^^^^ assignment requires that `'2` must outlive `'1`
   |
   = note: requirement occurs because of a mutable reference to `VaListImpl<'_>`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

error[E0597]: `ap1` does not live long enough
   |
   |
LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                                        - let's call the lifetime of this reference `'3`
LL |     ap0 = &mut ap1;
   |     |     |
   |     |     borrowed value does not live long enough
   |     |     borrowed value does not live long enough
   |     assignment requires that `ap1` is borrowed for `'3`
LL | }
LL | }
   | - `ap1` dropped here while still borrowed
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:35:12
   |
   |
LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1.clone();
   |            ^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:35:12
   |
   |
LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `VaListImpl<'2>`
   |                                               has type `VaListImpl<'2>`
LL |     *ap0 = ap1.clone();
   |            ^^^^^^^^^^^ argument requires that `'2` must outlive `'1`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/error-codes/E0621-does-not-trigger-for-closures.rs stdout ----
diff of stderr:

4 LL |     invoke(&x, |a, b| if a > b { a } else { b });
5    |                    --                       ^ returning this value requires that `'1` must outlive `'2`
6    |                    ||
-    |                    |return type of closure is &'2 i32
+    |                    |return type of closure is &'1 i32
8    |                    has type `&'1 i32`
10 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0621-does-not-trigger-for-closures/E0621-does-not-trigger-for-closures.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0621-does-not-trigger-for-closures.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0621-does-not-trigger-for-closures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0621-does-not-trigger-for-closures" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0621-does-not-trigger-for-closures/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     invoke(&x, |a, b| if a > b { a } else { b }); //~ ERROR lifetime may not live long enough
   |                    --                       ^ returning this value requires that `'1` must outlive `'2`
   |                    ||
   |                    |return type of closure is &'1 i32
   |                    has type `&'1 i32`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-52533.rs stdout ----
diff of stderr:

5    |          -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
6    |          |  |
7    |          |  has type `&'1 u32`
-    |          has type `&'2 u32`
+    |          has type `&'1 u32`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52533/issue-52533.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-52533.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-52533.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52533" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52533/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
---
diff of stderr:

17   --> $DIR/propagate-approximated-fail-no-postdom.rs:46:13
18    |
19 LL |         |_outlives1, _outlives2, _outlives3, x, y| {
-    |          ----------              ---------- has type `Cell<&'2 &'_#3r u32>`
+    |          ----------              ---------- has type `Cell<&'_#1r &'1 u32>`
21    |          |
22    |          has type `Cell<&'_#1r &'1 u32>`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom/propagate-approximated-fail-no-postdom.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom/propagate-approximated-fail-no-postdom.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-fail-no-postdom.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom/auxiliary"
stdout: none
--- stderr -------------------------------
note: no external requirements
   |
   |
LL |         |_outlives1, _outlives2, _outlives3, x, y| {
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's> extern "rust-call" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) u32>, std::cell::Cell<&'_#2r &ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) &'_#3r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>)),
               (),
   = note: late-bound region is '_#4r
   = note: late-bound region is '_#5r
   = note: late-bound region is '_#6r


error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs:46:13
   |
LL |         |_outlives1, _outlives2, _outlives3, x, y| {
   |          ----------              ---------- has type `Cell<&'_#1r &'1 u32>`
   |          |
   |          has type `Cell<&'_#1r &'1 u32>`
...
LL |             demand_y(x, y, p) //~ ERROR
   |             ^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs:38:1
   |
   |
LL | / fn supply<'a, 'b, 'c>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>, cell_c: Cell<&'c u32>) {
LL | |     establish_relationships(
LL | |         cell_a,
LL | |         cell_b,
LL | |     );
LL | | }
   | |_^
   |
---
diff of stderr:

16   --> $DIR/propagate-fail-to-approximate-longer-wrong-bounds.rs:41:9
17    |
18 LL |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
-    |                                                ----------  ---------- has type `&'_#8r Cell<&'2 &'_#2r u32>`
+    |                                                ----------  ---------- has type `&'_#6r Cell<&'1 &'_#1r u32>`
20    |                                                |
21    |                                                has type `&'_#6r Cell<&'1 &'_#1r u32>`
22 LL |         // Only works if 'x: 'y:

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds/propagate-fail-to-approximate-longer-wrong-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
note: no external requirements
   |
   |
LL |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0, 't1, 't2, 't3> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 2, kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 3, kind: BrNamed('t1) }) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 4, kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 5, kind: BrNamed('t3) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 3, kind: BrNamed('t1) }) u32>)),
               (),
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#4r

error: lifetime may not live long enough
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs:41:9
   |
LL |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
   |                                                ----------  ---------- has type `&'_#6r Cell<&'1 &'_#1r u32>`
   |                                                |
   |                                                has type `&'_#6r Cell<&'1 &'_#1r u32>`
LL |         // Only works if 'x: 'y:
LL |         demand_y(x, y, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs:38:1
   |
   |
LL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |         //~^ ERROR
LL | |     });
LL | | }
   |
   = note: defining type: supply

error: aborting due to previous error
error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs stdout ----
diff of stderr:

18 LL |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
19    |                                                ---------  - has type `&'_#7r Cell<&'1 u32>`
20    |                                                |
-    |                                                has type `&'_#5r Cell<&'2 &'_#1r u32>`
+    |                                                has type `&'_#7r Cell<&'1 u32>`
22 LL |         // Only works if 'x: 'y:
23 LL |         demand_y(x, y, x.get())
24    |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/propagate-fail-to-approximate-longer-no-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
note: no external requirements
   |
   |
LL |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0, 't1, 't2> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 2, kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 3, kind: BrNamed('t1) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 4, kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>)),
               (),
   = note: late-bound region is '_#2r
   = note: late-bound region is '_#3r

error: lifetime may not live long enough
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs:37:9
   |
LL |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
   |                                                ---------  - has type `&'_#7r Cell<&'1 u32>`
   |                                                |
   |                                                has type `&'_#7r Cell<&'1 u32>`
LL |         // Only works if 'x: 'y:
LL |         demand_y(x, y, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs:34:1
   |
   |
LL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |         //~^ ERROR
LL | |     });
LL | | }
   |
   = note: defining type: supply

error: aborting due to previous error
error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs stdout ----
diff of stderr:

17    |                 -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
18    |                 |  |
19    |                 |  has type `&'1 i32`
-    |                 has type `&'2 i32`
+    |                 has type `&'1 i32`
22 note: no external requirements
23   --> $DIR/return-wrong-bound-region.rs:10:1



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region/return-wrong-bound-region.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/return-wrong-bound-region.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region/auxiliary"
stdout: none
--- stderr -------------------------------
note: no external requirements
   |
   |
LL |     expect_sig(|a, b| b); // ought to return `a`
   |
   |
   = note: defining type: test::{closure#0} with closure substs [
               i16,
               for<'r, 's> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) i32, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) i32)) -> &ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) i32,
               (),

error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs:11:23
   |
   |
LL |     expect_sig(|a, b| b); // ought to return `a`
   |                 -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
   |                 |  |
   |                 |  has type `&'1 i32`
   |                 has type `&'1 i32`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs:10:1
   |
LL | / fn test() {
LL | / fn test() {
LL | |     expect_sig(|a, b| b); // ought to return `a`
LL | |     //~^ ERROR
LL | | }
   |
   = note: defining type: test

error: aborting due to previous error
error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/nll/issue-52533-1.rs stdout ----
diff of stderr:

5    |            -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
6    |            |  |
7    |            |  has type `&Foo<'_, '1, u32>`
-    |            has type `&Foo<'_, '2, u32>`
+    |            has type `&Foo<'_, '1, u32>`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52533-1/issue-52533-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-52533-1.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52533-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52533-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52533-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     gimme(|x, y| y)
   |            -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
   |            |  |
   |            |  has type `&Foo<'_, '1, u32>`
   |            has type `&Foo<'_, '1, u32>`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-52742.rs stdout ----
diff of stderr:

4 LL |     fn take_bar(&mut self, b: Bar<'_>) {
5    |                 ---------  - has type `Bar<'1>`
-    |                 has type `&mut Foo<'_, '2>`
+    |                 has type `Bar<'1>`
+    |                 has type `Bar<'1>`
8 LL |         self.y = b.z
9    |         ^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742/issue-52742.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742/issue-52742.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-52742.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52742.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     fn take_bar(&mut self, b: Bar<'_>) {
   |                 ---------  - has type `Bar<'1>`
   |                 has type `Bar<'1>`
   |                 has type `Bar<'1>`
LL |         self.y = b.z
   |         ^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-58053.rs stdout ----
diff of stderr:

4 LL |     let f = |x: &i32| -> &i32 { x };
5    |                 -        -      ^ returning this value requires that `'1` must outlive `'2`
6    |                 |        |
-    |                 |        let's call the lifetime of this reference `'2`
+    |                 |        let's call the lifetime of this reference `'1`
8    |                 let's call the lifetime of this reference `'1`
10 error: lifetime may not live long enough


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-58053/issue-58053.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-58053.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-58053.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-58053" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-58053/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     let f = |x: &i32| -> &i32 { x };
   |                 -        -      ^ returning this value requires that `'1` must outlive `'2`
   |                 |        |
   |                 |        let's call the lifetime of this reference `'1`
   |                 let's call the lifetime of this reference `'1`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/issue-58053.rs:8:25
   |
   |
LL |     let g = |x: &i32| { x };
   |                 -   -   ^ returning this value requires that `'1` must outlive `'2`
   |                 |   |
   |                 |   return type of closure is &'2 i32
   |                 let's call the lifetime of this reference `'1`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/regions/regions-escape-method.rs stdout ----
diff of stderr:

4 LL |     s.f(|p| p)
5    |          -- ^ returning this value requires that `'1` must outlive `'2`
6    |          ||
-    |          |return type of closure is &'2 i32
+    |          |return type of closure is &'1 i32
8    |          has type `&'1 i32`
10 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-escape-method/regions-escape-method.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-escape-method.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-escape-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-escape-method" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-escape-method/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     s.f(|p| p) //~ ERROR lifetime may not live long enough
   |          -- ^ returning this value requires that `'1` must outlive `'2`
   |          ||
   |          |return type of closure is &'1 i32
   |          has type `&'1 i32`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/regions-escape-via-trait-or-not.rs stdout ----
diff of stderr:

4 LL |     with(|o| o)
5    |           -- ^ returning this value requires that `'1` must outlive `'2`
6    |           ||
-    |           |return type of closure is &'2 isize
+    |           |return type of closure is &'1 isize
8    |           has type `&'1 isize`
10 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-escape-via-trait-or-not/regions-escape-via-trait-or-not.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-escape-via-trait-or-not.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-escape-via-trait-or-not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-escape-via-trait-or-not" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-escape-via-trait-or-not/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     with(|o| o) //~ ERROR lifetime may not live long enough
   |           -- ^ returning this value requires that `'1` must outlive `'2`
   |           ||
   |           |return type of closure is &'1 isize
   |           has type `&'1 isize`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/regions-infer-call-3.rs stdout ----
diff of stderr:

4 LL |     let z = with(|y| { select(x, y) });
5    |                   --   ^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
6    |                   ||
-    |                   |return type of closure is &'2 isize
+    |                   |return type of closure is &'1 isize
8    |                   has type `&'1 isize`
10 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-call-3/regions-infer-call-3.stderr
To only update this specific test, also pass `--test-args regions/regions-infer-call-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-infer-call-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-call-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-call-3/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     let z = with(|y| { select(x, y) });
   |                   --   ^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
   |                   ||
   |                   |return type of closure is &'1 isize
   |                   has type `&'1 isize`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/regions-ret-borrowed-1.rs stdout ----
diff of stderr:

4 LL |     with(|o| o)
5    |           -- ^ returning this value requires that `'1` must outlive `'2`
6    |           ||
-    |           |return type of closure is &'2 isize
+    |           |return type of closure is &'1 isize
8    |           has type `&'1 isize`
10 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-ret-borrowed-1/regions-ret-borrowed-1.stderr
To only update this specific test, also pass `--test-args regions/regions-ret-borrowed-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-ret-borrowed-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-ret-borrowed-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-ret-borrowed-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     with(|o| o)
   |           -- ^ returning this value requires that `'1` must outlive `'2`
   |           ||
   |           |return type of closure is &'1 isize
   |           has type `&'1 isize`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/regions-ret-borrowed.rs stdout ----
diff of stderr:

4 LL |     with(|o| o)
5    |           -- ^ returning this value requires that `'1` must outlive `'2`
6    |           ||
-    |           |return type of closure is &'2 isize
+    |           |return type of closure is &'1 isize
8    |           has type `&'1 isize`
10 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-ret-borrowed/regions-ret-borrowed.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-ret-borrowed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-ret-borrowed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-ret-borrowed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-ret-borrowed/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     with(|o| o)
   |           -- ^ returning this value requires that `'1` must outlive `'2`
   |           ||
   |           |return type of closure is &'1 isize
   |           has type `&'1 isize`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs stdout ----
diff of stderr:

5    |                          -         -               ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
6    |                          |         |
7    |                          |         let's call the lifetime of this reference `'1`
-    |                          let's call the lifetime of this reference `'2`
+    |                          let's call the lifetime of this reference `'1`
10 help: consider introducing a named lifetime parameter and update trait if needed
11    |


