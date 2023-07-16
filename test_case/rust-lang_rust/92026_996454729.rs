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
No error code explanation was removed!
ES-Check: there were no ES version matching errors!  🎉
+ eslint ../src/librustdoc/html/static/js/externs.js ../src/librustdoc/html/static/js/main.js ../src/librustdoc/html/static/js/scrape-examples.js ../src/librustdoc/html/static/js/search.js ../src/librustdoc/html/static/js/settings.js ../src/librustdoc/html/static/js/source-script.js ../src/librustdoc/html/static/js/storage.js

/checkout/src/librustdoc/html/static/js/externs.js
  1:5   error  'searchState' is defined but never used  no-unused-vars
  2:10  error  'initSearch' is defined but never used   no-unused-vars
  2:21  error  'searchIndex' is defined but never used  no-unused-vars
/checkout/src/librustdoc/html/static/js/search.js
/checkout/src/librustdoc/html/static/js/search.js
   17:5  error  'CrateCorpus' is defined but never used  no-unused-vars
   22:5  error  'Corpus' is defined but never used       no-unused-vars
  904:9  error  'ParsedQuery' is defined but never used  no-unused-vars

✖ 6 problems (6 errors, 0 warnings)
