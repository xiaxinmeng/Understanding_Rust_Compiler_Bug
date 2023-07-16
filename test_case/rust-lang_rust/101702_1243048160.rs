plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
info: ES-Check: there were no ES version matching errors!  🎉
+ eslint -c ../src/librustdoc/html/static/.eslintrc.js ../src/librustdoc/html/static/js/externs.js ../src/librustdoc/html/static/js/main.js ../src/librustdoc/html/static/js/scrape-examples.js ../src/librustdoc/html/static/js/search.js ../src/librustdoc/html/static/js/settings.js ../src/librustdoc/html/static/js/source-script.js ../src/librustdoc/html/static/js/storage.js

/checkout/src/librustdoc/html/static/js/settings.js
  150:64  error  Missing space before =>  arrow-spacing
  150:67  error  Missing space after =>   arrow-spacing
/checkout/src/librustdoc/html/static/js/storage.js
/checkout/src/librustdoc/html/static/js/storage.js
  151:28  error  Expected '!==' and instead saw '!='  eqeqeq

✖ 3 problems (3 errors, 0 warnings)
  2 errors and 0 warnings potentially fixable with the `--fix` option.
