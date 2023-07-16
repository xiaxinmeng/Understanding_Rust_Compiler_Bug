plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +a5d5c8b18da6d26c8237c831cfda59089bccf845:refs/remotes/origin/try
---
Could not determine the LLVM submodule commit hash. Assuming that an LLVM rebuild is not necessary.
To force LLVM to rebuild, remove the file `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/llvm-finished-building`
Rustbook (x86_64-unknown-linux-gnu) - book
Rustbook (x86_64-unknown-linux-gnu) - book/first-edition
[2020-11-15T19:56:22Z WARN  mdbook::renderer::html_handlebars::hbs_renderer] Previous versions of mdBook erroneously accepted `./src/theme` as an automatic theme directory
[2020-11-15T19:56:22Z WARN  mdbook::renderer::html_handlebars::hbs_renderer] Please move your theme files to `./theme` for them to continue being used
Rustbook (x86_64-unknown-linux-gnu) - book/second-edition
[2020-11-15T19:56:22Z WARN  mdbook::renderer::html_handlebars::hbs_renderer] Previous versions of mdBook erroneously accepted `./src/theme` as an automatic theme directory
[2020-11-15T19:56:22Z WARN  mdbook::renderer::html_handlebars::hbs_renderer] Please move your theme files to `./theme` for them to continue being used
Rustbook (x86_64-unknown-linux-gnu) - book/2018-edition
[2020-11-15T19:56:22Z WARN  mdbook::renderer::html_handlebars::hbs_renderer] Previous versions of mdBook erroneously accepted `./src/theme` as an automatic theme directory
[2020-11-15T19:56:22Z WARN  mdbook::renderer::html_handlebars::hbs_renderer] Please move your theme files to `./theme` for them to continue being used
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
---
   Compiling tempfile v3.1.0
   Compiling serde_json v1.0.59
   Compiling lint-docs v0.1.0 (/checkout/src/tools/lint-docs)
    Finished release [optimized] target(s) in 9.18s
error: failed to test example in lint docs for `legacy_derive_helpers` in /checkout/compiler/rustc_lint_defs/src/builtin.rs:2745: lint docs should contain the line `### Example`


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/lint-docs" "--src" "/checkout/compiler" "--out" "/checkout/obj/build/x86_64-unknown-linux-gnu/md-doc/rustc/src/lints" "--rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustc-target" "x86_64-unknown-linux-gnu"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest
Build completed unsuccessfully in 0:23:40
