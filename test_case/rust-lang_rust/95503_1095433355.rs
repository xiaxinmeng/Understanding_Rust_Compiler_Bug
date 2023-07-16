plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/bootstrap/builder/tests.rs at line 41:
         assert_eq!(
             first(builder.cache.all::<compile::Std>()),
             &[
-                compile::Std { compiler: Compiler { host: a, stage: 0 }, target: a, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 1 }, target: a, tail_args: vec![] },
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 0 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 1 },
+                    target: a,
+                    tail_args: vec![]
             ]
         );
         );
         assert!(!builder.cache.all::<compile::Assemble>().is_empty());
Diff in /checkout/src/bootstrap/builder/tests.rs at line 55:
         assert_eq!(
         assert_eq!(
             first(builder.cache.all::<compile::Rustc>()),
-            &[compile::Rustc { compiler: Compiler { host: a, stage: 0 }, target: a, tail_args: vec![] },]
+            &[compile::Rustc {
+                compiler: Compiler { host: a, stage: 0 },
+                target: a,
+                tail_args: vec![]
+            },]
     }
 
Diff in /checkout/src/bootstrap/builder/tests.rs at line 69:
Diff in /checkout/src/bootstrap/builder/tests.rs at line 69:
         let a = TargetSelection::from_user("A");
         assert_eq!(
             first(builder.cache.all::<compile::Std>()),
-            &[compile::Std { compiler: Compiler { host: a, stage: 0 }, target: a, tail_args: vec![] },]
+            &[compile::Std {
+                compiler: Compiler { host: a, stage: 0 },
+                target: a,
+                tail_args: vec![]
+            },]
         );
         assert!(!builder.cache.all::<compile::Assemble>().is_empty());
Diff in /checkout/src/bootstrap/builder/tests.rs at line 99:
         assert_eq!(
         assert_eq!(
             first(builder.cache.all::<compile::Std>()),
             &[
-                compile::Std { compiler: Compiler { host: a, stage: 0 }, target: a, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 1 }, target: a, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 0 }, target: b, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 1 }, target: b, tail_args: vec![] },
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 0 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 1 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 0 },
+                    target: b,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 1 },
+                    target: b,
+                    tail_args: vec![]
             ]
         );
         assert_eq!(
Diff in /checkout/src/bootstrap/builder/tests.rs at line 123:
Diff in /checkout/src/bootstrap/builder/tests.rs at line 123:
         assert_eq!(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/util.rs" "/checkout/src/bootstrap/lib.rs" "/checkout/src/librustdoc/lib.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/librustdoc/config.rs" "/checkout/src/bootstrap/config.rs" "/checkout/src/librustdoc/externalfiles.rs" "/checkout/src/bootstrap/builder/tests.rs"` failed.
             first(builder.cache.all::<compile::Rustc>()),
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
             &[
-                compile::Rustc { compiler: Compiler { host: a, stage: 0 }, target: a, tail_args: vec![] },
-                compile::Rustc { compiler: Compiler { host: a, stage: 0 }, target: b, tail_args: vec![] },
+                compile::Rustc {
+                    compiler: Compiler { host: a, stage: 0 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Rustc {
+                compile::Rustc {
+                    compiler: Compiler { host: a, stage: 0 },
+                    target: b,
+                    tail_args: vec![]
             ]
         );
     }
Diff in /checkout/src/bootstrap/builder/tests.rs at line 259:
Diff in /checkout/src/bootstrap/builder/tests.rs at line 259:
         assert_eq!(
             first(builder.cache.all::<compile::Std>()),
             &[
-                compile::Std { compiler: Compiler { host: a, stage: 0 }, target: a, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 1 }, target: a, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 2 }, target: a, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 1 }, target: b, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 2 }, target: b, tail_args: vec![] },
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 0 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 1 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 2 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 1 },
+                    target: b,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 2 },
+                    target: b,
+                    tail_args: vec![]
             ],
         );
         );
         assert_eq!(first(builder.cache.all::<dist::Src>()), &[dist::Src]);
