plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 9f4d5d2a28849ec0ecb2976ddc9946f65b626fe8 and 8bc3b917ec7b2bc0dc0085b02047977b2e7f9d2f
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

 error: named parameter zero is used as a positional parameter
    |
    |
 LL |     println!("{} {1:?}", zero = 0, one = 1);
    |                ^ help: replace it with: `zero`
    |
    = note: `-D clippy::positional-named-format-parameters` implied by `-D warnings`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:16:19
    |
    |
 LL |     println!("{} {1:?}", zero = 0, one = 1);
    |                   ^ help: replace it with: `one`
 
 error: named parameter zero is used as a positional parameter
    |
    |
 LL |     println!("This is a test { } {000001:?}", zero = 0, one = 1);
    |                               ^ help: replace it with: `zero`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:17:35
    |
    |
 LL |     println!("This is a test { } {000001:?}", zero = 0, one = 1);
    |                                   ^^^^^^ help: replace it with: `one`
 
 error: named parameter zero is used as a positional parameter
    |
    |
 LL |     println!("Hello {1} is {2:.0$}", zero = 5, one = hello, two = 0.01);
    |                                ^ help: replace it with: `zero`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:18:22
    |
    |
 LL |     println!("Hello {1} is {2:.0$}", zero = 5, one = hello, two = 0.01);
    |                      ^ help: replace it with: `one`
 
 error: named parameter two is used as a positional parameter
    |
    |
 LL |     println!("Hello {1} is {2:.0$}", zero = 5, one = hello, two = 0.01);
    |                             ^ help: replace it with: `two`
 
 error: named parameter zero is used as a positional parameter
    |
    |
 LL |     println!("Hello {1:0$}!", zero = 5, one = 1);
    |                        ^ help: replace it with: `zero`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:19:22
    |
    |
 LL |     println!("Hello {1:0$}!", zero = 5, one = 1);
    |                      ^ help: replace it with: `one`
 
 error: named parameter zero is used as a positional parameter
    |
    |
 LL |     println!("Hello {0:1$}!", zero = 4, one = 1);
    |                      ^ help: replace it with: `zero`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:20:24
    |
    |
 LL |     println!("Hello {0:1$}!", zero = 4, one = 1);
    |                        ^ help: replace it with: `one`
 
 error: named parameter zero is used as a positional parameter
    |
    |
 LL |     println!("Hello {0:01$}!", zero = 4, one = 1);
    |                      ^ help: replace it with: `zero`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:21:25
    |
    |
 LL |     println!("Hello {0:01$}!", zero = 4, one = 1);
    |                         ^ help: replace it with: `one`
 
-error: named parameter zero is used as a positional parameter
-   |
-   |
-LL |     println!("Hello is {1:.*}", zero = 5, one = 0.01);
-   |                            ^ help: replace it with: `zero$`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:22:25
    |
    |
 LL |     println!("Hello is {1:.*}", zero = 5, one = 0.01);
    |                         ^ help: replace it with: `one`
 
-error: named parameter zero is used as a positional parameter
-   |
-   |
-LL |     println!("Hello is {:<6.*}", zero = 5, one = 0.01);
-   |                             ^ help: replace it with: `zero$`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:23:25
    |
    |
 LL |     println!("Hello is {:<6.*}", zero = 5, one = 0.01);
    |                         ^ help: replace it with: `one`
 
 error: named parameter zero is used as a positional parameter
    |
    |
 LL |     println!("{}, `{two:>8.*}` has 3", zero = hello, one = 3, two = hello);
    |                ^ help: replace it with: `zero`
