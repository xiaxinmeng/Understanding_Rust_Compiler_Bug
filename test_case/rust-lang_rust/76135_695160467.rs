console
Command failed. Attempt 5/5:
    Updating crates.io index
warning: spurious network error (2 tries remaining): error inflating zlib stream; class=Zlib (5)
warning: spurious network error (1 tries remaining): error inflating zlib stream; class=Zlib (5)
error: failed to get `cc` as a dependency of package `bootstrap v0.0.0 (D:\a\rust\rust\src\bootstrap)`

Caused by:
  failed to fetch `https://github.com/rust-lang/crates.io-index`

Caused by:
  error inflating zlib stream; class=Zlib (5)
failed to run: D:\a\rust\rust\build\i686-pc-windows-gnu\stage0\bin\cargo.exe build --manifest-path D:\a\rust\rust\src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:21
make: *** [Makefile:60: prepare] Error 1
The command has failed after 5 attempts.
