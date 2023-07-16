patch
diff --git a/Cargo.lock b/Cargo.lock
index ef9f91fdb43..1c9ce7f75e0 100644
--- a/Cargo.lock
+++ b/Cargo.lock
@@ -1078,6 +1078,17 @@ dependencies = [
  "rustc-std-workspace-core",
 ]
 
+[[package]]
+name = "eh_personality"
+version = "0.0.0"
+dependencies = [
+ "cfg-if 0.1.10",
+ "compiler_builtins",
+ "core",
+ "libc",
+ "unwind",
+]
+
 [[package]]
 name = "either"
 version = "1.6.0"
@@ -5066,6 +5077,7 @@ dependencies = [
  "compiler_builtins",
  "core",
  "dlmalloc",
+ "eh_personality",
  "fortanix-sgx-abi",
  "hashbrown",
  "hermit-abi",
diff --git a/library/eh_personality/Cargo.toml b/library/eh_personality/Cargo.toml
new file mode 100644
index 00000000000..47cdf9ee7e9
--- /dev/null
+++ b/library/eh_personality/Cargo.toml
@@ -0,0 +1,19 @@
+[package]
+name = "eh_personality"
+version = "0.0.0"
+license = "MIT OR Apache-2.0"
+repository = "https://github.com/rust-lang/rust.git"
+description = "Personality function for unwinding Rust stack frames"
+edition = "2021"
+
+[lib]
+test = false
+bench = false
+doc = false
+
+[dependencies]
+core = { path = "../core" }
+libc = { version = "0.2", default-features = false }
+unwind = { path = "../unwind" }
+compiler_builtins = "0.1.0"
+cfg-if = "0.1.8"
diff --git a/library/eh_personality/src/lib.rs b/library/eh_personality/src/lib.rs
new file mode 100644
index 00000000000..0c9ac1ac8e4
--- /dev/null
+++ b/library/eh_personality/src/lib.rs
@@ -0,0 +1 @@
+#![no_std]
diff --git a/library/std/Cargo.toml b/library/std/Cargo.toml
index 232ccdf39d4..66c913f1911 100644
--- a/library/std/Cargo.toml
+++ b/library/std/Cargo.toml
@@ -21,6 +21,7 @@ profiler_builtins = { path = "../profiler_builtins", optional = true }
 unwind = { path = "../unwind" }
 hashbrown = { version = "0.11", default-features = false, features = ['rustc-dep-of-std'] }
 std_detect = { path = "../stdarch/crates/std_detect", default-features = false, features = ['rustc-dep-of-std'] }
+eh_personality = { path = "../eh_personality" }
 
 # Dependencies of the `backtrace` crate
 addr2line = { version = "0.16.0", optional = true, default-features = false }
diff --git a/library/std/src/lib.rs b/library/std/src/lib.rs
index 4ba4e2a528e..27704b7d880 100644
--- a/library/std/src/lib.rs
+++ b/library/std/src/lib.rs
@@ -368,6 +368,10 @@
 #[allow(unused_extern_crates)]
 extern crate unwind;
 
+#[doc(masked)]
+#[allow(unused_extern_crates)]
+extern crate eh_personality;
+
 // During testing, this crate is not actually the "real" std library, but rather
 // it links to the real std library, which was compiled from this same source
 // code. So any lang items std defines are conditionally excluded (or else they
