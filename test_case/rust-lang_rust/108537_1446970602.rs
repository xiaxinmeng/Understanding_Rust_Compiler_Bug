plain
info: ES-Check: there were no ES version matching errors!  🎉
+ eslint -c ../src/librustdoc/html/static/.eslintrc.js ../src/librustdoc/html/static/js/externs.js ../src/librustdoc/html/static/js/main.js ../src/librustdoc/html/static/js/scrape-examples.js ../src/librustdoc/html/static/js/search.js ../src/librustdoc/html/static/js/settings.js ../src/librustdoc/html/static/js/source-script.js ../src/librustdoc/html/static/js/storage.js

/checkout/src/librustdoc/html/static/js/search.js
  124:14  error  'isWhitespace' is defined but never used. Allowed unused vars must match /^_/u  no-unused-vars
  341:40  error  'prevIsClosingGeneric' is not defined                                           no-undef
  506:48  error  Strings must use doublequote                                                    quotes
  590:48  error  Strings must use doublequote                                                    quotes
  607:19  error  'c' is assigned a value but never used. Allowed unused vars must match /^_/u    no-unused-vars

✖ 5 problems (5 errors, 0 warnings)
  2 errors and 0 warnings potentially fixable with the `--fix` option.
