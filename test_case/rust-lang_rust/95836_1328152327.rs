
  $( for file in /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-abort/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
