
$ src/ci/docker/run.sh mingw-check
...
Build completed successfully in 0:00:33
+ /scripts/validate-toolstate.sh
Cloning into 'rust-toolstate'...
<Nothing changed>
+ /scripts/validate-error-codes.sh
/scripts/validate-error-codes.sh: line 6: BASE_COMMIT: unbound variable
+ es-check es5 ../src/librustdoc/html/static/main.js ../src/librustdoc/html/static/search.js ../src/librustdoc/html/static/settings.js ../src/librustdoc/html/static/source-script.js ../src/librustdoc/html/static/storage.js
ES-Check: there were no ES version matching errors!  🎉
+ eslint ../src/librustdoc/html/static/main.js ../src/librustdoc/html/static/search.js ../src/librustdoc/html/static/settings.js ../src/librustdoc/html/static/source-script.js ../src/librustdoc/html/static/storage.js
Compile requests               237
Compile requests executed      215
Cache hits                      13
Cache misses                   202
Cache timeouts                   0
Cache read errors                0
Forced recaches                  0
Cache write errors               0
Compilation failures             0
Cache errors                     0
Non-cacheable compilations       0
Non-cacheable calls             18
Non-compilation calls            4
Unsupported compiler calls       0
Average cache write          0.000 s
Average cache read miss      0.105 s
Average cache read hit       0.000 s
Cache location             Local disk: "/sccache"
Cache size                     491 KiB
Max cache size                  10 GiB
== clock drift check ==
  local time: Mon Jun 28 19:08:09 UTC 2021
  network time: Mon, 28 Jun 2021 01:34:02 GMT
== end clock drift check ==
