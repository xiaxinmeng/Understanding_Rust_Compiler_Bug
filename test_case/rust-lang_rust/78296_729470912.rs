plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +3d7a430081377f37233ad25b70ec193884238670:refs/remotes/origin/try
---
Could not determine the LLVM submodule commit hash. Assuming that an LLVM rebuild is not necessary.
To force LLVM to rebuild, remove the file `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/llvm-finished-building`
Rustbook (x86_64-unknown-linux-gnu) - book
Rustbook (x86_64-unknown-linux-gnu) - book/first-edition
[2020-11-18T06:29:50Z WARN  mdbook::renderer::html_handlebars::hbs_renderer] Previous versions of mdBook erroneously accepted `./src/theme` as an automatic theme directory
[2020-11-18T06:29:50Z WARN  mdbook::renderer::html_handlebars::hbs_renderer] Please move your theme files to `./theme` for them to continue being used
Rustbook (x86_64-unknown-linux-gnu) - book/second-edition
[2020-11-18T06:29:50Z WARN  mdbook::renderer::html_handlebars::hbs_renderer] Previous versions of mdBook erroneously accepted `./src/theme` as an automatic theme directory
[2020-11-18T06:29:50Z WARN  mdbook::renderer::html_handlebars::hbs_renderer] Please move your theme files to `./theme` for them to continue being used
Rustbook (x86_64-unknown-linux-gnu) - book/2018-edition
[2020-11-18T06:29:51Z WARN  mdbook::renderer::html_handlebars::hbs_renderer] Previous versions of mdBook erroneously accepted `./src/theme` as an automatic theme directory
[2020-11-18T06:29:51Z WARN  mdbook::renderer::html_handlebars::hbs_renderer] Please move your theme files to `./theme` for them to continue being used
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
---
   --> compiler/rustc_typeck/src/check/upvar.rs:354:9
    |
354 |       /// 