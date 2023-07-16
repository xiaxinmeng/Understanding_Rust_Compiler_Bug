
error[E0464]: multiple matching crates for `rustc_macros`
 --> /usr/pkgsrc/lang/rust/work/rustc-1.38.0-src/vendor/rustc-ap-rustc_errors/diagnostic.rs:9:5
  |
9 | use syntax_pos::{MultiSpan, Span};
  |     ^^^^^^^^^^
  |
  = note: candidates:
          crate `rustc_macros`: /usr/pkgsrc/lang/rust/work/rustc-1.38.0-src/build/x86_64-unknown-netbsd/stage1/lib/rustlib/x86_64-unknown-netbsd/lib/librustc_macros-47279f083f076c01.so
          crate `rustc_macros`: /usr/pkgsrc/lang/rust/work/rustc-1.38.0-src/build/x86_64-unknown-netbsd/stage1-tools/release/deps/librustc_macros-ebd9d84768c4b363.so

error[E0461]: couldn't find crate `rustc_macros` with expected target triple x86_64-unknown-netbsd which `syntax_pos` depends on
 --> /usr/pkgsrc/lang/rust/work/rustc-1.38.0-src/vendor/rustc-ap-rustc_errors/diagnostic.rs:9:5
  |
9 | use syntax_pos::{MultiSpan, Span};
  |     ^^^^^^^^^^
  |
  = note: the following crate versions were found:
          crate `rustc_macros`, target triple armv7-unknown-netbsd-eabihf: /usr/pkgsrc/lang/rust/work/rustc-1.38.0-src/build/x86_64-unknown-netbsd/stage1-tools/armv7-unknown-netbsd-eabihf/release/deps/librustc_macros-edac7f465b9c2524.so

error[E0464]: multiple matching crates for `rustc_macros`
  --> /usr/pkgsrc/lang/rust/work/rustc-1.38.0-src/vendor/rustc-ap-rustc_target/abi/mod.rs:11:5
   |
11 | use syntax_pos::symbol::{sym, Symbol};
   |     ^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_macros`: /usr/pkgsrc/lang/rust/work/rustc-1.38.0-src/build/x86_64-unknown-netbsd/stage1/lib/rustlib/x86_64-unknown-netbsd/lib/librustc_macros-47279f083f076c01.so
           crate `rustc_macros`: /usr/pkgsrc/lang/rust/work/rustc-1.38.0-src/build/x86_64-unknown-netbsd/stage1-tools/release/deps/librustc_macros-ebd9d84768c4b363.so

error[E0461]: couldn't find crate `rustc_macros` with expected target triple x86_64-unknown-netbsd which `syntax_pos` depends on
  --> /usr/pkgsrc/lang/rust/work/rustc-1.38.0-src/vendor/rustc-ap-rustc_target/abi/mod.rs:11:5
   |
11 | use syntax_pos::symbol::{sym, Symbol};
   |     ^^^^^^^^^^
   |
   = note: the following crate versions were found:
           crate `rustc_macros`, target triple armv7-unknown-netbsd-eabihf: /usr/pkgsrc/lang/rust/work/rustc-1.38.0-src/build/x86_64-unknown-netbsd/stage1-tools/armv7-unknown-netbsd-eabihf/release/deps/librustc_macros-edac7f465b9c2524.so

error: aborting due to 2 previous errors

error: aborting due to 2 previous errors

error: Could not compile `rustc-ap-rustc_errors`.
warning: build failed, waiting for other jobs to finish...
error: Could not compile `rustc-ap-rustc_target`.
