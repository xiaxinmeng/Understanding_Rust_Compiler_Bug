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
  244:13  error  'settingsState' is not defined             no-undef
  248:13  error  'settingsState' is not defined             no-undef
  259:13  error  'settingsState' is not defined             no-undef
  343:9   error  'main' is assigned a value but never used  no-unused-vars
  353:13  error  'settingsState' is not defined             no-undef
  440:9   error  'settingsState' is not defined             no-undef
/checkout/src/librustdoc/html/static/js/settings.js
/checkout/src/librustdoc/html/static/js/settings.js
  145:13  error  'settingsState' is not defined       no-undef
  149:33  error  'MAIN_ID' is not defined             no-undef
  153:22  error  'getVar' is not defined              no-undef
  220:5   error  'searchState' is not defined         no-undef
  222:5   error  'getSettingsElement' is not defined  no-undef
  223:13  error  'hasClass' is not defined            no-undef
  224:13  error  'settingsState' is not defined       no-undef
  226:13  error  'settingsState' is not defined       no-undef
  235:17  error  'searchState' is not defined         no-undef

✖ 15 problems (15 errors, 0 warnings)
