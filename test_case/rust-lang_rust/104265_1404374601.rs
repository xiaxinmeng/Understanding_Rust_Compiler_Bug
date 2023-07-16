plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:8c19d39c6d9d7e831f6e393b2a871216393a5761)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
Rustbook (x86_64-unknown-linux-gnu) - edition-guide
Rustbook (x86_64-unknown-linux-gnu) - style-guide
Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.23s
book/ch06-01-defining-an-enum.html:309: broken link - `std/net/enum.IpAddr.html`
book/print.html:5534: broken link - `std/net/enum.IpAddr.html`
number of HTML files scanned: 33469
number of HTML redirects found: 10249
number of links checked: 2965056
number of links ignored due to external: 99180
