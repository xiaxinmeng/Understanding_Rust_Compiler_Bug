plain
Removing intermediate container 1aba9896658a
 ---> d4bbc62b5142
Step 11/13 : RUN npm install -g browser-ui-test@0.7.0 --unsafe-perm=true
 ---> Running in 7633f7b30d58
npm WARN deprecated puppeteer@3.3.0: Version no longer supported. Upgrade to @latest

> puppeteer@3.3.0 install /node-v14.4.0-linux-x64/lib/node_modules/browser-ui-test/node_modules/puppeteer
> node install.js



Chromium (756035) downloaded to /node-v14.4.0-linux-x64/lib/node_modules/browser-ui-test/node_modules/puppeteer/.local-chromium/linux-756035
added 49 packages from 86 contributors in 9.45s
Removing intermediate container 7633f7b30d58
 ---> 8907c8fda46c
Step 12/13 : ENV RUST_CONFIGURE_ARGS   --build=x86_64-unknown-linux-gnu   --save-toolstates=/tmp/toolstate/toolstates.json
---
-hello dup fd
-

The actual stdout differed from the expected stdout.
Actual stdout saved to /tmp/compiletestZ5vPy7/fs.stage-id.stdout

-hello dup fd
+error: unsupported operation: can't call foreign function: readdir64
+   --> /checkout/library/std/src/sys/unix/fs.rs:482:33
---
+
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestZ5vPy7/fs.stage-id.stderr
To only update this specific test, also pass `--test-args fs.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/fs.rs" "-L" "/tmp/compiletestZ5vPy7" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestZ5vPy7/fs.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-L" "/tmp/compiletestZ5vPy7/fs.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestZ5vPy7/portable-simd.stage-id.stderr
To only update this specific test, also pass `--test-args portable-simd.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/portable-simd.rs" "-L" "/tmp/compiletestZ5vPy7" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestZ5vPy7/portable-simd.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestZ5vPy7/portable-simd.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
......... (50/59)
.........  (59/59)


/checkout/src/test/rustdoc-gui/sidebar-mobile.goml sidebar-mobile... FAILED
[ERROR] (line 42) Error: Evaluation failed: different Y values: 543 != 542.96875: for command `assert-position: (".block.keyword li:nth-child(1)", {"y": 542.96875})`
Build completed unsuccessfully in 0:00:20
