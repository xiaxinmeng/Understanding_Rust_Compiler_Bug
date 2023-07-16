plain
Successfully built ac0560f80972
Successfully tagged rust-ci:latest
Built container sha256:ac0560f8097251bdc641500d3fe63e480a23c2b8e6028803ea601bdc23588e83
Uploading finished image to https://ci-caches.rust-lang.org/docker/a27a2e8501e6bda8d9ec9572725b52c65accf3f919588efe2ef5cb584fdeae418747b3be4fa090dfcc3ff7ae8714c60234c3f32ed53a403a0831a7e22eb564d1
upload failed: - to s3://rust-lang-ci-sccache2/docker/a27a2e8501e6bda8d9ec9572725b52c65accf3f919588efe2ef5cb584fdeae418747b3be4fa090dfcc3ff7ae8714c60234c3f32ed53a403a0831a7e22eb564d1 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
........................................................................................ 3256/13768
........................................................................................ 3344/13768
...........................................................iiiii........................ 3432/13768
........................................................................................ 3520/13768
.....................F......F........................................................... 3608/13768
........................................................................................ 3784/13768
........................................................................................ 3872/13768
........................................................................................ 3960/13768
......i..........i..........i........................................................... 4048/13768
---
normalized stderr:
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
  --> $DIR/borrowck-unsafe-static-mutable-borrows.rs:16:17
   |
LL |         let x = (*sfoo).x();
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: `#[warn(implicit_unsafe_autorefs)]` on by default
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         let x = (&mut (*sfoo)).x();

warning: 1 warning emitted


---
To only update this specific test, also pass `--test-args borrowck/borrowck-unsafe-static-mutable-borrows.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-unsafe-static-mutable-borrows.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-unsafe-static-mutable-borrows/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-unsafe-static-mutable-borrows/auxiliary"
stdout: none
--- stderr -------------------------------
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL |         let x = (*sfoo).x();
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: `#[warn(implicit_unsafe_autorefs)]` on by default
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         let x = (&mut (*sfoo)).x();

warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/dynamically-sized-types/dst-coerce-custom.rs stdout ----
normalized stderr:
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
  --> $DIR/dst-coerce-custom.rs:41:20
   |
LL |         assert_eq!((*b.x).get(), 42);
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: `#[warn(implicit_unsafe_autorefs)]` on by default
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         assert_eq!((&(*b.x)).get(), 42);
   |                    ++      +
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args dynamically-sized-types/dst-coerce-custom.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dynamically-sized-types/dst-coerce-custom.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dynamically-sized-types/dst-coerce-custom/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dynamically-sized-types/dst-coerce-custom/auxiliary"
stdout: none
--- stderr -------------------------------
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
   |
   |
LL |         assert_eq!((*b.x).get(), 42);
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: `#[warn(implicit_unsafe_autorefs)]` on by default
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         assert_eq!((&(*b.x)).get(), 42);
   |                    ++      +
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/dynamically-sized-types/dst-raw.rs stdout ----
normalized stderr:
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
  --> $DIR/dst-raw.rs:54:19
   |
LL |         let len = (*a).len();
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: `#[warn(implicit_unsafe_autorefs)]` on by default
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         let len = (&(*a)).len();
   |                   ++    +
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
  --> $DIR/dst-raw.rs:63:19
   |
   |
LL |         let len = (*a).len();
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         let len = (&(*a)).len();
   |                   ++    +
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
  --> $DIR/dst-raw.rs:111:19
   |
   |
LL |         let len = (*a).len();
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         let len = (&(*a)).len();
   |                   ++    +
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
  --> $DIR/dst-raw.rs:119:19
   |
   |
LL |         let len = (*a).len();
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         let len = (&(*a)).len();
   |                   ++    +
warning: 4 warnings emitted



---
To only update this specific test, also pass `--test-args dynamically-sized-types/dst-raw.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dynamically-sized-types/dst-raw.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dynamically-sized-types/dst-raw/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dynamically-sized-types/dst-raw/auxiliary"
stdout: none
--- stderr -------------------------------
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
   |
   |
LL |         let len = (*a).len();
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: `#[warn(implicit_unsafe_autorefs)]` on by default
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         let len = (&(*a)).len();
   |                   ++    +
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
  --> /checkout/src/test/ui/dynamically-sized-types/dst-raw.rs:63:19
   |
   |
LL |         let len = (*a).len();
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         let len = (&(*a)).len();
   |                   ++    +
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
  --> /checkout/src/test/ui/dynamically-sized-types/dst-raw.rs:111:19
   |
   |
LL |         let len = (*a).len();
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         let len = (&(*a)).len();
   |                   ++    +
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
  --> /checkout/src/test/ui/dynamically-sized-types/dst-raw.rs:119:19
   |
   |
LL |         let len = (*a).len();
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         let len = (&(*a)).len();
   |                   ++    +
warning: 4 warnings emitted
------------------------------------------



---- [ui] src/test/ui/self/arbitrary_self_types_raw_pointer_struct.rs stdout ----
normalized stderr:
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
  --> $DIR/arbitrary_self_types_raw_pointer_struct.rs:10:9
   |
LL |         (*self).0.as_ref()
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: `#[warn(implicit_unsafe_autorefs)]` on by default
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         (&(*self).0).as_ref()
   |         ++         +
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
  --> $DIR/arbitrary_self_types_raw_pointer_struct.rs:18:9
   |
   |
LL |         (**self).0.as_ref()
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         (&(**self).0).as_ref()
   |         ++          +
warning: 2 warnings emitted



---
To only update this specific test, also pass `--test-args self/arbitrary_self_types_raw_pointer_struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_raw_pointer_struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_raw_pointer_struct/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_raw_pointer_struct/auxiliary"
stdout: none
--- stderr -------------------------------
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
   |
   |
LL |         (*self).0.as_ref()
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: `#[warn(implicit_unsafe_autorefs)]` on by default
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         (&(*self).0).as_ref()
   |         ++         +
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
  --> /checkout/src/test/ui/self/arbitrary_self_types_raw_pointer_struct.rs:18:9
   |
   |
LL |         (**self).0.as_ref()
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         (&(**self).0).as_ref()
   |         ++          +
warning: 2 warnings emitted
------------------------------------------



---- [ui] src/test/ui/union/issue-99375.rs stdout ----
normalized stderr:
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
  --> $DIR/issue-99375.rs:18:29
   |
LL |     (*params).result.init = ((*params).function)();
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: `#[warn(implicit_unsafe_autorefs)]` on by default
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |     (*params).result.init = (&((*params).function))();
   |                             ++                    +
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args union/issue-99375.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/issue-99375.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/issue-99375" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/issue-99375/auxiliary"
stdout: none
--- stderr -------------------------------
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
   |
   |
LL |     (*params).result.init = ((*params).function)();
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: `#[warn(implicit_unsafe_autorefs)]` on by default
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |     (*params).result.init = (&((*params).function))();
   |                             ++                    +
warning: 1 warning emitted
------------------------------------------


