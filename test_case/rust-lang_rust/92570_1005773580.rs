plain

4 packages are looking for funding
  run `npm fund` for details

8 vulnerabilities (6 moderate, 2 high)
To address issues that do not require attention, run:
To address issues that do not require attention, run:
  npm audit fix

Some issues need review, and may require choosing


Run `npm audit` for details.
npm notice 
npm notice New major version of npm available! 7.21.1 -> 8.3.0
npm notice Changelog: <https://github.com/npm/cli/releases/tag/v8.3.0>
npm notice Run `npm install -g npm@8.3.0` to update!
npm notice 
 ---> 9654bf266040
Step 6/12 : RUN npm install eslint@8.6.0 -g
 ---> Running in 8c35cdfd2a1f

---
Successfully built 8da64fdf25c5
Successfully tagged rust-ci:latest
Built container sha256:8da64fdf25c5f1c58ea1e8b1b585ae9118958ddfac6124bbdfaac299a045fd03
Uploading finished image to https://ci-caches.rust-lang.org/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef
upload failed: - to s3://rust-lang-ci-sccache2/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
info: ES-Check: there were no ES version matching errors!  🎉
+ eslint ../src/librustdoc/html/static/js/externs.js ../src/librustdoc/html/static/js/main.js ../src/librustdoc/html/static/js/scrape-examples.js ../src/librustdoc/html/static/js/search.js ../src/librustdoc/html/static/js/settings.js ../src/librustdoc/html/static/js/source-script.js ../src/librustdoc/html/static/js/storage.js

/checkout/src/librustdoc/html/static/js/search.js
     1:51  error  'initSearch' is defined but never used  no-unused-vars
     1:63  error  'onEach' is defined but never used      no-unused-vars
  1638:9   error  'exports' is not defined                no-undef
  1639:9   error  'exports' is not defined                no-undef
  1640:9   error  'exports' is not defined                no-undef
  1643:2   error  Unnecessary semicolon                   no-extra-semi

✖ 6 problems (6 errors, 0 warnings)
  1 error and 0 warnings potentially fixable with the `--fix` option.
