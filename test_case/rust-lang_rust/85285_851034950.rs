plain
+ es-check@5.2.3
added 95 packages from 44 contributors in 4.054s
Removing intermediate container 015db9bf0727
 ---> bf1824b74aa3
Step 6/11 : RUN npm install eslint@7.20.0 -g
 ---> Running in 3e371f31611f
/node-v14.4.0-linux-x64/bin/eslint -> /node-v14.4.0-linux-x64/lib/node_modules/eslint/bin/eslint.js
+ eslint@7.20.0
Removing intermediate container 3e371f31611f
 ---> 5d92f3d095c8
Step 7/11 : COPY scripts/sccache.sh /scripts/
 ---> 34f104ee0b45
---
Step 10/11 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Running in dba24bd31cd9
Removing intermediate container dba24bd31cd9
 ---> 3efd30bad7ff
Step 11/11 : ENV SCRIPT python3 ../x.py --stage 2 test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test --stage 2 src/tools/tidy &&            python3 ../x.py doc --stage 0 library/test &&            /scripts/validate-toolstate.sh &&            es-check es5 ../src/librustdoc/html/static/*.js &&            eslint ../src/librustdoc/html/static/*.js
Removing intermediate container 688671f1aab4
 ---> 3d3d6c83f7f7
Successfully built 3d3d6c83f7f7
Successfully tagged rust-ci:latest
---
ES-Check: there were no ES version matching errors!  🎉
+ eslint ../src/librustdoc/html/static/main.js ../src/librustdoc/html/static/search.js ../src/librustdoc/html/static/settings.js ../src/librustdoc/html/static/sidebar-items.js ../src/librustdoc/html/static/source-script.js ../src/librustdoc/html/static/storage.js

/checkout/src/librustdoc/html/static/sidebar-items.js
  1:1  error  'initSidebarItems' is not defined  no-undef

✖ 1 problem (1 error, 0 warnings)
