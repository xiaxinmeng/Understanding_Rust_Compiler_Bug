plain
Successfully built 9d43cd916acf
Successfully tagged rust-ci:latest
Built container sha256:9d43cd916acf68eab712b5458ddaba1713920632e268ca873d75de0b75fb869d
Uploading finished image to https://ci-caches.rust-lang.org/docker/e4557e5cc9cf865a1ddaf5c5e6287408bf7b19baa6c1b94858e8c86892ec52cdea793ace9dcc3fb0fcaf0c05d569f11a3ff6c252773a00583490260070615770
upload failed: - to s3://rust-lang-ci-sccache2/docker/e4557e5cc9cf865a1ddaf5c5e6287408bf7b19baa6c1b94858e8c86892ec52cdea793ace9dcc3fb0fcaf0c05d569f11a3ff6c252773a00583490260070615770 Unable to locate credentials
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
Build completed successfully in 0:00:40
+ /scripts/validate-toolstate.sh
Cloning into 'rust-toolstate'...
<Nothing changed>
+ /scripts/validate-error-codes.sh
sh: 1: /scripts/validate-error-codes.sh: Permission denied
