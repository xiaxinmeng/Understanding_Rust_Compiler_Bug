plain
info: ES-Check: there were no ES version matching errors!  🎉
+ eslint -c ../src/librustdoc/html/static/.eslintrc.js ../src/librustdoc/html/static/js/externs.js ../src/librustdoc/html/static/js/main.js ../src/librustdoc/html/static/js/scrape-examples.js ../src/librustdoc/html/static/js/search.js ../src/librustdoc/html/static/js/settings.js ../src/librustdoc/html/static/js/source-script.js ../src/librustdoc/html/static/js/storage.js

/checkout/src/librustdoc/html/static/js/search.js
  1747:25  error  'start' is never reassigned. Use 'const' instead  prefer-const
  1748:25  error  'end' is never reassigned. Use 'const' instead    prefer-const

✖ 2 problems (2 errors, 0 warnings)
  2 errors and 0 warnings potentially fixable with the `--fix` option.
