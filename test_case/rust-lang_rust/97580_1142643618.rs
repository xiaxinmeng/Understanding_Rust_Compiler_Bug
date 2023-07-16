`
022-05-31T21:04:41.2826600Z test [ui (nll)] src/test/ui/xcrate/xcrate-unit-struct-2.rs ... [32mok(B[m
2022-05-31T21:04:41.3229451Z test [ui (nll)] src/test/ui/zero-sized/zero-sized-tuple-struct.rs ... [32mok(B[m
2022-05-31T21:04:41.3781260Z test [ui (nll)] src/test/ui/write-fmt-errors.rs ... [32mok(B[m
2022-05-31T21:04:41.3950080Z test [ui (nll)] src/test/ui/zero-sized/zero-sized-linkedlist-push.rs ... [32mok(B[m
2022-05-31T21:04:41.4206508Z test [ui (nll)] src/test/ui/wait-forked-but-failed-child.rs ... [32mok(B[m
2022-05-31T21:04:41.4426811Z test [ui (nll)] src/test/ui/zero-sized/zero-sized-binary-heap-push.rs ... [32mok(B[m
2022-05-31T21:04:41.7266802Z test [ui (nll)] src/test/ui/zero-sized/zero-sized-btreemap-insert.rs ... [32mok(B[m
2022-05-31T21:04:42.7832003Z test [ui (nll)] src/test/ui/wrong-hashset-issue-42918.rs ... [32mok(B[m
2022-05-31T21:04:42.8037332Z 
2022-05-31T21:04:42.8038033Z failures:
2022-05-31T21:04:42.8051195Z 
2022-05-31T21:04:42.8052059Z ---- [ui (nll)] src/test/ui/borrowck/issue-71546.rs stdout ----
2022-05-31T21:04:42.8052443Z diff of stderr:
2022-05-31T21:04:42.8052615Z 
2022-05-31T21:04:42.8053622Z -	error[E0310]: the associated type `<&'a V as IntoIterator>::Item` may not live long enough
2022-05-31T21:04:42.8054162Z +	error: higher-ranked lifetime error
2022-05-31T21:04:42.8054562Z 2	  --> $DIR/issue-71546.rs:9:27
2022-05-31T21:04:42.8054850Z 3	   |
2022-05-31T21:04:42.8055172Z 4	LL |       let csv_str: String = value
2022-05-31T21:04:42.8055377Z 
2022-05-31T21:04:42.8055538Z 7	LL | |         .map(|elem| elem.to_string())
2022-05-31T21:04:42.8055872Z 8	   | |_____________________________________^
2022-05-31T21:04:42.8056164Z 9	   |
2022-05-31T21:04:42.8056778Z -	   = help: consider adding an explicit lifetime bound `<&'a V as IntoIterator>::Item: 'static`...
2022-05-31T21:04:42.8057524Z -	   = note: ...so that the type `<&'a V as IntoIterator>::Item` will meet its required lifetime bounds...
2022-05-31T21:04:42.8058054Z -	note: ...that is required by this bound
2022-05-31T21:04:42.8058447Z -	  --> $DIR/issue-71546.rs:7:55
2022-05-31T21:04:42.8058978Z +	   = note: could not prove for<'r> [closure@$DIR/issue-71546.rs:11:14: 11:37] well-formed
2022-05-31T21:04:42.8059353Z +	
2022-05-31T21:04:42.8059700Z +	error: higher-ranked lifetime error
2022-05-31T21:04:42.8060092Z +	  --> $DIR/issue-71546.rs:9:27
2022-05-31T21:04:42.8060390Z 14	   |
2022-05-31T21:04:42.8061186Z -	LL |     for<'a> <&'a V as IntoIterator>::Item: ToString + 'static,
2022-05-31T21:04:42.8061733Z -	   |                                                       ^^^^^^^
2022-05-31T21:04:42.8062078Z +	LL |       let csv_str: String = value
2022-05-31T21:04:42.8062387Z +	   |  ___________________________^
2022-05-31T21:04:42.8062683Z +	LL | |         .into_iter()
2022-05-31T21:04:42.8063029Z +	LL | |         .map(|elem| elem.to_string())
2022-05-31T21:04:42.8063359Z +	   | |_____________________________________^
2022-05-31T21:04:42.8063643Z +	   |
2022-05-31T21:04:42.8064318Z +	   = note: could not prove for<'r, 's> Map<<&'r V as IntoIterator>::IntoIter, [closure@$DIR/issue-71546.rs:11:14: 11:37]> well-formed
2022-05-31T21:04:42.8064730Z 17	
2022-05-31T21:04:42.8065078Z -	error: aborting due to previous error
2022-05-31T21:04:42.8065509Z +	error: higher-ranked lifetime error
2022-05-31T21:04:42.8065902Z +	  --> $DIR/issue-71546.rs:9:27
2022-05-31T21:04:42.8066461Z +	   |
2022-05-31T21:04:42.8066722Z +	LL |       let csv_str: String = value
2022-05-31T21:04:42.8066996Z +	   |  ___________________________^
2022-05-31T21:04:42.8067252Z +	LL | |         .into_iter()
2022-05-31T21:04:42.8067558Z +	LL | |         .map(|elem| elem.to_string())
2022-05-31T21:04:42.8067985Z +	LL | |         .collect::<String>();
2022-05-31T21:04:42.8068267Z +	   | |____________________________^
2022-05-31T21:04:42.8068519Z +	   |
2022-05-31T21:04:42.8069308Z +	   = note: could not prove for<'r, 's> Map<<&'r V as IntoIterator>::IntoIter, [closure@$DIR/issue-71546.rs:11:14: 11:37]> well-formed
2022-05-31T21:04:42.8069703Z 19	
2022-05-31T21:04:42.8070131Z -	For more information about this error, try `rustc --explain E0310`.
2022-05-31T21:04:42.8070594Z +	error: higher-ranked lifetime error
2022-05-31T21:04:42.8070975Z +	  --> $DIR/issue-71546.rs:11:14
2022-05-31T21:04:42.8071247Z +	   |
2022-05-31T21:04:42.8071538Z +	LL |         .map(|elem| elem.to_string())
2022-05-31T21:04:42.8071858Z +	   |              ^^^^^^^^^^^^^^^^^^^^^^^
2022-05-31T21:04:42.8072123Z +	   |
2022-05-31T21:04:42.8072624Z +	   = note: could not prove for<'a> <&'a V as IntoIterator>::Item: 'static
2022-05-31T21:04:42.8072964Z +	
2022-05-31T21:04:42.8073413Z +	error: higher-ranked lifetime error
2022-05-31T21:04:42.8073782Z +	  --> $DIR/issue-71546.rs:12:10
2022-05-31T21:04:42.8074057Z +	   |
2022-05-31T21:04:42.8074439Z +	LL |         .collect::<String>();
2022-05-31T21:04:42.8074907Z +	   |          ^^^^^^^
2022-05-31T21:04:42.8075178Z +	
2022-05-31T21:04:42.8075624Z +	error: aborting due to 5 previous errors
2022-05-31T21:04:42.8075925Z +	
2022-05-31T21:04:42.8076166Z 21	
2022-05-31T21:04:42.8076309Z 
2022-05-31T21:04:42.8076315Z 
2022-05-31T21:04:42.8076503Z The actual stderr differed from the expected stderr.
2022-05-31T21:04:42.8077204Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-71546.nll/issue-71546.nll.stderr
2022-05-31T21:04:42.8078055Z To update references, rerun the tests and pass the `--bless` flag
2022-05-31T21:04:42.8078590Z To only update this specific test, also pass `--test-args borrowck/issue-71546.rs`
2022-05-31T21:04:42.8078845Z 
2022-05-31T21:04:42.8078984Z error: 1 errors occurred comparing output.
2022-05-31T21:04:42.8079287Z status: exit status: 1
2022-05-31T21:04:42.8080906Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-71546.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-71546.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-71546.nll/auxiliary"
2022-05-31T21:04:42.8082465Z stdout: none
2022-05-31T21:04:42.8082858Z --- stderr -------------------------------
2022-05-31T21:04:42.8083370Z error: higher-ranked lifetime error
2022-05-31T21:04:42.8083889Z   --> /checkout/src/test/ui/borrowck/issue-71546.rs:9:27
2022-05-31T21:04:42.8084214Z    |
2022-05-31T21:04:42.8084915Z LL |       let csv_str: String = value //~ ERROR: the associated type `<&'a V as IntoIterator>::Item` may not live long enough
2022-05-31T21:04:42.8085373Z    |  ___________________________^
2022-05-31T21:04:42.8085667Z LL | |         .into_iter()
2022-05-31T21:04:42.8085997Z LL | |         .map(|elem| elem.to_string())
2022-05-31T21:04:42.8086342Z    | |_____________________________________^
2022-05-31T21:04:42.8086616Z    |
2022-05-31T21:04:42.8087311Z    = note: could not prove for<'r> [closure@/checkout/src/test/ui/borrowck/issue-71546.rs:11:14: 11:37] well-formed
2022-05-31T21:04:42.8087625Z 
2022-05-31T21:04:42.8087860Z error: higher-ranked lifetime error
2022-05-31T21:04:42.8088332Z   --> /checkout/src/test/ui/borrowck/issue-71546.rs:9:27
2022-05-31T21:04:42.8088656Z    |
2022-05-31T21:04:42.8089320Z LL |       let csv_str: String = value //~ ERROR: the associated type `<&'a V as IntoIterator>::Item` may not live long enough
2022-05-31T21:04:42.8089759Z    |  ___________________________^
2022-05-31T21:04:42.8090048Z LL | |         .into_iter()
2022-05-31T21:04:42.8090373Z LL | |         .map(|elem| elem.to_string())
2022-05-31T21:04:42.8090927Z    | |_____________________________________^
2022-05-31T21:04:42.8091161Z    |
2022-05-31T21:04:42.8091783Z    = note: could not prove for<'r, 's> Map<<&'r V as IntoIterator>::IntoIter, [closure@/checkout/src/test/ui/borrowck/issue-71546.rs:11:14: 11:37]> well-formed
2022-05-31T21:04:42.8092109Z 
2022-05-31T21:04:42.8092293Z error: higher-ranked lifetime error
2022-05-31T21:04:42.8092706Z   --> /checkout/src/test/ui/borrowck/issue-71546.rs:9:27
2022-05-31T21:04:42.8093001Z    |
2022-05-31T21:04:42.8093554Z LL |       let csv_str: String = value //~ ERROR: the associated type `<&'a V as IntoIterator>::Item` may not live long enough
2022-05-31T21:04:42.8093939Z    |  ___________________________^
2022-05-31T21:04:42.8094197Z LL | |         .into_iter()
2022-05-31T21:04:42.8094501Z LL | |         .map(|elem| elem.to_string())
2022-05-31T21:04:42.8094799Z LL | |         .collect::<String>();
2022-05-31T21:04:42.8095068Z    | |____________________________^
2022-05-31T21:04:42.8095667Z Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
2022-05-31T21:04:42.8096189Z    |
2022-05-31T21:04:42.8096829Z    = note: could not prove for<'r, 's> Map<<&'r V as IntoIterator>::IntoIter, [closure@/checkout/src/test/ui/borrowck/issue-71546.rs:11:14: 11:37]> well-formed
2022-05-31T21:04:42.8097134Z 
2022-05-31T21:04:42.8097318Z error: higher-ranked lifetime error
2022-05-31T21:04:42.8097732Z   --> /checkout/src/test/ui/borrowck/issue-71546.rs:11:14
2022-05-31T21:04:42.8098030Z    |
2022-05-31T21:04:42.8098282Z LL |         .map(|elem| elem.to_string())
2022-05-31T21:04:42.8098564Z    |              ^^^^^^^^^^^^^^^^^^^^^^^
2022-05-31T21:04:42.8098806Z    |
2022-05-31T21:04:42.8099254Z    = note: could not prove for<'a> <&'a V as IntoIterator>::Item: 'static
2022-05-31T21:04:42.8099479Z 
2022-05-31T21:04:42.8099661Z error: higher-ranked lifetime error
2022-05-31T21:04:42.8100070Z   --> /checkout/src/test/ui/borrowck/issue-71546.rs:12:10
2022-05-31T21:04:42.8100369Z    |
2022-05-31T21:04:42.8100605Z LL |         .collect::<String>();
2022-05-31T21:04:42.8100865Z    |          ^^^^^^^
2022-05-31T21:04:42.8101026Z 
2022-05-31T21:04:42.8101162Z error: aborting due to 5 previous errors
2022-05-31T21:04:42.8101539Z ------------------------------------------
2022-05-31T21:04:42.8101725Z 
2022-05-31T21:04:42.8101729Z 
2022-05-31T21:04:42.8101734Z 
2022-05-31T21:04:42.8101829Z failures:
2022-05-31T21:04:42.8102186Z     [ui (nll)] src/test/ui/borrowck/issue-71546.rs
2022-05-31T21:04:42.8102382Z 
2022-05-31T21:04:42.8102744Z test result: [31mFAILED(B[m. 12662 passed; 1 failed; 635 ignored; 0 measured; 0 filtered out; finished in 112.14s
2022-05-31T21:04:42.8103100Z 
2022-05-31T21:04:42.8139772Z Build completed unsuccessfully in 0:19:37
2022-05-31T21:04:42.8217569Z == clock drift check ==
2022-05-31T21:04:42.8226676Z   local time: Tue May 31 21:04:42 UTC 2022
2022-05-31T21:04:42.8778915Z   network time: Tue, 31 May 2022 21:04:42 GMT
2022-05-31T21:04:42.8779733Z == end clock drift check ==
2022-05-31T21:04:43.6188063Z ##[error]Process completed with exit code 1.
2022-05-31T21:04:43.6283221Z Post job cleanup.
2022-05-31T21:04:43.7559487Z [command]/usr/bin/git version
