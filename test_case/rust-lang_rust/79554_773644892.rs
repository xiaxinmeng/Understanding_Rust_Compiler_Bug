
---- [ui] ui/generic-associated-types/issue-79422.rs stdout ----

The actual stderr differed from the expected stderr.
Actual stderr saved to /Users/bn/Documents/rust-local-fork/gat-tp/build/x86_64-apple-darwin/test/ui/generic-associated-types/issue-79422/issue-79422.stderr
Actual stderr saved to /Users/bn/Documents/rust-local-fork/gat-tp/src/test/ui/generic-associated-types/issue-79422.stderr

error: /Users/bn/Documents/rust-local-fork/gat-tp/src/test/ui/generic-associated-types/issue-79422.rs:20: unexpected error: '20:5: 20:39: generic associated types are unstable [E0658]'

error: /Users/bn/Documents/rust-local-fork/gat-tp/src/test/ui/generic-associated-types/issue-79422.rs:26: unexpected error: '26:5: 26:31: generic associated types are unstable [E0658]'

error: /Users/bn/Documents/rust-local-fork/gat-tp/src/test/ui/generic-associated-types/issue-79422.rs:35: unexpected error: '35:5: 35:32: generic associated types are unstable [E0658]'

error: 3 unexpected errors found, 0 expected errors not found
status: exit code: 1
command: "/Users/bn/Documents/rust-local-fork/gat-tp/build/x86_64-apple-darwin/stage1/bin/rustc" "/Users/bn/Documents/rust-local-fork/gat-tp/src/test/ui/generic-associated-types/issue-79422.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/bn/Documents/rust-local-fork/gat-tp/build/x86_64-apple-darwin/test/ui/generic-associated-types/issue-79422" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/bn/Documents/rust-local-fork/gat-tp/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/bn/Documents/rust-local-fork/gat-tp/build/x86_64-apple-darwin/test/ui/generic-associated-types/issue-79422/auxiliary"
unexpected errors (from JSON output): [
    Error {
        line_num: 20,
        kind: Some(
            Error,
        ),
        msg: "20:5: 20:39: generic associated types are unstable [E0658]",
    },
    Error {
        line_num: 26,
        kind: Some(
            Error,
        ),
        msg: "26:5: 26:31: generic associated types are unstable [E0658]",
    },
    Error {
        line_num: 35,
        kind: Some(
            Error,
        ),
        msg: "35:5: 35:32: generic associated types are unstable [E0658]",
    },
]
