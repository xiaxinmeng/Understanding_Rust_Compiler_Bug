plain
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Failed building wheel for PyYAML
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-7y4b89zd/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmpygr4bnw4pip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
  |
3 | use std::rc::Rc;
  |     ^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: aborting due to previous error

[RUSTC-TIMING] coretests test:true 8.854
error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "wasm32-unknown-unknown" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units library/core
Build completed unsuccessfully in 0:23:13
