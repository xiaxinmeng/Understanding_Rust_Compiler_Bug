plain
   Compiling snap v1.0.1
   Compiling ansi_term v0.12.1
   Compiling build_helper v0.1.0 (/checkout/src/build_helper)
   Compiling fixedbitset v0.2.0
error: use of deprecated associated function `std::path::Path::read_dir`: use `std::fs::read_dir` instead
    |
    |
150 |         .read_dir()
    |          ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::read_dir`
    |
    = note: `-D deprecated` implied by `-D warnings`

error: use of deprecated associated function `std::path::Path::read_dir`: use `std::fs::read_dir` instead
    |
    |
158 |             stack.extend(path.read_dir().unwrap().map(|e| e.unwrap()));
    |                               ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::read_dir`

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
    |
    |
175 |     if !dst.exists() {
    |             ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``
error: aborting due to 3 previous errors

error: could not compile `build_helper`

