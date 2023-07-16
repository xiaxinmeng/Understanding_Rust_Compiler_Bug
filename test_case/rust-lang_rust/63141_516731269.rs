plain
2019-07-31T07:23:27.2422317Z normalized stderr:
2019-07-31T07:23:27.2422474Z error: incorrect use of `await`
2019-07-31T07:23:27.2424233Z   --> $DIR/async-fn.rs:14:13
2019-07-31T07:23:27.2424349Z    |
2019-07-31T07:23:27.2424412Z 14 |     let y = await!(async { *y + *z });
2019-07-31T07:23:27.2426363Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^ help: `await` is a postfix operation: `async { *y + *z }.await`
2019-07-31T07:23:27.2426516Z error[E0557]: feature has been removed
2019-07-31T07:23:27.2426875Z  --> $DIR/async-fn.rs:3:5
2019-07-31T07:23:27.2426961Z   |
2019-07-31T07:23:27.2427019Z 3 |     await_macro,
2019-07-31T07:23:27.2427019Z 3 |     await_macro,
2019-07-31T07:23:27.2427098Z   |     ^^^^^^^^^^^
2019-07-31T07:23:27.2427159Z   |
2019-07-31T07:23:27.2427908Z note: subsumed by `.await` syntax
2019-07-31T07:23:27.2428241Z  --> $DIR/async-fn.rs:3:5
2019-07-31T07:23:27.2428384Z 3 |     await_macro,
2019-07-31T07:23:27.2428462Z   |     ^^^^^^^^^^^
2019-07-31T07:23:27.2428520Z 
2019-07-31T07:23:27.2428596Z error: aborting due to 2 previous errors
---
2019-07-31T07:23:27.2430073Z 
2019-07-31T07:23:27.2430128Z +error: incorrect use of `await`
2019-07-31T07:23:27.2430392Z +  --> $DIR/async-fn.rs:14:13
2019-07-31T07:23:27.2431801Z +   |
2019-07-31T07:23:27.2433029Z +14 |     let y = await!(async { *y + *z });
2019-07-31T07:23:27.2434773Z +   |             ^^^^^^^^^^^^^^^^^^^^^^^^^ help: `await` is a postfix operation: `async { *y + *z }.await`
2019-07-31T07:23:27.2440905Z +error[E0557]: feature has been removed
2019-07-31T07:23:27.2444616Z + --> $DIR/async-fn.rs:3:5
2019-07-31T07:23:27.2447568Z +  |
2019-07-31T07:23:27.2453484Z +3 |     await_macro,
2019-07-31T07:23:27.2453484Z +3 |     await_macro,
2019-07-31T07:23:27.2453599Z +  |     ^^^^^^^^^^^
2019-07-31T07:23:27.2469674Z +  |
2019-07-31T07:23:27.2469785Z +note: subsumed by `.await` syntax
2019-07-31T07:23:27.2470223Z + --> $DIR/async-fn.rs:3:5
2019-07-31T07:23:27.2470314Z +  |
2019-07-31T07:23:27.2470393Z +3 |     await_macro,
2019-07-31T07:23:27.2470475Z +  |     ^^^^^^^^^^^
2019-07-31T07:23:27.2470615Z +error: aborting due to 2 previous errors
2019-07-31T07:23:27.2470682Z +
2019-07-31T07:23:27.2471387Z +For more information about this error, try `rustc --explain E0557`.
2019-07-31T07:23:27.2471494Z +
2019-07-31T07:23:27.2471494Z +
2019-07-31T07:23:27.2471550Z 
2019-07-31T07:23:27.2471619Z The actual stderr differed from the expected stderr.
2019-07-31T07:23:27.2471959Z Actual stderr saved to /tmp/compiletest6HSvn8/async-fn.stderr
2019-07-31T07:23:27.2472048Z To update references, run this command from build directory:
2019-07-31T07:23:27.2472361Z tests/run-pass/update-references.sh '/tmp/compiletest6HSvn8' 'async-fn.rs'
2019-07-31T07:23:27.2472497Z error: 1 errors occurred comparing output.
2019-07-31T07:23:27.2472566Z status: exit code: 1
2019-07-31T07:23:27.2472566Z status: exit code: 1
2019-07-31T07:23:27.2473308Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletest6HSvn8" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest6HSvn8/async-fn.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest6HSvn8/async-fn.stage-id.aux" "-A" "unused"
2019-07-31T07:23:27.2473774Z ------------------------------------------
2019-07-31T07:23:27.2473823Z 
2019-07-31T07:23:27.2474078Z ------------------------------------------
2019-07-31T07:23:27.2474148Z stderr:
2019-07-31T07:23:27.2474148Z stderr:
2019-07-31T07:23:27.2474549Z ------------------------------------------
2019-07-31T07:23:27.2476297Z {"message":"incorrect use of `await`","code":null,"level":"error","spans":[{"file_name":"tests/run-pass/async-fn.rs","byte_start":306,"byte_end":331,"line_start":14,"line_end":14,"column_start":13,"column_end":38,"is_primary":true,"text":[{"text":"    let y = await!(async { *y + *z });","highlight_start":13,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`await` is a postfix operation","code":null,"level":"help","spans":[{"file_name":"tests/run-pass/async-fn.rs","byte_start":306,"byte_end":331,"line_start":14,"line_end":14,"column_start":13,"column_end":38,"is_primary":true,"text":[{"text":"    let y = await!(async { *y + *z });","highlight_start":13,"highlight_end":38}],"label":null,"suggested_replacement":"async { *y + *z }.await","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: incorrect use of `await`\n  --> tests/run-pass/async-fn.rs:14:13\n   |\n14 |     let y = await!(async { *y + *z });\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^ help: `await` is a postfix operation: `async { *y + *z }.await`\n\n"}
2019-07-31T07:23:27.2478605Z {"message":"feature has been removed","code":{"code":"E0557","explanation":"\nA feature attribute named a feature that has been removed.\n\nErroneous code example:\n\n