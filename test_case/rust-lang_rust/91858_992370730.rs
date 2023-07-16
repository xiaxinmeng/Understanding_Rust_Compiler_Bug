plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0308]: mismatched types
  --> compiler/rustc_codegen_ssa/src/back/rpath.rs:42:22
   |
42 |             ret.push(rpath.clone());
   |                      |
   |                      expected `&str`, found struct `std::string::String`
   |                      help: consider borrowing here: `&rpath`

---
    | |_- in this expansion of `format!`
    |
   ::: compiler/rustc_codegen_ssa/src/back/rpath.rs:44:22
    |
44  |               ret.push(format!("-Wl,-rpath,{}", &(*rpath)));

error[E0308]: mismatched types
  --> compiler/rustc_codegen_ssa/src/back/rpath.rs:48:5
   |
   |
34 | fn rpaths_to_flags(rpaths: &[String]) -> Vec<String> {
   |                                          ----------- expected `Vec<std::string::String>` because of return type
48 |     ret
   |     ^^^ expected struct `std::string::String`, found `&str`
   |
   = note: expected struct `Vec<std::string::String>`
   = note: expected struct `Vec<std::string::String>`
              found struct `Vec<&str>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_ssa` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
