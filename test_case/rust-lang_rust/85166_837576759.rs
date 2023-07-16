plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking std v0.0.0 (/checkout/library/std)
error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:460:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
460 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:470:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
470 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:480:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
480 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:490:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
490 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:500:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
500 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:510:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
510 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:520:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
520 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:530:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
530 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:540:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
540 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:550:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
550 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:560:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
560 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:570:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
570 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:580:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
580 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:590:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
590 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:600:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
600 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:610:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
610 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:620:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
620 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:630:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
630 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:640:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
640 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:650:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
650 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:660:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
660 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:670:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
670 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:680:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
680 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:690:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
690 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:700:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
700 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:710:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
710 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:720:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
720 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:730:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
730 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:740:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
740 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:750:23
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
750 |          extension: None);
    |                         ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:759:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
759 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:769:20
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
769 |       extension: None
    |                      ^ missing tokens in macro arguments

error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:779:27
    |
6   | / macro_rules! t (
7   | |     ($path:expr, iter: $iter:expr) => (
8   | |         {
9   | |             let path = Path::new($path);
104 | |     );
105 | | );
    | |__- when calling this macro
...
...
779 |       extension: Some("txt")
    |                             ^ missing tokens in macro arguments
error: unexpected end of macro invocation
   --> library/std/src/path/tests.rs:789:27
    |
6   | / macro_rules! t (
6   | / macro_rules! t (
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "term" "-p" "panic_unwind" "-p" "alloc" "-p" "unwind" "-p" "panic_abort" "-p" "proc_macro" "-p" "core" "-p" "std" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:34
