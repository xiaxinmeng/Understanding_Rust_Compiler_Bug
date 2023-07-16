plain
[RUSTC-TIMING] cargo_metadata test:false 7.895
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
  --> src\tools\tidy\src/main.rs:49:21
   |
41 | /         macro_rules! check {
42 | |             ($p:ident $(, $args:expr)* ) => {
43 | |                 while handles.len() >= concurrency.get() {
44 | |                     handles.pop_front().unwrap().join().unwrap();
...  |
49 | |                     $p::check($($args),* , &mut flag);
   | |                     ^^^^^^^^^              --------- supplied 2 arguments
   | |                     expected 3 arguments
...  |
55 | |             }
56 | |         }
56 | |         }
   | |_________- in this expansion of `check!`
...
82 |               check!(bins, &src_path);
   |
note: function defined here
  --> D:\a\rust\rust\src\tools\tidy\src\bins.rs:19:12
   |
   |
19 |     pub fn check(_path: &Path, _output: &Path, _bad: &mut bool) {}

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
  --> src\tools\tidy\src/main.rs:49:21
   |
   |
41 | /         macro_rules! check {
42 | |             ($p:ident $(, $args:expr)* ) => {
43 | |                 while handles.len() >= concurrency.get() {
44 | |                     handles.pop_front().unwrap().join().unwrap();
...  |
49 | |                     $p::check($($args),* , &mut flag);
   | |                     ^^^^^^^^^              --------- supplied 2 arguments
   | |                     expected 3 arguments
...  |
55 | |             }
56 | |         }
56 | |         }
   | |_________- in this expansion of `check!`
...
83 |               check!(bins, &compiler_path);
   |
note: function defined here
  --> D:\a\rust\rust\src\tools\tidy\src\bins.rs:19:12
   |
   |
19 |     pub fn check(_path: &Path, _output: &Path, _bad: &mut bool) {}

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
  --> src\tools\tidy\src/main.rs:49:21
   |
   |
41 | /         macro_rules! check {
42 | |             ($p:ident $(, $args:expr)* ) => {
43 | |                 while handles.len() >= concurrency.get() {
44 | |                     handles.pop_front().unwrap().join().unwrap();
...  |
49 | |                     $p::check($($args),* , &mut flag);
   | |                     ^^^^^^^^^              --------- supplied 2 arguments
   | |                     expected 3 arguments
...  |
55 | |             }
56 | |         }
56 | |         }
   | |_________- in this expansion of `check!`
...
84 |               check!(bins, &library_path);
   |
note: function defined here
  --> D:\a\rust\rust\src\tools\tidy\src\bins.rs:19:12
   |
   |
19 |     pub fn check(_path: &Path, _output: &Path, _bad: &mut bool) {}

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0061`.
---
command did not execute successfully: "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "build" "--target" "x86_64-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--manifest-path" "D:\\a\\rust\\rust\\src/tools/tidy\\Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 --exclude src/test/ui --exclude src/tools/linkchecker
Build completed unsuccessfully in 0:00:32
make: *** [Makefile:76: ci-subset-1] Error 1
