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
    3:54  error  'NOT_DISPLAYED_ID' is defined but never used        no-unused-vars
    4:47  error  'switchDisplayedElement' is defined but never used  no-unused-vars
    4:71  error  'getNotDisplayedElem' is defined but never used     no-unused-vars
  219:27  error  Unnecessary escape character: \"                    no-useless-escape
  219:33  error  Unnecessary escape character: \"                    no-useless-escape
  219:41  error  Unnecessary escape character: \"                    no-useless-escape
  219:61  error  Unnecessary escape character: \"                    no-useless-escape
  219:72  error  Unnecessary escape character: \"                    no-useless-escape
  219:89  error  Unnecessary escape character: \"                    no-useless-escape

✖ 9 problems (9 errors, 0 warnings)
