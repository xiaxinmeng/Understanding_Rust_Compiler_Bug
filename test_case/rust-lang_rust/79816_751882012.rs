plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
    
    --- stdout
    
    running 6 tests
    test /tmp/mdbook-yRyaFL/destructors.md - Destructors (line 6) ... ignored
    test /tmp/mdbook-yRyaFL/destructors.md - Destructors (line 55) ... FAILED
    test /tmp/mdbook-yRyaFL/destructors.md - Destructors (line 28) ... FAILED
    test /tmp/mdbook-yRyaFL/destructors.md - Destructors (line 128) ... FAILED
    test /tmp/mdbook-yRyaFL/destructors.md - Destructors (line 96) ... ok
    test /tmp/mdbook-yRyaFL/destructors.md - Destructors (line 110) ... ok
    failures:
    
    
    ---- /tmp/mdbook-yRyaFL/destructors.md - Destructors (line 55) stdout ----
    error[E0432]: unresolved import `std::alloc::AllocRef`
     --> /tmp/mdbook-yRyaFL/destructors.md:58:18
    4 | use std::alloc::{AllocRef, Global, GlobalAlloc, Layout};
      |                  ^^^^^^^^ no `AllocRef` in `alloc`
    
    error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
    error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
      --> /tmp/mdbook-yRyaFL/destructors.md:69:20
    15 |             Global.dealloc(c.cast(), Layout::new::<T>());
       |                    ^^^^^^^ method not found in `std::alloc::Global`
    
    error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
    error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
      --> /tmp/mdbook-yRyaFL/destructors.md:82:20
    28 |             Global.dealloc(c.cast::<u8>(), Layout::new::<T>());
       |                    ^^^^^^^ method not found in `std::alloc::Global`
    
    error: aborting due to 3 previous errors
    error: aborting due to 3 previous errors
    
    Some errors have detailed explanations: E0432, E0599.
    For more information about an error, try `rustc --explain E0432`.
    Couldn't compile the test.
    ---- /tmp/mdbook-yRyaFL/destructors.md - Destructors (line 28) stdout ----
    error[E0432]: unresolved import `std::alloc::AllocRef`
     --> /tmp/mdbook-yRyaFL/destructors.md:31:18
    4 | use std::alloc::{AllocRef, Global, GlobalAlloc, Layout};
      |                  ^^^^^^^^ no `AllocRef` in `alloc`
    
    error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
    error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
      --> /tmp/mdbook-yRyaFL/destructors.md:42:20
    15 |             Global.dealloc(c.cast(), Layout::new::<T>())
       |                    ^^^^^^^ method not found in `std::alloc::Global`
    
    error: aborting due to 2 previous errors
    error: aborting due to 2 previous errors
    
    Some errors have detailed explanations: E0432, E0599.
    For more information about an error, try `rustc --explain E0432`.
    Couldn't compile the test.
    ---- /tmp/mdbook-yRyaFL/destructors.md - Destructors (line 128) stdout ----
    error[E0432]: unresolved import `std::alloc::AllocRef`
     --> /tmp/mdbook-yRyaFL/destructors.md:131:18
    4 | use std::alloc::{AllocRef, GlobalAlloc, Global, Layout};
      |                  ^^^^^^^^ no `AllocRef` in `alloc`
    
    error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
    error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
      --> /tmp/mdbook-yRyaFL/destructors.md:142:20
    15 |             Global.dealloc(c.cast(), Layout::new::<T>());
       |                    ^^^^^^^ method not found in `std::alloc::Global`
    
    error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
    error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
      --> /tmp/mdbook-yRyaFL/destructors.md:157:20
    30 |             Global.dealloc(c.cast(), Layout::new::<T>());
       |                    ^^^^^^^ method not found in `std::alloc::Global`
    
    error: aborting due to 3 previous errors
    error: aborting due to 3 previous errors
    
    Some errors have detailed explanations: E0432, E0599.
    For more information about an error, try `rustc --explain E0432`.
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-yRyaFL/destructors.md - Destructors (line 128)
        /tmp/mdbook-yRyaFL/destructors.md - Destructors (line 28)
        /tmp/mdbook-yRyaFL/destructors.md - Destructors (line 55)
    test result: FAILED. 2 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.21s
    
    
    --- stderr
    --- stderr
    
[2020-12-28T22:10:26Z ERROR mdbook::book] rustdoc returned an error:
    
    --- stdout
    
    running 1 test
    test /tmp/mdbook-yRyaFL/vec-final.md - The_Final_Code (line 3) ... FAILED
    failures:
    
    
    ---- /tmp/mdbook-yRyaFL/vec-final.md - The_Final_Code (line 3) stdout ----
    error[E0432]: unresolved import `std::alloc::AllocRef`
      --> /tmp/mdbook-yRyaFL/vec-final.md:13:5
    11 |     AllocRef,
       |     ^^^^^^^^ no `AllocRef` in `alloc`
    
    error[E0599]: no method named `alloc` found for struct `std::alloc::Global` in the current scope
    error[E0599]: no method named `alloc` found for struct `std::alloc::Global` in the current scope
      --> /tmp/mdbook-yRyaFL/vec-final.md:43:34
    41 |                 let ptr = Global.alloc(Layout::array::<T>(1).unwrap());
       |                                  ^^^^^ method not found in `std::alloc::Global`
    
    error[E0599]: no method named `grow` found for struct `std::alloc::Global` in the current scope
    error[E0599]: no method named `grow` found for struct `std::alloc::Global` in the current scope
      --> /tmp/mdbook-yRyaFL/vec-final.md:48:34
    46 |                 let ptr = Global.grow(c.cast(),
       |                                  ^^^^ method not found in `std::alloc::Global`
       |
       = help: items from traits can only be used if the trait is in scope
       = help: items from traits can only be used if the trait is in scope
    help: the following trait is implemented but not in scope; perhaps add a `use` for it:
       |
    6  | use std::alloc::Allocator;
       |
    
    error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
      --> /tmp/mdbook-yRyaFL/vec-final.md:75:24
    73 |                 Global.dealloc(c.cast(),
       |                        ^^^^^^^ method not found in `std::alloc::Global`
    
    error: aborting due to 4 previous errors
    error: aborting due to 4 previous errors
    
    Some errors have detailed explanations: E0432, E0599.
    For more information about an error, try `rustc --explain E0432`.
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-yRyaFL/vec-final.md - The_Final_Code (line 3)
    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s
    
    
    --- stderr
---
This PR updated 'src/doc/book', verifying if status is 'test-pass'...
Verifying status of nomicon...
This PR updated 'src/doc/nomicon', verifying if status is 'test-pass'...

We detected that this PR updated 'nomicon', but its tests failed.

If you do intend to update 'nomicon', please check the error messages above and
commit another update.

If you do NOT intend to update 'nomicon', please ensure you did not accidentally
change the submodule at 'src/doc/nomicon'. You may ask your reviewer for the
proper steps.
{"rust-by-example":"test-pass","miri":"build-fail","edition-guide":"test-pass","rls":"test-pass","rustbook":"test-fail","embedded-book":"test-pass","book":"test-pass","rustfmt":"test-pass","nomicon":"test-fail","cargo-miri":"test-fail","reference":"test-pass"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
