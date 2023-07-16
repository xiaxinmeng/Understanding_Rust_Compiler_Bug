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

/checkout/src/librustdoc/html/static/js/main.js
  254:54  error  'getCurrentValue' is not defined  no-undef
/checkout/src/librustdoc/html/static/js/search.js
/checkout/src/librustdoc/html/static/js/search.js
  1286:23  error  'getCurrentValue' is not defined               no-undef
  1652:14  error  'runSearchIfNeeded' is defined but never used  no-unused-vars
/checkout/src/librustdoc/html/static/js/storage.js
/checkout/src/librustdoc/html/static/js/storage.js
  123:10  error  'removeLocalStorageValue' is defined but never used  no-unused-vars

✖ 4 problems (4 errors, 0 warnings)