Diff in /checkout/src/bootstrap/builder/tests.rs at line 287:
         assert_eq!(
             first(builder.cache.all::<compile::Rustc>()),
             &[
-                compile::Rustc { compiler: Compiler { host: a, stage: 0 }, target: a, tail_args: vec![] },
-                compile::Rustc { compiler: Compiler { host: a, stage: 1 }, target: b, tail_args: vec![] },
+                compile::Rustc {
+                    compiler: Compiler { host: a, stage: 0 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Rustc {
+                compile::Rustc {
+                    compiler: Compiler { host: a, stage: 1 },
+                    target: b,
+                    tail_args: vec![]
             ]
         );
     }
Diff in /checkout/src/bootstrap/builder/tests.rs at line 382:
Diff in /checkout/src/bootstrap/builder/tests.rs at line 382:
         assert_eq!(
             first(builder.cache.all::<compile::Std>()),
             &[
-                compile::Std { compiler: Compiler { host: a, stage: 0 }, target: a, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 1 }, target: a, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 2 }, target: a, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 1 }, target: b, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 2 }, target: b, tail_args: vec![] },
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 0 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 1 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 2 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 1 },
+                    target: b,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 2 },
+                    target: b,
+                    tail_args: vec![]
             ]
         );
         assert_eq!(
Diff in /checkout/src/bootstrap/builder/tests.rs at line 416:
Diff in /checkout/src/bootstrap/builder/tests.rs at line 416:
         assert_eq!(
             first(builder.cache.all::<compile::Std>()),
             &[
-                compile::Std { compiler: Compiler { host: a, stage: 0 }, target: a, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 1 }, target: a, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 2 }, target: a, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 1 }, target: b, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 2 }, target: b, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 2 }, target: c, tail_args: vec![] },
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 0 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 1 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 2 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 1 },
+                    target: b,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 2 },
+                    target: b,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 2 },
+                    target: c,
+                    tail_args: vec![]
             ]
         );
         );
         assert!(!builder.cache.all::<compile::Assemble>().is_empty());
Diff in /checkout/src/bootstrap/builder/tests.rs at line 428:
         assert_eq!(
             first(builder.cache.all::<compile::Rustc>()),
             &[
-                compile::Rustc { compiler: Compiler { host: a, stage: 0 }, target: a, tail_args: vec![] },
-                compile::Rustc { compiler: Compiler { host: a, stage: 1 }, target: a, tail_args: vec![] },
-                compile::Rustc { compiler: Compiler { host: a, stage: 2 }, target: a, tail_args: vec![] },
-                compile::Rustc { compiler: Compiler { host: a, stage: 1 }, target: b, tail_args: vec![] },
-                compile::Rustc { compiler: Compiler { host: a, stage: 2 }, target: b, tail_args: vec![] },
+                compile::Rustc {
+                    compiler: Compiler { host: a, stage: 0 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Rustc {
+                compile::Rustc {
+                    compiler: Compiler { host: a, stage: 1 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Rustc {
+                compile::Rustc {
+                    compiler: Compiler { host: a, stage: 2 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Rustc {
+                compile::Rustc {
+                    compiler: Compiler { host: a, stage: 1 },
+                    target: b,
+                    tail_args: vec![]
+                compile::Rustc {
+                compile::Rustc {
+                    compiler: Compiler { host: a, stage: 2 },
+                    target: b,
+                    tail_args: vec![]
             ]
         );
     }
Diff in /checkout/src/bootstrap/builder/tests.rs at line 450:
Diff in /checkout/src/bootstrap/builder/tests.rs at line 450:
         assert_eq!(
             first(builder.cache.all::<compile::Std>()),
             &[
-                compile::Std { compiler: Compiler { host: a, stage: 0 }, target: a, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 1 }, target: a, tail_args: vec![] },
-                compile::Std { compiler: Compiler { host: a, stage: 2 }, target: c, tail_args: vec![] },
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 0 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 1 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Std {
+                compile::Std {
+                    compiler: Compiler { host: a, stage: 2 },
+                    target: c,
+                    tail_args: vec![]
             ]
         );
         assert_eq!(
Diff in /checkout/src/bootstrap/builder/tests.rs at line 466:
Diff in /checkout/src/bootstrap/builder/tests.rs at line 466:
         assert_eq!(
             first(builder.cache.all::<compile::Rustc>()),
             &[
-                compile::Rustc { compiler: Compiler { host: a, stage: 0 }, target: a, tail_args: vec![] },
-                compile::Rustc { compiler: Compiler { host: a, stage: 1 }, target: a, tail_args: vec![] },
+                compile::Rustc {
+                    compiler: Compiler { host: a, stage: 0 },
+                    target: a,
+                    tail_args: vec![]
+                compile::Rustc {
+                compile::Rustc {
+                    compiler: Compiler { host: a, stage: 1 },
+                    target: a,
+                    tail_args: vec![]
             ]
         );
     }
Build completed unsuccessfully in 0:00:11
