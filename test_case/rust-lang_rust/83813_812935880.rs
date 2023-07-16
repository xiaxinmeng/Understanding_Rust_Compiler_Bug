plain
Command failed. Attempt 4/5:
Sending build context to Docker daemon  494.1kB

Step 1/10 : FROM ubuntu:18.04
Get https://registry-1.docker.io/v2/: net/http: request canceled (Client.Timeout exceeded while awaiting headers)
Sending build context to Docker daemon  494.1kB

Step 1/10 : FROM ubuntu:18.04
18.04: Pulling from library/ubuntu
---
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
Checking which error codes lack tests...
Found 435 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/src/test/codegen/remap_path_prefix/double-remap-imported.rs: missing trailing newline
tidy error: /checkout/src/test/codegen/remap_path_prefix/issue-73167-remap-std.rs:7: trailing whitespace
tidy error: /checkout/src/test/codegen/remap_path_prefix/issue-73167-remap-std.rs:11: trailing whitespace
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:13