-error: named parameter one is used as a positional parameter
-  --> $DIR/positional_named_format_parameters.rs:24:28
-   |
-   |
-LL |     println!("{}, `{two:>8.*}` has 3", zero = hello, one = 3, two = hello);
-   |                            ^ help: replace it with: `one$`
-
 error: named parameter zero is used as a positional parameter
    |
    |
 LL |     println!("Hello {1} is {2:.0$}", zero = 5, one = hello, two = 0.01);
    |                                ^ help: replace it with: `zero`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:25:22
    |
    |
 LL |     println!("Hello {1} is {2:.0$}", zero = 5, one = hello, two = 0.01);
    |                      ^ help: replace it with: `one`
 
 error: named parameter two is used as a positional parameter
    |
    |
 LL |     println!("Hello {1} is {2:.0$}", zero = 5, one = hello, two = 0.01);
    |                             ^ help: replace it with: `two`
 error: named parameter world is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:26:30
    |
    |
 LL |     println!("Hello {world} {}!", world = 5);
    |                              ^ help: replace it with: `world`
 
 error: named parameter zero is used as a positional parameter
    |
    |
 LL |     writeln!(v, "{} {1:?}", zero = 0, one = 1);
    |                   ^ help: replace it with: `zero`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:28:22
    |
    |
 LL |     writeln!(v, "{} {1:?}", zero = 0, one = 1);
    |                      ^ help: replace it with: `one`
 
 error: named parameter zero is used as a positional parameter
    |
    |
 LL |     writeln!(v, "This is a test { } {000001:?}", zero = 0, one = 1);
    |                                  ^ help: replace it with: `zero`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:29:38
    |
    |
 LL |     writeln!(v, "This is a test { } {000001:?}", zero = 0, one = 1);
    |                                      ^^^^^^ help: replace it with: `one`
 
 error: named parameter zero is used as a positional parameter
    |
    |
 LL |     writeln!(v, "Hello {1} is {2:.0$}", zero = 5, one = hello, two = 0.01);
    |                                   ^ help: replace it with: `zero`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:30:25
    |
    |
 LL |     writeln!(v, "Hello {1} is {2:.0$}", zero = 5, one = hello, two = 0.01);
    |                         ^ help: replace it with: `one`
 
 error: named parameter two is used as a positional parameter
    |
    |
 LL |     writeln!(v, "Hello {1} is {2:.0$}", zero = 5, one = hello, two = 0.01);
    |                                ^ help: replace it with: `two`
 
 error: named parameter zero is used as a positional parameter
    |
    |
 LL |     writeln!(v, "Hello {1:0$}!", zero = 4, one = 1);
    |                           ^ help: replace it with: `zero`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:31:25
    |
    |
 LL |     writeln!(v, "Hello {1:0$}!", zero = 4, one = 1);
    |                         ^ help: replace it with: `one`
 
 error: named parameter zero is used as a positional parameter
    |
    |
 LL |     writeln!(v, "Hello {0:1$}!", zero = 4, one = 1);
    |                         ^ help: replace it with: `zero`
 
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:32:27
    |
    |
 LL |     writeln!(v, "Hello {0:1$}!", zero = 4, one = 1);
    |                           ^ help: replace it with: `one`
 
 error: named parameter zero is used as a positional parameter
    |
    |
 LL |     writeln!(v, "Hello {0:01$}!", zero = 4, one = 1);
    |                         ^ help: replace it with: `zero`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:33:28
    |
    |
 LL |     writeln!(v, "Hello {0:01$}!", zero = 4, one = 1);
    |                            ^ help: replace it with: `one`
 
-error: named parameter zero is used as a positional parameter
-   |
-   |
-LL |     writeln!(v, "Hello is {1:.*}", zero = 3, one = 0.01);
-   |                               ^ help: replace it with: `zero$`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:34:28
    |
    |
 LL |     writeln!(v, "Hello is {1:.*}", zero = 3, one = 0.01);
    |                            ^ help: replace it with: `one`
 
-error: named parameter zero is used as a positional parameter
-   |
-   |
-LL |     writeln!(v, "Hello is {:<6.*}", zero = 2, one = 0.01);
-   |                                ^ help: replace it with: `zero$`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:35:28
    |
    |
 LL |     writeln!(v, "Hello is {:<6.*}", zero = 2, one = 0.01);
    |                            ^ help: replace it with: `one`
 
 error: named parameter zero is used as a positional parameter
    |
    |
 LL |     writeln!(v, "{}, `{two:>8.*}` has 3", zero = hello, one = 3, two = hello);
    |                   ^ help: replace it with: `zero`
