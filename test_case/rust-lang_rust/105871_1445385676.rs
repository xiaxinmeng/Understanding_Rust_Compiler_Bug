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
    Finished release [optimized] target(s) in 0.19s
std/option/enum.Option.html:68: broken intra-doc link - [<code>[T]::first()</code>]
std/option/enum.Option.html:69: broken intra-doc link - [<code>[T]::first()</code>]
std/option/enum.Option.html:98: broken intra-doc link - [<code>[T]::first-mut</code>]
std/option/enum.Option.html:99: broken intra-doc link - [<code>[T]::first-mut</code>]
core/option/enum.Option.html:68: broken intra-doc link - [<code>[T]::first()</code>]
core/option/enum.Option.html:69: broken intra-doc link - [<code>[T]::first()</code>]
core/option/enum.Option.html:98: broken intra-doc link - [<code>[T]::first-mut</code>]
core/option/enum.Option.html:99: broken intra-doc link - [<code>[T]::first-mut</code>]
number of HTML files scanned: 33476
number of HTML redirects found: 10245
number of links checked: 2974925
number of links ignored due to external: 96503
