diff
$ diff -u ~/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-std-workspace-std-1.0.*/Cargo.toml
--- ~/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-std-workspace-std-1.0.0/Cargo.toml        1970-01-01 01:00:00.000000000 +0100
+++ ~/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-std-workspace-std-1.0.1/Cargo.toml        1970-01-01 01:00:00.000000000 +0100
@@ -12,9 +12,10 @@
 
 [package]
 name = "rustc-std-workspace-std"
-version = "1.0.0"
-authors = ["Charles Lew <crlf0710@gmail.com>"]
-description = "Explicitly empty crate for rust-lang/rust integration\n"
+version = "1.0.1"
+authors = ["Alex Crichton <alex@alexcrichton.com>"]
+description = "Workaround for rustbuild"
 license = "MIT/Apache-2.0"
 
-[dependencies]
+[lib]
+name = "std"
$ diff -u ~/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-std-workspace-std-1.0.*/src/lib.rs
--- ~/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-std-workspace-std-1.0.0/src/lib.rs        2019-07-21 04:49:25.000000000 +0200
+++ ~/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-std-workspace-std-1.0.1/src/lib.rs        2019-09-09 17:52:02.000000000 +0200
@@ -0,0 +1 @@
+pub use std::*;
