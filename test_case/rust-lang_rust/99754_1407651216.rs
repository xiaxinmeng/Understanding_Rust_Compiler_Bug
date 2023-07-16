plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ece894d15649f0f9d7884f915dc821f00bd0418b)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---

- error: anonymous structs are not allowed outside of unnamed struct or union fields
-   --> $DIR/restrict_anonymous_structs.rs:4:11
-    |
- LL | fn f() -> struct { field: u8 } {}
-    |           ^^^^^^^^^^^^^^^^^^^^ anonymous struct declared here
- error: anonymous structs are not allowed outside of unnamed struct or union fields
-   --> $DIR/restrict_anonymous_structs.rs:7:10
-    |
-    |
- LL | fn f2(a: struct { field: u8 } ) {}
-    |          ^^^^^^^^^^^^^^^^^^^^ anonymous struct declared here
- error: anonymous structs are not allowed outside of unnamed struct or union fields
-   --> $DIR/restrict_anonymous_structs.rs:12:12
-    |
-    |
- LL |     field: struct { field: u8 }
-    |            ^^^^^^^^^^^^^^^^^^^^ anonymous struct declared here
- error: anonymous structs are not allowed outside of unnamed struct or union fields
-   --> $DIR/restrict_anonymous_structs.rs:17:13
-    |
-    |
- LL |     field1: struct { field: u8 }
-    |             ^^^^^^^^^^^^^^^^^^^^ anonymous struct declared here
- error: anonymous structs are not allowed outside of unnamed struct or union fields
-   --> $DIR/restrict_anonymous_structs.rs:21:10
-    |
-    |
- LL | struct I(struct { field: u8 }, u8);
-    |          ^^^^^^^^^^^^^^^^^^^^ anonymous struct declared here
- error: anonymous structs are not allowed outside of unnamed struct or union fields
-   --> $DIR/restrict_anonymous_structs.rs:25:7
-    |
-    |
- LL |     K(struct { field: u8 }),
-    |       ^^^^^^^^^^^^^^^^^^^^ anonymous struct declared here
- error: anonymous fields are not allowed outside of structs or unions
-   --> $DIR/restrict_anonymous_structs.rs:28:9
-    |
-    |
- LL |         _ : struct { field: u8 }
-    |         -^^^^^^^^^^^^^^^^^^^^^^^
-    |         anonymous field declared here
- 
- error: anonymous structs are not allowed outside of unnamed struct or union fields
-   --> $DIR/restrict_anonymous_structs.rs:28:13
-   --> $DIR/restrict_anonymous_structs.rs:28:13
-    |
- LL |         _ : struct { field: u8 }
-    |             ^^^^^^^^^^^^^^^^^^^^ anonymous struct declared here
- error: anonymous fields are not allowed outside of structs or unions
-   --> $DIR/restrict_anonymous_structs.rs:33:9
-    |
- LL |         _ : u8
- LL |         _ : u8
-    |         -^^^^^
-    |         |
-    |         anonymous field declared here
- 
- error: anonymous structs are not allowed outside of unnamed struct or union fields
-   --> $DIR/restrict_anonymous_structs.rs:37:10
-    |
- LL | const L: struct { field: u8 } = 0;
-    |          ^^^^^^^^^^^^^^^^^^^^ anonymous struct declared here
- error: anonymous structs are not allowed outside of unnamed struct or union fields
-   --> $DIR/restrict_anonymous_structs.rs:40:11
-    |
-    |
- LL | static M: struct { field: u8 } = 0;
-    |           ^^^^^^^^^^^^^^^^^^^^ anonymous struct declared here
- error: anonymous structs are not allowed outside of unnamed struct or union fields
-   --> $DIR/restrict_anonymous_structs.rs:43:10
-    |
-    |
- LL | type N = struct { field: u8 };
-    |          ^^^^^^^^^^^^^^^^^^^^ anonymous struct declared here
- error: anonymous structs are not allowed outside of unnamed struct or union fields
+ error: expected type, found keyword `struct`
78   --> $DIR/restrict_anonymous_structs.rs:46:6
79    |
79    |
80 LL | impl struct { field: u8 } {}
-    |      ^^^^^^^^^^^^^^^^^^^^ anonymous struct declared here
+    |      ^^^^^^ expected type
82 
- error: anonymous structs are not allowed outside of unnamed struct or union fields
- error: anonymous structs are not allowed outside of unnamed struct or union fields
-   --> $DIR/restrict_anonymous_structs.rs:51:14
-    |
- LL | impl Foo for struct { field: u8 } {}
-    |              ^^^^^^^^^^^^^^^^^^^^ anonymous struct declared here
- error: anonymous structs are not allowed outside of unnamed struct or union fields
-   --> $DIR/restrict_anonymous_structs.rs:55:13
-    |
-    |
- LL |     let p: [struct { field: u8 }; 1];
-    |             ^^^^^^^^^^^^^^^^^^^^ anonymous struct declared here
- error: anonymous structs are not allowed outside of unnamed struct or union fields
-   --> $DIR/restrict_anonymous_structs.rs:58:13
-    |
-    |
- LL |     let q: (struct { field: u8 }, u8);
-    |             ^^^^^^^^^^^^^^^^^^^^ anonymous struct declared here
- error: anonymous structs are not allowed outside of unnamed struct or union fields
-   --> $DIR/restrict_anonymous_structs.rs:61:19
-    |
-    |
- LL |     let c = || -> struct { field: u8 } {};
-    |                   ^^^^^^^^^^^^^^^^^^^^ anonymous struct declared here
- error: anonymous structs are unimplemented
-   --> $DIR/restrict_anonymous_structs.rs:4:11
-    |
-    |
- LL | fn f() -> struct { field: u8 } {}
- 
- error: anonymous structs are unimplemented
-   --> $DIR/restrict_anonymous_structs.rs:7:10
-    |
-    |
- LL | fn f2(a: struct { field: u8 } ) {}
- 
- error: anonymous structs are unimplemented
-   --> $DIR/restrict_anonymous_structs.rs:12:12
-    |
-    |
- LL |     field: struct { field: u8 }
- 
- error: anonymous structs are unimplemented
-   --> $DIR/restrict_anonymous_structs.rs:17:13
-    |
-    |
- LL |     field1: struct { field: u8 }
- 
- error: anonymous structs are unimplemented
-   --> $DIR/restrict_anonymous_structs.rs:21:10
-    |
-    |
- LL | struct I(struct { field: u8 }, u8);
- 
- error: anonymous structs are unimplemented
-   --> $DIR/restrict_anonymous_structs.rs:25:7
-    |
-    |
- LL |     K(struct { field: u8 }),
- 
- error: anonymous structs are unimplemented
-   --> $DIR/restrict_anonymous_structs.rs:28:13
-    |
-    |
- LL |         _ : struct { field: u8 }
- 
- error: anonymous structs are unimplemented
-   --> $DIR/restrict_anonymous_structs.rs:37:10
-    |
-    |
- LL | const L: struct { field: u8 } = 0;
- 
- error: anonymous structs are unimplemented
-   --> $DIR/restrict_anonymous_structs.rs:40:11
-    |
-    |
- LL | static M: struct { field: u8 } = 0;
- 
- error: anonymous structs are unimplemented
-   --> $DIR/restrict_anonymous_structs.rs:43:10
-    |
-    |
- LL | type N = struct { field: u8 };
- 
- error: anonymous structs are unimplemented
-   --> $DIR/restrict_anonymous_structs.rs:46:6
-    |
-    |
- LL | impl struct { field: u8 } {}
- 
- error: anonymous structs are unimplemented
-   --> $DIR/restrict_anonymous_structs.rs:51:14
-    |
-    |
- LL | impl Foo for struct { field: u8 } {}
- 
- error: anonymous structs are unimplemented
-   --> $DIR/restrict_anonymous_structs.rs:55:13
-    |
-    |
- LL |     let p: [struct { field: u8 }; 1];
- 
- error: anonymous structs are unimplemented
-   --> $DIR/restrict_anonymous_structs.rs:58:13
-    |
-    |
- LL |     let q: (struct { field: u8 }, u8);
- 
- error: anonymous structs are unimplemented
-   --> $DIR/restrict_anonymous_structs.rs:61:19
-    |
-    |
- LL |     let c = || -> struct { field: u8 } {};
- 
- error: aborting due to 32 previous errors
+ error: aborting due to previous error
198 
---
To only update this specific test, also pass `--test-args unnamed-fields/restrict_anonymous_structs.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/unnamed-fields/restrict_anonymous_structs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unnamed-fields/restrict_anonymous_structs" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unnamed-fields/restrict_anonymous_structs/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected type, found keyword `struct`
  --> fake-test-src-base/unnamed-fields/restrict_anonymous_structs.rs:46:6
   |
LL | impl struct { field: u8 } {} //~ ERROR anonymous structs are not allowed outside of unnamed struct or union fields

error: aborting due to previous error
------------------------------------------

