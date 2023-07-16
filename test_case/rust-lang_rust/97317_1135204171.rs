plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 222c5724ecc922fe67815f428c19f82c129d9386 and 1065305d43255fe16835fd7abd09bfc4e7b0e930
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
......... (60/64)
....       (64/64)


/checkout/src/test/rustdoc-gui/settings.goml settings... FAILED
[ERROR] (line 37) XPath "//*[@class='setting-line']/*[text()='Use system theme']" not found: for command `assert: "//*[@class='setting-line']/*[text()='Use system theme']"`
[ERROR] (line 61) ".setting-line:last-child .toggle + div" not found: for command `click: ".setting-line:last-child .toggle + div"`
Build completed unsuccessfully in 0:00:16
