plain
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

/checkout/src/librustdoc/html/static/main.js
  866:47  error  'ev' is defined but never used  no-unused-vars
  869:7   error  Missing semicolon               semi

✖ 2 problems (2 errors, 0 warnings)
  1 error and 0 warnings potentially fixable with the `--fix` option.
