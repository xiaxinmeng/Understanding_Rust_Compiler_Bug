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
   600:17  error  'lev_distance' is assigned a value but never used  no-unused-vars
   601:17  error  'x' is defined but never used                      no-unused-vars
   601:20  error  'length' is defined but never used                 no-unused-vars
   601:28  error  'firstGeneric' is defined but never used           no-unused-vars
   601:42  error  'elem' is defined but never used                   no-unused-vars
  1044:21  error  'elem' is already defined                          no-redeclare
  1045:26  error  'i' is already defined                             no-redeclare
  1045:33  error  'nSearchWords' is already defined                  no-redeclare
  1052:21  error  'elem' is already defined                          no-redeclare
  1052:64  error  'ty' is already defined                            no-redeclare
  1053:26  error  'i' is already defined                             no-redeclare
  1053:33  error  'nSearchWords' is already defined                  no-redeclare
  1068:22  error  'i' is already defined                             no-redeclare
  1068:29  error  'nSearchWords' is already defined                  no-redeclare
  1721:17  error  'hasClass' is not defined                          no-undef

✖ 15 problems (15 errors, 0 warnings)