-error: named parameter one is used as a positional parameter
-  --> $DIR/positional_named_format_parameters.rs:36:31
-   |
-   |
-LL |     writeln!(v, "{}, `{two:>8.*}` has 3", zero = hello, one = 3, two = hello);
-   |                               ^ help: replace it with: `one$`
-
 error: named parameter zero is used as a positional parameter
    |
    |
 LL |     writeln!(v, "Hello {1} is {2:.0$}", zero = 1, one = hello, two = 0.01);
    |                                   ^ help: replace it with: `zero`
 error: named parameter one is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:37:25
    |
    |
 LL |     writeln!(v, "Hello {1} is {2:.0$}", zero = 1, one = hello, two = 0.01);
    |                         ^ help: replace it with: `one`
 
 error: named parameter two is used as a positional parameter
    |
    |
 LL |     writeln!(v, "Hello {1} is {2:.0$}", zero = 1, one = hello, two = 0.01);
    |                                ^ help: replace it with: `two`
 error: named parameter world is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:38:33
    |
    |
 LL |     writeln!(v, "Hello {world} {}!", world = 0);
    |                                 ^ help: replace it with: `world`
 
 error: named parameter w is used as a positional parameter
    |
    |
 LL |     println!("{:w$}", w = 1);
    |                ^ help: replace it with: `w`
 
 error: named parameter p is used as a positional parameter
    |
    |
 LL |     println!("{:.p$}", p = 1);
    |                ^ help: replace it with: `p`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:43:16
    |
    |
 LL |     println!("{}", v = 1);
    |                ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:44:16
    |
    |
 LL |     println!("{:0$}", v = 1);
    |                ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:44:17
    |
    |
 LL |     println!("{:0$}", v = 1);
    |                 ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:45:16
    |
    |
 LL |     println!("{0:0$}", v = 1);
    |                ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:45:18
    |
    |
 LL |     println!("{0:0$}", v = 1);
    |                  ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:46:16
    |
    |
 LL |     println!("{:0$.0$}", v = 1);
    |                ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:46:20
    |
    |
 LL |     println!("{:0$.0$}", v = 1);
    |                    ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:46:17
    |
    |
 LL |     println!("{:0$.0$}", v = 1);
    |                 ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:47:16
    |
    |
 LL |     println!("{0:0$.0$}", v = 1);
    |                ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:47:21
    |
    |
 LL |     println!("{0:0$.0$}", v = 1);
    |                     ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:47:18
    |
    |
 LL |     println!("{0:0$.0$}", v = 1);
    |                  ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:48:16
    |
    |
 LL |     println!("{0:0$.v$}", v = 1);
    |                ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:48:18
    |
    |
 LL |     println!("{0:0$.v$}", v = 1);
    |                  ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:49:16
    |
    |
 LL |     println!("{0:v$.0$}", v = 1);
    |                ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:49:21
    |
    |
 LL |     println!("{0:v$.0$}", v = 1);
    |                     ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:50:21
    |
    |
 LL |     println!("{v:0$.0$}", v = 1);
    |                     ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:50:18
    |
    |
 LL |     println!("{v:0$.0$}", v = 1);
    |                  ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:51:21
    |
    |
 LL |     println!("{v:v$.0$}", v = 1);
    |                     ^ help: replace it with: `v`
 error: named parameter v is used as a positional parameter
   --> $DIR/positional_named_format_parameters.rs:52:18
    |
    |
 LL |     println!("{v:0$.v$}", v = 1);
    |                  ^ help: replace it with: `v`
 
 error: named parameter w is used as a positional parameter
    |
    |
 LL |     println!("{:w$}", w = 1);
    |                ^ help: replace it with: `w`
 
 error: named parameter p is used as a positional parameter
    |
    |
 LL |     println!("{:.p$}", p = 1);
    |                ^ help: replace it with: `p`
