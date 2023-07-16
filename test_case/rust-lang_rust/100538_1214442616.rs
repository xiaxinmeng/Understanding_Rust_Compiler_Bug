plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              beta       -> FETCH_HEAD
Searching for toolstate changes between fb2194acc4ee897e2e987c79fd58b263bb32db9c and 420c9f906957a67b9d7ccc246b8f73f724e57438
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
    
    running 13 tests
    test /tmp/mdbook-YmZQBP/expressions/loop-expr.md - Loops::Iterator_loops (line 181) ... ignored
    test /tmp/mdbook-YmZQBP/expressions/loop-expr.md - Loops::Iterator_loops (line 190) ... ignored
    test /tmp/mdbook-YmZQBP/expressions/loop-expr.md - Loops::Predicate_loops::_patterns (line 104) ... ignored
    test /tmp/mdbook-YmZQBP/expressions/loop-expr.md - Loops::Predicate_loops::_patterns (line 95) ... ignored
    test /tmp/mdbook-YmZQBP/expressions/loop-expr.md - Loops::Predicate_loops::_condition_chains (line 133) ... FAILED
    test /tmp/mdbook-YmZQBP/expressions/loop-expr.md - Loops::_expressions (line 246) ... ok
    test /tmp/mdbook-YmZQBP/expressions/loop-expr.md - Loops::Iterator_loops (line 160) ... ok
    test /tmp/mdbook-YmZQBP/expressions/loop-expr.md - Loops::Predicate_loops (line 65) ... ok
    test /tmp/mdbook-YmZQBP/expressions/loop-expr.md - Loops::_and_loop_values (line 274) ... ok
    test /tmp/mdbook-YmZQBP/expressions/loop-expr.md - Loops::_expressions (line 231) ... ok
    test /tmp/mdbook-YmZQBP/expressions/loop-expr.md - Loops::Iterator_loops (line 170) ... ok
    test /tmp/mdbook-YmZQBP/expressions/loop-expr.md - Loops::Predicate_loops::_patterns (line 116) ... ok
    test /tmp/mdbook-YmZQBP/expressions/loop-expr.md - Loops::Predicate_loops::_patterns (line 79) ... ok
    failures:
    
    
    ---- /tmp/mdbook-YmZQBP/expressions/loop-expr.md - Loops::Predicate_loops::_condition_chains (line 133) stdout ----
    error[E0658]: `let` expressions in this position are unstable
     --> /tmp/mdbook-YmZQBP/expressions/loop-expr.md:137:11
      |
    5 |     while let Some(inner_opt) = outer_opt
      |
      = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
      = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
      = help: add `#![feature(let_chains)]` to the crate attributes to enable
    error[E0658]: `let` expressions in this position are unstable
    error[E0658]: `let` expressions in this position are unstable
     --> /tmp/mdbook-YmZQBP/expressions/loop-expr.md:138:12
      |
    6 |         && let Some(number) = inner_opt
      |
      = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
      = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
      = help: add `#![feature(let_chains)]` to the crate attributes to enable
    error: aborting due to 2 previous errors
    
    For more information about this error, try `rustc --explain E0658`.
    Couldn't compile the test.
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-YmZQBP/expressions/loop-expr.md - Loops::Predicate_loops::_condition_chains (line 133)
    test result: FAILED. 8 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out; finished in 0.25s
    
    
    --- stderr
    --- stderr
    
[2022-08-14T20:16:48Z ERROR mdbook::book] rustdoc returned an error:
    
    --- stdout
    
    running 6 tests
    test /tmp/mdbook-YmZQBP/expressions/if-expr.md - _expressions::Chains_of_conditions (line 133) ... FAILED
    test /tmp/mdbook-YmZQBP/expressions/if-expr.md - _expressions::Chains_of_conditions (line 101) ... FAILED
    test /tmp/mdbook-YmZQBP/expressions/if-expr.md - _expressions::Chains_of_conditions (line 116) ... ok
    test /tmp/mdbook-YmZQBP/expressions/if-expr.md - _expressions::_patterns (line 79) ... ok
    test /tmp/mdbook-YmZQBP/expressions/if-expr.md - _expressions::_patterns (line 55) ... ok
    test /tmp/mdbook-YmZQBP/expressions/if-expr.md - _expressions::Syntax (line 31) ... ok
    failures:
    
    
    ---- /tmp/mdbook-YmZQBP/expressions/if-expr.md - _expressions::Chains_of_conditions (line 133) stdout ----
    error[E0658]: `let` expressions in this position are unstable
     --> /tmp/mdbook-YmZQBP/expressions/if-expr.md:138:4
      |
    7 | if let Some(x) = foo && (condition1 || condition2) { /*...*/ }
      |
      = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
      = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
      = help: add `#![feature(let_chains)]` to the crate attributes to enable
    error: aborting due to previous error
    
    For more information about this error, try `rustc --explain E0658`.
    Couldn't compile the test.
    Couldn't compile the test.
    ---- /tmp/mdbook-YmZQBP/expressions/if-expr.md - _expressions::Chains_of_conditions (line 101) stdout ----
    error[E0658]: `let` expressions in this position are unstable
     --> /tmp/mdbook-YmZQBP/expressions/if-expr.md:105:8
      |
    6 |     if let Some(inner_opt) = outer_opt
      |
      = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
      = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
      = help: add `#![feature(let_chains)]` to the crate attributes to enable
    error[E0658]: `let` expressions in this position are unstable
    error[E0658]: `let` expressions in this position are unstable
     --> /tmp/mdbook-YmZQBP/expressions/if-expr.md:106:12
      |
    7 |         && let Some(number) = inner_opt
      |
      = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
      = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
      = help: add `#![feature(let_chains)]` to the crate attributes to enable
    error: aborting due to 2 previous errors
    
    For more information about this error, try `rustc --explain E0658`.
    Couldn't compile the test.
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-YmZQBP/expressions/if-expr.md - _expressions::Chains_of_conditions (line 101)
        /tmp/mdbook-YmZQBP/expressions/if-expr.md - _expressions::Chains_of_conditions (line 133)
    test result: FAILED. 4 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s
    
    
    --- stderr
---
Verifying status of rls...
Verifying status of miri...
Verifying status of embedded-book...
Cloning into 'rust-toolstate'...
error: Tool `reference` should be test-pass but is test-fail
