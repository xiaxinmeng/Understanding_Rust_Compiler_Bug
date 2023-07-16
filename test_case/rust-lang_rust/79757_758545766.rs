
$ ./build/x86_64-apple-darwin/stage1/bin/rustc src/test/ui/terminal-width/tabs-trimming.rs --error-format=json
{"message":"variable `v` is not bound in all patterns","code":{"code":"E0408","explanation":"An \"or\" pattern was used where the variable bindings are not consistently bound\nacross patterns.\n\nErroneous code example:\n\n