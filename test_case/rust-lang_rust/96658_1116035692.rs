plain
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

/checkout/src/librustdoc/html/static/js/settings.js
  122:44  error  Unexpected function expression  prefer-arrow-callback
  269:16  error  Unexpected function expression  prefer-arrow-callback

✖ 2 problems (2 errors, 0 warnings)
  2 errors and 0 warnings potentially fixable with the `--fix` option.
