plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between edf0182213a9e30982eb34f3925ddc4cf5ed3471 and 6819c8b1da854c58160c6d7edbf2f11a3351ed33
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Successfully built 4fe54101dba6
Successfully tagged rust-ci:latest
Built container sha256:4fe54101dba6a2a0f52a3be72323412ebc290327f3be5bc7f7b9d2f92091f028
Uploading finished image to https://ci-caches.rust-lang.org/docker/61e28f9363a104fa13b801f9ace616a6a4d19b4df369bd7271eb4e926ff08156e7e481439aee5047771406735211659d7a0eb94514676916b78c7c20382ee352
upload failed: - to s3://rust-lang-ci-sccache2/docker/61e28f9363a104fa13b801f9ace616a6a4d19b4df369bd7271eb4e926ff08156e7e481439aee5047771406735211659d7a0eb94514676916b78c7c20382ee352 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
+++ <stderr output>
+warning: implicit auto-ref creates a reference to a dereference of a raw pointer
+  --> $DIR/dst-raw.rs:LL:CC
+   |
+LL |         let len = (*a).len();
+   |
+   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
+   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
+   = note: `#[warn(implicit_unsafe_autorefs)]` on by default
+help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
+   |
+LL |         let len = (&(*a)).len();
+   |                   ++    +
+warning: implicit auto-ref creates a reference to a dereference of a raw pointer
+  --> $DIR/dst-raw.rs:LL:CC
+   |
+   |
+LL |         let len = (*a).len();
+   |
+   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
+   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
+help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
+   |
+LL |         let len = (&(*a)).len();
+   |                   ++    +
+warning: implicit auto-ref creates a reference to a dereference of a raw pointer
+  --> $DIR/dst-raw.rs:LL:CC
+   |
+   |
+LL |         let len = (*a).len();
+   |
+   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
+   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
+help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
+   |
+LL |         let len = (&(*a)).len();
+   |                   ++    +
+warning: implicit auto-ref creates a reference to a dereference of a raw pointer
+  --> $DIR/dst-raw.rs:LL:CC
+   |
+   |
+LL |         let len = (*a).len();
+   |
+   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
+   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
+help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
+   |
+LL |         let len = (&(*a)).len();
+   |                   ++    +
+


full stderr:
full stderr:
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
  --> tests/pass/dst-raw.rs:38:19
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
  --> tests/pass/dst-raw.rs:47:19
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
  --> tests/pass/dst-raw.rs:75:19
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
  --> tests/pass/dst-raw.rs:83:19
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



tests/pass/stacked-borrows/interior_mutability.rs FAILED:
---
+++ <stderr output>
+warning: implicit auto-ref creates a reference to a dereference of a raw pointer
+  --> $DIR/interior_mutability.rs:LL:CC
+   |
+LL |         (*x.get()).push(0);
+   |
+   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
+   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
+   = note: `#[warn(implicit_unsafe_autorefs)]` on by default
+help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
+   |
+LL |         (&mut (*x.get())).push(0);
+
+



full stderr:
warning: implicit auto-ref creates a reference to a dereference of a raw pointer
  --> tests/pass/stacked-borrows/interior_mutability.rs:73:9
   |
LL |         (*x.get()).push(0);
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: `#[warn(implicit_unsafe_autorefs)]` on by default
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
LL |         (&mut (*x.get())).push(0);



FAILURES:
