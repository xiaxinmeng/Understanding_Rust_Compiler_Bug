plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              beta       -> FETCH_HEAD
Searching for toolstate changes between 9bbbf60b0442f1d56fc39f30274be77acc79164c and f92aeaa7eaad79cfc6ee761dcf12f1cda7886242
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CI_ONLY_WHEN_SUBMODULES_CHANGED: 1
  IMAGE: x86_64-gnu-tools
##[endgroup]
fatal: unknown commit 53fd98ca776cb875bc9e5514f56b52eb74f9e7a9
All commits in `HEAD` are present in `master`
src/ci/scripts/verify-stable-version-number.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
    
    --- stdout
    
    running 11 tests
    test /tmp/mdbook-m5bKhI/dropck.md - Drop_Check (line 12) ... ignored
    test /tmp/mdbook-m5bKhI/dropck.md - Drop_Check (line 20) ... ignored
    test /tmp/mdbook-m5bKhI/dropck.md - Drop_Check (line 37) ... ignored
    test /tmp/mdbook-m5bKhI/dropck.md - Drop_Check::An_Escape_Hatch (line 232) ... FAILED
    test /tmp/mdbook-m5bKhI/dropck.md - Drop_Check::An_Escape_Hatch (line 267) ... FAILED
    test /tmp/mdbook-m5bKhI/dropck.md - Drop_Check (line 173) - compile fail ... ok
    test /tmp/mdbook-m5bKhI/dropck.md - Drop_Check (line 145) - compile fail ... ok
    test /tmp/mdbook-m5bKhI/dropck.md - Drop_Check (line 74) - compile fail ... ok
    test /tmp/mdbook-m5bKhI/dropck.md - Drop_Check::An_Escape_Hatch (line 292) ... ok
    test /tmp/mdbook-m5bKhI/dropck.md - Drop_Check::An_Escape_Hatch (line 306) ... ok
    test /tmp/mdbook-m5bKhI/dropck.md - Drop_Check (line 51) ... ok
    failures:
    
    
    ---- /tmp/mdbook-m5bKhI/dropck.md - Drop_Check::An_Escape_Hatch (line 232) stdout ----
    error[E0554]: `#![feature]` may not be used on the beta release channel
     --> /tmp/mdbook-m5bKhI/dropck.md:233:1
      |
    2 | #![feature(dropck_eyepatch)]
    
    error: aborting due to previous error
    
    For more information about this error, try `rustc --explain E0554`.
    For more information about this error, try `rustc --explain E0554`.
    Couldn't compile the test.
    ---- /tmp/mdbook-m5bKhI/dropck.md - Drop_Check::An_Escape_Hatch (line 267) stdout ----
    error[E0554]: `#![feature]` may not be used on the beta release channel
     --> /tmp/mdbook-m5bKhI/dropck.md:267:1
      |
    2 | #![feature(dropck_eyepatch)]
    
    error: aborting due to previous error
    
    For more information about this error, try `rustc --explain E0554`.
    For more information about this error, try `rustc --explain E0554`.
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-m5bKhI/dropck.md - Drop_Check::An_Escape_Hatch (line 232)
        /tmp/mdbook-m5bKhI/dropck.md - Drop_Check::An_Escape_Hatch (line 267)
    test result: FAILED. 6 passed; 2 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.24s
    
    
    --- stderr
    --- stderr
    
[2022-08-08T13:03:52Z ERROR mdbook::book] rustdoc returned an error:
    
    --- stdout
    
    running 6 tests
    test /tmp/mdbook-m5bKhI/destructors.md - Destructors (line 7) ... ignored
    test /tmp/mdbook-m5bKhI/destructors.md - Destructors (line 29) ... FAILED
    test /tmp/mdbook-m5bKhI/destructors.md - Destructors (line 56) ... FAILED
    test /tmp/mdbook-m5bKhI/destructors.md - Destructors (line 129) ... FAILED
    test /tmp/mdbook-m5bKhI/destructors.md - Destructors (line 97) ... ok
    test /tmp/mdbook-m5bKhI/destructors.md - Destructors (line 111) ... ok
    failures:
    
    
    ---- /tmp/mdbook-m5bKhI/destructors.md - Destructors (line 29) stdout ----
    error[E0554]: `#![feature]` may not be used on the beta release channel
     --> /tmp/mdbook-m5bKhI/destructors.md:30:12
      |
    2 | #![feature(ptr_internals, allocator_api)]
    
    
    error[E0554]: `#![feature]` may not be used on the beta release channel
     --> /tmp/mdbook-m5bKhI/destructors.md:30:27
      |
    2 | #![feature(ptr_internals, allocator_api)]
    
    error: aborting due to 2 previous errors
    
    For more information about this error, try `rustc --explain E0554`.
    For more information about this error, try `rustc --explain E0554`.
    Couldn't compile the test.
    ---- /tmp/mdbook-m5bKhI/destructors.md - Destructors (line 56) stdout ----
    error[E0554]: `#![feature]` may not be used on the beta release channel
     --> /tmp/mdbook-m5bKhI/destructors.md:57:12
      |
    2 | #![feature(allocator_api, ptr_internals)]
    
    
    error[E0554]: `#![feature]` may not be used on the beta release channel
     --> /tmp/mdbook-m5bKhI/destructors.md:57:27
      |
    2 | #![feature(allocator_api, ptr_internals)]
    
    error: aborting due to 2 previous errors
    
    For more information about this error, try `rustc --explain E0554`.
    For more information about this error, try `rustc --explain E0554`.
    Couldn't compile the test.
    ---- /tmp/mdbook-m5bKhI/destructors.md - Destructors (line 129) stdout ----
    error[E0554]: `#![feature]` may not be used on the beta release channel
     --> /tmp/mdbook-m5bKhI/destructors.md:130:12
      |
    2 | #![feature(allocator_api, ptr_internals)]
    
    
    error[E0554]: `#![feature]` may not be used on the beta release channel
     --> /tmp/mdbook-m5bKhI/destructors.md:130:27
      |
    2 | #![feature(allocator_api, ptr_internals)]
    
    error: aborting due to 2 previous errors
    
    For more information about this error, try `rustc --explain E0554`.
    For more information about this error, try `rustc --explain E0554`.
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-m5bKhI/destructors.md - Destructors (line 129)
        /tmp/mdbook-m5bKhI/destructors.md - Destructors (line 29)
        /tmp/mdbook-m5bKhI/destructors.md - Destructors (line 56)
    test result: FAILED. 2 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.21s
    
    
    --- stderr
    --- stderr
    
[2022-08-08T13:03:53Z ERROR mdbook::book] rustdoc returned an error:
    
    --- stdout
    
    running 8 tests
    test /tmp/mdbook-m5bKhI/send-and-sync.md - Send_and_Sync (line 56) ... FAILED
    test /tmp/mdbook-m5bKhI/send-and-sync.md - Send_and_Sync::Example (line 140) ... ok
    test /tmp/mdbook-m5bKhI/send-and-sync.md - Send_and_Sync::Example (line 195) ... ok
    test /tmp/mdbook-m5bKhI/send-and-sync.md - Send_and_Sync::Example (line 223) ... ok
    test /tmp/mdbook-m5bKhI/send-and-sync.md - Send_and_Sync::Example (line 181) ... ok
    test /tmp/mdbook-m5bKhI/send-and-sync.md - Send_and_Sync::Example (line 212) ... ok
    test /tmp/mdbook-m5bKhI/send-and-sync.md - Send_and_Sync::Example (line 87) ... ok
    test /tmp/mdbook-m5bKhI/send-and-sync.md - Send_and_Sync (line 46) ... ok
    failures:
    
    
    ---- /tmp/mdbook-m5bKhI/send-and-sync.md - Send_and_Sync (line 56) stdout ----
    error[E0554]: `#![feature]` may not be used on the beta release channel
     --> /tmp/mdbook-m5bKhI/send-and-sync.md:56:1
    2 | #![feature(negative_impls)]
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    
    error: aborting due to previous error
    error: aborting due to previous error
    
    For more information about this error, try `rustc --explain E0554`.
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-m5bKhI/send-and-sync.md - Send_and_Sync (line 56)
    test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s
    
    
    --- stderr
---
Verifying status of rls...
Verifying status of miri...
Verifying status of embedded-book...
Cloning into 'rust-toolstate'...
error: Tool `nomicon` should be test-pass but is test-fail
