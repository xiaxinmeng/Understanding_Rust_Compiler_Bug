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
+ eslint -c ../src/librustdoc/html/static/.eslintrc.js ../src/librustdoc/html/static/js/externs.js ../src/librustdoc/html/static/js/main.js ../src/librustdoc/html/static/js/scrape-examples.js ../src/librustdoc/html/static/js/search.js ../src/librustdoc/html/static/js/settings.js ../src/librustdoc/html/static/js/source-script.js ../src/librustdoc/html/static/js/storage.js

/checkout/src/librustdoc/html/static/js/settings.js
  3:11  error  'addClass' is defined but never used. Allowed unused vars must match /^_/u  no-unused-vars
/checkout/src/librustdoc/html/static/js/storage.js
/checkout/src/librustdoc/html/static/js/storage.js
  39:7  error  'localStoredTheme' is assigned a value but never used. Allowed unused vars must match /^_/u  no-unused-vars

✖ 2 problems (2 errors, 0 warnings)
