
$ RUSTDOCFLAGS="--output-format json" ./x.py doc
Updating only changed submodules
Submodules updated in 0.02 seconds
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
Generating unstable book md files (x86_64-unknown-linux-gnu)
Building stage0 tool unstable-book-gen (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.08s
Building stage0 tool rustbook (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.09s
Rustbook (x86_64-unknown-linux-gnu) - unstable-book
Documenting standalone (x86_64-unknown-linux-gnu)
Documenting book redirect pages (x86_64-unknown-linux-gnu)
Documenting stage0 std (x86_64-unknown-linux-gnu)
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.08s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/home/euclio/repos/rust/library/core)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Item { id: Id("0:2436"), crate_id: 0, name: Some("borrow"), source: Some(Span { filename: "library/core/src/borrow.rs", begin: (210, 4), end: (212, 5) }), visibility: Public, docs: None, links: {}, attrs: ["#[rustc_diagnostic_item = \"noop_method_borrow\"]"], deprecation: None, inner: Method(Method { decl: FnDecl { inputs: [("", BorrowedRef { lifetime: None, mutable: false, type_: Generic("Self") })], output: Some(BorrowedRef { lifetime: None, mutable: false, type_: Generic("T") }), c_variadic: false }, generics: Generics { params: [], where_predicates: [] }, header: {}, abi: "\"Rust\"", has_body: true }) }`,
 right: `Item { id: Id("0:2436"), crate_id: 0, name: Some("borrow"), source: Some(Span { filename: "library/core/src/borrow.rs", begin: (210, 4), end: (212, 5) }), visibility: Default, docs: None, links: {}, attrs: ["#[rustc_diagnostic_item = \"noop_method_borrow\"]"], deprecation: None, inner: Method(Method { decl: FnDecl { inputs: [("self", BorrowedRef { lifetime: None, mutable: false, type_: Generic("Self") })], output: Some(BorrowedRef { lifetime: None, mutable: false, type_: Generic("T") }), c_variadic: false }, generics: Generics { params: [], where_predicates: [] }, header: {}, abi: "\"Rust\"", has_body: true }) }`', src/librustdoc/json/mod.rs:175:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

error: Unrecognized option: 'markdown-css'

error: could not document `core`

Caused by:
  process didn't exit successfully: `/home/euclio/repos/rust/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /home/euclio/repos/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.53.0 --index-page /home/euclio/repos/rust/src/doc/index.md -L dependency=/home/euclio/repos/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/euclio/repos/rust/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap --output-format json -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version 1.53.0-dev` (exit code: 1)


command did not execute successfully: "/home/euclio/repos/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "24" "--release" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/home/euclio/repos/rust/library/test/Cargo.toml" "-p" "core" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.53.0" "--index-page" "/home/euclio/repos/rust/src/doc/index.md"
expected success, got: exit code: 101


failed to run: /home/euclio/repos/rust/build/bootstrap/debug/bootstrap doc