-error: aborting due to 69 previous errors
+error: aborting due to 63 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/positional_named_format_parameters.stage-id.stderr
diff of fixed:

 // run-rustfix
 #![allow(unused_must_use)]
 #![allow(named_arguments_used_positionally)] // Unstable at time of writing.
 #![warn(clippy::positional_named_format_parameters)]
 use std::io::Write;
 
 fn main() {
 fn main() {
     let mut v = Vec::new();
     let hello = "Hello";
 
     println!("{hello:.foo$}", foo = 2);
     writeln!(v, "{hello:.foo$}", foo = 2);
     // Warnings
     // Warnings
     println!("{zero} {one:?}", zero = 0, one = 1);
     println!("This is a test {zero} {one:?}", zero = 0, one = 1);
     println!("Hello {one} is {two:.zero$}", zero = 5, one = hello, two = 0.01);
     println!("Hello {one:zero$}!", zero = 5, one = 1);
     println!("Hello {zero:one$}!", zero = 4, one = 1);
     println!("Hello {zero:0one$}!", zero = 4, one = 1);
-    println!("Hello is {one:.zero$}", zero = 5, one = 0.01);
-    println!("Hello is {one:<6.zero$}", zero = 5, one = 0.01);
-    println!("{zero}, `{two:>8.one$}` has 3", zero = hello, one = 3, two = hello);
+    println!("Hello is {one:.*}", zero = 5, one = 0.01);
+    println!("Hello is {one:<6.*}", zero = 5, one = 0.01);
+    println!("{zero}, `{two:>8.*}` has 3", zero = hello, one = 3, two = hello);
     println!("Hello {one} is {two:.zero$}", zero = 5, one = hello, two = 0.01);
     println!("Hello {world} {world}!", world = 5);
 
     writeln!(v, "{zero} {one:?}", zero = 0, one = 1);
     writeln!(v, "This is a test {zero} {one:?}", zero = 0, one = 1);
     writeln!(v, "Hello {one} is {two:.zero$}", zero = 5, one = hello, two = 0.01);
     writeln!(v, "Hello {one:zero$}!", zero = 4, one = 1);
     writeln!(v, "Hello {zero:one$}!", zero = 4, one = 1);
     writeln!(v, "Hello {zero:0one$}!", zero = 4, one = 1);
-    writeln!(v, "Hello is {one:.zero$}", zero = 3, one = 0.01);
-    writeln!(v, "Hello is {one:<6.zero$}", zero = 2, one = 0.01);
-    writeln!(v, "{zero}, `{two:>8.one$}` has 3", zero = hello, one = 3, two = hello);
+    writeln!(v, "Hello is {one:.*}", zero = 3, one = 0.01);
+    writeln!(v, "Hello is {one:<6.*}", zero = 2, one = 0.01);
+    writeln!(v, "{zero}, `{two:>8.*}` has 3", zero = hello, one = 3, two = hello);
     writeln!(v, "Hello {one} is {two:.zero$}", zero = 1, one = hello, two = 0.01);
     writeln!(v, "Hello {world} {world}!", world = 0);
 
     // Tests from other files
     println!("{w:w$}", w = 1);
     println!("{p:.p$}", p = 1);
     println!("{v}", v = 1);
     println!("{v:v$}", v = 1);
     println!("{v:v$}", v = 1);
     println!("{v:v$.v$}", v = 1);
     println!("{v:v$.v$}", v = 1);
     println!("{v:v$.v$}", v = 1);
     println!("{v:v$.v$}", v = 1);
     println!("{v:v$.v$}", v = 1);
     println!("{v:v$.v$}", v = 1);
     println!("{v:v$.v$}", v = 1);
     println!("{w:w$}", w = 1);
     println!("{p:.p$}", p = 1);
     println!("{:p$.w$}", 1, w = 1, p = 1);
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/positional_named_format_parameters.stage-id.fixed
To only update this specific test, also pass `--test-args positional_named_format_parameters.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
---
 
 error: this trait bound is already specified in trait declaration
   --> $DIR/trait_duplication_in_bounds_unfixable.rs:49:15
    |
 LL |         Self: Default + Clone;
    |
    = help: consider removing this trait bound
 
 error: this trait bound is already specified in trait declaration
 error: this trait bound is already specified in trait declaration
   --> $DIR/trait_duplication_in_bounds_unfixable.rs:55:15
    |
 LL |         Self: Default + Clone;
    |
    = help: consider removing this trait bound
 
 error: this trait bound is already specified in trait declaration
 error: this trait bound is already specified in trait declaration
   --> $DIR/trait_duplication_in_bounds_unfixable.rs:55:25
    |
 LL |         Self: Default + Clone;
    |
    = help: consider removing this trait bound
 
 error: this trait bound is already specified in trait declaration
---
 
 error: this trait bound is already specified in trait declaration
   --> $DIR/trait_duplication_in_bounds_unfixable.rs:93:15
    |
 LL |         Self: Iterator<Item = Foo>,
    |
    = help: consider removing this trait bound
 
 error: aborting due to 8 previous errors
---
To only update this specific test, also pass `--test-args trait_duplication_in_bounds_unfixable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/trait_duplication_in_bounds_unfixable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/trait_duplication_in_bounds_unfixable.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-14dbc812a1f5dba0.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-68b3adac889f3bfe.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-47229815ed3188f9.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c6aa3eacac0eeebb.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-7a1f88b04e57c3b0.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7dc368fb32eb8aae.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-4b46e2e2788394f2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-eed8221ad604f845.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-4554cde6a1339e03.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-11c942eb60796e9d.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/trait_duplication_in_bounds_unfixable.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this trait bound is already specified in the where clause","code":{"code":"clippy::trait_duplication_in_bounds","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trait_duplication_in_bounds_unfixable.rs","byte_start":174,"byte_end":179,"line_start":6,"line_end":6,"column_start":15,"column_end":20,"is_primary":true,"text":[{"text":"fn bad_foo<T: Clone + Default, Z: Copy>(arg0: T, arg1: Z)","highlight_start":15,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/trait_duplication_in_bounds_unfixable.rs","byte_start":8,"byte_end":43,"line_start":1,"line_end":1,"column_start":9,"column_end":44,"is_primary":true,"text":[{"text":"#![deny(clippy::trait_duplication_in_bounds)]","highlight_start":9,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider removing this trait bound","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this trait bound is already specified in the where clause\n  --> tests/ui/trait_duplication_in_bounds_unfixable.rs:6:15\n   |\nLL | fn bad_foo<T: Clone + Default, Z: Copy>(arg0: T, arg1: Z)\n   |               ^^^^^\n   |\nnote: the lint level is defined here\n  --> tests/ui/trait_duplication_in_bounds_unfixable.rs:1:9\n   |\nLL | #![deny(clippy::trait_duplication_in_bounds)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = help: consider removing this trait bound\n\n"}
{"message":"this trait bound is already specified in the where clause","code":{"code":"clippy::trait_duplication_in_bounds","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trait_duplication_in_bounds_unfixable.rs","byte_start":182,"byte_end":189,"line_start":6,"line_end":6,"column_start":23,"column_end":30,"is_primary":true,"text":[{"text":"fn bad_foo<T: Clone + Default, Z: Copy>(arg0: T, arg1: Z)","highlight_start":23,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing this trait bound","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this trait bound is already specified in the where clause\n  --> tests/ui/trait_duplication_in_bounds_unfixable.rs:6:23\n   |\nLL | fn bad_foo<T: Clone + Default, Z: Copy>(arg0: T, arg1: Z)\n   |                       ^^^^^^^\n   |\n   = help: consider removing this trait bound\n\n"}
{"message":"this trait bound is already specified in trait declaration","code":{"code":"clippy::trait_duplication_in_bounds","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trait_duplication_in_bounds_unfixable.rs","byte_start":564,"byte_end":571,"line_start":35,"line_end":35,"column_start":15,"column_end":22,"is_primary":true,"text":[{"text":"        Self: Default;","highlight_start":15,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing this trait bound","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this trait bound is already specified in trait declaration\n  --> tests/ui/trait_duplication_in_bounds_unfixable.rs:35:15\n   |\nLL |         Self: Default;\n   |               ^^^^^^^\n   |\n   = help: consider removing this trait bound\n\n"}
{"message":"this trait bound is already specified in trait declaration","code":{"code":"clippy::trait_duplication_in_bounds","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trait_duplication_in_bounds_unfixable.rs","byte_start":719,"byte_end":726,"line_start":49,"line_end":49,"column_start":15,"column_end":22,"is_primary":true,"text":[{"text":"        Self: Default + Clone;","highlight_start":15,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing this trait bound","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this trait bound is already specified in trait declaration\n  --> tests/ui/trait_duplication_in_bounds_unfixable.rs:49:15\n   |\nLL |         Self: Default + Clone;\n   |               ^^^^^^^\n   |\n   = help: consider removing this trait bound\n\n"}
{"message":"this trait bound is already specified in trait declaration","code":{"code":"clippy::trait_duplication_in_bounds","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trait_duplication_in_bounds_unfixable.rs","byte_start":808,"byte_end":815,"line_start":55,"line_end":55,"column_start":15,"column_end":22,"is_primary":true,"text":[{"text":"        Self: Default + Clone;","highlight_start":15,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing this trait bound","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this trait bound is already specified in trait declaration\n  --> tests/ui/trait_duplication_in_bounds_unfixable.rs:55:15\n   |\nLL |         Self: Default + Clone;\n   |               ^^^^^^^\n   |\n   = help: consider removing this trait bound\n\n"}
{"message":"this trait bound is already specified in trait declaration","code":{"code":"clippy::trait_duplication_in_bounds","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trait_duplication_in_bounds_unfixable.rs","byte_start":818,"byte_end":823,"line_start":55,"line_end":55,"column_start":25,"column_end":30,"is_primary":true,"text":[{"text":"        Self: Default + Clone;","highlight_start":25,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing this trait bound","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this trait bound is already specified in trait declaration\n  --> tests/ui/trait_duplication_in_bounds_unfixable.rs:55:25\n   |\nLL |         Self: Default + Clone;\n   |                         ^^^^^\n   |\n   = help: consider removing this trait bound\n\n"}
{"message":"this trait bound is already specified in trait declaration","code":{"code":"clippy::trait_duplication_in_bounds","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trait_duplication_in_bounds_unfixable.rs","byte_start":860,"byte_end":867,"line_start":58,"line_end":58,"column_start":15,"column_end":22,"is_primary":true,"text":[{"text":"        Self: Default;","highlight_start":15,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing this trait bound","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this trait bound is already specified in trait declaration\n  --> tests/ui/trait_duplication_in_bounds_unfixable.rs:58:15\n   |\nLL |         Self: Default;\n   |               ^^^^^^^\n   |\n   = help: consider removing this trait bound\n\n"}
{"message":"this trait bound is already specified in trait declaration","code":{"code":"clippy::trait_duplication_in_bounds","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trait_duplication_in_bounds_unfixable.rs","byte_start":1390,"byte_end":1410,"line_start":93,"line_end":93,"column_start":15,"column_end":35,"is_primary":true,"text":[{"text":"        Self: Iterator<Item = Foo>,","highlight_start":15,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing this trait bound","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this trait bound is already specified in trait declaration\n  --> tests/ui/trait_duplication_in_bounds_unfixable.rs:93:15\n   |\nLL |         Self: Iterator<Item = Foo>,\n   |               ^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider removing this trait bound\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.8.0/src/lib.rs:111:22
