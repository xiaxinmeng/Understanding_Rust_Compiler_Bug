
error: `#[deprecated]` cannot be used in staged api, use `#[rustc_deprecated]` instead
  --> C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\gcc-0.3.52\src\lib.rs:57:1
   |
57 | pub type Config = Build;
   | ^^^^^^^^^^^^^^^^^^^^^^^^
error: `#[deprecated]` cannot be used in staged api, use `#[rustc_deprecated]` instead
   --> C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\gcc-0.3.52\src\lib.rs:237:1
    |
237 | / pub fn compile_library(output: &str, files: &[&str]) {
238 | |     let mut c = Build::new();
239 | |     for f in files.iter() {
240 | |         c.file(*f);
241 | |     }
242 | |     c.compile(output);
243 | | }
    | |_^
error: aborting due to 2 previous errors
error: Could not compile `gcc`.
