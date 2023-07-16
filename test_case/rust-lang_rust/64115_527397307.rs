plain
2019-09-03T10:14:34.4700460Z test result: ok. 1 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out
2019-09-03T10:14:34.4700580Z 
2019-09-03T10:14:34.4713780Z  finished in 4.699
2019-09-03T10:14:34.4715670Z [TIMING] DocTest { compiler: Compiler { stage: 2, host: "i686-apple-darwin" }, path: "src/doc/rustc", name: "rustc", is_ext_doc: false } -- 4.699
2019-09-03T10:14:34.8841970Z tmp.js:1
2019-09-03T10:14:34.8842450Z (function (exports, require, module, __filename, __dirname) { var N=null,E="",T="t",U="u",searchIndex={{}};
2019-09-03T10:14:34.8843680Z 
2019-09-03T10:14:34.8843770Z SyntaxError: Unexpected token {
2019-09-03T10:14:34.8843770Z SyntaxError: Unexpected token {
2019-09-03T10:14:34.8843860Z     at createScript (vm.js:56:10)
2019-09-03T10:14:34.8844170Z     at Object.runInThisContext (vm.js:97:10)
2019-09-03T10:14:34.8844850Z     at Module._compile (module.js:549:28)
2019-09-03T10:14:34.8846200Z     at loadContent (/Users/vsts/agent/2.155.1/work/1/s/src/tools/rustdoc-js-std/tester.js:166:7)
2019-09-03T10:14:34.8847150Z     at main (/Users/vsts/agent/2.155.1/work/1/s/src/tools/rustdoc-js-std/tester.js:264:19)
2019-09-03T10:14:34.8848210Z     at Object.<anonymous> (/Users/vsts/agent/2.155.1/work/1/s/src/tools/rustdoc-js-std/tester.js:344:14)
2019-09-03T10:14:34.8848420Z     at Module._compile (module.js:577:32)
2019-09-03T10:14:34.8848580Z     at Object.Module._extensions..js (module.js:586:10)
2019-09-03T10:14:34.8848750Z     at Module.load (module.js:494:32)
2019-09-03T10:14:34.8848890Z     at tryModuleLoad (module.js:453:12)
2019-09-03T10:14:34.8864640Z 
2019-09-03T10:14:34.8864640Z 
2019-09-03T10:14:34.8865980Z command did not execute successfully: "/usr/local/bin/node" "src/tools/rustdoc-js-std/tester.js" "i686-apple-darwin"
2019-09-03T10:14:34.8866270Z 
2019-09-03T10:14:34.8866310Z 
2019-09-03T10:14:34.8876890Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-09-03T10:14:34.8877050Z Build completed unsuccessfully in 1:43:56
2019-09-03T10:14:34.8877050Z Build completed unsuccessfully in 1:43:56
2019-09-03T10:14:34.8947710Z == clock drift check ==
2019-09-03T10:14:34.9023720Z   local time: Tue Sep  3 10:14:34 UTC 2019
2019-09-03T10:14:35.0463700Z   network time: Tue, 03 Sep 2019 10:14:35 GMT
2019-09-03T10:14:35.0465840Z == end clock drift check ==
2019-09-03T10:14:35.0642600Z ##[error]Bash exited with code '1'.
2019-09-03T10:14:35.0687000Z ##[section]Starting: Upload CPU usage statistics
2019-09-03T10:14:35.0717600Z ==============================================================================
2019-09-03T10:14:35.0717700Z Task         : Bash
2019-09-03T10:14:35.0717780Z Description  : Run a Bash script on macOS, Linux, or Windows
