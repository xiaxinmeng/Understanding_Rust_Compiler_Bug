plain
.......... (50/51)
.          (51/51)


An exception occured: connect ECONNREFUSED 127.0.0.1:36721
== STACKTRACE ==
Error
    at innerRunTestCode (/node-v14.4.0-linux-x64/lib/node_modules/browser-ui-test/src/index.js:442:16)
    at processTicksAndRejections (internal/process/task_queues.js:97:5)
    at async main (/checkout/src/tools/rustdoc-gui/tester.js:223:13)


command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:27
