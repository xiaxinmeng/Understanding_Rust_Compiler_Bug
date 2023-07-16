plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:aa723573e04016ede7da6c5d7b029e72cb8a05a3)
Complete job name: PR (x86_64-gnu-tools, false, 1, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 7c3f0d6f30eeefa58170012df10008736ac89755 and d1d85508897c4f3eac594a495858072c94a0ae22
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (90/97)
...    (97/97)


/checkout/tests/rustdoc-gui/scrape-examples-toggle.goml scrape-examples-toggle... FAILED
[ERROR] (around line 46): JS errors occurred: Error: TypeError: Cannot set properties of undefined (setting 'innerText')
    at collapseAllDocs (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/main-364b95f387bc3166.js:1:12034)
    at HTMLButtonElement.toggleAllDocs (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/main-364b95f387bc3166.js:1:12208)
/checkout/tests/rustdoc-gui/shortcuts.goml shortcuts... FAILED
[ERROR] (line 16) Error: Evaluation failed: The following errors happened: [`` isn't equal to `[−]`]: for command `assert-text: ("#toggle-all-docs", "[\u2212]")`
[ERROR] (around line 17): JS errors occurred: Error: TypeError: Cannot set properties of undefined (setting 'innerText')
    at collapseAllDocs (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/main-364b95f387bc3166.js:1:12034)
    at HTMLDocument.handleShortcut (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/main-364b95f387bc3166.js:1:7085)
/checkout/tests/rustdoc-gui/sidebar.goml sidebar... FAILED
[ERROR] (around line 146): JS errors occurred: Error: TypeError: Cannot set properties of undefined (setting 'innerText')
    at collapseAllDocs (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/main-364b95f387bc3166.js:1:12034)
    at HTMLButtonElement.toggleAllDocs (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/main-364b95f387bc3166.js:1:12208)
/checkout/tests/rustdoc-gui/toggle-docs.goml toggle-docs... FAILED
[ERROR] (line 4) Error: Evaluation failed: The following errors happened: [`` isn't equal to `[−]`]: for command `assert-text: ("#toggle-all-docs", "[−]")`
[ERROR] (around line 5): JS errors occurred: Error: TypeError: Cannot set properties of undefined (setting 'innerText')
    at collapseAllDocs (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/main-364b95f387bc3166.js:1:12034)
    at HTMLButtonElement.toggleAllDocs (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/main-364b95f387bc3166.js:1:12208)
