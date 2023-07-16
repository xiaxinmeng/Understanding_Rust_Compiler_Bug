plain
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
  --> src/librustdoc/html/length_limit/tests.rs:84:18
   |
84 |         buf.push(n.to_string())?;
   |                  |
   |                  expected `&str`, found struct `std::string::String`
   |                  help: consider borrowing here: `&n.to_string()`
   |
   |
note: return type inferred to be `&str` here
  --> src/librustdoc/html/length_limit/tests.rs:83:9
   |
83 |         buf.push("word#")?;

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to previous error



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


Build completed unsuccessfully in 0:25:54
