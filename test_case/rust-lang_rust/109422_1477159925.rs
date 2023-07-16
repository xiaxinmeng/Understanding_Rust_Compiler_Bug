plain
info: ES-Check: there were no ES version matching errors!  🎉
+ eslint -c ../src/librustdoc/html/static/.eslintrc.js ../src/librustdoc/html/static/js/externs.js ../src/librustdoc/html/static/js/main.js ../src/librustdoc/html/static/js/scrape-examples.js ../src/librustdoc/html/static/js/search.js ../src/librustdoc/html/static/js/settings.js ../src/librustdoc/html/static/js/source-script.js ../src/librustdoc/html/static/js/storage.js

/checkout/src/librustdoc/html/static/js/main.js
  375:1   error  This line has a length of 141. Maximum allowed is 100  max-len
  376:1   error  This line has a length of 117. Maximum allowed is 100  max-len
  377:29  error  'numbered' is never reassigned. Use 'const' instead    prefer-const
/checkout/src/librustdoc/html/static/js/search.js
/checkout/src/librustdoc/html/static/js/search.js
  2173:15  error  Trailing spaces not allowed  no-trailing-spaces

✖ 4 problems (4 errors, 0 warnings)
  2 errors and 0 warnings potentially fixable with the `--fix` option.
