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

/checkout/src/librustdoc/html/static/js/main.js
  142:5  error  'availableThemes' is not defined  no-undef
/checkout/src/librustdoc/html/static/js/storage.js
/checkout/src/librustdoc/html/static/js/storage.js
  2:11  error  'resourcesSuffix' is defined but never used  no-unused-vars

✖ 2 problems (2 errors, 0 warnings)
