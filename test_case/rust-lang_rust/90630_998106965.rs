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
ES-Check: there were no ES version matching errors!  🎉
+ eslint ../src/librustdoc/html/static/js/main.js ../src/librustdoc/html/static/js/scrape-examples.js ../src/librustdoc/html/static/js/search.js ../src/librustdoc/html/static/js/settings.js ../src/librustdoc/html/static/js/source-script.js ../src/librustdoc/html/static/js/storage.js

/checkout/src/librustdoc/html/static/js/search.js
   64:10  error  'removeEmptyStringsFromArray' is defined but never used  no-unused-vars
  384:18  error  'itemTypeFromName' is defined but never used             no-unused-vars
  519:18  error  'extractGenerics' is defined but never used              no-unused-vars

✖ 3 problems (3 errors, 0 warnings)
