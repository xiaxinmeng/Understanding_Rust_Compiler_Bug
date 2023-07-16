diff
diff --git a/cargo.out b/cargo.out
index d2607d1..2735df2 100644
--- a/cargo.out
+++ b/cargo.out
@@ -1,25 +1,21 @@
-   Compiling futuretest v0.1.0 (file:///mnt/c/Users/Mahmoud/git/futuretest)
-error[E0271]: type mismatch resolving `<futures::FutureResult<(), std::string::String> as futures::IntoFuture>::Error == &str`
+   Compiling futuretest v0.1.0 (/mnt/d/GIT/futuretest)
+error[E0271]: type mismatch resolving `<Failed<(), String> as futures::IntoFuture>::Error == &str`
   --> src/main.rs:13:10
    |
 13 |         .and_then(|_|
-   |          ^^^^^^^^ expected struct `std::string::String`, found &str
-   |
-   = note: expected type `std::string::String`
-              found type `&str`
+   |          ^^^^^^^^ expected struct `String`, found `&str`

-error[E0271]: type mismatch resolving `<futures::FutureResult<(), std::string::String> as futures::IntoFuture>::Error == &str`
+error[E0271]: type mismatch resolving `<Failed<(), String> as futures::IntoFuture>::Error == &str`
   --> src/main.rs:20:10
    |
 20 |     core.run(f).unwrap();
-   |          ^^^ expected struct `std::string::String`, found &str
+   |          ^^^ expected struct `String`, found `&str`
    |
-   = note: expected type `std::string::String`
-              found type `&str`
-   = note: required because of the requirements on the impl of `futures::Future` for `futures::AndThen<futures::Map<futures::MapErr<futures::FutureResult<(), ()>, [closure@src/main.rs:11:18: 11:43]>, [closure@src/main.rs:12:14: 12:67]>, futures::FutureResult<(), std::string::String>, [closure@src/main.rs:13:19: 16:14]>`
+   = note: required because of the requirements on the impl of `futures::Future` for `futures::AndThen<futures::Map<futures::MapErr<Failed<(), ()>, [closure@src/main.rs:11:18: 11:43]>, [closure@src/main.rs:12:14: 12:67]>, Failed<(), String>, [closure@src/main.rs:13:19: 16:14]>`

 error: aborting due to 2 previous errors

-error: Could not compile `futuretest`.
+For more information about this error, try `rustc --explain E0271`.
+error: could not compile `futuretest`
