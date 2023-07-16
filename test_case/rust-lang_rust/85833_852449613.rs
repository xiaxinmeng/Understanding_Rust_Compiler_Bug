plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
ES-Check: there were no ES version matching errors!  🎉
+ eslint ../src/librustdoc/html/static/main.js ../src/librustdoc/html/static/search.js ../src/librustdoc/html/static/settings.js ../src/librustdoc/html/static/sidebar-items.js ../src/librustdoc/html/static/source-script.js ../src/librustdoc/html/static/storage.js

/checkout/src/librustdoc/html/static/main.js
  1128:26  error  'createSimpleToggle' is not defined  no-undef
  1130:27  error  'createToggle' is not defined        no-undef
  1141:45  error  Missing semicolon                    semi
  1156:48  error  Missing semicolon                    semi

✖ 4 problems (4 errors, 0 warnings)
  2 errors and 0 warnings potentially fixable with the `--fix` option.
