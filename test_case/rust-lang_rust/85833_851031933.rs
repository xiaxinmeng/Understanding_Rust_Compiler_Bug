plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Finished release [optimized] target(s) in 10.72s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: /checkout/src/librustdoc/scrape_examples.rs:110: TODO is deprecated; use FIXME
tidy error: /checkout/src/librustdoc/scrape_examples.rs:114: TODO is deprecated; use FIXME
tidy error: /checkout/src/librustdoc/scrape_examples.rs:119: TODO is deprecated; use FIXME
* highest error code: E0783
* highest error code: E0783
tidy error: /checkout/src/librustdoc/html/render/mod.rs:2448: TODO is deprecated; use FIXME
tidy error: /checkout/src/librustdoc/html/static/rustdoc.css:1824: line longer than 100 chars
tidy error: /checkout/src/librustdoc/html/static/main.js:1004: line longer than 100 chars
tidy error: /checkout/src/librustdoc/html/static/main.js:1008: line longer than 100 chars
tidy error: /checkout/src/librustdoc/html/static/main.js:1050: line longer than 100 chars
tidy error: /checkout/src/librustdoc/html/static/main.js:1078: line longer than 100 chars
tidy error: /checkout/src/librustdoc/html/static/main.js:1116: line longer than 100 chars
tidy error: /checkout/src/librustdoc/html/static/main.js:1140: line longer than 100 chars
tidy error: /checkout/src/librustdoc/lib.rs:601: TODO is deprecated; use FIXME
Found 0 error codes with no tests
Done!
* 337 features
some tidy checks failed
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:16
