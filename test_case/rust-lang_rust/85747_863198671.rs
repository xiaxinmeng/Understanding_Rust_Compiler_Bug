plain
test src\thread\mod.rs - thread::spawn (line 581) ... ok

failures:

---- src\fs.rs - fs::Metadata::is_symlink (line 1014) stdout ----
error[E0433]: failed to resolve: could not find `unix` in `os`
  |
6 | use std::os::unix::fs::symlink;
6 | use std::os::unix::fs::symlink;
  |              ^^^^ could not find `unix` in `os`

error[E0425]: cannot find function `symlink` in this scope
   |
   |
10 |     symlink("/origin_does_not_exists/", link_path)?;

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0433.
Some errors have detailed explanations: E0425, E0433.
For more information about an error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src\path.rs - path::Path::is_symlink (line 2605) stdout ----
error[E0433]: failed to resolve: could not find `unix` in `os`
  |
6 | use std::os::unix::fs::symlink;
6 | use std::os::unix::fs::symlink;
  |              ^^^^ could not find `unix` in `os`

error[E0425]: cannot find function `symlink` in this scope
  |
  |
9 | symlink("/origin_does_not_exists/", link_path).unwrap();

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0433.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "\\\\?\\D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage0\\bin\\cargo.exe" "test" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "D:\\a\\rust\\rust\\library/test/Cargo.toml" "-p" "std" "--"


failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 --exclude src/test/ui
Build completed unsuccessfully in 0:42:36
Build completed unsuccessfully in 0:42:36
make: *** [Makefile:80: ci-mingw-subset-1] Error 1
