plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
Rustbook (x86_64-unknown-linux-gnu) - edition-guide
Rustbook (x86_64-unknown-linux-gnu) - style-guide
Building tool linkchecker (stage0)
    Finished release [optimized] target(s) in 0.17s
guide-unsafe.html:32: broken link - `book/unsafe.html`
guide.html:32: broken link - `book/README.html`
guide-pointers.html:33: broken link - `book/raw-pointers.html`
guide-strings.html:32: broken link - `book/strings.html`
guide-crates.html:32: broken link - `book/crates-and-modules.html`
guide-macros.html:32: broken link - `book/macros.html`
guide-tasks.html:32: broken link - `book/concurrency.html`
guide-ffi.html:32: broken link - `book/ffi.html`
intro.html:31: broken link - `book/README.html`
rustdoc.html:31: broken link - `book/documentation.html`
number of HTML files scanned: 33151
number of HTML redirects found: 10245
number of links checked: 2942727
number of links ignored due to external: 95588
