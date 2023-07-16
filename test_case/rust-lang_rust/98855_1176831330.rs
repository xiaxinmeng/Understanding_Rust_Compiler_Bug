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
* highest error code: E0788
Found 505 error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/src/librustdoc/html/static/css/themes/light.css:199: line longer than 100 chars
tidy error: /checkout/src/librustdoc/html/static/css/themes/ayu.css:231: line longer than 100 chars
tidy error: /checkout/src/librustdoc/html/static/css/themes/dark.css:208: line longer than 100 chars
some tidy checks failed
Build completed unsuccessfully in 0:00:10
