plain
+ es-check@5.2.3
added 95 packages from 44 contributors in 3.764s
Removing intermediate container fbe984166b49
 ---> b1ff41fd0f42
Step 6/11 : RUN npm install eslint@7.20.0 -g
 ---> Running in bf85813a983f
/node-v14.4.0-linux-x64/bin/eslint -> /node-v14.4.0-linux-x64/lib/node_modules/eslint/bin/eslint.js
+ eslint@7.20.0
Removing intermediate container bf85813a983f
 ---> 66899c4d4550
Step 7/11 : COPY scripts/sccache.sh /scripts/
 ---> 8f391c343ea8
---
Step 10/11 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Running in 38ec4803a9c2
Removing intermediate container 38ec4803a9c2
 ---> 8b1393bcdaea
Step 11/11 : ENV SCRIPT python3 ../x.py --stage 2 test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test --stage 2 src/tools/tidy &&            python3 ../x.py doc --stage 0 library/std &&            /scripts/validate-toolstate.sh &&            es-check es5 ../src/librustdoc/html/static/*.js &&     eslint ../src/librustdoc/html/static/*.js
Removing intermediate container 9a01330632a4
 ---> 5cd69b342094
Successfully built 5cd69b342094
Successfully tagged rust-ci:latest
Successfully tagged rust-ci:latest
Built container sha256:5cd69b342094ee75a337dc08dfa130c22175793add0b14f7036035256fb46ef9
Uploading finished image to https://ci-caches.rust-lang.org/docker/2f3c49e41c70ba68a1eded858a8e9a27b37062f8201807c55f5b605a5a94cdcfee60a35a62a0f7f084f31f8357e06c47bf071157ba7c77eed9a594720576097b
upload failed: - to s3://rust-lang-ci-sccache2/docker/2f3c49e41c70ba68a1eded858a8e9a27b37062f8201807c55f5b605a5a94cdcfee60a35a62a0f7f084f31f8357e06c47bf071157ba7c77eed9a594720576097b Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
ES-Check: there were no ES version matching errors!  🎉
+ eslint ../src/librustdoc/html/static/main.js ../src/librustdoc/html/static/search.js ../src/librustdoc/html/static/settings.js ../src/librustdoc/html/static/source-script.js ../src/librustdoc/html/static/storage.js

/checkout/src/librustdoc/html/static/source-script.js
  183:5   error  'onEachLazy' is not defined   no-undef
  184:9   error  'onEachLazy' is not defined   no-undef
  203:13  error  'searchState' is not defined  no-undef
  237:44  error  'ev' is not defined           no-undef
  241:1   error  'onEachLazy' is not defined   no-undef

✖ 5 problems (5 errors, 0 warnings)
