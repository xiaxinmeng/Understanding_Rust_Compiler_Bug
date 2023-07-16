
2020-07-07T19:34:06.0985373Z 
2020-07-07T19:34:06.0985883Z failures:
2020-07-07T19:34:06.0986029Z 
2020-07-07T19:34:06.0986639Z ---- [ui] ui/issues/issue-25279.rs stdout ----
2020-07-07T19:34:06.0986749Z 
2020-07-07T19:34:06.0986890Z error: test run failed!
2020-07-07T19:34:06.0987074Z status: exit code: 1
2020-07-07T19:34:06.0987566Z command: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25279/a.wasm"
2020-07-07T19:34:06.0987763Z stdout:
2020-07-07T19:34:06.0988066Z ------------------------------------------
2020-07-07T19:34:06.0988163Z 
2020-07-07T19:34:06.0988452Z ------------------------------------------
2020-07-07T19:34:06.0990977Z stderr:
2020-07-07T19:34:06.0991949Z ------------------------------------------
2020-07-07T19:34:06.0992235Z internal/process/per_thread.js:258
2020-07-07T19:34:06.0992512Z   for (const [ from, expansion ] of aliases) {
2020-07-07T19:34:06.0992799Z                                     ^
2020-07-07T19:34:06.0993003Z 
2020-07-07T19:34:06.0993243Z TypeError: aliases is not iterable
2020-07-07T19:34:06.0993532Z     at Object.buildAllowedFlags (internal/process/per_thread.js:258:37)
2020-07-07T19:34:06.0993883Z     at process.get [as allowedNodeEnvironmentFlags] (internal/bootstrap/node.js:160:34)
2020-07-07T19:34:06.0994183Z     at get (<anonymous>)
2020-07-07T19:34:06.0994454Z     at getOwn (internal/bootstrap/loaders.js:150:5)
2020-07-07T19:34:06.0994763Z     at NativeModule.syncExports (internal/bootstrap/loaders.js:258:31)
2020-07-07T19:34:06.0995085Z     at ModuleWrap.<anonymous> (internal/bootstrap/loaders.js:238:22)
2020-07-07T19:34:06.0995409Z     at NativeModule.getESMFacade (internal/bootstrap/loaders.js:243:17)
2020-07-07T19:34:06.0995769Z     at NativeModule.compileForPublicLoader (internal/bootstrap/loaders.js:225:10)
2020-07-07T19:34:06.0996103Z     at loadNativeModule (internal/modules/cjs/helpers.js:25:9)
2020-07-07T19:34:06.0996424Z     at Function.Module._load (internal/modules/cjs/loader.js:908:15)
2020-07-07T19:34:06.0996594Z 
2020-07-07T19:34:06.0997148Z ------------------------------------------
2020-07-07T19:34:06.0997625Z 
2020-07-07T19:34:06.0997761Z 
2020-07-07T19:34:06.0997876Z 
2020-07-07T19:34:06.0998102Z failures:
2020-07-07T19:34:06.0998644Z     [ui] ui/issues/issue-25279.rs
2020-07-07T19:34:06.0998809Z 
2020-07-07T19:34:06.0999456Z test result: [31mFAILED(B[m. 9893 passed; 1 failed; 547 ignored; 0 measured; 0 filtered out
2020-07-07T19:34:06.0999635Z 
2020-07-07T19:34:06.1034305Z thread 'main' panicked at 'Some t
