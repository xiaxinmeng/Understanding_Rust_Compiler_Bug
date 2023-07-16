plain
Step 11/12 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Running in 6bc005978122
Removing intermediate container 6bc005978122
 ---> ea9509b6cb7d
Step 12/12 : ENV SCRIPT python3 ../x.py --stage 2 test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test --stage 2 src/tools/tidy &&            python3 ../x.py doc --stage 0 library/test &&            /scripts/validate-toolstate.sh &&            /scripts/validate-error-codes.sh &&            es-check es5 ../src/librustdoc/html/static/*.js &&            eslint ../src/librustdoc/html/static/js/*.js
Removing intermediate container 48c686eba214
 ---> b79dcde315d9
Successfully built b79dcde315d9
Successfully tagged rust-ci:latest
Successfully tagged rust-ci:latest
Built container sha256:b79dcde315d966bfe48b01ba8c16b46fcc1844997a7c11528babf33427e7416b
Uploading finished image to https://ci-caches.rust-lang.org/docker/823e6da8f83f2e4617172f340bb27ff0f63c626fe726a9e11a2ef648ecf10a5f6405a67a1fbd62f822a25c77c56a75aaf11058af1a031b9e184ee2d1fa200251
upload failed: - to s3://rust-lang-ci-sccache2/docker/823e6da8f83f2e4617172f340bb27ff0f63c626fe726a9e11a2ef648ecf10a5f6405a67a1fbd62f822a25c77c56a75aaf11058af1a031b9e184ee2d1fa200251 Unable to locate credentials
 * branch              master     -> FETCH_HEAD
[CI_JOB_NAME=mingw-check]
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
