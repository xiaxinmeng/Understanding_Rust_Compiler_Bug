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
  35:9  error  'addClass' is not defined     no-undef
  36:9  error  'removeClass' is not defined  no-undef
  38:9  error  'removeClass' is not defined  no-undef
  43:9  error  'addClass' is not defined     no-undef
  45:9  error  'addClass' is not defined     no-undef
  47:9  error  'removeClass' is not defined  no-undef

✖ 6 problems (6 errors, 0 warnings)
