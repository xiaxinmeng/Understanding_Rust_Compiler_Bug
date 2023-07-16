plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:5f3e9487b084c5235556ffd8baa8b183de9eb120)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
Rustbook (x86_64-unknown-linux-gnu) - edition-guide
Rustbook (x86_64-unknown-linux-gnu) - style-guide
Building tool linkchecker (stage0)
    Finished release [optimized] target(s) in 0.13s
std/option/index.html:301: broken link fragment `#impl-FromIterator%3COption%3CA%3E%3E-for-Option%3CV%3E` pointing to `std/option/enum.Option.html`
std/option/index.html:312: broken link fragment `#impl-Product%3COption%3CU%3E%3E-for-Option%3CT%3E` pointing to `std/option/enum.Option.html`
std/option/index.html:313: broken link fragment `#impl-Sum%3COption%3CU%3E%3E-for-Option%3CT%3E` pointing to `std/option/enum.Option.html`
std/hash/trait.Hash.html:54: broken link fragment `#impl-Hash-for-str` pointing to `std/primitive.str.html`
std/char/struct.CharTryFromError.html:2: broken link fragment `#impl-TryFrom%3Cu32%3E-for-char` pointing to `std/primitive.char.html`
std/convert/trait.TryFrom.html:64: broken link fragment `#impl-From%3Cu8%3E-for-char` pointing to `std/primitive.char.html`
std/primitive.char.html:1012: broken link fragment `#impl-From%3Cu8%3E-for-char` pointing to `std/primitive.char.html`
std/primitive.u8.html:1310: broken link fragment `#impl-From%3Cu8%3E-for-char` pointing to `std/primitive.char.html`
std/result/index.html:320: broken link fragment `#impl-FromIterator%3CResult%3CA,+E%3E%3E-for-Result%3CV,+E%3E` pointing to `std/result/enum.Result.html`
std/result/index.html:331: broken link fragment `#impl-Product%3CResult%3CU,+E%3E%3E-for-Result%3CT,+E%3E` pointing to `std/result/enum.Result.html`
std/result/index.html:332: broken link fragment `#impl-Sum%3CResult%3CU,+E%3E%3E-for-Result%3CT,+E%3E` pointing to `std/result/enum.Result.html`
core/option/index.html:301: broken link fragment `#impl-FromIterator%3COption%3CA%3E%3E-for-Option%3CV%3E` pointing to `core/option/enum.Option.html`
core/option/index.html:312: broken link fragment `#impl-Product%3COption%3CU%3E%3E-for-Option%3CT%3E` pointing to `core/option/enum.Option.html`
core/option/index.html:313: broken link fragment `#impl-Sum%3COption%3CU%3E%3E-for-Option%3CT%3E` pointing to `core/option/enum.Option.html`
core/hash/trait.Hash.html:52: broken link fragment `#impl-Hash-for-str` pointing to `std/primitive.str.html`
core/char/struct.CharTryFromError.html:2: broken link fragment `#impl-TryFrom%3Cu32%3E-for-char` pointing to `core/primitive.char.html`
core/convert/trait.TryFrom.html:64: broken link fragment `#impl-From%3Cu8%3E-for-char` pointing to `core/primitive.char.html`
core/primitive.char.html:1008: broken link fragment `#impl-From%3Cu8%3E-for-char` pointing to `core/primitive.char.html`
core/primitive.u8.html:1309: broken link fragment `#impl-From%3Cu8%3E-for-char` pointing to `core/primitive.char.html`
core/result/index.html:320: broken link fragment `#impl-FromIterator%3CResult%3CA,+E%3E%3E-for-Result%3CV,+E%3E` pointing to `core/result/enum.Result.html`
core/result/index.html:331: broken link fragment `#impl-Product%3CResult%3CU,+E%3E%3E-for-Result%3CT,+E%3E` pointing to `core/result/enum.Result.html`
core/result/index.html:332: broken link fragment `#impl-Sum%3CResult%3CU,+E%3E%3E-for-Result%3CT,+E%3E` pointing to `core/result/enum.Result.html`
number of HTML files scanned: 33510
number of HTML redirects found: 10252
number of links checked: 2960248
number of links ignored due to external: 97051
