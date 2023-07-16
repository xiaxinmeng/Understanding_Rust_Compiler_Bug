plain
.................................................................................................... 9000/11244
.................................................................................................... 9100/11244
.................................................................................................... 9200/11244
........................................i......i.................................................... 9300/11244
...............................................................................iiiiii..iiiiii.i..... 9400/11244
.................................................................................................... 9600/11244
.................................................................................................... 9700/11244
.................................................................................................... 9800/11244
.................................................................................................... 9900/11244
---
 finished in 0.438 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.074 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii.....ii....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.44s

 finished in 2.519 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
   Compiling rustc-rayon v0.3.0
   Compiling tempfile v3.1.0
   Compiling regex v1.3.9
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error: use of deprecated associated function `std::path::Path::is_file`: other processes may remove, rename, or replace files at any time
    |
    |
418 |             if !p.is_file() {
    |                   ^^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::File::open` or `std::fs::metadata``
    |
    = note: `-D deprecated` implied by `-D warnings`

error: use of deprecated associated function `std::path::Path::is_file`: other processes may remove, rename, or replace files at any time
    |
    |
431 |                 if !theme_file.is_file() {
    |                                ^^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::File::open` or `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::is_file`: other processes may remove, rename, or replace files at any time
    |
    |
501 |             if !index_page.is_file() {
    |                            ^^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::File::open` or `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
   --> src/librustdoc/html/render/mod.rs:903:17
    |
903 |         if path.exists() {
    |                 ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
   --> src/librustdoc/html/render/mod.rs:929:17
    |
929 |         if path.exists() {
    |                 ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::is_dir`: other processes may remove, rename, or replace directories at any time
  --> src/librustdoc/html/render/cache.rs:35:23
   |
35 |     if local_location.is_dir() {
   |                       ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::read_dir` or `std::fs::metadata``
error: aborting due to 6 previous errors

error: could not compile `rustdoc`

