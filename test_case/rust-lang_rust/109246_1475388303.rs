plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
.......... (90/98)
...   (98/98)


/checkout/tests/rustdoc-gui/scrape-examples-button-focus.goml scrape-examples-button-focus... FAILED
[ERROR] (around line 3): JS errors occurred: Error: SyntaxError: Unexpected end of JSON input
    at JSON.parse (<anonymous>)
    at updateScrapedExample (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:806)
    at file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:1976
    at onEach (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/storage-9184409068f70b79.js:1:1155)
    at onEachLazy (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/storage-9184409068f70b79.js:1:1247)
    at file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:1947
    at file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:2429
/checkout/tests/rustdoc-gui/scrape-examples-color.goml scrape-examples-color... FAILED
[ERROR] (around line 2): JS errors occurred: Error: SyntaxError: Unexpected end of JSON input
    at JSON.parse (<anonymous>)
    at updateScrapedExample (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:806)
    at file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:1976
    at onEach (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/storage-9184409068f70b79.js:1:1155)
    at onEachLazy (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/storage-9184409068f70b79.js:1:1247)
    at file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:1947
    at file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:2429
/checkout/tests/rustdoc-gui/scrape-examples-fonts.goml scrape-examples-fonts... FAILED
[ERROR] (around line 2): JS errors occurred: Error: SyntaxError: Unexpected end of JSON input
    at JSON.parse (<anonymous>)
    at updateScrapedExample (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:806)
    at file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:1976
    at onEach (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/storage-9184409068f70b79.js:1:1155)
    at onEachLazy (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/storage-9184409068f70b79.js:1:1247)
    at file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:1947
    at file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:2429
/checkout/tests/rustdoc-gui/scrape-examples-layout.goml scrape-examples-layout... FAILED
[ERROR] (around line 2): JS errors occurred: Error: SyntaxError: Unexpected end of JSON input
    at JSON.parse (<anonymous>)
    at updateScrapedExample (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:806)
    at file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:1976
    at onEach (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/storage-9184409068f70b79.js:1:1155)
    at onEachLazy (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/storage-9184409068f70b79.js:1:1247)
    at file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:1947
    at file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:2429
/checkout/tests/rustdoc-gui/scrape-examples-toggle.goml scrape-examples-toggle... FAILED
[ERROR] (around line 2): JS errors occurred: Error: SyntaxError: Unexpected end of JSON input
    at JSON.parse (<anonymous>)
    at updateScrapedExample (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:806)
    at file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:1976
    at onEach (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/storage-9184409068f70b79.js:1:1155)
    at onEachLazy (file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/storage-9184409068f70b79.js:1:1247)
    at file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:1947
    at file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/static.files/scrape-examples-ef1e698c1d417c0c.js:1:2429
