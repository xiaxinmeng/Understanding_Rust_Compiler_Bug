plain
2020-03-21T19:44:22.2318582Z ========================== Starting Command Output ===========================
2020-03-21T19:44:22.2321361Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cff79fe2-4a16-4d37-8150-8f1f89ddc35e.sh
2020-03-21T19:44:22.2321655Z 
2020-03-21T19:44:22.2325517Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-21T19:44:22.2345142Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70242/merge to s
2020-03-21T19:44:22.2348444Z Task         : Get sources
2020-03-21T19:44:22.2348779Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-21T19:44:22.2349086Z Version      : 1.0.0
2020-03-21T19:44:22.2349295Z Author       : Microsoft
---
2020-03-21T19:44:23.2290200Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-21T19:44:23.2296882Z ##[command]git config gc.auto 0
2020-03-21T19:44:23.2302574Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-21T19:44:23.2307433Z ##[command]git config --get-all http.proxy
2020-03-21T19:44:23.2315332Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70242/merge:refs/remotes/pull/70242/merge
---
2020-03-21T20:40:43.5151194Z .................................................................................................... 1700/9815
2020-03-21T20:40:48.3072520Z .................................................................................................... 1800/9815
2020-03-21T20:40:58.9682784Z .............................................................................i...................... 1900/9815
2020-03-21T20:41:05.3761076Z .................................................................................................... 2000/9815
2020-03-21T20:41:13.0323207Z ...................................................................iiiii............................ 2100/9815
2020-03-21T20:41:32.2985785Z .................................................................................................... 2300/9815
2020-03-21T20:41:34.6186881Z .................................................................................................... 2400/9815
2020-03-21T20:41:37.4361120Z .................................................................................................... 2500/9815
2020-03-21T20:41:54.6595490Z .................................................................................................... 2600/9815
---
2020-03-21T20:44:34.7480661Z .........................................i...............i.......................................... 5000/9815
2020-03-21T20:44:43.5284188Z .................................................................................................... 5100/9815
2020-03-21T20:44:49.7014997Z .....................................................................................i.............. 5200/9815
2020-03-21T20:44:55.3190565Z .................................................................................................... 5300/9815
2020-03-21T20:45:05.1769949Z ...................................................................ii.ii........i...iF....F......... 5400/9815
2020-03-21T20:45:13.0946062Z .......i............................................................................................ 5600/9815
2020-03-21T20:45:22.3375956Z ............i....................................................................................... 5700/9815
2020-03-21T20:45:28.0774419Z .............................ii...................................i................................. 5800/9815
2020-03-21T20:45:34.7437349Z .................................................................................................... 5900/9815
2020-03-21T20:45:34.7437349Z .................................................................................................... 5900/9815
2020-03-21T20:45:41.2153452Z .................................................................................................... 6000/9815
2020-03-21T20:45:50.2343861Z ............................................................ii...i..ii...........i.................. 6100/9815
2020-03-21T20:46:10.1798347Z .................................................................................................... 6300/9815
2020-03-21T20:46:15.0627874Z .................................................................................................... 6400/9815
2020-03-21T20:46:15.0627874Z .................................................................................................... 6400/9815
2020-03-21T20:46:19.3806773Z ..........................................................................................i..ii..... 6500/9815
2020-03-21T20:46:40.3358511Z .................................................................................................... 6700/9815
2020-03-21T20:46:50.5333252Z .........................................................................................i.......... 6800/9815
2020-03-21T20:46:52.5936112Z .................................................................................................... 6900/9815
2020-03-21T20:46:54.7121704Z .................................................................................................... 7000/9815
---
2020-03-21T20:48:36.9067351Z .................................................................................................... 7800/9815
2020-03-21T20:48:41.4222427Z .................................................................................................... 7900/9815
2020-03-21T20:48:47.3675792Z .............................................................................i...................... 8000/9815
2020-03-21T20:48:57.2284752Z .................................................................................................... 8100/9815
2020-03-21T20:49:02.4357493Z ..........................iiiiiiiiii.i.............................................................. 8200/9815
2020-03-21T20:49:16.3090008Z .................................................................................................... 8400/9815
2020-03-21T20:49:21.4938322Z .................................................................................................... 8500/9815
2020-03-21T20:49:36.4207060Z .................................................................................................... 8600/9815
2020-03-21T20:49:43.4153491Z .................................................................................................... 8700/9815
---
2020-03-21T20:51:34.3337250Z 
2020-03-21T20:51:34.3341581Z ---- [ui] ui/json-bom-plus-crlf-multifile.rs stdout ----
2020-03-21T20:51:34.3341997Z diff of stderr:
2020-03-21T20:51:34.3342133Z 
2020-03-21T20:51:34.3342420Z 15 variable. It can happen in several cases, the most common being a mismatch
2020-03-21T20:51:34.3342821Z 16 between the type that the compiler inferred for a variable based on its
2020-03-21T20:51:34.3343229Z 17 initializing expression, on the one hand, and the type the author explicitly
2020-03-21T20:51:34.3343808Z - assigned to the variable, on the other hand."
2020-03-21T20:51:34.3350706Z - },"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":621,"byte_end":622,"line_start":17,"line_end":17,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":22,"highlight_end":23}],"label":"expected struct `std::string::String`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":612,"byte_end":618,"line_start":17,"line_end":17,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using a conversion method","code":null,"level":"help","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":621,"byte_end":622,"line_start":17,"line_end":17,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":22,"highlight_end":23}],"label":null,"suggested_replacement":"1.to_string()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf-multifile-aux.rs:17:22: error[E0308]: mismatched types
2020-03-21T20:51:34.3355801Z + assigned to the variable, on the other hand.
2020-03-21T20:51:34.3364029Z + "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":621,"byte_end":622,"line_start":17,"line_end":17,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":22,"highlight_end":23}],"label":"expected struct `std::string::String`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":612,"byte_end":618,"line_start":17,"line_end":17,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using a conversion method","code":null,"level":"help","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":621,"byte_end":622,"line_start":17,"line_end":17,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":22,"highlight_end":23}],"label":null,"suggested_replacement":"1.to_string()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf-multifile-aux.rs:17:22: error[E0308]: mismatched types
2020-03-21T20:51:34.3369134Z 20 "}
2020-03-21T20:51:34.3369526Z 21 {"message":"mismatched types","code":{"code":"E0308","explanation":"Expected type did not match the received type.
2020-03-21T20:51:34.3370037Z 
2020-03-21T20:51:34.3370189Z 31 //      |
2020-03-21T20:51:34.3370424Z 32 //    type `i32` assigned to variable `x`
2020-03-21T20:51:34.3370653Z 33 