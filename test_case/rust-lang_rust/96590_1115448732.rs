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

/checkout/src/librustdoc/html/static/js/search.js
    49:13  error  'foundCurrentTab' is not defined                 no-undef
    59:13  error  'foundCurrentResultSet' is not defined           no-undef
    65:9   error  'foundCurrentTab' is not defined                 no-undef
    65:28  error  'foundCurrentResultSet' is not defined           no-undef
  1759:18  error  'elem' is never reassigned. Use 'const' instead  prefer-const
  1760:17  error  'j' is never reassigned. Use 'const' instead     prefer-const

✖ 6 problems (6 errors, 0 warnings)
  2 errors and 0 warnings potentially fixable with the `--fix` option.
