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
    2:61  error  'settingsState' is defined but never used    no-unused-vars
  383:13  error  'search' is assigned a value but never used  no-unused-vars
  465:13  error  'search' is assigned a value but never used  no-unused-vars
/checkout/src/librustdoc/html/static/js/search.js
/checkout/src/librustdoc/html/static/js/search.js
     2:49  error  'hasClass' is defined but never used        no-unused-vars
  1259:13  error  'browserSupportsHistoryApi' is not defined  no-undef
  1442:17  error  'browserSupportsHistoryApi' is not defined  no-undef
  1454:21  error  'browserSupportsHistoryApi' is not defined  no-undef
  1515:63  error  'e' is defined but never used               no-unused-vars
  1536:13  error  'browserSupportsHistoryApi' is not defined  no-undef
/checkout/src/librustdoc/html/static/js/settings.js
/checkout/src/librustdoc/html/static/js/settings.js
    2:82  error  'hideMain' is defined but never used       no-unused-vars
    3:68  error  'showMain' is defined but never used       no-unused-vars
    3:78  error  'settingsState' is defined but never used  no-unused-vars
    4:28  error  'searchState' is defined but never used    no-unused-vars
    4:61  error  'hasClass' is defined but never used       no-unused-vars
  157:17  error  'switchDisplayedElement' is not defined    no-undef
  161:13  error  'getNotDisplayedElem' is not defined       no-undef
  244:51  error  'NOT_DISPLAYED_ID' is not defined          no-undef
  245:17  error  'switchDisplayedElement' is not defined    no-undef
  251:13  error  'switchDisplayedElement' is not defined    no-undef
  260:13  error  'switchDisplayedElement' is not defined    no-undef

✖ 20 problems (20 errors, 0 warnings)
