plain
        fragment: None,
    },
    method: "get",
    authorization: None,
    if_modified_since: None,
    if_none_match: None,
req: Request {
    url: Url {
        scheme: "http",
        cannot_be_a_base: false,
---
        ),
        port: Some(
            35837,
        ),
        path: "/index/a-/b-/a-b-c",
        fragment: None,
    },
    method: "get",
    authorization: None,
    authorization: None,
    if_modified_since: None,
    if_none_match: None,
req: Request {
    url: Url {
        scheme: "http",
        cannot_be_a_base: false,
---
        ),
        port: Some(
            35837,
        ),
        path: "/index/a-/b_/a-b_c",
        fragment: None,
    },
    method: "get",
    authorization: None,
    authorization: None,
    if_modified_since: None,
    if_none_match: None,
req: Request {
    url: Url {
        scheme: "http",
        cannot_be_a_base: false,
---
        ),
        port: Some(
            35837,
        ),
        path: "/index/a_/b-/a_b-c",
        fragment: None,
    },
    method: "get",
    authorization: None,
    authorization: None,
    if_modified_since: None,
    if_none_match: None,
req: Request {
    url: Url {
        scheme: "http",
        cannot_be_a_base: false,
---
        ),
        port: Some(
            35837,
        ),
        path: "/index/a_/b_/a_b_c",
        fragment: None,
    },
    method: "get",
    authorization: None,
    authorization: None,
    if_modified_since: None,
    if_none_match: None,
thread 'registry::not_found_permutations' panicked at 'assertion failed: `(left == right)`
thread 'registry::not_found_permutations' panicked at 'assertion failed: `(left == right)`
  left: `["/index/a-/b-/a-b-c", "/index/a-/b_/a-b_c", "/index/a_/b-/a_b-c", "/index/a_/b_/a_b_c"]`,
 right: `["/index/a-/b-/a-b-c", "/index/a_/b-/a_b-c", "/index/a-/b_/a-b_c", "/index/a_/b_/a_b_c"]`', src/tools/cargo/tests/testsuite/registry.rs:3187:5

failures:
    registry::not_found_permutations


test result: FAILED. 2653 passed; 1 failed; 167 ignored; 0 measured; 0 filtered out; finished in 77.25s

error: test failed, to rerun pass `--test testsuite`
Build completed unsuccessfully in 0:19:27
make: *** [Makefile:44: check-aux] Error 1
