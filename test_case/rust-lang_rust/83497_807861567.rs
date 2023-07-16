plain
Setting up libpng12-0:amd64 (1.2.54-1ubuntu1.1) ...
Setting up libfreetype6:amd64 (2.6.1-0.1ubuntu2.5) ...
Setting up libfontconfig1:amd64 (2.11.94-0ubuntu1.1) ...
Setting up fontconfig (2.11.94-0ubuntu1.1) ...
Regenerating fonts cache... done.
Setting up fonts-ipafont-gothic (00303-13ubuntu1) ...
update-alternatives: using /usr/share/fonts/opentype/ipafont-gothic/ipag.ttf to provide /usr/share/fonts/truetype/fonts-japanese-gothic.ttf (fonts-japanese-gothic.ttf) in auto mode
Setting up fonts-kacst (2.01+mry-12) ...
Setting up fonts-wqy-zenhei (0.9.45-6ubuntu1) ...
Setting up libxfixes3:amd64 (1:5.0.1-2) ...
Setting up libxshmfence1:amd64 (1.2-1) ...
Setting up x11-common (1:7.7+13ubuntu3.1) ...
debconf: unable to initialize frontend: Dialog
---
Step 5/11 : RUN npm install browser-ui-test -g --unsafe-perm=true
 ---> Running in 8593dd5069cb
/node-v14.4.0-linux-x64/bin/browser-ui-test -> /node-v14.4.0-linux-x64/lib/node_modules/browser-ui-test/src/index.js

> puppeteer@2.1.1 install /node-v14.4.0-linux-x64/lib/node_modules/browser-ui-test/node_modules/puppeteer
> node install.js


Chromium downloaded to /node-v14.4.0-linux-x64/lib/node_modules/browser-ui-test/node_modules/puppeteer/.local-chromium/linux-722234
+ browser-ui-test@0.2.9
Removing intermediate container 8593dd5069cb
 ---> b1224c0289c8
Step 6/11 : COPY scripts/sccache.sh /scripts/
 ---> 2cb050414ba8
---
test test::test_workspaces_cwd ... ok

failures:

---- build_script::custom_build_script_failed stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build -v`
thread 'build_script::custom_build_script_failed' panicked at '
Expected: execs
    but: differences:
  6 - |  process didn't exit successfully: `[..]/build-script-build` (exit code: 101)|
    + |  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t359/foo/target/debug/build/foo-f1bc02729adbe558/build-script-build` (exit status: 101)|

other output:
``', src/tools/cargo/tests/testsuite/build_script.rs:43:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
---


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/cargo src/tools/cargotest
Build completed unsuccessfully in 0:25:13
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
