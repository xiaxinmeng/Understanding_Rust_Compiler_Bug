plain
+ eslint -c ../src/librustdoc/html/static/.eslintrc.js ../src/librustdoc/html/static/js/externs.js ../src/librustdoc/html/static/js/main.js ../src/librustdoc/html/static/js/scrape-examples.js ../src/librustdoc/html/static/js/search.js ../src/librustdoc/html/static/js/settings.js ../src/librustdoc/html/static/js/source-script.js ../src/librustdoc/html/static/js/storage.js
+ eslint -c ../src/tools/rustdoc-js/.eslintrc.js ../src/tools/rustdoc-js/tester.js

/checkout/src/tools/rustdoc-js/tester.js
  1:9  error  'error' is assigned a value but never used. Allowed unused vars must match /^_/u  no-unused-vars

✖ 1 problem (1 error, 0 warnings)
