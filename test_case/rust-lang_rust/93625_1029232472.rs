plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 8b7853fe1f87a40ceaddf63aa404817bbfa69676 and 000e6654cb85ddf3054ae43ac94fccb67a4ab916
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Removing intermediate container 9b833e94171f
 ---> 27045c129891
Step 11/13 : RUN npm install -g browser-ui-test@0.7.0 --unsafe-perm=true
 ---> Running in ba5ff6ec443e
npm WARN deprecated puppeteer@3.3.0: Version no longer supported. Upgrade to @latest

> puppeteer@3.3.0 install /node-v14.4.0-linux-x64/lib/node_modules/browser-ui-test/node_modules/puppeteer
> node install.js



Chromium (756035) downloaded to /node-v14.4.0-linux-x64/lib/node_modules/browser-ui-test/node_modules/puppeteer/.local-chromium/linux-756035
added 49 packages from 86 contributors in 8.124s
Removing intermediate container ba5ff6ec443e
 ---> 9174feadfde3
Step 12/13 : ENV RUST_CONFIGURE_ARGS   --build=x86_64-unknown-linux-gnu   --save-toolstates=/tmp/toolstate/toolstates.json
---
Successfully built 352ddbcb3635
Successfully tagged rust-ci:latest
Built container sha256:352ddbcb3635d664ccdb3c201dced8f4da946b329d186bb67cd77f540e7efd27
Uploading finished image to https://ci-caches.rust-lang.org/docker/a4c684db48535f57528cdd21253d8c15716f7121004c370c2ef9bf59ff9399512222798f9e2717aef35869aa31277fbb2aec48fb767ea8d7306761745af95dfb
upload failed: - to s3://rust-lang-ci-sccache2/docker/a4c684db48535f57528cdd21253d8c15716f7121004c370c2ef9bf59ff9399512222798f9e2717aef35869aa31277fbb2aec48fb767ea8d7306761745af95dfb Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
-hello dup fd
-

The actual stdout differed from the expected stdout.
Actual stdout saved to /tmp/compiletestz0ZsfV/fs.stage-id.stdout

-hello dup fd
+error: unsupported operation: can't call foreign function: readdir64
+   --> /checkout/library/std/src/sys/unix/fs.rs:482:33
---
+
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestz0ZsfV/fs.stage-id.stderr
To only update this specific test, also pass `--test-args fs.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/fs.rs" "-L" "/tmp/compiletestz0ZsfV" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0ZsfV/fs.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-L" "/tmp/compiletestz0ZsfV/fs.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---

test [ui] run-pass/concurrency/libc_pthread_cond.rs ... ok

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestz0ZsfV/portable-simd.stage-id.stderr
To only update this specific test, also pass `--test-args portable-simd.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/portable-simd.rs" "-L" "/tmp/compiletestz0ZsfV" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0ZsfV/portable-simd.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestz0ZsfV/portable-simd.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
......... (50/59)
.........  (59/59)


/checkout/src/test/rustdoc-gui/sidebar-mobile.goml sidebar-mobile... FAILED
[ERROR] (line 42) Error: Evaluation failed: different Y values: 543 != 542.96875: for command `assert-position: (".block.keyword li:nth-child(1)", {"y": 542.96875})`
Build completed unsuccessfully in 0:00:16
