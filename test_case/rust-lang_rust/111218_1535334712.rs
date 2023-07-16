plain
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180119 sha256=9fa76c45f3193f307e02ea67d1a48cfe7ef2b854a074b766938a88e046bc7887
  Stored in directory: /tmp/pip-ephem-wheel-cache-_iip56li/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Step 14/15 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Running in c89b0eeb7f82
Removing intermediate container c89b0eeb7f82
 ---> 8062bcf9c5c0
Step 15/15 : ENV SCRIPT python3 ../x.py --stage 2 test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test --stage 0 core alloc std test proc_macro &&            python3 ../x.py b translatable-limits && ../build/host/stage0-tools-bin/translatable-lints ../x.py            RUSTDOCFLAGS=\"--document-private-items --document-hidden-items\" python3 ../x.py doc --stage 0 compiler &&            RUSTDOCFLAGS=\"--document-private-items --document-hidden-items\" python3 ../x.py doc --stage 0 library/test &&            /scripts/validate-toolstate.sh &&            /scripts/validate-error-codes.sh &&            reuse lint &&            es-check es6 ../src/librustdoc/html/static/js/*.js &&            eslint -c ../src/librustdoc/html/static/.eslintrc.js ../src/librustdoc/html/static/js/*.js &&            eslint -c ../src/tools/rustdoc-js/.eslintrc.js ../src/tools/rustdoc-js/tester.js &&            eslint -c ../src/tools/rustdoc-gui/.eslintrc.js ../src/tools/rustdoc-gui/tester.js
Removing intermediate container 929ac39d31cc
 ---> 10573fa2f045
Successfully built 10573fa2f045
Successfully tagged rust-ci:latest
Successfully tagged rust-ci:latest
Built container sha256:10573fa2f045adddcd7368bc48b0d11c3e939e997319b9488524a350f534c62f
Uploading finished image to https://ci-caches.rust-lang.org/docker/bf2c1d00f62352a59be06180272f98a184a901a56c57a0c4a5e3f67032096b0c0c62eeb6df964a1aae1615725c5b91b4b1bc8b67c6dae641c1053f2ff0a8845a
upload failed: - to s3://rust-lang-ci-sccache2/docker/bf2c1d00f62352a59be06180272f98a184a901a56c57a0c4a5e3f67032096b0c0c62eeb6df964a1aae1615725c5b91b4b1bc8b67c6dae641c1053f2ff0a8845a Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
+ python3 ../x.py b translatable-limits
##[group]Building bootstrap
    Finished dev [unoptimized] target(s) in 0.04s
##[endgroup]
##[group]Building stage0 tool translatable-limits (x86_64-unknown-linux-gnu)
Building stage0 tool translatable-limits (x86_64-unknown-linux-gnu)
   Compiling translatable-limits v0.1.0 (/checkout/src/tools/translatable-limits)
##[endgroup]
Build completed successfully in 0:00:01
Build completed successfully in 0:00:01
+ ../build/host/stage0-tools-bin/translatable-lints ../x.py RUSTDOCFLAGS=--document-private-items --document-hidden-items python3 ../x.py doc --stage 0 compiler
sh: 1: ../build/host/stage0-tools-bin/translatable-lints: not found
