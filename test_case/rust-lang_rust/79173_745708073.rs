plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +6d3f41f6b448ed8904851e46639b7ae8a2303e0c:refs/remotes/pull/79173/merge
---
   Compiling libc v0.2.79
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.36
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0433]: failed to resolve: use of undeclared type `ZipNew`
  --> library/core/src/iter/adapters/zip.rs:22:9
   |
22 |         ZipNew::new(a, b)
   |         ^^^^^^ use of undeclared type `ZipNew`

error[E0405]: cannot find trait `ZipNew` in this scope
  --> library/core/src/iter/adapters/zip.rs:37:12
   |
37 | impl<A, B> ZipNew<A, B> for Zip<A, B>


error[E0405]: cannot find trait `ZipNew` in this scope
  --> library/core/src/iter/adapters/zip.rs:53:12
   |
53 | impl<A, B> ZipNew<A, B> for Zip<A, B>

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0405, E0433.
Some errors have detailed explanations: E0405, E0433.
For more information about an error, try `rustc --explain E0405`.
error: could not compile `core`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:03
== clock drift check ==
  local time: Wed Dec 16 01:41:26 UTC 2020
