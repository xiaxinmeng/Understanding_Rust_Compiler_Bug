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
  2:61  error  'getCurrentValue' is defined but never used  no-unused-vars
/checkout/src/librustdoc/html/static/js/search.js
/checkout/src/librustdoc/html/static/js/search.js
     2:49  error  'updateLocalStorage' is defined but never used       no-unused-vars
     3:21  error  'getCurrentValue' is defined but never used          no-unused-vars
     3:38  error  'removeLocalStorageValue' is defined but never used  no-unused-vars
  1652:13  error  'searchParams' is assigned a value but never used    no-unused-vars

✖ 5 problems (5 errors, 0 warnings)
