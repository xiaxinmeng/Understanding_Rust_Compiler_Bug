plain
2020-02-09T06:01:42.1111749Z ========================== Starting Command Output ===========================
2020-02-09T06:01:42.1126956Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d7031850-5a25-4ab9-a0a0-27190b57bfee.sh
2020-02-09T06:01:42.4162988Z 
2020-02-09T06:01:42.4227332Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-09T06:01:42.4234553Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68985/merge to s
2020-02-09T06:01:42.4236015Z Task         : Get sources
2020-02-09T06:01:42.4236043Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T06:01:42.4236070Z Version      : 1.0.0
2020-02-09T06:01:42.4236097Z Author       : Microsoft
---
2020-02-09T06:01:46.2602167Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-09T06:01:46.2875921Z ##[command]git config gc.auto 0
2020-02-09T06:01:46.2959582Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-09T06:01:46.3012754Z ##[command]git config --get-all http.proxy
2020-02-09T06:01:46.3182505Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68985/merge:refs/remotes/pull/68985/merge
---
2020-02-09T06:58:12.6336672Z .................................................................................................... 1700/9613
2020-02-09T06:58:17.4088387Z .................................................................................................... 1800/9613
2020-02-09T06:58:28.7228519Z ...............................i.................................................................... 1900/9613
2020-02-09T06:58:36.0934016Z .................................................................................................... 2000/9613
2020-02-09T06:58:49.6402700Z .....................iiiii.......................................................................... 2100/9613
2020-02-09T06:58:59.1067472Z .................................................................................................... 2300/9613
2020-02-09T06:59:01.5471761Z .................................................................................................... 2400/9613
2020-02-09T06:59:06.1533807Z .................................................................................................... 2500/9613
2020-02-09T06:59:26.1665317Z .................................................................................................... 2600/9613
---
2020-02-09T07:01:56.3234078Z .........................................................................i...............i.......... 4900/9613
2020-02-09T07:02:03.9285922Z .................................................................................................... 5000/9613
2020-02-09T07:02:11.9834072Z .................................................................................................... 5100/9613
2020-02-09T07:02:16.8413685Z ................i................................................................................... 5200/9613
2020-02-09T07:02:27.5859123Z ..........................................................................................ii.ii..... 5300/9613
2020-02-09T07:02:31.8823566Z ...i...i............................................................................................ 5400/9613
2020-02-09T07:02:41.8878393Z .................................................................................................... 5600/9613
2020-02-09T07:02:52.5615642Z ..............................................................................i..................... 5700/9613
2020-02-09T07:03:00.3625917Z .................................................................................................... 5800/9613
2020-02-09T07:03:06.6614516Z ...........................................................................................F........ 5900/9613
2020-02-09T07:03:06.6614516Z ...........................................................................................F........ 5900/9613
2020-02-09T07:03:16.9906223Z ......................................................................ii...i..ii...........i........ 6000/9613
2020-02-09T07:03:39.0361383Z .................................................................................................... 6200/9613
2020-02-09T07:03:46.8697844Z .................................................................................................... 6300/9613
2020-02-09T07:03:54.1643381Z ..........................................F................F......................................i. 6400/9613
2020-02-09T07:03:54.1643381Z ..........................................F................F......................................i. 6400/9613
2020-02-09T07:04:07.5910895Z .ii................................................................................................. 6500/9613
2020-02-09T07:04:27.2434337Z .....................................................................................i.............. 6700/9613
2020-02-09T07:04:29.4475275Z .................................................................................................... 6800/9613
2020-02-09T07:04:31.6472412Z .............................................................................................i...... 6900/9613
2020-02-09T07:04:34.3033078Z .................................................................................................... 7000/9613
---
2020-02-09T07:06:13.1688122Z .................................................................................................... 7600/9613
2020-02-09T07:06:18.1894487Z .................................................................................................... 7700/9613
2020-02-09T07:06:23.3173581Z .................................................................................................... 7800/9613
2020-02-09T07:06:32.6938483Z .................................................................................................... 7900/9613
2020-02-09T07:06:42.0459628Z .........................................................iiiiiii.i.................................. 8000/9613
2020-02-09T07:06:50.8639806Z .................................................................................................i.. 8100/9613
2020-02-09T07:07:02.3825869Z .................................................................................................... 8300/9613
2020-02-09T07:07:17.6044577Z .................................................................................................... 8400/9613
2020-02-09T07:07:25.9705105Z .................................................................................................... 8500/9613
2020-02-09T07:07:33.8670812Z ..................................................................F...F............................. 8600/9613
2020-02-09T07:07:33.8670812Z ..................................................................F...F............................. 8600/9613
2020-02-09T07:08:11.7774155Z .................................................................................................... 8700/9613
2020-02-09T07:08:20.3918624Z .................................................................................................... 8800/9613
2020-02-09T07:08:27.9617292Z .................................................................................................... 8900/9613
2020-02-09T07:08:34.6130220Z .................................................................................................... 9000/9613
2020-02-09T07:08:39.7367880Z ............................................F.................FFFF......FFF......................... 9100/9613
2020-02-09T07:08:54.3775246Z .................................................................................................... 9300/9613
2020-02-09T07:09:03.4190528Z .................................................................................................... 9400/9613
2020-02-09T07:09:10.6697669Z .................................................................................................... 9500/9613
2020-02-09T07:09:19.0376761Z .................i.................................................................................. 9600/9613
---
2020-02-09T07:09:26.4635839Z 
2020-02-09T07:09:26.4636073Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4636797Z +   --> $DIR/coerce-expect-unsized-ascribed.rs:9:13
2020-02-09T07:09:26.4637124Z +    |
2020-02-09T07:09:26.4637361Z + LL |     let _ = box { [1, 2, 3] }: Box<[i32]>;
2020-02-09T07:09:26.4637598Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(box { [1, 2, 3] }: Box<[i32]>)`
2020-02-09T07:09:26.4638398Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4638951Z +   --> $DIR/coerce-expect-unsized-ascribed.rs:10:13
2020-02-09T07:09:26.4639251Z +    |
2020-02-09T07:09:26.4639251Z +    |
2020-02-09T07:09:26.4639487Z + LL |     let _ = box if true { [1, 2, 3] } else { [1, 3, 4] }: Box<[i32]>;
2020-02-09T07:09:26.4639922Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(box if true { [1, 2, 3] } else { [1, 3, 4] }: Box<[i32]>)`
2020-02-09T07:09:26.4642025Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4642636Z +   --> $DIR/coerce-expect-unsized-ascribed.rs:11:13
2020-02-09T07:09:26.4643295Z +    |
2020-02-09T07:09:26.4643295Z +    |
2020-02-09T07:09:26.4643731Z + LL |     let _ = box match true { true => [1, 2, 3], false => [1, 3, 4] }: Box<[i32]>;
2020-02-09T07:09:26.4644005Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(box match true { true => [1, 2, 3], false => [1, 3, 4] }: Box<[i32]>)`
2020-02-09T07:09:26.4644525Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4645008Z +   --> $DIR/coerce-expect-unsized-ascribed.rs:13:13
2020-02-09T07:09:26.4645328Z +    |
2020-02-09T07:09:26.4645328Z +    |
2020-02-09T07:09:26.4645839Z + LL |     let _ = box { |x| (x as u8) }: Box<dyn Fn(i32) -> _>;
2020-02-09T07:09:26.4646494Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(box { |x| (x as u8) }: Box<dyn Fn(i32) -> _>)`
2020-02-09T07:09:26.4648068Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4663200Z +   --> $DIR/coerce-expect-unsized-ascribed.rs:14:13
2020-02-09T07:09:26.4663586Z +    |
2020-02-09T07:09:26.4663586Z +    |
2020-02-09T07:09:26.4663835Z + LL |     let _ = box if true { false } else { true }: Box<dyn Debug>;
2020-02-09T07:09:26.4664129Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(box if true { false } else { true }: Box<dyn Debug>)`
2020-02-09T07:09:26.4670446Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4675677Z +   --> $DIR/coerce-expect-unsized-ascribed.rs:15:13
2020-02-09T07:09:26.4676781Z +    |
2020-02-09T07:09:26.4676781Z +    |
2020-02-09T07:09:26.4677431Z + LL |     let _ = box match true { true => 'a', false => 'b' }: Box<dyn Debug>;
2020-02-09T07:09:26.4684352Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(box match true { true => 'a', false => 'b' }: Box<dyn Debug>)`
2020-02-09T07:09:26.4684480Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4684755Z +   --> $DIR/coerce-expect-unsized-ascribed.rs:17:13
2020-02-09T07:09:26.4684809Z +    |
2020-02-09T07:09:26.4684809Z +    |
2020-02-09T07:09:26.4684871Z + LL |     let _ = &{ [1, 2, 3] }: &[i32];
2020-02-09T07:09:26.4684951Z +    |             ^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(&{ [1, 2, 3] }: &[i32])`
2020-02-09T07:09:26.4685052Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4685314Z +   --> $DIR/coerce-expect-unsized-ascribed.rs:18:13
2020-02-09T07:09:26.4685373Z +    |
2020-02-09T07:09:26.4685373Z +    |
2020-02-09T07:09:26.4685423Z + LL |     let _ = &if true { [1, 2, 3] } else { [1, 3, 4] }: &[i32];
2020-02-09T07:09:26.4685510Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(&if true { [1, 2, 3] } else { [1, 3, 4] }: &[i32])`
2020-02-09T07:09:26.4685615Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4685877Z +   --> $DIR/coerce-expect-unsized-ascribed.rs:19:13
2020-02-09T07:09:26.4685925Z +    |
2020-02-09T07:09:26.4685925Z +    |
2020-02-09T07:09:26.4686148Z + LL |     let _ = &match true { true => [1, 2, 3], false => [1, 3, 4] }: &[i32];
2020-02-09T07:09:26.4686254Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(&match true { true => [1, 2, 3], false => [1, 3, 4] }: &[i32])`
2020-02-09T07:09:26.4686356Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4686742Z +   --> $DIR/coerce-expect-unsized-ascribed.rs:21:13
2020-02-09T07:09:26.4686792Z +    |
2020-02-09T07:09:26.4686792Z +    |
2020-02-09T07:09:26.4687026Z + LL |     let _ = &{ |x| (x as u8) }: &dyn Fn(i32) -> _;
2020-02-09T07:09:26.4687393Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(&{ |x| (x as u8) }: &dyn Fn(i32) -> _)`
2020-02-09T07:09:26.4687496Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4687752Z +   --> $DIR/coerce-expect-unsized-ascribed.rs:22:13
2020-02-09T07:09:26.4687800Z +    |
2020-02-09T07:09:26.4687800Z +    |
2020-02-09T07:09:26.4687860Z + LL |     let _ = &if true { false } else { true }: &dyn Debug;
2020-02-09T07:09:26.4687946Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(&if true { false } else { true }: &dyn Debug)`
2020-02-09T07:09:26.4688046Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4688343Z +   --> $DIR/coerce-expect-unsized-ascribed.rs:23:13
2020-02-09T07:09:26.4688393Z +    |
2020-02-09T07:09:26.4688393Z +    |
2020-02-09T07:09:26.4688642Z + LL |     let _ = &match true { true => 'a', false => 'b' }: &dyn Debug;
2020-02-09T07:09:26.4689029Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(&match true { true => 'a', false => 'b' }: &dyn Debug)`
2020-02-09T07:09:26.4689135Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4689485Z +   --> $DIR/coerce-expect-unsized-ascribed.rs:25:13
2020-02-09T07:09:26.4689536Z +    |
2020-02-09T07:09:26.4689536Z +    |
2020-02-09T07:09:26.4689582Z + LL |     let _ = Box::new([1, 2, 3]): Box<[i32]>;
2020-02-09T07:09:26.4689661Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(Box::new([1, 2, 3]): Box<[i32]>)`
2020-02-09T07:09:26.4689761Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4690010Z +   --> $DIR/coerce-expect-unsized-ascribed.rs:26:13
2020-02-09T07:09:26.4690139Z +    |
2020-02-09T07:09:26.4690139Z +    |
2020-02-09T07:09:26.4690381Z + LL |     let _ = Box::new(|x| (x as u8)): Box<dyn Fn(i32) -> _>;
2020-02-09T07:09:26.4690737Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(Box::new(|x| (x as u8)): Box<dyn Fn(i32) -> _>)`
2020-02-09T07:09:26.4690860Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4691102Z +   --> $DIR/coerce-expect-unsized-ascribed.rs:28:13
2020-02-09T07:09:26.4691171Z +    |
2020-02-09T07:09:26.4691215Z + LL |       let _ = vec![
2020-02-09T07:09:26.4691215Z + LL |       let _ = vec![
2020-02-09T07:09:26.4691261Z +    |  _____________^
2020-02-09T07:09:26.4691331Z + LL | |         Box::new(|x| (x as u8)),
2020-02-09T07:09:26.4691380Z + LL | |         box |x| (x as i16 as u8),
2020-02-09T07:09:26.4691661Z + LL | |     ]: Vec<Box<dyn Fn(i32) -> _>>;
2020-02-09T07:09:26.4691786Z +    |
2020-02-09T07:09:26.4691786Z +    |
2020-02-09T07:09:26.4691832Z + help: try surrounding the expression with parentheses
2020-02-09T07:09:26.4691939Z + LL |     let _ = (vec![
2020-02-09T07:09:26.4691939Z + LL |     let _ = (vec![
2020-02-09T07:09:26.4691985Z + LL |         Box::new(|x| (x as u8)),
2020-02-09T07:09:26.4692032Z + LL |         box |x| (x as i16 as u8),
2020-02-09T07:09:26.4692278Z + LL |     ]: Vec<Box<dyn Fn(i32) -> _>>);
2020-02-09T07:09:26.4692366Z + 
2020-02-09T07:09:26.4692429Z 1 error[E0308]: mismatched types
2020-02-09T07:09:26.4692754Z 2   --> $DIR/coerce-expect-unsized-ascribed.rs:9:13
2020-02-09T07:09:26.4692813Z 3    |
2020-02-09T07:09:26.4692813Z 3    |
2020-02-09T07:09:26.4692844Z 
2020-02-09T07:09:26.4693153Z 124    = note: expected struct `std::boxed::Box<dyn std::ops::Fn(i32) -> _>`
2020-02-09T07:09:26.4693468Z 125               found struct `std::boxed::Box<[closure@$DIR/coerce-expect-unsized-ascribed.rs:26:22: 26:35]>`
2020-02-09T07:09:26.4693891Z - error: aborting due to 14 previous errors
2020-02-09T07:09:26.4693945Z + error: aborting due to 29 previous errors
2020-02-09T07:09:26.4693989Z 128 
2020-02-09T07:09:26.4694239Z 129 For more information about this error, try `rustc --explain E0308`.
2020-02-09T07:09:26.4694239Z 129 For more information about this error, try `rustc --explain E0308`.
2020-02-09T07:09:26.4694308Z 130 
2020-02-09T07:09:26.4694337Z 
2020-02-09T07:09:26.4694365Z 
2020-02-09T07:09:26.4694411Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.4694778Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-expect-unsized-ascribed/coerce-expect-unsized-ascribed.stderr
2020-02-09T07:09:26.4695036Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.4695433Z To only update this specific test, also pass `--test-args coercion/coerce-expect-unsized-ascribed.rs`
2020-02-09T07:09:26.4695527Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.4695585Z status: exit code: 1
2020-02-09T07:09:26.4695585Z status: exit code: 1
2020-02-09T07:09:26.4696512Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-expect-unsized-ascribed" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-expect-unsized-ascribed/auxiliary" "-A" "unused"
2020-02-09T07:09:26.4696885Z ------------------------------------------
2020-02-09T07:09:26.4696925Z 
2020-02-09T07:09:26.4697165Z ------------------------------------------
2020-02-09T07:09:26.4697267Z stderr:
2020-02-09T07:09:26.4697267Z stderr:
2020-02-09T07:09:26.4697505Z ------------------------------------------
2020-02-09T07:09:26.4697573Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4697877Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:9:13
2020-02-09T07:09:26.4697935Z    |
2020-02-09T07:09:26.4697989Z LL |     let _ = box { [1, 2, 3] }: Box<[i32]>; //~ ERROR mismatched types
2020-02-09T07:09:26.4698074Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(box { [1, 2, 3] }: Box<[i32]>)`
2020-02-09T07:09:26.4698169Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4698482Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:10:13
2020-02-09T07:09:26.4698538Z    |
2020-02-09T07:09:26.4698538Z    |
2020-02-09T07:09:26.4698593Z LL |     let _ = box if true { [1, 2, 3] } else { [1, 3, 4] }: Box<[i32]>; //~ ERROR mismatched types
2020-02-09T07:09:26.4698685Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(box if true { [1, 2, 3] } else { [1, 3, 4] }: Box<[i32]>)`
2020-02-09T07:09:26.4698789Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4699125Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:11:13
2020-02-09T07:09:26.4699183Z    |
2020-02-09T07:09:26.4699183Z    |
2020-02-09T07:09:26.4699238Z LL |     let _ = box match true { true => [1, 2, 3], false => [1, 3, 4] }: Box<[i32]>;
2020-02-09T07:09:26.4699424Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(box match true { true => [1, 2, 3], false => [1, 3, 4] }: Box<[i32]>)`
2020-02-09T07:09:26.4699531Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4699866Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:13:13
2020-02-09T07:09:26.4699919Z    |
2020-02-09T07:09:26.4699919Z    |
2020-02-09T07:09:26.4700209Z LL |     let _ = box { |x| (x as u8) }: Box<dyn Fn(i32) -> _>; //~ ERROR mismatched types
2020-02-09T07:09:26.4700725Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(box { |x| (x as u8) }: Box<dyn Fn(i32) -> _>)`
2020-02-09T07:09:26.4700825Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4701110Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:14:13
2020-02-09T07:09:26.4701183Z    |
2020-02-09T07:09:26.4701183Z    |
2020-02-09T07:09:26.4701238Z LL |     let _ = box if true { false } else { true }: Box<dyn Debug>; //~ ERROR mismatched types
2020-02-09T07:09:26.4701319Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(box if true { false } else { true }: Box<dyn Debug>)`
2020-02-09T07:09:26.4701431Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4701713Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:15:13
2020-02-09T07:09:26.4701794Z    |
2020-02-09T07:09:26.4701794Z    |
2020-02-09T07:09:26.4702101Z LL |     let _ = box match true { true => 'a', false => 'b' }: Box<dyn Debug>; //~ ERROR mismatched types
2020-02-09T07:09:26.4702505Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(box match true { true => 'a', false => 'b' }: Box<dyn Debug>)`
2020-02-09T07:09:26.4702625Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4702906Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:17:13
2020-02-09T07:09:26.4702987Z    |
2020-02-09T07:09:26.4702987Z    |
2020-02-09T07:09:26.4703040Z LL |     let _ = &{ [1, 2, 3] }: &[i32]; //~ ERROR mismatched types
2020-02-09T07:09:26.4703259Z    |             ^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(&{ [1, 2, 3] }: &[i32])`
2020-02-09T07:09:26.4703360Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4703806Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:18:13
2020-02-09T07:09:26.4703857Z    |
2020-02-09T07:09:26.4703857Z    |
2020-02-09T07:09:26.4703929Z LL |     let _ = &if true { [1, 2, 3] } else { [1, 3, 4] }: &[i32]; //~ ERROR mismatched types
2020-02-09T07:09:26.4703998Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(&if true { [1, 2, 3] } else { [1, 3, 4] }: &[i32])`
2020-02-09T07:09:26.4704104Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4704375Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:19:13
2020-02-09T07:09:26.4704425Z    |
2020-02-09T07:09:26.4704425Z    |
2020-02-09T07:09:26.4704493Z LL |     let _ = &match true { true => [1, 2, 3], false => [1, 3, 4] }: &[i32];
2020-02-09T07:09:26.4704564Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(&match true { true => [1, 2, 3], false => [1, 3, 4] }: &[i32])`
2020-02-09T07:09:26.4704662Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4704992Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:21:13
2020-02-09T07:09:26.4705042Z    |
2020-02-09T07:09:26.4705042Z    |
2020-02-09T07:09:26.4705301Z LL |     let _ = &{ |x| (x as u8) }: &dyn Fn(i32) -> _; //~ ERROR mismatched types
2020-02-09T07:09:26.4705665Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(&{ |x| (x as u8) }: &dyn Fn(i32) -> _)`
2020-02-09T07:09:26.4705844Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4706159Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:22:13
2020-02-09T07:09:26.4706210Z    |
2020-02-09T07:09:26.4706210Z    |
2020-02-09T07:09:26.4706260Z LL |     let _ = &if true { false } else { true }: &dyn Debug; //~ ERROR mismatched types
2020-02-09T07:09:26.4706347Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(&if true { false } else { true }: &dyn Debug)`
2020-02-09T07:09:26.4706527Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4706819Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:23:13
2020-02-09T07:09:26.4707630Z    |
2020-02-09T07:09:26.4707630Z    |
2020-02-09T07:09:26.4708504Z LL |     let _ = &match true { true => 'a', false => 'b' }: &dyn Debug; //~ ERROR mismatched types
2020-02-09T07:09:26.4709855Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(&match true { true => 'a', false => 'b' }: &dyn Debug)`
2020-02-09T07:09:26.4709985Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4710292Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:25:13
2020-02-09T07:09:26.4710365Z    |
2020-02-09T07:09:26.4710365Z    |
2020-02-09T07:09:26.4710424Z LL |     let _ = Box::new([1, 2, 3]): Box<[i32]>; //~ ERROR mismatched types
2020-02-09T07:09:26.4710670Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(Box::new([1, 2, 3]): Box<[i32]>)`
2020-02-09T07:09:26.4710782Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4711461Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:26:13
2020-02-09T07:09:26.4711539Z    |
2020-02-09T07:09:26.4711539Z    |
2020-02-09T07:09:26.4711812Z LL |     let _ = Box::new(|x| (x as u8)): Box<dyn Fn(i32) -> _>; //~ ERROR mismatched types
2020-02-09T07:09:26.4712179Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(Box::new(|x| (x as u8)): Box<dyn Fn(i32) -> _>)`
2020-02-09T07:09:26.4712293Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4712557Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:28:13
2020-02-09T07:09:26.4712618Z    |
2020-02-09T07:09:26.4712680Z LL |       let _ = vec![
2020-02-09T07:09:26.4712680Z LL |       let _ = vec![
2020-02-09T07:09:26.4712725Z    |  _____________^
2020-02-09T07:09:26.4712772Z LL | |         Box::new(|x| (x as u8)),
2020-02-09T07:09:26.4712837Z LL | |         box |x| (x as i16 as u8),
2020-02-09T07:09:26.4713069Z LL | |     ]: Vec<Box<dyn Fn(i32) -> _>>;
2020-02-09T07:09:26.4713165Z    |
2020-02-09T07:09:26.4713230Z help: try surrounding the expression with parentheses
2020-02-09T07:09:26.4713274Z    |
2020-02-09T07:09:26.4713317Z LL |     let _ = (vec![
2020-02-09T07:09:26.4713317Z LL |     let _ = (vec![
2020-02-09T07:09:26.4713393Z LL |         Box::new(|x| (x as u8)),
2020-02-09T07:09:26.4713441Z LL |         box |x| (x as i16 as u8),
2020-02-09T07:09:26.4714207Z LL |     ]: Vec<Box<dyn Fn(i32) -> _>>);
2020-02-09T07:09:26.4714318Z 
2020-02-09T07:09:26.4714363Z error[E0308]: mismatched types
2020-02-09T07:09:26.4714651Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:9:13
2020-02-09T07:09:26.4716148Z    |
2020-02-09T07:09:26.4716148Z    |
2020-02-09T07:09:26.4716643Z LL |     let _ = box { [1, 2, 3] }: Box<[i32]>; //~ ERROR mismatched types
2020-02-09T07:09:26.4718063Z    |             ^^^^^^^^^^^^^^^^^ expected slice `[i32]`, found array `[i32; 3]`
2020-02-09T07:09:26.4718416Z    = note: expected struct `std::boxed::Box<[i32]>`
2020-02-09T07:09:26.4718470Z               found struct `std::boxed::Box<[i32; 3]>`
2020-02-09T07:09:26.4718502Z 
2020-02-09T07:09:26.4718564Z error[E0308]: mismatched types
2020-02-09T07:09:26.4718564Z error[E0308]: mismatched types
2020-02-09T07:09:26.4720457Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:10:13
2020-02-09T07:09:26.4720550Z    |
2020-02-09T07:09:26.4720625Z LL |     let _ = box if true { [1, 2, 3] } else { [1, 3, 4] }: Box<[i32]>; //~ ERROR mismatched types
2020-02-09T07:09:26.4720687Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected slice `[i32]`, found array `[i32; 3]`
2020-02-09T07:09:26.4720892Z    = note: expected struct `std::boxed::Box<[i32]>`
2020-02-09T07:09:26.4720942Z               found struct `std::boxed::Box<[i32; 3]>`
2020-02-09T07:09:26.4720974Z 
2020-02-09T07:09:26.4721017Z error[E0308]: mismatched types
2020-02-09T07:09:26.4721017Z error[E0308]: mismatched types
2020-02-09T07:09:26.4721447Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:11:13
2020-02-09T07:09:26.4721501Z    |
2020-02-09T07:09:26.4721552Z LL |     let _ = box match true { true => [1, 2, 3], false => [1, 3, 4] }: Box<[i32]>;
2020-02-09T07:09:26.4721644Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected slice `[i32]`, found array `[i32; 3]`
2020-02-09T07:09:26.4721739Z    = note: expected struct `std::boxed::Box<[i32]>`
2020-02-09T07:09:26.4721810Z               found struct `std::boxed::Box<[i32; 3]>`
2020-02-09T07:09:26.4721841Z 
2020-02-09T07:09:26.4721884Z error[E0308]: mismatched types
2020-02-09T07:09:26.4721884Z error[E0308]: mismatched types
2020-02-09T07:09:26.4722172Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:13:13
2020-02-09T07:09:26.4722233Z    |
2020-02-09T07:09:26.4722845Z LL |     let _ = box { |x| (x as u8) }: Box<dyn Fn(i32) -> _>; //~ ERROR mismatched types
2020-02-09T07:09:26.4722932Z    |             ^^^^^^^^^^^^^^^^^^^^^ expected trait object `dyn std::ops::Fn`, found closure
2020-02-09T07:09:26.4722979Z    |
2020-02-09T07:09:26.4723237Z    = note: expected struct `std::boxed::Box<dyn std::ops::Fn(i32) -> u8>`
2020-02-09T07:09:26.4723561Z               found struct `std::boxed::Box<[closure@/checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:13:19: 13:32]>`
2020-02-09T07:09:26.4723677Z error[E0308]: mismatched types
2020-02-09T07:09:26.4723939Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:14:13
2020-02-09T07:09:26.4727595Z    |
2020-02-09T07:09:26.4727595Z    |
2020-02-09T07:09:26.4727653Z LL |     let _ = box if true { false } else { true }: Box<dyn Debug>; //~ ERROR mismatched types
2020-02-09T07:09:26.4727713Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected trait object `dyn std::fmt::Debug`, found `bool`
2020-02-09T07:09:26.4727842Z    = note: expected struct `std::boxed::Box<dyn std::fmt::Debug>`
2020-02-09T07:09:26.4727893Z               found struct `std::boxed::Box<bool>`
2020-02-09T07:09:26.4727925Z 
2020-02-09T07:09:26.4727987Z error[E0308]: mismatched types
2020-02-09T07:09:26.4727987Z error[E0308]: mismatched types
2020-02-09T07:09:26.4728347Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:15:13
2020-02-09T07:09:26.4728399Z    |
2020-02-09T07:09:26.4728716Z LL |     let _ = box match true { true => 'a', false => 'b' }: Box<dyn Debug>; //~ ERROR mismatched types
2020-02-09T07:09:26.4728783Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected trait object `dyn std::fmt::Debug`, found `char`
2020-02-09T07:09:26.4728898Z    = note: expected struct `std::boxed::Box<dyn std::fmt::Debug>`
2020-02-09T07:09:26.4728949Z               found struct `std::boxed::Box<char>`
2020-02-09T07:09:26.4728987Z 
2020-02-09T07:09:26.4729032Z error[E0308]: mismatched types
2020-02-09T07:09:26.4729032Z error[E0308]: mismatched types
2020-02-09T07:09:26.4729315Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:17:13
2020-02-09T07:09:26.4729562Z    |
2020-02-09T07:09:26.4729608Z LL |     let _ = &{ [1, 2, 3] }: &[i32]; //~ ERROR mismatched types
2020-02-09T07:09:26.4729835Z    |             ^^^^^^^^^^^^^^ expected slice `[i32]`, found array `[i32; 3]`
2020-02-09T07:09:26.4729919Z    = note: expected reference `&[i32]`
2020-02-09T07:09:26.4729981Z               found reference `&[i32; 3]`
2020-02-09T07:09:26.4730009Z 
2020-02-09T07:09:26.4730168Z error[E0308]: mismatched types
2020-02-09T07:09:26.4730168Z error[E0308]: mismatched types
2020-02-09T07:09:26.4731789Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:18:13
2020-02-09T07:09:26.4731873Z    |
2020-02-09T07:09:26.4731923Z LL |     let _ = &if true { [1, 2, 3] } else { [1, 3, 4] }: &[i32]; //~ ERROR mismatched types
2020-02-09T07:09:26.4731983Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected slice `[i32]`, found array `[i32; 3]`
2020-02-09T07:09:26.4732657Z    = note: expected reference `&[i32]`
2020-02-09T07:09:26.4732704Z               found reference `&[i32; 3]`
2020-02-09T07:09:26.4732754Z 
2020-02-09T07:09:26.4732798Z error[E0308]: mismatched types
2020-02-09T07:09:26.4732798Z error[E0308]: mismatched types
2020-02-09T07:09:26.4733145Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:19:13
2020-02-09T07:09:26.4733197Z    |
2020-02-09T07:09:26.4733266Z LL |     let _ = &match true { true => [1, 2, 3], false => [1, 3, 4] }: &[i32];
2020-02-09T07:09:26.4733338Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected slice `[i32]`, found array `[i32; 3]`
2020-02-09T07:09:26.4733450Z    = note: expected reference `&[i32]`
2020-02-09T07:09:26.4733498Z               found reference `&[i32; 3]`
2020-02-09T07:09:26.4733527Z 
2020-02-09T07:09:26.4733590Z error[E0308]: mismatched types
2020-02-09T07:09:26.4733590Z error[E0308]: mismatched types
2020-02-09T07:09:26.4733854Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:21:13
2020-02-09T07:09:26.4733915Z    |
2020-02-09T07:09:26.4734178Z LL |     let _ = &{ |x| (x as u8) }: &dyn Fn(i32) -> _; //~ ERROR mismatched types
2020-02-09T07:09:26.4734259Z    |             ^^^^^^^^^^^^^^^^^^ expected trait object `dyn std::ops::Fn`, found closure
2020-02-09T07:09:26.4734460Z    |
2020-02-09T07:09:26.4734845Z    = note: expected reference `&dyn std::ops::Fn(i32) -> u8`
2020-02-09T07:09:26.4735153Z               found reference `&[closure@/checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:21:16: 21:29]`
2020-02-09T07:09:26.4735239Z error[E0308]: mismatched types
2020-02-09T07:09:26.4735497Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:22:13
2020-02-09T07:09:26.4735544Z    |
2020-02-09T07:09:26.4735544Z    |
2020-02-09T07:09:26.4735591Z LL |     let _ = &if true { false } else { true }: &dyn Debug; //~ ERROR mismatched types
2020-02-09T07:09:26.4735663Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected trait object `dyn std::fmt::Debug`, found `bool`
2020-02-09T07:09:26.4735759Z    = note: expected reference `&dyn std::fmt::Debug`
2020-02-09T07:09:26.4735822Z               found reference `&bool`
2020-02-09T07:09:26.4735850Z 
2020-02-09T07:09:26.4735891Z error[E0308]: mismatched types
2020-02-09T07:09:26.4735891Z error[E0308]: mismatched types
2020-02-09T07:09:26.4736136Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:23:13
2020-02-09T07:09:26.4736202Z    |
2020-02-09T07:09:26.4736647Z LL |     let _ = &match true { true => 'a', false => 'b' }: &dyn Debug; //~ ERROR mismatched types
2020-02-09T07:09:26.4736720Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected trait object `dyn std::fmt::Debug`, found `char`
2020-02-09T07:09:26.4736833Z    = note: expected reference `&dyn std::fmt::Debug`
2020-02-09T07:09:26.4736881Z               found reference `&char`
2020-02-09T07:09:26.4736911Z 
2020-02-09T07:09:26.4736972Z error[E0308]: mismatched types
2020-02-09T07:09:26.4736972Z error[E0308]: mismatched types
2020-02-09T07:09:26.4737255Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:25:13
2020-02-09T07:09:26.4737306Z    |
2020-02-09T07:09:26.4737373Z LL |     let _ = Box::new([1, 2, 3]): Box<[i32]>; //~ ERROR mismatched types
2020-02-09T07:09:26.4737428Z    |             ^^^^^^^^^^^^^^^^^^^ expected slice `[i32]`, found array `[i32; 3]`
2020-02-09T07:09:26.4737538Z    = note: expected struct `std::boxed::Box<[i32]>`
2020-02-09T07:09:26.4737588Z               found struct `std::boxed::Box<[i32; 3]>`
2020-02-09T07:09:26.4737619Z 
2020-02-09T07:09:26.4737778Z error[E0308]: mismatched types
2020-02-09T07:09:26.4737778Z error[E0308]: mismatched types
2020-02-09T07:09:26.4738101Z   --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:26:13
2020-02-09T07:09:26.4738153Z    |
2020-02-09T07:09:26.4738420Z LL |     let _ = Box::new(|x| (x as u8)): Box<dyn Fn(i32) -> _>; //~ ERROR mismatched types
2020-02-09T07:09:26.4738543Z    |             ^^^^^^^^^^^^^^^^^^^^^^^ expected trait object `dyn std::ops::Fn`, found closure
2020-02-09T07:09:26.4738783Z    |
2020-02-09T07:09:26.4739079Z    = note: expected struct `std::boxed::Box<dyn std::ops::Fn(i32) -> _>`
2020-02-09T07:09:26.4739430Z               found struct `std::boxed::Box<[closure@/checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:26:22: 26:35]>`
2020-02-09T07:09:26.4739517Z error: aborting due to 29 previous errors
2020-02-09T07:09:26.4739546Z 
2020-02-09T07:09:26.4739812Z For more information about this error, try `rustc --explain E0308`.
2020-02-09T07:09:26.4739847Z 
---
2020-02-09T07:09:26.4741137Z 2   --> $DIR/const-external-macro-const-err.rs:12:5
2020-02-09T07:09:26.4741202Z 3    |
2020-02-09T07:09:26.4741244Z 4 LL |     static_assert!(2 + 2 == 5);
2020-02-09T07:09:26.4741271Z 
2020-02-09T07:09:26.4741523Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ index out of bounds: the len is 1 but the index is 1
2020-02-09T07:09:26.4741601Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `($ test : bool)`
2020-02-09T07:09:26.4741858Z -    = note: `#[deny(const_err)]` on by default
2020-02-09T07:09:26.4742156Z 8    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-09T07:09:26.4742208Z 9 
2020-02-09T07:09:26.4742251Z 10 error: aborting due to previous error
2020-02-09T07:09:26.4742251Z 10 error: aborting due to previous error
2020-02-09T07:09:26.4742278Z 
2020-02-09T07:09:26.4742323Z 
2020-02-09T07:09:26.4742366Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.4742686Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-external-macro-const-err/const-external-macro-const-err.stderr
2020-02-09T07:09:26.4742955Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.4743243Z To only update this specific test, also pass `--test-args consts/const-external-macro-const-err.rs`
2020-02-09T07:09:26.4743346Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.4743394Z status: exit code: 1
2020-02-09T07:09:26.4743394Z status: exit code: 1
2020-02-09T07:09:26.4744243Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-external-macro-const-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-external-macro-const-err" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-external-macro-const-err/auxiliary" "-A" "unused"
2020-02-09T07:09:26.4744606Z ------------------------------------------
2020-02-09T07:09:26.4744662Z 
2020-02-09T07:09:26.4744889Z ------------------------------------------
2020-02-09T07:09:26.4744936Z stderr:
2020-02-09T07:09:26.4744936Z stderr:
2020-02-09T07:09:26.4745154Z ------------------------------------------
2020-02-09T07:09:26.4745225Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4745574Z   --> /checkout/src/test/ui/consts/const-external-macro-const-err.rs:12:5
2020-02-09T07:09:26.4745635Z    |
2020-02-09T07:09:26.4745963Z LL |     static_assert!(2 + 2 == 5); //~ ERROR
2020-02-09T07:09:26.4746021Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `($ test : bool)`
2020-02-09T07:09:26.4746644Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-09T07:09:26.4746685Z 
2020-02-09T07:09:26.4746731Z error: aborting due to previous error
2020-02-09T07:09:26.4746804Z 
---
2020-02-09T07:09:26.4748148Z 4 LL |     let a = 10: u8;
2020-02-09T07:09:26.4748177Z 
2020-02-09T07:09:26.4748379Z -    |             ^^^^^^
2020-02-09T07:09:26.4748559Z -    |
2020-02-09T07:09:26.4748987Z -    = note: for more information, see ***/issues/23416
2020-02-09T07:09:26.4749269Z -    = help: add `#![feature(type_ascription)]` to the crate attributes to enable
2020-02-09T07:09:26.4749351Z +    |             ^^^^^^ help: try surrounding the expression with parentheses: `(10: u8)`
2020-02-09T07:09:26.4749445Z 10 error: aborting due to previous error
2020-02-09T07:09:26.4749488Z 11 
2020-02-09T07:09:26.4749535Z 
2020-02-09T07:09:26.4749786Z - For more information about this error, try `rustc --explain E0658`.
2020-02-09T07:09:26.4749786Z - For more information about this error, try `rustc --explain E0658`.
2020-02-09T07:09:26.4749836Z 13 
2020-02-09T07:09:26.4749873Z 
2020-02-09T07:09:26.4749918Z 
2020-02-09T07:09:26.4749966Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.4750313Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-type_ascription/feature-gate-type_ascription.stderr
2020-02-09T07:09:26.4750757Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.4751071Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-type_ascription.rs`
2020-02-09T07:09:26.4751175Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.4751223Z status: exit code: 1
2020-02-09T07:09:26.4751223Z status: exit code: 1
2020-02-09T07:09:26.4752097Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-type_ascription.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-type_ascription" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-type_ascription/auxiliary" "-A" "unused"
2020-02-09T07:09:26.4752459Z ------------------------------------------
2020-02-09T07:09:26.4752636Z 
2020-02-09T07:09:26.4752866Z ------------------------------------------
2020-02-09T07:09:26.4752915Z stderr:
2020-02-09T07:09:26.4752915Z stderr:
2020-02-09T07:09:26.4753140Z ------------------------------------------
2020-02-09T07:09:26.4753212Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4754355Z   --> /checkout/src/test/ui/feature-gates/feature-gate-type_ascription.rs:4:13
2020-02-09T07:09:26.4754416Z    |
2020-02-09T07:09:26.4754492Z LL |     let a = 10: u8; //~ ERROR type ascription is experimental
2020-02-09T07:09:26.4754673Z    |             ^^^^^^ help: try surrounding the expression with parentheses: `(10: u8)`
2020-02-09T07:09:26.4754779Z error: aborting due to previous error
2020-02-09T07:09:26.4754807Z 
2020-02-09T07:09:26.4754834Z 
2020-02-09T07:09:26.4755099Z ------------------------------------------
2020-02-09T07:09:26.4755099Z ------------------------------------------
2020-02-09T07:09:26.4755131Z 
2020-02-09T07:09:26.4755269Z 
2020-02-09T07:09:26.4755517Z ---- [ui] ui/issues/issue-22644.rs stdout ----
2020-02-09T07:09:26.4755566Z diff of stderr:
2020-02-09T07:09:26.4755593Z 
2020-02-09T07:09:26.4755660Z 25    |                    |          not interpreted as comparison
2020-02-09T07:09:26.4755713Z 26    |                    help: try comparing the cast value: `(a as usize)`
2020-02-09T07:09:26.4755998Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4756376Z +   --> $DIR/issue-22644.rs:12:20
2020-02-09T07:09:26.4756421Z +    |
2020-02-09T07:09:26.4756688Z + LL |     println!("{}", a: usize > long_name);
2020-02-09T07:09:26.4756688Z + LL |     println!("{}", a: usize > long_name);
2020-02-09T07:09:26.4756747Z +    |                    ^^^^^^^^ help: try surrounding the expression with parentheses: `(a: usize)`
2020-02-09T07:09:26.4756801Z + 
2020-02-09T07:09:26.4756876Z 28 error: `<` is interpreted as a start of generic arguments for `usize`, not a comparison
2020-02-09T07:09:26.4757186Z 30    |
2020-02-09T07:09:26.4757217Z 
2020-02-09T07:09:26.4757287Z 34    |                      |        not interpreted as comparison
2020-02-09T07:09:26.4757287Z 34    |                      |        not interpreted as comparison
2020-02-09T07:09:26.4757345Z 35    |                      help: try comparing the cast value: `(a: usize)`
2020-02-09T07:09:26.4757463Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4757704Z +   --> $DIR/issue-22644.rs:13:22
2020-02-09T07:09:26.4757755Z +    |
2020-02-09T07:09:26.4757755Z +    |
2020-02-09T07:09:26.4758230Z + LL |     println!("{}{}", a: usize < long_name, long_name);
2020-02-09T07:09:26.4758321Z +    |                      ^^^^^^^^ help: try surrounding the expression with parentheses: `(a: usize)`
2020-02-09T07:09:26.4758371Z + 
2020-02-09T07:09:26.4758442Z 37 error: `<` is interpreted as a start of generic arguments for `usize`, not a comparison
2020-02-09T07:09:26.4758781Z 39    |
2020-02-09T07:09:26.4758809Z 
2020-02-09T07:09:26.4758876Z 43    |                    |        not interpreted as comparison
2020-02-09T07:09:26.4758876Z 43    |                    |        not interpreted as comparison
2020-02-09T07:09:26.4758938Z 44    |                    help: try comparing the cast value: `(a: usize)`
2020-02-09T07:09:26.4759051Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4759279Z +   --> $DIR/issue-22644.rs:15:20
2020-02-09T07:09:26.4759326Z +    |
2020-02-09T07:09:26.4759326Z +    |
2020-02-09T07:09:26.4759391Z + LL |     println!("{}", a: usize < 4);
2020-02-09T07:09:26.4759445Z +    |                    ^^^^^^^^ help: try surrounding the expression with parentheses: `(a: usize)`
2020-02-09T07:09:26.4759496Z + 
2020-02-09T07:09:26.4759558Z 46 error: `<` is interpreted as a start of generic arguments for `usize`, not a comparison
2020-02-09T07:09:26.4760022Z 48    |
2020-02-09T07:09:26.4760048Z 
2020-02-09T07:09:26.4760048Z 
2020-02-09T07:09:26.4760117Z 96    = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
2020-02-09T07:09:26.4760759Z 97    = note: for more information, see ***/issues/23416
2020-02-09T07:09:26.4761099Z - error: aborting due to 9 previous errors
2020-02-09T07:09:26.4761149Z + error: aborting due to 12 previous errors
2020-02-09T07:09:26.4761193Z 100 
2020-02-09T07:09:26.4761253Z 101 
2020-02-09T07:09:26.4761253Z 101 
2020-02-09T07:09:26.4761281Z 
2020-02-09T07:09:26.4761308Z 
2020-02-09T07:09:26.4761353Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.4761665Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22644/issue-22644.stderr
2020-02-09T07:09:26.4762032Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.4762334Z To only update this specific test, also pass `--test-args issues/issue-22644.rs`
2020-02-09T07:09:26.4762435Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.4762480Z status: exit code: 1
2020-02-09T07:09:26.4762480Z status: exit code: 1
2020-02-09T07:09:26.4763263Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-22644.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22644" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22644/auxiliary" "-A" "unused"
2020-02-09T07:09:26.4771670Z ------------------------------------------
2020-02-09T07:09:26.4771744Z 
2020-02-09T07:09:26.4771989Z ------------------------------------------
2020-02-09T07:09:26.4772037Z stderr:
2020-02-09T07:09:26.4772037Z stderr:
2020-02-09T07:09:26.4772270Z ------------------------------------------
2020-02-09T07:09:26.4772328Z error: `<` is interpreted as a start of generic arguments for `usize`, not a comparison
2020-02-09T07:09:26.4772656Z    |
2020-02-09T07:09:26.4772656Z    |
2020-02-09T07:09:26.4772709Z LL |     println!("{}", a as usize < long_name); //~ ERROR `<` is interpreted as a start of generic
2020-02-09T07:09:26.4772972Z    |                    ---------- ^ --------- interpreted as generic arguments
2020-02-09T07:09:26.4773095Z    |                    |          not interpreted as comparison
2020-02-09T07:09:26.4773095Z    |                    |          not interpreted as comparison
2020-02-09T07:09:26.4773148Z    |                    help: try comparing the cast value: `(a as usize)`
2020-02-09T07:09:26.4773180Z 
2020-02-09T07:09:26.4773255Z error: `<` is interpreted as a start of generic arguments for `usize`, not a comparison
2020-02-09T07:09:26.4773547Z    |
2020-02-09T07:09:26.4773547Z    |
2020-02-09T07:09:26.4773614Z LL |     println!("{}{}", a as usize < long_name, long_name);
2020-02-09T07:09:26.4773879Z    |                      ---------- ^ -------------------- interpreted as generic arguments
2020-02-09T07:09:26.4774011Z    |                      |          not interpreted as comparison
2020-02-09T07:09:26.4774011Z    |                      |          not interpreted as comparison
2020-02-09T07:09:26.4774064Z    |                      help: try comparing the cast value: `(a as usize)`
2020-02-09T07:09:26.4774097Z 
2020-02-09T07:09:26.4774164Z error: `<` is interpreted as a start of generic arguments for `usize`, not a comparison
2020-02-09T07:09:26.4774454Z    |
2020-02-09T07:09:26.4774454Z    |
2020-02-09T07:09:26.4774511Z LL |     println!("{}", a as usize < 4); //~ ERROR `<` is interpreted as a start of generic
2020-02-09T07:09:26.4774783Z    |                    ---------- ^ - interpreted as generic arguments
2020-02-09T07:09:26.4774883Z    |                    |          not interpreted as comparison
2020-02-09T07:09:26.4774883Z    |                    |          not interpreted as comparison
2020-02-09T07:09:26.4774951Z    |                    help: try comparing the cast value: `(a as usize)`
2020-02-09T07:09:26.4775036Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4775290Z   --> /checkout/src/test/ui/issues/issue-22644.rs:12:20
2020-02-09T07:09:26.4775337Z    |
2020-02-09T07:09:26.4775383Z LL |     println!("{}", a: usize > long_name);
2020-02-09T07:09:26.4775383Z LL |     println!("{}", a: usize > long_name);
2020-02-09T07:09:26.4775455Z    |                    ^^^^^^^^ help: try surrounding the expression with parentheses: `(a: usize)`
2020-02-09T07:09:26.4775491Z 
2020-02-09T07:09:26.4775545Z error: `<` is interpreted as a start of generic arguments for `usize`, not a comparison
2020-02-09T07:09:26.4776027Z    |
2020-02-09T07:09:26.4776027Z    |
2020-02-09T07:09:26.4776078Z LL |     println!("{}{}", a: usize < long_name, long_name);
2020-02-09T07:09:26.4776640Z    |                      -------- ^ -------------------- interpreted as generic arguments
2020-02-09T07:09:26.4776860Z    |                      |        not interpreted as comparison
2020-02-09T07:09:26.4776860Z    |                      |        not interpreted as comparison
2020-02-09T07:09:26.4776914Z    |                      help: try comparing the cast value: `(a: usize)`
2020-02-09T07:09:26.4777337Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4777667Z   --> /checkout/src/test/ui/issues/issue-22644.rs:13:22
2020-02-09T07:09:26.4777717Z    |
2020-02-09T07:09:26.4777717Z    |
2020-02-09T07:09:26.4778588Z LL |     println!("{}{}", a: usize < long_name, long_name);
2020-02-09T07:09:26.4778654Z    |                      ^^^^^^^^ help: try surrounding the expression with parentheses: `(a: usize)`
2020-02-09T07:09:26.4778704Z 
2020-02-09T07:09:26.4778777Z error: `<` is interpreted as a start of generic arguments for `usize`, not a comparison
2020-02-09T07:09:26.4779196Z    |
2020-02-09T07:09:26.4779196Z    |
2020-02-09T07:09:26.4779267Z LL |     println!("{}", a: usize < 4); //~ ERROR `<` is interpreted as a start of generic
2020-02-09T07:09:26.4779538Z    |                    -------- ^ - interpreted as generic arguments
2020-02-09T07:09:26.4779663Z    |                    |        not interpreted as comparison
2020-02-09T07:09:26.4779663Z    |                    |        not interpreted as comparison
2020-02-09T07:09:26.4779717Z    |                    help: try comparing the cast value: `(a: usize)`
2020-02-09T07:09:26.4779795Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4780059Z   --> /checkout/src/test/ui/issues/issue-22644.rs:15:20
2020-02-09T07:09:26.4780107Z    |
2020-02-09T07:09:26.4780107Z    |
2020-02-09T07:09:26.4780167Z LL |     println!("{}", a: usize < 4); //~ ERROR `<` is interpreted as a start of generic
2020-02-09T07:09:26.4780245Z    |                    ^^^^^^^^ help: try surrounding the expression with parentheses: `(a: usize)`
2020-02-09T07:09:26.4780281Z 
2020-02-09T07:09:26.4780331Z error: `<` is interpreted as a start of generic arguments for `usize`, not a comparison
2020-02-09T07:09:26.4780655Z    |
2020-02-09T07:09:26.4780655Z    |
2020-02-09T07:09:26.4780705Z LL |                    < //~ ERROR `<` is interpreted as a start of generic
2020-02-09T07:09:26.4780823Z LL |                    4);
2020-02-09T07:09:26.4781066Z    |                    - interpreted as generic arguments
2020-02-09T07:09:26.4781273Z    |
2020-02-09T07:09:26.4781335Z help: try comparing the cast value
2020-02-09T07:09:26.4781335Z help: try comparing the cast value
2020-02-09T07:09:26.4781378Z    |
2020-02-09T07:09:26.4781595Z LL |     println!("{}", (a
2020-02-09T07:09:26.4781667Z LL |                    as
2020-02-09T07:09:26.4781713Z LL |                    usize)
2020-02-09T07:09:26.4781756Z    |
2020-02-09T07:09:26.4781783Z 
2020-02-09T07:09:26.4781851Z error: `<` is interpreted as a start of generic arguments for `usize`, not a comparison
2020-02-09T07:09:26.4782557Z    |
2020-02-09T07:09:26.4782557Z    |
2020-02-09T07:09:26.4782625Z LL |                    < //~ ERROR `<` is interpreted as a start of generic
2020-02-09T07:09:26.4782723Z LL |                    5);
2020-02-09T07:09:26.4783143Z    |                    - interpreted as generic arguments
2020-02-09T07:09:26.4783209Z    |
2020-02-09T07:09:26.4783251Z help: try comparing the cast value
---
2020-02-09T07:09:26.4783672Z LL | 
2020-02-09T07:09:26.4783710Z LL | 
2020-02-09T07:09:26.4783748Z  ...
2020-02-09T07:09:26.4783792Z 
2020-02-09T07:09:26.4783840Z error: `<` is interpreted as a start of generic arguments for `usize`, not a shift
2020-02-09T07:09:26.4784185Z    |
2020-02-09T07:09:26.4784185Z    |
2020-02-09T07:09:26.4784322Z LL |     println!("{}", a as usize << long_name); //~ ERROR `<` is interpreted as a start of generic
2020-02-09T07:09:26.4784663Z    |                    |          |
2020-02-09T07:09:26.4784731Z    |                    |          not interpreted as shift
2020-02-09T07:09:26.4784731Z    |                    |          not interpreted as shift
2020-02-09T07:09:26.4784784Z    |                    help: try shifting the cast value: `(a as usize)`
2020-02-09T07:09:26.4784876Z error: expected type, found `4`
2020-02-09T07:09:26.4785119Z   --> /checkout/src/test/ui/issues/issue-22644.rs:34:28
2020-02-09T07:09:26.4785166Z    |
2020-02-09T07:09:26.4785166Z    |
2020-02-09T07:09:26.4785231Z LL |     println!("{}", a: &mut 4); //~ ERROR expected type, found `4`
2020-02-09T07:09:26.4785506Z    |                     |
2020-02-09T07:09:26.4785554Z    |                     tried to parse a type due to this type ascription
2020-02-09T07:09:26.4785627Z    |
2020-02-09T07:09:26.4785627Z    |
2020-02-09T07:09:26.4785677Z    = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
2020-02-09T07:09:26.4786426Z    = note: for more information, see ***/issues/23416
2020-02-09T07:09:26.4786519Z error: aborting due to 12 previous errors
2020-02-09T07:09:26.4786548Z 
2020-02-09T07:09:26.4786576Z 
2020-02-09T07:09:26.4786860Z ------------------------------------------
2020-02-09T07:09:26.4786860Z ------------------------------------------
2020-02-09T07:09:26.4786894Z 
2020-02-09T07:09:26.4786921Z 
2020-02-09T07:09:26.4787151Z ---- [ui] ui/mir/mir_ascription_coercion.rs stdout ----
2020-02-09T07:09:26.4787217Z 
2020-02-09T07:09:26.4787455Z error: test compilation failed although it shouldn't!
2020-02-09T07:09:26.4787507Z status: exit code: 1
2020-02-09T07:09:26.4788327Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/mir_ascription_coercion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_ascription_coercion/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_ascription_coercion/auxiliary"
2020-02-09T07:09:26.4788678Z ------------------------------------------
2020-02-09T07:09:26.4788713Z 
2020-02-09T07:09:26.4788931Z ------------------------------------------
2020-02-09T07:09:26.4788987Z stderr:
2020-02-09T07:09:26.4788987Z stderr:
2020-02-09T07:09:26.4789227Z ------------------------------------------
2020-02-09T07:09:26.4789279Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4793130Z   --> /checkout/src/test/ui/mir/mir_ascription_coercion.rs:9:23
2020-02-09T07:09:26.4793229Z    |
2020-02-09T07:09:26.4793282Z LL |     let _y : &[i32] = &x : &[i32; 3];
2020-02-09T07:09:26.4793358Z    |                       ^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(&x : &[i32; 3])`
2020-02-09T07:09:26.4793463Z error: aborting due to previous error
2020-02-09T07:09:26.4793493Z 
2020-02-09T07:09:26.4793520Z 
2020-02-09T07:09:26.4794127Z ------------------------------------------
---
2020-02-09T07:09:26.4795478Z -   --> $DIR/issue-57731-ascibed-coupled-types.rs:17:5
2020-02-09T07:09:26.4795556Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4795799Z +   --> $DIR/issue-57731-ascibed-coupled-types.rs:11:22
2020-02-09T07:09:26.4795847Z 3    |
2020-02-09T07:09:26.4796127Z - LL | fn coupled_wilds_rhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
2020-02-09T07:09:26.4796497Z -    |                      -- lifetime `'a` defined here
2020-02-09T07:09:26.4796742Z - LL |     let ((y, _z),) = ((s, _x),): (PairCoupledTypes<_>,);
2020-02-09T07:09:26.4796956Z - LL |     y
2020-02-09T07:09:26.4797205Z -    |     ^ returning this value requires that `'a` must outlive `'static`
2020-02-09T07:09:26.4797263Z + LL |     let ((y, _z),) = ((s, _x),): (PairUncoupled<_>,);
2020-02-09T07:09:26.4797344Z +    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(((s, _x),): (PairUncoupled<_>,))`
2020-02-09T07:09:26.4797466Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4797736Z +   --> $DIR/issue-57731-ascibed-coupled-types.rs:16:22
2020-02-09T07:09:26.4797786Z 9    |
2020-02-09T07:09:26.4798012Z -    = help: consider replacing `'a` with `'static`
2020-02-09T07:09:26.4798012Z -    = help: consider replacing `'a` with `'static`
2020-02-09T07:09:26.4798089Z + LL |     let ((y, _z),) = ((s, _x),): (PairCoupledTypes<_>,);
2020-02-09T07:09:26.4798166Z +    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(((s, _x),): (PairCoupledTypes<_>,))`
2020-02-09T07:09:26.4798494Z - error: lifetime may not live long enough
2020-02-09T07:09:26.4798758Z -   --> $DIR/issue-57731-ascibed-coupled-types.rs:22:5
2020-02-09T07:09:26.4798816Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4799095Z +   --> $DIR/issue-57731-ascibed-coupled-types.rs:21:22
2020-02-09T07:09:26.4799095Z +   --> $DIR/issue-57731-ascibed-coupled-types.rs:21:22
2020-02-09T07:09:26.4799148Z 14    |
2020-02-09T07:09:26.4799438Z - LL | fn coupled_regions_rhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
2020-02-09T07:09:26.4799720Z -    |                        -- lifetime `'a` defined here
2020-02-09T07:09:26.4799780Z 17 LL |     let ((y, _z),) = ((s, _x),): (PairCoupledRegions<_>,);
2020-02-09T07:09:26.4800148Z - LL |     y
2020-02-09T07:09:26.4800392Z -    |     ^ returning this value requires that `'a` must outlive `'static`
2020-02-09T07:09:26.4800823Z -    = help: consider replacing `'a` with `'static`
2020-02-09T07:09:26.4800823Z -    = help: consider replacing `'a` with `'static`
2020-02-09T07:09:26.4800887Z +    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(((s, _x),): (PairCoupledRegions<_>,))`
2020-02-09T07:09:26.4801169Z - error: lifetime may not live long enough
2020-02-09T07:09:26.4801393Z -   --> $DIR/issue-57731-ascibed-coupled-types.rs:32:5
2020-02-09T07:09:26.4801592Z -    |
2020-02-09T07:09:26.4801592Z -    |
2020-02-09T07:09:26.4801853Z - LL | fn cast_coupled_wilds_rhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
2020-02-09T07:09:26.4802089Z -    |                           -- lifetime `'a` defined here
2020-02-09T07:09:26.4802526Z - LL |     let ((y, _z),) = ((s, _x),) as (PairCoupledTypes<_>,);
2020-02-09T07:09:26.4802716Z - LL |     y
2020-02-09T07:09:26.4802966Z -    |     ^ returning this value requires that `'a` must outlive `'static`
2020-02-09T07:09:26.4803409Z -    = help: consider replacing `'a` with `'static`
2020-02-09T07:09:26.4803601Z - 
2020-02-09T07:09:26.4803838Z - error: lifetime may not live long enough
2020-02-09T07:09:26.4804118Z -   --> $DIR/issue-57731-ascibed-coupled-types.rs:37:5
2020-02-09T07:09:26.4804118Z -   --> $DIR/issue-57731-ascibed-coupled-types.rs:37:5
2020-02-09T07:09:26.4804319Z -    |
2020-02-09T07:09:26.4804603Z - LL | fn cast_coupled_regions_rhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
2020-02-09T07:09:26.4805172Z -    |                             -- lifetime `'a` defined here
2020-02-09T07:09:26.4806791Z - LL |     let ((y, _z),) = ((s, _x),) as (PairCoupledRegions<_>,);
2020-02-09T07:09:26.4807229Z - LL |     y
2020-02-09T07:09:26.4807574Z -    |     ^ returning this value requires that `'a` must outlive `'static`
2020-02-09T07:09:26.4807994Z -    = help: consider replacing `'a` with `'static`
2020-02-09T07:09:26.4808204Z - 
2020-02-09T07:09:26.4808448Z - error: aborting due to 4 previous errors
2020-02-09T07:09:26.4808603Z + error: aborting due to 3 previous errors
2020-02-09T07:09:26.4808603Z + error: aborting due to 3 previous errors
2020-02-09T07:09:26.4808671Z 46 
2020-02-09T07:09:26.4808716Z 47 
2020-02-09T07:09:26.4808747Z 
2020-02-09T07:09:26.4808777Z 
2020-02-09T07:09:26.4808828Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.4809270Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-57731-ascibed-coupled-types/issue-57731-ascibed-coupled-types.stderr
2020-02-09T07:09:26.4809698Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.4810004Z To only update this specific test, also pass `--test-args nll/user-annotations/issue-57731-ascibed-coupled-types.rs`
2020-02-09T07:09:26.4810274Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.4810319Z status: exit code: 1
2020-02-09T07:09:26.4810319Z status: exit code: 1
2020-02-09T07:09:26.4811270Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/issue-57731-ascibed-coupled-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-57731-ascibed-coupled-types" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-57731-ascibed-coupled-types/auxiliary" "-A" "unused"
2020-02-09T07:09:26.4811634Z ------------------------------------------
2020-02-09T07:09:26.4811670Z 
2020-02-09T07:09:26.4811888Z ------------------------------------------
2020-02-09T07:09:26.4811955Z stderr:
2020-02-09T07:09:26.4811955Z stderr:
2020-02-09T07:09:26.4812164Z ------------------------------------------
2020-02-09T07:09:26.4812215Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4812498Z   --> /checkout/src/test/ui/nll/user-annotations/issue-57731-ascibed-coupled-types.rs:11:22
2020-02-09T07:09:26.4812562Z    |
2020-02-09T07:09:26.4812609Z LL |     let ((y, _z),) = ((s, _x),): (PairUncoupled<_>,);
2020-02-09T07:09:26.4812694Z    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(((s, _x),): (PairUncoupled<_>,))`
2020-02-09T07:09:26.4812780Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4813070Z   --> /checkout/src/test/ui/nll/user-annotations/issue-57731-ascibed-coupled-types.rs:16:22
2020-02-09T07:09:26.4813121Z    |
2020-02-09T07:09:26.4813121Z    |
2020-02-09T07:09:26.4813177Z LL |     let ((y, _z),) = ((s, _x),): (PairCoupledTypes<_>,);
2020-02-09T07:09:26.4813257Z    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(((s, _x),): (PairCoupledTypes<_>,))`
2020-02-09T07:09:26.4813341Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4813618Z   --> /checkout/src/test/ui/nll/user-annotations/issue-57731-ascibed-coupled-types.rs:21:22
2020-02-09T07:09:26.4813690Z    |
2020-02-09T07:09:26.4813690Z    |
2020-02-09T07:09:26.4813797Z LL |     let ((y, _z),) = ((s, _x),): (PairCoupledRegions<_>,);
2020-02-09T07:09:26.4813859Z    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(((s, _x),): (PairCoupledRegions<_>,))`
2020-02-09T07:09:26.4813960Z error: aborting due to 3 previous errors
2020-02-09T07:09:26.4813990Z 
2020-02-09T07:09:26.4814015Z 
2020-02-09T07:09:26.4814357Z ------------------------------------------
---
2020-02-09T07:09:26.4815025Z - error[E0597]: `x` does not live long enough
2020-02-09T07:09:26.4815498Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4816195Z 2   --> $DIR/type_ascription_static_lifetime.rs:6:19
2020-02-09T07:09:26.4816254Z 3    |
2020-02-09T07:09:26.4816515Z 4 LL |     let y: &u32 = &x: &'static u32;
2020-02-09T07:09:26.4816790Z -    |                   ^^--------------
2020-02-09T07:09:26.4816992Z -    |                   |
2020-02-09T07:09:26.4817232Z -    |                   borrowed value does not live long enough
2020-02-09T07:09:26.4817565Z -    |                   type annotation requires that `x` is borrowed for `'static`
2020-02-09T07:09:26.4817565Z -    |                   type annotation requires that `x` is borrowed for `'static`
2020-02-09T07:09:26.4817815Z - LL | }
2020-02-09T07:09:26.4818043Z -    | - `x` dropped here while still borrowed
2020-02-09T07:09:26.4818501Z +    |                   ^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(&x: &'static u32)`
2020-02-09T07:09:26.4818607Z 12 error: aborting due to previous error
2020-02-09T07:09:26.4818671Z 13 
2020-02-09T07:09:26.4818711Z 
2020-02-09T07:09:26.4818971Z - For more information about this error, try `rustc --explain E0597`.
2020-02-09T07:09:26.4818971Z - For more information about this error, try `rustc --explain E0597`.
2020-02-09T07:09:26.4819021Z 15 
2020-02-09T07:09:26.4819067Z 
2020-02-09T07:09:26.4819094Z 
2020-02-09T07:09:26.4819142Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.4819496Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/type_ascription_static_lifetime/type_ascription_static_lifetime.stderr
2020-02-09T07:09:26.4819772Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.4820096Z To only update this specific test, also pass `--test-args nll/user-annotations/type_ascription_static_lifetime.rs`
2020-02-09T07:09:26.4820207Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.4820256Z status: exit code: 1
2020-02-09T07:09:26.4820256Z status: exit code: 1
2020-02-09T07:09:26.4821814Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/type_ascription_static_lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/type_ascription_static_lifetime" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/type_ascription_static_lifetime/auxiliary" "-A" "unused"
2020-02-09T07:09:26.4822179Z ------------------------------------------
2020-02-09T07:09:26.4822213Z 
2020-02-09T07:09:26.4822423Z ------------------------------------------
2020-02-09T07:09:26.4822467Z stderr:
2020-02-09T07:09:26.4822467Z stderr:
2020-02-09T07:09:26.4822688Z ------------------------------------------
2020-02-09T07:09:26.4822737Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4823713Z   --> /checkout/src/test/ui/nll/user-annotations/type_ascription_static_lifetime.rs:6:19
2020-02-09T07:09:26.4823804Z    |
2020-02-09T07:09:26.4824043Z LL |     let y: &u32 = &x: &'static u32; //~ ERROR E0597
2020-02-09T07:09:26.4824323Z    |                   ^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(&x: &'static u32)`
2020-02-09T07:09:26.4824426Z error: aborting due to previous error
2020-02-09T07:09:26.4824454Z 
2020-02-09T07:09:26.4824479Z 
2020-02-09T07:09:26.4824704Z ------------------------------------------
---
2020-02-09T07:09:26.4825223Z 
2020-02-09T07:09:26.4825266Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4825523Z +   --> $DIR/or-patterns-syntactic-fail.rs:23:18
2020-02-09T07:09:26.4825588Z +    |
2020-02-09T07:09:26.4825895Z + LL |     let _ = |A | B: E| ();
2020-02-09T07:09:26.4825947Z +    |                  ^^^^ help: try surrounding the expression with parentheses: `(B: E)`
2020-02-09T07:09:26.4826467Z 1 error: an or-pattern parameter must be wrapped in parenthesis
2020-02-09T07:09:26.4826702Z 2   --> $DIR/or-patterns-syntactic-fail.rs:27:13
2020-02-09T07:09:26.4826769Z 3    |
2020-02-09T07:09:26.4826797Z 
---
2020-02-09T07:09:26.4827620Z 122 For more information about an error, try `rustc --explain E0308`.
2020-02-09T07:09:26.4827665Z 
2020-02-09T07:09:26.4827711Z 
2020-02-09T07:09:26.4827963Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.4828758Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail/or-patterns-syntactic-fail.stderr
2020-02-09T07:09:26.4829041Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.4829353Z To only update this specific test, also pass `--test-args or-patterns/or-patterns-syntactic-fail.rs`
2020-02-09T07:09:26.4829474Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.4829526Z status: exit code: 1
2020-02-09T07:09:26.4829526Z status: exit code: 1
2020-02-09T07:09:26.4830415Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail/auxiliary" "-A" "unused"
2020-02-09T07:09:26.4830802Z ------------------------------------------
2020-02-09T07:09:26.4830858Z 
2020-02-09T07:09:26.4831100Z ------------------------------------------
2020-02-09T07:09:26.4831312Z stderr:
2020-02-09T07:09:26.4831312Z stderr:
2020-02-09T07:09:26.4831531Z ------------------------------------------
2020-02-09T07:09:26.4831599Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4831854Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:23:18
2020-02-09T07:09:26.4831906Z    |
2020-02-09T07:09:26.4831974Z LL |     let _ = |A | B: E| (); //~ ERROR no implementation for `E | ()`
2020-02-09T07:09:26.4832039Z    |                  ^^^^ help: try surrounding the expression with parentheses: `(B: E)`
2020-02-09T07:09:26.4832328Z error: an or-pattern parameter must be wrapped in parenthesis
2020-02-09T07:09:26.4832577Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:27:13
2020-02-09T07:09:26.4832625Z    |
2020-02-09T07:09:26.4832625Z    |
2020-02-09T07:09:26.4832902Z LL |     fn fun1(A | B: E) {} //~ ERROR an or-pattern parameter must be wrapped in parenthesis
2020-02-09T07:09:26.4832960Z    |             ^^^^^ help: wrap the pattern in parenthesis: `(A | B)`
2020-02-09T07:09:26.4833168Z error: a leading `|` is not allowed in a parameter pattern
2020-02-09T07:09:26.4833474Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:29:13
2020-02-09T07:09:26.4833524Z    |
2020-02-09T07:09:26.4833524Z    |
2020-02-09T07:09:26.4833690Z LL |     fn fun2(| A | B: E) {}
2020-02-09T07:09:26.4833759Z    |             ^ help: remove the `|`
2020-02-09T07:09:26.4834188Z error: an or-pattern parameter must be wrapped in parenthesis
2020-02-09T07:09:26.4834437Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:29:15
2020-02-09T07:09:26.4834505Z    |
2020-02-09T07:09:26.4834505Z    |
2020-02-09T07:09:26.4834548Z LL |     fn fun2(| A | B: E) {}
2020-02-09T07:09:26.4834598Z    |               ^^^^^ help: wrap the pattern in parenthesis: `(A | B)`
2020-02-09T07:09:26.4834876Z error: a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4835120Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:40:11
2020-02-09T07:09:26.4835196Z    |
2020-02-09T07:09:26.4835196Z    |
2020-02-09T07:09:26.4835460Z LL |     let ( | A | B) = E::A; //~ ERROR a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4835515Z    |           ^ help: remove the `|`
2020-02-09T07:09:26.4835788Z error: a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4836033Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:41:11
2020-02-09T07:09:26.4836091Z    |
2020-02-09T07:09:26.4836091Z    |
2020-02-09T07:09:26.4836853Z LL |     let ( | A | B,) = (E::B,); //~ ERROR a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4836919Z    |           ^ help: remove the `|`
2020-02-09T07:09:26.4837195Z error: a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4837474Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:42:11
2020-02-09T07:09:26.4837524Z    |
2020-02-09T07:09:26.4837524Z    |
2020-02-09T07:09:26.4837803Z LL |     let [ | A | B ] = [E::A]; //~ ERROR a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4837879Z    |           ^ help: remove the `|`
2020-02-09T07:09:26.4838146Z error: a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4838422Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:43:13
2020-02-09T07:09:26.4838474Z    |
2020-02-09T07:09:26.4838474Z    |
2020-02-09T07:09:26.4838755Z LL |     let TS( | A | B ); //~ ERROR a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4838990Z    |             ^ help: remove the `|`
2020-02-09T07:09:26.4839244Z error: a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4839893Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:44:17
2020-02-09T07:09:26.4839957Z    |
2020-02-09T07:09:26.4839957Z    |
2020-02-09T07:09:26.4840219Z LL |     let NS { f: | A | B }; //~ ERROR a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4840273Z    |                 ^ help: remove the `|`
2020-02-09T07:09:26.4840560Z error: a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4840806Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:46:11
2020-02-09T07:09:26.4840854Z    |
2020-02-09T07:09:26.4840854Z    |
2020-02-09T07:09:26.4841133Z LL |     let ( || A | B) = E::A; //~ ERROR a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4841196Z    |           ^^ help: remove the `||`
2020-02-09T07:09:26.4841237Z    |
2020-02-09T07:09:26.4841497Z    = note: alternatives in or-patterns are separated with `|`, not `||`
2020-02-09T07:09:26.4841754Z error: a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4842017Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:47:11
2020-02-09T07:09:26.4842067Z    |
2020-02-09T07:09:26.4842067Z    |
2020-02-09T07:09:26.4842330Z LL |     let [ || A | B ] = [E::A]; //~ ERROR a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4842385Z    |           ^^ help: remove the `||`
2020-02-09T07:09:26.4842557Z    |
2020-02-09T07:09:26.4842840Z    = note: alternatives in or-patterns are separated with `|`, not `||`
2020-02-09T07:09:26.4843121Z error: a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4843371Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:48:13
2020-02-09T07:09:26.4843506Z    |
2020-02-09T07:09:26.4843506Z    |
2020-02-09T07:09:26.4843811Z LL |     let TS( || A | B ); //~ ERROR a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4843868Z    |             ^^ help: remove the `||`
2020-02-09T07:09:26.4843910Z    |
2020-02-09T07:09:26.4844145Z    = note: alternatives in or-patterns are separated with `|`, not `||`
2020-02-09T07:09:26.4844425Z error: a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4844670Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:49:17
2020-02-09T07:09:26.4844737Z    |
2020-02-09T07:09:26.4844737Z    |
2020-02-09T07:09:26.4845182Z LL |     let NS { f: || A | B }; //~ ERROR a leading `|` is only allowed in a top-level pattern
2020-02-09T07:09:26.4845243Z    |                 ^^ help: remove the `||`
2020-02-09T07:09:26.4845306Z    |
2020-02-09T07:09:26.4845581Z    = note: alternatives in or-patterns are separated with `|`, not `||`
2020-02-09T07:09:26.4845661Z error: no rules expected the token `|`
2020-02-09T07:09:26.4846173Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:13:15
2020-02-09T07:09:26.4846223Z    |
2020-02-09T07:09:26.4846223Z    |
2020-02-09T07:09:26.4846267Z LL | macro_rules! accept_pat {
2020-02-09T07:09:26.4846546Z    | ----------------------- when calling this macro
2020-02-09T07:09:26.4846596Z ...
2020-02-09T07:09:26.4846646Z LL | accept_pat!(p | q); //~ ERROR no rules expected the token `|`
2020-02-09T07:09:26.4846799Z    |               ^ no rules expected this token in macro call
2020-02-09T07:09:26.4846876Z error: no rules expected the token `|`
2020-02-09T07:09:26.4847142Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:14:13
2020-02-09T07:09:26.4847211Z    |
2020-02-09T07:09:26.4847211Z    |
2020-02-09T07:09:26.4847254Z LL | macro_rules! accept_pat {
2020-02-09T07:09:26.4847489Z    | ----------------------- when calling this macro
2020-02-09T07:09:26.4847560Z ...
2020-02-09T07:09:26.4847609Z LL | accept_pat!(| p | q); //~ ERROR no rules expected the token `|`
2020-02-09T07:09:26.4847670Z    |             ^ no rules expected this token in macro call
2020-02-09T07:09:26.4847701Z 
2020-02-09T07:09:26.4847765Z error[E0369]: no implementation for `E | ()`
2020-02-09T07:09:26.4848078Z    |
2020-02-09T07:09:26.4848078Z    |
2020-02-09T07:09:26.4848145Z LL |     let _ = |A | B: E| (); //~ ERROR no implementation for `E | ()`
2020-02-09T07:09:26.4848360Z    |                  ----^ -- ()
2020-02-09T07:09:26.4848471Z    |                  E
2020-02-09T07:09:26.4848513Z    |
2020-02-09T07:09:26.4848570Z    = note: an implementation of `std::ops::BitOr` might be missing for `E`
2020-02-09T07:09:26.4848604Z 
2020-02-09T07:09:26.4848604Z 
2020-02-09T07:09:26.4848667Z error[E0308]: mismatched types
2020-02-09T07:09:26.4848928Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:51:36
2020-02-09T07:09:26.4848979Z    |
2020-02-09T07:09:26.4849047Z LL |     let recovery_witness: String = 0; //~ ERROR mismatched types
2020-02-09T07:09:26.4849332Z    |                           |        |
2020-02-09T07:09:26.4849405Z    |                           |        expected struct `std::string::String`, found integer
2020-02-09T07:09:26.4849464Z    |                           |        help: try using a conversion method: `0.to_string()`
2020-02-09T07:09:26.4849658Z    |                           expected due to this
---
2020-02-09T07:09:26.4850966Z ---- [ui] ui/raw-ref-op/raw-ref-temp-deref.rs stdout ----
2020-02-09T07:09:26.4851098Z 
2020-02-09T07:09:26.4851359Z error: test compilation failed although it shouldn't!
2020-02-09T07:09:26.4851413Z status: exit code: 1
2020-02-09T07:09:26.4852282Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/raw-ref-op/raw-ref-temp-deref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-ref-op/raw-ref-temp-deref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-ref-op/raw-ref-temp-deref/auxiliary" "-A" "unused"
2020-02-09T07:09:26.4852628Z ------------------------------------------
2020-02-09T07:09:26.4852664Z 
2020-02-09T07:09:26.4852898Z ------------------------------------------
2020-02-09T07:09:26.4852965Z stderr:
2020-02-09T07:09:26.4852965Z stderr:
2020-02-09T07:09:26.4853182Z ------------------------------------------
2020-02-09T07:09:26.4853234Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4853679Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp-deref.rs:21:35
2020-02-09T07:09:26.4853737Z    |
2020-02-09T07:09:26.4853786Z LL |     let ascribe_ref = &raw const (x: i32);
2020-02-09T07:09:26.4853866Z    |                                   ^^^^^^ help: try surrounding the expression with parentheses: `(x: i32)`
2020-02-09T07:09:26.4853962Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4854267Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp-deref.rs:22:37
2020-02-09T07:09:26.4854318Z    |
2020-02-09T07:09:26.4854318Z    |
2020-02-09T07:09:26.4854366Z LL |     let ascribe_deref = &raw const (*ARRAY_REF: [i32; 2]);
2020-02-09T07:09:26.4854447Z    |                                     ^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(*ARRAY_REF: [i32; 2])`
2020-02-09T07:09:26.4854544Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4854816Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp-deref.rs:23:43
2020-02-09T07:09:26.4854867Z    |
2020-02-09T07:09:26.4854867Z    |
2020-02-09T07:09:26.4854915Z LL |     let ascribe_index_deref = &raw const (ARRAY_REF[0]: i32);
2020-02-09T07:09:26.4854977Z    |                                           ^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(ARRAY_REF[0]: i32)`
2020-02-09T07:09:26.4855089Z error: aborting due to 3 previous errors
2020-02-09T07:09:26.4855120Z 
2020-02-09T07:09:26.4855147Z 
2020-02-09T07:09:26.4855391Z ------------------------------------------
---
2020-02-09T07:09:26.4855785Z 
2020-02-09T07:09:26.4855831Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4856050Z +   --> $DIR/raw-ref-temp.rs:26:35
2020-02-09T07:09:26.4856116Z +    |
2020-02-09T07:09:26.4856163Z + LL |     let ref_ascribe = &raw const (2: i32);
2020-02-09T07:09:26.4856221Z +    |                                   ^^^^^^ help: try surrounding the expression with parentheses: `(2: i32)`
2020-02-09T07:09:26.4856336Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4856551Z +   --> $DIR/raw-ref-temp.rs:27:37
2020-02-09T07:09:26.4856617Z +    |
2020-02-09T07:09:26.4856617Z +    |
2020-02-09T07:09:26.4856760Z + LL |     let mut_ref_ascribe = &raw mut (3: i32);
2020-02-09T07:09:26.4856826Z +    |                                     ^^^^^^ help: try surrounding the expression with parentheses: `(3: i32)`
2020-02-09T07:09:26.4856940Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4857193Z +   --> $DIR/raw-ref-temp.rs:29:41
2020-02-09T07:09:26.4857328Z +    |
2020-02-09T07:09:26.4857328Z +    |
2020-02-09T07:09:26.4857395Z + LL |     let ascribe_field_ref = &raw const (PAIR.0: i32);
2020-02-09T07:09:26.4857455Z +    |                                         ^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(PAIR.0: i32)`
2020-02-09T07:09:26.4857716Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4858000Z +   --> $DIR/raw-ref-temp.rs:30:39
2020-02-09T07:09:26.4858048Z +    |
2020-02-09T07:09:26.4858048Z +    |
2020-02-09T07:09:26.4858114Z + LL |     let ascribe_index_ref = &raw mut (ARRAY[0]: i32);
2020-02-09T07:09:26.4858186Z +    |                                       ^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(ARRAY[0]: i32)`
2020-02-09T07:09:26.4858302Z 1 error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4858524Z 2   --> $DIR/raw-ref-temp.rs:11:31
2020-02-09T07:09:26.4858570Z 3    |
2020-02-09T07:09:26.4858599Z 
2020-02-09T07:09:26.4858599Z 
2020-02-09T07:09:26.4858674Z 94 LL |     let ascribe_index_ref = &raw mut (ARRAY[0]: i32);
2020-02-09T07:09:26.4858727Z 95    |                                      ^^^^^^^^^^^^^^^ temporary value
2020-02-09T07:09:26.4859019Z - error: aborting due to 16 previous errors
2020-02-09T07:09:26.4859072Z + error: aborting due to 20 previous errors
2020-02-09T07:09:26.4859116Z 98 
2020-02-09T07:09:26.4859890Z 99 For more information about this error, try `rustc --explain E0745`.
2020-02-09T07:09:26.4859890Z 99 For more information about this error, try `rustc --explain E0745`.
2020-02-09T07:09:26.4859938Z 100 
2020-02-09T07:09:26.4859965Z 
2020-02-09T07:09:26.4859992Z 
2020-02-09T07:09:26.4860063Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.4860366Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-ref-op/raw-ref-temp/raw-ref-temp.stderr
2020-02-09T07:09:26.4860606Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.4860897Z To only update this specific test, also pass `--test-args raw-ref-op/raw-ref-temp.rs`
2020-02-09T07:09:26.4860993Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.4861042Z status: exit code: 1
2020-02-09T07:09:26.4861042Z status: exit code: 1
2020-02-09T07:09:26.4862355Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-ref-op/raw-ref-temp" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-ref-op/raw-ref-temp/auxiliary" "-A" "unused"
2020-02-09T07:09:26.4862736Z ------------------------------------------
2020-02-09T07:09:26.4862773Z 
2020-02-09T07:09:26.4862996Z ------------------------------------------
2020-02-09T07:09:26.4863074Z stderr:
2020-02-09T07:09:26.4863074Z stderr:
2020-02-09T07:09:26.4863450Z ------------------------------------------
2020-02-09T07:09:26.4863502Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4863752Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:26:35
2020-02-09T07:09:26.4863803Z    |
2020-02-09T07:09:26.4863854Z LL |     let ref_ascribe = &raw const (2: i32);              //~ ERROR cannot take address
2020-02-09T07:09:26.4863933Z    |                                   ^^^^^^ help: try surrounding the expression with parentheses: `(2: i32)`
2020-02-09T07:09:26.4864441Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4864725Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:27:37
2020-02-09T07:09:26.4864794Z    |
2020-02-09T07:09:26.4864794Z    |
2020-02-09T07:09:26.4864844Z LL |     let mut_ref_ascribe = &raw mut (3: i32);            //~ ERROR cannot take address
2020-02-09T07:09:26.4864902Z    |                                     ^^^^^^ help: try surrounding the expression with parentheses: `(3: i32)`
2020-02-09T07:09:26.4865078Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4865340Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:29:41
2020-02-09T07:09:26.4865409Z    |
2020-02-09T07:09:26.4865409Z    |
2020-02-09T07:09:26.4865460Z LL |     let ascribe_field_ref = &raw const (PAIR.0: i32);   //~ ERROR cannot take address
2020-02-09T07:09:26.4865519Z    |                                         ^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(PAIR.0: i32)`
2020-02-09T07:09:26.4865632Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4865871Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:30:39
2020-02-09T07:09:26.4865938Z    |
2020-02-09T07:09:26.4865938Z    |
2020-02-09T07:09:26.4865988Z LL |     let ascribe_index_ref = &raw mut (ARRAY[0]: i32);   //~ ERROR cannot take address
2020-02-09T07:09:26.4866048Z    |                                       ^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(ARRAY[0]: i32)`
2020-02-09T07:09:26.4866157Z error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4866640Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:11:31
2020-02-09T07:09:26.4866689Z    |
2020-02-09T07:09:26.4866689Z    |
2020-02-09T07:09:26.4866758Z LL |     let ref_expr = &raw const 2;                        //~ ERROR cannot take address
2020-02-09T07:09:26.4866811Z    |                               ^ temporary value
2020-02-09T07:09:26.4866888Z error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4867321Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:12:33
2020-02-09T07:09:26.4867381Z    |
2020-02-09T07:09:26.4867381Z    |
2020-02-09T07:09:26.4867433Z LL |     let mut_ref_expr = &raw mut 3;                      //~ ERROR cannot take address
2020-02-09T07:09:26.4867508Z    |                                 ^ temporary value
2020-02-09T07:09:26.4867583Z error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4867894Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:13:32
2020-02-09T07:09:26.4867944Z    |
2020-02-09T07:09:26.4867944Z    |
2020-02-09T07:09:26.4867994Z LL |     let ref_const = &raw const FOUR;                    //~ ERROR cannot take address
2020-02-09T07:09:26.4868048Z    |                                ^^^^ temporary value
2020-02-09T07:09:26.4868143Z error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4868386Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:14:34
2020-02-09T07:09:26.4868453Z    |
2020-02-09T07:09:26.4868453Z    |
2020-02-09T07:09:26.4868512Z LL |     let mut_ref_const = &raw mut FOUR;                  //~ ERROR cannot take address
2020-02-09T07:09:26.4868566Z    |                                  ^^^^ temporary value
2020-02-09T07:09:26.4868663Z error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4868909Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:16:37
2020-02-09T07:09:26.4868958Z    |
2020-02-09T07:09:26.4868958Z    |
2020-02-09T07:09:26.4869037Z LL |     let field_ref_expr = &raw const (1, 2).0;           //~ ERROR cannot take address
2020-02-09T07:09:26.4869091Z    |                                     ^^^^^^^^ temporary value
2020-02-09T07:09:26.4869188Z error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4869435Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:17:39
2020-02-09T07:09:26.4869767Z    |
2020-02-09T07:09:26.4869767Z    |
2020-02-09T07:09:26.4869823Z LL |     let mut_field_ref_expr = &raw mut (1, 2).0;         //~ ERROR cannot take address
2020-02-09T07:09:26.4870151Z    |                                       ^^^^^^^^ temporary value
2020-02-09T07:09:26.4870241Z error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4870594Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:18:32
2020-02-09T07:09:26.4870647Z    |
2020-02-09T07:09:26.4870647Z    |
2020-02-09T07:09:26.4870698Z LL |     let field_ref = &raw const PAIR.0;                  //~ ERROR cannot take address
2020-02-09T07:09:26.4870859Z    |                                ^^^^^^ temporary value
2020-02-09T07:09:26.4870937Z error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4871214Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:19:34
2020-02-09T07:09:26.4871283Z    |
2020-02-09T07:09:26.4871283Z    |
2020-02-09T07:09:26.4871334Z LL |     let mut_field_ref = &raw mut PAIR.0;                //~ ERROR cannot take address
2020-02-09T07:09:26.4871388Z    |                                  ^^^^^^ temporary value
2020-02-09T07:09:26.4871600Z error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4871855Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:21:37
2020-02-09T07:09:26.4871903Z    |
2020-02-09T07:09:26.4871903Z    |
2020-02-09T07:09:26.4871973Z LL |     let index_ref_expr = &raw const [1, 2][0];          //~ ERROR cannot take address
2020-02-09T07:09:26.4872027Z    |                                     ^^^^^^^^^ temporary value
2020-02-09T07:09:26.4872132Z error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4872380Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:22:39
2020-02-09T07:09:26.4872429Z    |
2020-02-09T07:09:26.4872429Z    |
2020-02-09T07:09:26.4872499Z LL |     let mut_index_ref_expr = &raw mut [1, 2][0];        //~ ERROR cannot take address
2020-02-09T07:09:26.4872553Z    |                                       ^^^^^^^^^ temporary value
2020-02-09T07:09:26.4872630Z error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4872894Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:23:32
2020-02-09T07:09:26.4872942Z    |
2020-02-09T07:09:26.4872942Z    |
2020-02-09T07:09:26.4873001Z LL |     let index_ref = &raw const ARRAY[0];                //~ ERROR cannot take address
2020-02-09T07:09:26.4873074Z    |                                ^^^^^^^^ temporary value
2020-02-09T07:09:26.4873152Z error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4873397Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:24:34
2020-02-09T07:09:26.4873474Z    |
2020-02-09T07:09:26.4873474Z    |
2020-02-09T07:09:26.4873525Z LL |     let mut_index_ref = &raw mut ARRAY[1];              //~ ERROR cannot take address
2020-02-09T07:09:26.4873845Z    |                                  ^^^^^^^^ temporary value
2020-02-09T07:09:26.4873946Z error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4874247Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:26:34
2020-02-09T07:09:26.4874316Z    |
2020-02-09T07:09:26.4874316Z    |
2020-02-09T07:09:26.4874367Z LL |     let ref_ascribe = &raw const (2: i32);              //~ ERROR cannot take address
2020-02-09T07:09:26.4874433Z    |                                  ^^^^^^^^ temporary value
2020-02-09T07:09:26.4874530Z error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4874780Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:27:36
2020-02-09T07:09:26.4874830Z    |
2020-02-09T07:09:26.4874830Z    |
2020-02-09T07:09:26.4874900Z LL |     let mut_ref_ascribe = &raw mut (3: i32);            //~ ERROR cannot take address
2020-02-09T07:09:26.4874963Z    |                                    ^^^^^^^^ temporary value
2020-02-09T07:09:26.4875040Z error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4875305Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:29:40
2020-02-09T07:09:26.4875354Z    |
2020-02-09T07:09:26.4875354Z    |
2020-02-09T07:09:26.4875404Z LL |     let ascribe_field_ref = &raw const (PAIR.0: i32);   //~ ERROR cannot take address
2020-02-09T07:09:26.4875477Z    |                                        ^^^^^^^^^^^^^ temporary value
2020-02-09T07:09:26.4875667Z error[E0745]: cannot take address of a temporary
2020-02-09T07:09:26.4875978Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:30:38
2020-02-09T07:09:26.4876028Z    |
2020-02-09T07:09:26.4876028Z    |
2020-02-09T07:09:26.4876078Z LL |     let ascribe_index_ref = &raw mut (ARRAY[0]: i32);   //~ ERROR cannot take address
2020-02-09T07:09:26.4876134Z    |                                      ^^^^^^^^^^^^^^^ temporary value
2020-02-09T07:09:26.4876323Z error: aborting due to 20 previous errors
2020-02-09T07:09:26.4876353Z 
2020-02-09T07:09:26.4876632Z For more information about this error, try `rustc --explain E0745`.
2020-02-09T07:09:26.4876690Z 
---
2020-02-09T07:09:26.4877289Z 
2020-02-09T07:09:26.4877336Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4877578Z +   --> $DIR/expr_type.rs:9:13
2020-02-09T07:09:26.4877627Z +    |
2020-02-09T07:09:26.4877671Z + LL |     let x = {return}: !;
2020-02-09T07:09:26.4877746Z +    |             ^^^^^^^^^^^ help: try surrounding the expression with parentheses: `({return}: !)`
2020-02-09T07:09:26.4877841Z 1 error: unreachable expression
2020-02-09T07:09:26.4878084Z 2   --> $DIR/expr_type.rs:9:13
2020-02-09T07:09:26.4878131Z 3    |
2020-02-09T07:09:26.4878159Z 
---
2020-02-09T07:09:26.4878713Z 
2020-02-09T07:09:26.4878740Z 
2020-02-09T07:09:26.4878807Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.4879317Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_type/expr_type.stderr
2020-02-09T07:09:26.4879561Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.4879855Z To only update this specific test, also pass `--test-args reachable/expr_type.rs`
2020-02-09T07:09:26.4879949Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.4880014Z status: exit code: 1
2020-02-09T07:09:26.4880014Z status: exit code: 1
2020-02-09T07:09:26.4880828Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reachable/expr_type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_type/auxiliary" "-A" "unused"
2020-02-09T07:09:26.4881182Z ------------------------------------------
2020-02-09T07:09:26.4881219Z 
2020-02-09T07:09:26.4881472Z ------------------------------------------
2020-02-09T07:09:26.4881521Z stderr:
2020-02-09T07:09:26.4881521Z stderr:
2020-02-09T07:09:26.4881745Z ------------------------------------------
2020-02-09T07:09:26.4881808Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4882077Z   --> /checkout/src/test/ui/reachable/expr_type.rs:9:13
2020-02-09T07:09:26.4882129Z    |
2020-02-09T07:09:26.4882179Z LL |     let x = {return}: !; //~ ERROR unreachable
2020-02-09T07:09:26.4882258Z    |             ^^^^^^^^^^^ help: try surrounding the expression with parentheses: `({return}: !)`
2020-02-09T07:09:26.4882341Z error: unreachable expression
2020-02-09T07:09:26.4882610Z   --> /checkout/src/test/ui/reachable/expr_type.rs:9:13
2020-02-09T07:09:26.4882765Z    |
2020-02-09T07:09:26.4882765Z    |
2020-02-09T07:09:26.4882822Z LL |     let x = {return}: !; //~ ERROR unreachable
2020-02-09T07:09:26.4883146Z    |             ||
2020-02-09T07:09:26.4883196Z    |             |any code following this expression is unreachable
2020-02-09T07:09:26.4883248Z    |             unreachable expression
2020-02-09T07:09:26.4883397Z    |
---
2020-02-09T07:09:26.4884712Z 
2020-02-09T07:09:26.4884760Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4885213Z +   --> $DIR/type-ascription-instead-of-let.rs:5:9
2020-02-09T07:09:26.4885268Z +    |
2020-02-09T07:09:26.4885339Z + LL |         temp: i32 = fun(5i32);
2020-02-09T07:09:26.4885406Z +    |         ^^^^^^^^^ help: try surrounding the expression with parentheses: `(temp: i32)`
2020-02-09T07:09:26.4885524Z 1 error[E0425]: cannot find value `temp` in this scope
2020-02-09T07:09:26.4885779Z 2   --> $DIR/type-ascription-instead-of-let.rs:5:9
2020-02-09T07:09:26.4885829Z 3    |
2020-02-09T07:09:26.4885877Z 
2020-02-09T07:09:26.4885877Z 
2020-02-09T07:09:26.4885921Z 13 LL |         temp + value + 1
2020-02-09T07:09:26.4886016Z 15 
2020-02-09T07:09:26.4886443Z - error: aborting due to 2 previous errors
2020-02-09T07:09:26.4886500Z + error: aborting due to 3 previous errors
2020-02-09T07:09:26.4886544Z 17 
2020-02-09T07:09:26.4886544Z 17 
2020-02-09T07:09:26.4896293Z 18 For more information about this error, try `rustc --explain E0425`.
2020-02-09T07:09:26.4896360Z 19 
2020-02-09T07:09:26.4896392Z 
2020-02-09T07:09:26.4896420Z 
2020-02-09T07:09:26.4896489Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.4896880Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-let/type-ascription-instead-of-let.stderr
2020-02-09T07:09:26.4897144Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.4897500Z To only update this specific test, also pass `--test-args suggestions/type-ascription-instead-of-let.rs`
2020-02-09T07:09:26.4897592Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.4897662Z status: exit code: 1
2020-02-09T07:09:26.4897662Z status: exit code: 1
2020-02-09T07:09:26.4898577Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/type-ascription-instead-of-let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-let" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-let/auxiliary" "-A" "unused"
2020-02-09T07:09:26.4898962Z ------------------------------------------
2020-02-09T07:09:26.4899002Z 
2020-02-09T07:09:26.4899258Z ------------------------------------------
2020-02-09T07:09:26.4899311Z stderr:
2020-02-09T07:09:26.4899311Z stderr:
2020-02-09T07:09:26.4899549Z ------------------------------------------
2020-02-09T07:09:26.4899769Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4900131Z   --> /checkout/src/test/ui/suggestions/type-ascription-instead-of-let.rs:5:9
2020-02-09T07:09:26.4900189Z    |
2020-02-09T07:09:26.4900240Z LL |         temp: i32 = fun(5i32);
2020-02-09T07:09:26.4900318Z    |         ^^^^^^^^^ help: try surrounding the expression with parentheses: `(temp: i32)`
2020-02-09T07:09:26.4900585Z error[E0425]: cannot find value `temp` in this scope
2020-02-09T07:09:26.4931373Z   --> /checkout/src/test/ui/suggestions/type-ascription-instead-of-let.rs:5:9
2020-02-09T07:09:26.4931446Z    |
2020-02-09T07:09:26.4931446Z    |
2020-02-09T07:09:26.4931493Z LL |         temp: i32 = fun(5i32);
2020-02-09T07:09:26.4931608Z    |         |
2020-02-09T07:09:26.4931654Z    |         not found in this scope
2020-02-09T07:09:26.4931706Z    |         help: maybe you meant to write an assignment here: `let temp`
2020-02-09T07:09:26.4931753Z 
2020-02-09T07:09:26.4931753Z 
2020-02-09T07:09:26.4931818Z error[E0425]: cannot find value `temp` in this scope
2020-02-09T07:09:26.4932349Z   --> /checkout/src/test/ui/suggestions/type-ascription-instead-of-let.rs:7:9
2020-02-09T07:09:26.4932415Z    |
2020-02-09T07:09:26.4932460Z LL |         temp + value + 1
2020-02-09T07:09:26.4932539Z 
2020-02-09T07:09:26.4932592Z error: aborting due to 3 previous errors
2020-02-09T07:09:26.4932635Z 
2020-02-09T07:09:26.4932904Z For more information about this error, try `rustc --explain E0425`.
---
2020-02-09T07:09:26.4933571Z 
2020-02-09T07:09:26.4933618Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4933855Z +   --> $DIR/type-ascription-instead-of-path.rs:2:5
2020-02-09T07:09:26.4933929Z +    |
2020-02-09T07:09:26.4933974Z + LL |     std:io::stdin();
2020-02-09T07:09:26.4934030Z +    |     ^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(std:io::stdin())`
2020-02-09T07:09:26.4934158Z 1 error[E0433]: failed to resolve: use of undeclared type or module `io`
2020-02-09T07:09:26.4934410Z 2   --> $DIR/type-ascription-instead-of-path.rs:2:9
2020-02-09T07:09:26.4934473Z 3    |
2020-02-09T07:09:26.4934502Z 
---
2020-02-09T07:09:26.4935283Z 18 For more information about an error, try `rustc --explain E0423`.
2020-02-09T07:09:26.4935330Z 
2020-02-09T07:09:26.4935366Z 
2020-02-09T07:09:26.4935413Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.4935762Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-path/type-ascription-instead-of-path.stderr
2020-02-09T07:09:26.4936032Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.4936364Z To only update this specific test, also pass `--test-args suggestions/type-ascription-instead-of-path.rs`
2020-02-09T07:09:26.4936464Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.4936515Z status: exit code: 1
2020-02-09T07:09:26.4936515Z status: exit code: 1
2020-02-09T07:09:26.4937591Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/type-ascription-instead-of-path.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-path" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-path/auxiliary" "-A" "unused"
2020-02-09T07:09:26.4938086Z ------------------------------------------
2020-02-09T07:09:26.4938135Z 
2020-02-09T07:09:26.4938715Z ------------------------------------------
2020-02-09T07:09:26.4938771Z stderr:
2020-02-09T07:09:26.4938771Z stderr:
2020-02-09T07:09:26.4939017Z ------------------------------------------
2020-02-09T07:09:26.4939072Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4939339Z   --> /checkout/src/test/ui/suggestions/type-ascription-instead-of-path.rs:2:5
2020-02-09T07:09:26.4939407Z    |
2020-02-09T07:09:26.4939455Z LL |     std:io::stdin();
2020-02-09T07:09:26.4939522Z    |     ^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(std:io::stdin())`
2020-02-09T07:09:26.4939633Z error[E0433]: failed to resolve: use of undeclared type or module `io`
2020-02-09T07:09:26.4939926Z   --> /checkout/src/test/ui/suggestions/type-ascription-instead-of-path.rs:2:9
2020-02-09T07:09:26.4939981Z    |
2020-02-09T07:09:26.4939981Z    |
2020-02-09T07:09:26.4940040Z LL |     std:io::stdin();
2020-02-09T07:09:26.4940101Z    |         ^^ use of undeclared type or module `io`
2020-02-09T07:09:26.4940184Z error[E0423]: expected value, found crate `std`
2020-02-09T07:09:26.4940489Z   --> /checkout/src/test/ui/suggestions/type-ascription-instead-of-path.rs:2:5
2020-02-09T07:09:26.4940545Z    |
2020-02-09T07:09:26.4940545Z    |
2020-02-09T07:09:26.4940592Z LL |     std:io::stdin();
2020-02-09T07:09:26.4940871Z    |     ^^^- help: maybe you meant to write a path separator here: `::`
2020-02-09T07:09:26.4940973Z    |     not a value
2020-02-09T07:09:26.4941005Z 
2020-02-09T07:09:26.4941071Z error: aborting due to 3 previous errors
2020-02-09T07:09:26.4941103Z 
---
2020-02-09T07:09:26.4942040Z ---- [ui] ui/type-ascription.rs stdout ----
2020-02-09T07:09:26.4942076Z 
2020-02-09T07:09:26.4942322Z error: test compilation failed although it shouldn't!
2020-02-09T07:09:26.4942391Z status: exit code: 1
2020-02-09T07:09:26.4943526Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-ascription.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-ascription/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-ascription/auxiliary"
2020-02-09T07:09:26.4943887Z ------------------------------------------
2020-02-09T07:09:26.4943925Z 
2020-02-09T07:09:26.4944178Z ------------------------------------------
2020-02-09T07:09:26.4944231Z stderr:
2020-02-09T07:09:26.4944231Z stderr:
2020-02-09T07:09:26.4944451Z ------------------------------------------
2020-02-09T07:09:26.4944509Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4945855Z   --> /checkout/src/test/ui/type-ascription.rs:11:16
2020-02-09T07:09:26.4945914Z    |
2020-02-09T07:09:26.4945959Z LL | const C1: u8 = 10: u8;
2020-02-09T07:09:26.4946030Z    |                ^^^^^^ help: try surrounding the expression with parentheses: `(10: u8)`
2020-02-09T07:09:26.4946119Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4946516Z   --> /checkout/src/test/ui/type-ascription.rs:12:16
2020-02-09T07:09:26.4946580Z    |
2020-02-09T07:09:26.4946580Z    |
2020-02-09T07:09:26.4946625Z LL | const C2: [u8; 1: usize] = [1];
2020-02-09T07:09:26.4946681Z    |                ^^^^^^^^ help: try surrounding the expression with parentheses: `(1: usize)`
2020-02-09T07:09:26.4946772Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4947112Z   --> /checkout/src/test/ui/type-ascription.rs:22:20
2020-02-09T07:09:26.4947170Z    |
2020-02-09T07:09:26.4947170Z    |
2020-02-09T07:09:26.4947215Z LL |     let s = S { a: 10: u8 };
2020-02-09T07:09:26.4947269Z    |                    ^^^^^^ help: try surrounding the expression with parentheses: `(10: u8)`
2020-02-09T07:09:26.4947361Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4947598Z   --> /checkout/src/test/ui/type-ascription.rs:25:17
2020-02-09T07:09:26.4947648Z    |
2020-02-09T07:09:26.4947648Z    |
2020-02-09T07:09:26.4947724Z LL |     let mut v = arr.iter().cloned().collect(): Vec<_>;
2020-02-09T07:09:26.4947794Z    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(arr.iter().cloned().collect(): Vec<_>)`
2020-02-09T07:09:26.4947904Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4948163Z   --> /checkout/src/test/ui/type-ascription.rs:29:13
2020-02-09T07:09:26.4948225Z    |
2020-02-09T07:09:26.4948281Z LL |     let a = 1: u8;
2020-02-09T07:09:26.4948281Z LL |     let a = 1: u8;
2020-02-09T07:09:26.4948339Z    |             ^^^^^ help: try surrounding the expression with parentheses: `(1: u8)`
2020-02-09T07:09:26.4948426Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4948692Z   --> /checkout/src/test/ui/type-ascription.rs:30:13
2020-02-09T07:09:26.4948744Z    |
2020-02-09T07:09:26.4948744Z    |
2020-02-09T07:09:26.4948792Z LL |     let b = a.into(): u16;
2020-02-09T07:09:26.4948866Z    |             ^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(a.into(): u16)`
2020-02-09T07:09:26.4948957Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4949213Z   --> /checkout/src/test/ui/type-ascription.rs:37:5
2020-02-09T07:09:26.4949276Z    |
2020-02-09T07:09:26.4949276Z    |
2020-02-09T07:09:26.4949328Z LL |     v: Vec<u8> = vec![1, 2, 3]; // Place expression type ascription
2020-02-09T07:09:26.4949389Z    |     ^^^^^^^^^^ help: try surrounding the expression with parentheses: `(v: Vec<u8>)`
2020-02-09T07:09:26.4949497Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4949755Z   --> /checkout/src/test/ui/type-ascription.rs:19:16
2020-02-09T07:09:26.4949820Z    |
2020-02-09T07:09:26.4949820Z    |
2020-02-09T07:09:26.4949870Z LL |     assert_eq!(C1.into(): i32, 10);
2020-02-09T07:09:26.4949929Z    |                ^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(C1.into(): i32)`
2020-02-09T07:09:26.4950034Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4950468Z   --> /checkout/src/test/ui/type-ascription.rs:31:18
2020-02-09T07:09:26.4950515Z    |
2020-02-09T07:09:26.4950515Z    |
2020-02-09T07:09:26.4950567Z LL |     assert_eq!(v[a.into(): usize], 2);
2020-02-09T07:09:26.4950623Z    |                  ^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(a.into(): usize)`
2020-02-09T07:09:26.4950704Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4950961Z   --> /checkout/src/test/ui/type-ascription.rs:34:19
2020-02-09T07:09:26.4951169Z    |
2020-02-09T07:09:26.4951169Z    |
2020-02-09T07:09:26.4951210Z LL |     assert_eq!(b, 1: u16);
2020-02-09T07:09:26.4951275Z    |                   ^^^^^^ help: try surrounding the expression with parentheses: `(1: u16)`
2020-02-09T07:09:26.4951348Z error: aborting due to 10 previous errors
2020-02-09T07:09:26.4951376Z 
2020-02-09T07:09:26.4951413Z 
2020-02-09T07:09:26.4951620Z ------------------------------------------
---
2020-02-09T07:09:26.4952078Z 
2020-02-09T07:09:26.4952120Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4952367Z +   --> $DIR/issue-34255-1.rs:7:9
2020-02-09T07:09:26.4952414Z +    |
2020-02-09T07:09:26.4952540Z + LL |         input_cells: Vec::new()
2020-02-09T07:09:26.4952602Z +    |         ^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(input_cells: Vec::new())`
2020-02-09T07:09:26.4952697Z 1 error[E0425]: cannot find value `input_cells` in this scope
2020-02-09T07:09:26.4952936Z 2   --> $DIR/issue-34255-1.rs:7:9
2020-02-09T07:09:26.4952990Z 3    |
2020-02-09T07:09:26.4953016Z 
---
2020-02-09T07:09:26.4953971Z 22 For more information about an error, try `rustc --explain E0107`.
2020-02-09T07:09:26.4954077Z 
2020-02-09T07:09:26.4954104Z 
2020-02-09T07:09:26.4954153Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.4954472Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-34255-1/issue-34255-1.stderr
2020-02-09T07:09:26.4954733Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.4955007Z To only update this specific test, also pass `--test-args type/ascription/issue-34255-1.rs`
2020-02-09T07:09:26.4955097Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.4955144Z status: exit code: 1
2020-02-09T07:09:26.4955144Z status: exit code: 1
2020-02-09T07:09:26.4956152Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/ascription/issue-34255-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-34255-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-34255-1/auxiliary" "-A" "unused"
2020-02-09T07:09:26.4956494Z ------------------------------------------
2020-02-09T07:09:26.4956542Z 
2020-02-09T07:09:26.4956766Z ------------------------------------------
2020-02-09T07:09:26.4956814Z stderr:
2020-02-09T07:09:26.4956814Z stderr:
2020-02-09T07:09:26.4957268Z ------------------------------------------
2020-02-09T07:09:26.4957368Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4957673Z   --> /checkout/src/test/ui/type/ascription/issue-34255-1.rs:7:9
2020-02-09T07:09:26.4957728Z    |
2020-02-09T07:09:26.4957790Z LL |         input_cells: Vec::new()
2020-02-09T07:09:26.4957847Z    |         ^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(input_cells: Vec::new())`
2020-02-09T07:09:26.4957956Z error[E0425]: cannot find value `input_cells` in this scope
2020-02-09T07:09:26.4958219Z   --> /checkout/src/test/ui/type/ascription/issue-34255-1.rs:7:9
2020-02-09T07:09:26.4958269Z    |
2020-02-09T07:09:26.4958321Z LL |         input_cells: Vec::new()
2020-02-09T07:09:26.4958321Z LL |         input_cells: Vec::new()
2020-02-09T07:09:26.4958371Z    |         ^^^^^^^^^^^ a field by this name exists in `Self`
2020-02-09T07:09:26.4958402Z 
2020-02-09T07:09:26.4958452Z error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
2020-02-09T07:09:26.4958838Z   --> /checkout/src/test/ui/type/ascription/issue-34255-1.rs:7:27
2020-02-09T07:09:26.4958899Z    |
2020-02-09T07:09:26.4958943Z LL |         input_cells: Vec::new()
2020-02-09T07:09:26.4959005Z    |                           ^^^^^ only `Fn` traits may use parentheses
2020-02-09T07:09:26.4959085Z error[E0107]: wrong number of type arguments: expected 1, found 0
2020-02-09T07:09:26.4959398Z   --> /checkout/src/test/ui/type/ascription/issue-34255-1.rs:7:22
2020-02-09T07:09:26.4959541Z    |
2020-02-09T07:09:26.4959586Z LL |         input_cells: Vec::new()
---
2020-02-09T07:09:26.4960923Z 
2020-02-09T07:09:26.4960976Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4961179Z +   --> $DIR/issue-47666.rs:2:13
2020-02-09T07:09:26.4961233Z +    |
2020-02-09T07:09:26.4961292Z + LL |     let _ = Option:Some(vec![0, 1]);
2020-02-09T07:09:26.4961347Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(Option:Some(vec![0, 1]))`
2020-02-09T07:09:26.4961451Z 1 error: expected type, found reserved keyword `box`
2020-02-09T07:09:26.4961655Z 2   --> $DIR/issue-47666.rs:2:25
2020-02-09T07:09:26.4961700Z 3    |
2020-02-09T07:09:26.4961726Z 
2020-02-09T07:09:26.4961726Z 
2020-02-09T07:09:26.4962286Z 13    = note: for more information, see ***/issues/23416
2020-02-09T07:09:26.4962671Z 15 
2020-02-09T07:09:26.4962890Z - error: aborting due to previous error
2020-02-09T07:09:26.4962938Z + error: aborting due to 2 previous errors
2020-02-09T07:09:26.4962979Z 17 
2020-02-09T07:09:26.4962979Z 17 
2020-02-09T07:09:26.4963025Z 18 
2020-02-09T07:09:26.4963051Z 
2020-02-09T07:09:26.4963085Z 
2020-02-09T07:09:26.4963128Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.4963437Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-47666/issue-47666.stderr
2020-02-09T07:09:26.4963672Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.4963923Z To only update this specific test, also pass `--test-args type/ascription/issue-47666.rs`
2020-02-09T07:09:26.4964016Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.4964059Z status: exit code: 1
2020-02-09T07:09:26.4964059Z status: exit code: 1
2020-02-09T07:09:26.4965054Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/ascription/issue-47666.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-47666" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-47666/auxiliary" "-A" "unused"
2020-02-09T07:09:26.4965387Z ------------------------------------------
2020-02-09T07:09:26.4965421Z 
2020-02-09T07:09:26.4965633Z ------------------------------------------
2020-02-09T07:09:26.4965679Z stderr:
2020-02-09T07:09:26.4965679Z stderr:
2020-02-09T07:09:26.4966124Z ------------------------------------------
2020-02-09T07:09:26.4966178Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4966538Z   --> /checkout/src/test/ui/type/ascription/issue-47666.rs:2:13
2020-02-09T07:09:26.4966615Z    |
2020-02-09T07:09:26.4966667Z LL |     let _ = Option:Some(vec![0, 1]); //~ ERROR expected type, found
2020-02-09T07:09:26.4966729Z    |             ^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression with parentheses: `(Option:Some(vec![0, 1]))`
2020-02-09T07:09:26.4966879Z error: expected type, found reserved keyword `box`
2020-02-09T07:09:26.4967172Z   --> /checkout/src/test/ui/type/ascription/issue-47666.rs:2:25
2020-02-09T07:09:26.4967238Z    |
2020-02-09T07:09:26.4967238Z    |
2020-02-09T07:09:26.4967288Z LL |     let _ = Option:Some(vec![0, 1]); //~ ERROR expected type, found
2020-02-09T07:09:26.4967575Z    |                   |     |
2020-02-09T07:09:26.4967622Z    |                   |     expected type
2020-02-09T07:09:26.4967671Z    |                   |     in this macro invocation
2020-02-09T07:09:26.4967945Z    |                   |     this macro call doesn't expand to a type
2020-02-09T07:09:26.4967945Z    |                   |     this macro call doesn't expand to a type
2020-02-09T07:09:26.4968004Z    |                   help: maybe write a path separator here: `::`
2020-02-09T07:09:26.4968151Z    |
2020-02-09T07:09:26.4968205Z    = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
2020-02-09T07:09:26.4968542Z    = note: for more information, see ***/issues/23416
2020-02-09T07:09:26.4974440Z 
2020-02-09T07:09:26.4974489Z error: aborting due to 2 previous errors
2020-02-09T07:09:26.4974518Z 
2020-02-09T07:09:26.4974544Z 
2020-02-09T07:09:26.4974544Z 
2020-02-09T07:09:26.4974785Z ------------------------------------------
2020-02-09T07:09:26.4974832Z 
2020-02-09T07:09:26.4974858Z 
2020-02-09T07:09:26.4975090Z ---- [ui] ui/type/ascription/issue-60933.rs stdout ----
2020-02-09T07:09:26.4975140Z diff of stderr:
2020-02-09T07:09:26.4975182Z 
2020-02-09T07:09:26.4975423Z - error: expected one of `!`, `::`, or `;`, found `(`
2020-02-09T07:09:26.4975704Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4975912Z +   --> $DIR/issue-60933.rs:2:20
2020-02-09T07:09:26.4975960Z 3    |
2020-02-09T07:09:26.4975960Z 3    |
2020-02-09T07:09:26.4976005Z 4 LL |     let u: usize = std::mem:size_of::<u32>();
2020-02-09T07:09:26.4976473Z -    |                            -              ^ expected one of `!`, `::`, or `;`
2020-02-09T07:09:26.4977030Z -    |                            help: maybe write a path separator here: `::`
2020-02-09T07:09:26.4977275Z +    |                    -----------------------^^
2020-02-09T07:09:26.4977326Z +    |                    |
2020-02-09T07:09:26.4977326Z +    |                    |
2020-02-09T07:09:26.4977384Z +    |                    help: try surrounding the expression with parentheses: `(std::mem:size_of::<u32>)`
2020-02-09T07:09:26.4977500Z + error[E0423]: expected value, found module `std::mem`
2020-02-09T07:09:26.4977724Z +   --> $DIR/issue-60933.rs:2:20
2020-02-09T07:09:26.4977786Z 8    |
2020-02-09T07:09:26.4977786Z 8    |
2020-02-09T07:09:26.4978072Z -    = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
2020-02-09T07:09:26.4978395Z -    = note: for more information, see ***/issues/23416
2020-02-09T07:09:26.4978478Z + LL |     let u: usize = std::mem:size_of::<u32>();
2020-02-09T07:09:26.4978767Z +    |                    ^^^^^^^^- help: maybe you meant to write a path separator here: `::`
2020-02-09T07:09:26.4978881Z +    |                    not a value
2020-02-09T07:09:26.4978926Z 11 
2020-02-09T07:09:26.4979143Z - error: aborting due to previous error
2020-02-09T07:09:26.4979206Z + error[E0412]: cannot find type `size_of` in this scope
2020-02-09T07:09:26.4979206Z + error[E0412]: cannot find type `size_of` in this scope
2020-02-09T07:09:26.4979420Z +   --> $DIR/issue-60933.rs:2:29
2020-02-09T07:09:26.4979467Z +    |
2020-02-09T07:09:26.4979679Z + LL |     let u: usize = std::mem:size_of::<u32>();
2020-02-09T07:09:26.4979988Z +    |                            -^^^^^^^ not found in this scope
2020-02-09T07:09:26.4980098Z +    |                            help: maybe you meant to write a path separator here: `::`
2020-02-09T07:09:26.4980160Z 13 
2020-02-09T07:09:26.4980298Z + error: aborting due to 3 previous errors
2020-02-09T07:09:26.4980342Z + 
2020-02-09T07:09:26.4980342Z + 
2020-02-09T07:09:26.4980401Z + Some errors have detailed explanations: E0412, E0423.
2020-02-09T07:09:26.4980691Z + For more information about an error, try `rustc --explain E0412`.
2020-02-09T07:09:26.4980741Z 14 
2020-02-09T07:09:26.4980770Z 
2020-02-09T07:09:26.4980812Z 
2020-02-09T07:09:26.4980859Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.4981329Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-60933/issue-60933.stderr
2020-02-09T07:09:26.4981757Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.4982048Z To only update this specific test, also pass `--test-args type/ascription/issue-60933.rs`
2020-02-09T07:09:26.4982131Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.4982191Z status: exit code: 1
2020-02-09T07:09:26.4982191Z status: exit code: 1
2020-02-09T07:09:26.4982990Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/ascription/issue-60933.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-60933" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-60933/auxiliary" "-A" "unused"
2020-02-09T07:09:26.4983351Z ------------------------------------------
2020-02-09T07:09:26.4983388Z 
2020-02-09T07:09:26.4983635Z ------------------------------------------
2020-02-09T07:09:26.4983683Z stderr:
2020-02-09T07:09:26.4983683Z stderr:
2020-02-09T07:09:26.4983903Z ------------------------------------------
2020-02-09T07:09:26.4983970Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4984232Z   --> /checkout/src/test/ui/type/ascription/issue-60933.rs:2:20
2020-02-09T07:09:26.4984284Z    |
2020-02-09T07:09:26.4984344Z LL |     let u: usize = std::mem:size_of::<u32>();
2020-02-09T07:09:26.4984626Z    |                    |
2020-02-09T07:09:26.4984626Z    |                    |
2020-02-09T07:09:26.4984688Z    |                    help: try surrounding the expression with parentheses: `(std::mem:size_of::<u32>)`
2020-02-09T07:09:26.4984774Z error[E0423]: expected value, found module `std::mem`
2020-02-09T07:09:26.4985043Z   --> /checkout/src/test/ui/type/ascription/issue-60933.rs:2:20
2020-02-09T07:09:26.4985094Z    |
2020-02-09T07:09:26.4985094Z    |
2020-02-09T07:09:26.4985140Z LL |     let u: usize = std::mem:size_of::<u32>();
2020-02-09T07:09:26.4985417Z    |                    ^^^^^^^^- help: maybe you meant to write a path separator here: `::`
2020-02-09T07:09:26.4985523Z    |                    not a value
2020-02-09T07:09:26.4985561Z 
2020-02-09T07:09:26.4985620Z error[E0412]: cannot find type `size_of` in this scope
2020-02-09T07:09:26.4985874Z   --> /checkout/src/test/ui/type/ascription/issue-60933.rs:2:29
2020-02-09T07:09:26.4985874Z   --> /checkout/src/test/ui/type/ascription/issue-60933.rs:2:29
2020-02-09T07:09:26.4985924Z    |
2020-02-09T07:09:26.4985985Z LL |     let u: usize = std::mem:size_of::<u32>();
2020-02-09T07:09:26.4986639Z    |                            -^^^^^^^ not found in this scope
2020-02-09T07:09:26.4986745Z    |                            help: maybe you meant to write a path separator here: `::`
2020-02-09T07:09:26.4986791Z 
2020-02-09T07:09:26.4986926Z error: aborting due to 3 previous errors
2020-02-09T07:09:26.4986964Z 
---
2020-02-09T07:09:26.4987737Z 
2020-02-09T07:09:26.4988008Z ---- [ui] ui/type/ascription/issue-54516.rs stdout ----
2020-02-09T07:09:26.4988060Z diff of stderr:
2020-02-09T07:09:26.4988089Z 
2020-02-09T07:09:26.4988329Z - error: expected one of `!`, `,`, or `::`, found `(`
2020-02-09T07:09:26.4988597Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.4988820Z +   --> $DIR/issue-54516.rs:4:20
2020-02-09T07:09:26.4988868Z 3    |
2020-02-09T07:09:26.4988868Z 3    |
2020-02-09T07:09:26.4988916Z 4 LL |     println!("{}", std::mem:size_of::<BTreeMap<u32, u32>>());
2020-02-09T07:09:26.4989224Z -    |                            -                             ^ expected one of `!`, `,`, or `::`
2020-02-09T07:09:26.4989707Z -    |                            help: maybe write a path separator here: `::`
2020-02-09T07:09:26.4989968Z +    |                    --------------------------------------^^
2020-02-09T07:09:26.4990030Z +    |                    |
2020-02-09T07:09:26.4990030Z +    |                    |
2020-02-09T07:09:26.4990087Z +    |                    help: try surrounding the expression with parentheses: `(std::mem:size_of::<BTreeMap<u32, u32>>)`
2020-02-09T07:09:26.4990201Z + error[E0423]: expected value, found module `std::mem`
2020-02-09T07:09:26.4990580Z +   --> $DIR/issue-54516.rs:4:20
2020-02-09T07:09:26.4990626Z 8    |
2020-02-09T07:09:26.4990626Z 8    |
2020-02-09T07:09:26.4990908Z -    = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
2020-02-09T07:09:26.4991216Z -    = note: for more information, see ***/issues/23416
2020-02-09T07:09:26.4991291Z + LL |     println!("{}", std::mem:size_of::<BTreeMap<u32, u32>>());
2020-02-09T07:09:26.4991587Z +    |                    ^^^^^^^^- help: maybe you meant to write a path separator here: `::`
2020-02-09T07:09:26.4991703Z +    |                    not a value
2020-02-09T07:09:26.4991756Z 11 
2020-02-09T07:09:26.4991986Z - error: aborting due to previous error
2020-02-09T07:09:26.4992040Z + error[E0412]: cannot find type `size_of` in this scope
2020-02-09T07:09:26.4992040Z + error[E0412]: cannot find type `size_of` in this scope
2020-02-09T07:09:26.4992278Z +   --> $DIR/issue-54516.rs:4:29
2020-02-09T07:09:26.4992328Z +    |
2020-02-09T07:09:26.4992377Z + LL |     println!("{}", std::mem:size_of::<BTreeMap<u32, u32>>());
2020-02-09T07:09:26.4992642Z +    |                            -^^^^^^^ not found in this scope
2020-02-09T07:09:26.4992752Z +    |                            help: maybe you meant to write a path separator here: `::`
2020-02-09T07:09:26.4992821Z 13 
2020-02-09T07:09:26.4992868Z + error: aborting due to 3 previous errors
2020-02-09T07:09:26.4992912Z + 
2020-02-09T07:09:26.4992912Z + 
2020-02-09T07:09:26.4992967Z + Some errors have detailed explanations: E0412, E0423.
2020-02-09T07:09:26.4993234Z + For more information about an error, try `rustc --explain E0412`.
2020-02-09T07:09:26.4993285Z 14 
2020-02-09T07:09:26.4993314Z 
2020-02-09T07:09:26.4993364Z 
2020-02-09T07:09:26.4993412Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.4994015Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-54516/issue-54516.stderr
2020-02-09T07:09:26.4994306Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.4994590Z To only update this specific test, also pass `--test-args type/ascription/issue-54516.rs`
2020-02-09T07:09:26.4994677Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.4994738Z status: exit code: 1
2020-02-09T07:09:26.4994738Z status: exit code: 1
2020-02-09T07:09:26.4995685Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/ascription/issue-54516.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-54516" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-54516/auxiliary" "-A" "unused"
2020-02-09T07:09:26.4996150Z ------------------------------------------
2020-02-09T07:09:26.4996188Z 
2020-02-09T07:09:26.4996436Z ------------------------------------------
2020-02-09T07:09:26.4996747Z stderr:
2020-02-09T07:09:26.4996747Z stderr:
2020-02-09T07:09:26.4996977Z ------------------------------------------
2020-02-09T07:09:26.4997048Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.4997304Z   --> /checkout/src/test/ui/type/ascription/issue-54516.rs:4:20
2020-02-09T07:09:26.4997357Z    |
2020-02-09T07:09:26.4997417Z LL |     println!("{}", std::mem:size_of::<BTreeMap<u32, u32>>());
2020-02-09T07:09:26.4997717Z    |                    |
2020-02-09T07:09:26.4997717Z    |                    |
2020-02-09T07:09:26.4997798Z    |                    help: try surrounding the expression with parentheses: `(std::mem:size_of::<BTreeMap<u32, u32>>)`
2020-02-09T07:09:26.4997887Z error[E0423]: expected value, found module `std::mem`
2020-02-09T07:09:26.4998138Z   --> /checkout/src/test/ui/type/ascription/issue-54516.rs:4:20
2020-02-09T07:09:26.4998204Z    |
2020-02-09T07:09:26.4998204Z    |
2020-02-09T07:09:26.4998252Z LL |     println!("{}", std::mem:size_of::<BTreeMap<u32, u32>>());
2020-02-09T07:09:26.4998524Z    |                    ^^^^^^^^- help: maybe you meant to write a path separator here: `::`
2020-02-09T07:09:26.4998646Z    |                    not a value
2020-02-09T07:09:26.4998676Z 
2020-02-09T07:09:26.4998733Z error[E0412]: cannot find type `size_of` in this scope
2020-02-09T07:09:26.4998986Z   --> /checkout/src/test/ui/type/ascription/issue-54516.rs:4:29
2020-02-09T07:09:26.4998986Z   --> /checkout/src/test/ui/type/ascription/issue-54516.rs:4:29
2020-02-09T07:09:26.4999036Z    |
2020-02-09T07:09:26.4999083Z LL |     println!("{}", std::mem:size_of::<BTreeMap<u32, u32>>());
2020-02-09T07:09:26.4999352Z    |                            -^^^^^^^ not found in this scope
2020-02-09T07:09:26.4999459Z    |                            help: maybe you meant to write a path separator here: `::`
2020-02-09T07:09:26.4999510Z 
2020-02-09T07:09:26.4999555Z error: aborting due to 3 previous errors
2020-02-09T07:09:26.4999585Z 
---
2020-02-09T07:09:26.5000620Z 
2020-02-09T07:09:26.5000681Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5000938Z +   --> $DIR/type-ascription-precedence.rs:29:5
2020-02-09T07:09:26.5000987Z +    |
2020-02-09T07:09:26.5001045Z + LL |     &S: &S; // OK
2020-02-09T07:09:26.5001098Z +    |     ^^^^^^ help: try surrounding the expression with parentheses: `(&S: &S)`
2020-02-09T07:09:26.5001203Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5001433Z +   --> $DIR/type-ascription-precedence.rs:30:5
2020-02-09T07:09:26.5001481Z +    |
2020-02-09T07:09:26.5001481Z +    |
2020-02-09T07:09:26.5001525Z + LL |     (&S): &S; // OK
2020-02-09T07:09:26.5001676Z +    |     ^^^^^^^^ help: try surrounding the expression with parentheses: `((&S): &S)`
2020-02-09T07:09:26.5001779Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5002210Z +   --> $DIR/type-ascription-precedence.rs:31:7
2020-02-09T07:09:26.5002258Z +    |
2020-02-09T07:09:26.5002258Z +    |
2020-02-09T07:09:26.5002299Z + LL |     &(S: &S);
2020-02-09T07:09:26.5002446Z +    |       ^^^^^ help: try surrounding the expression with parentheses: `(S: &S)`
2020-02-09T07:09:26.5002535Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5002804Z +   --> $DIR/type-ascription-precedence.rs:33:5
2020-02-09T07:09:26.5002852Z +    |
2020-02-09T07:09:26.5002852Z +    |
2020-02-09T07:09:26.5002894Z + LL |     *S: Z; // OK
2020-02-09T07:09:26.5002943Z +    |     ^^^^^ help: try surrounding the expression with parentheses: `(*S: Z)`
2020-02-09T07:09:26.5003050Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5003279Z +   --> $DIR/type-ascription-precedence.rs:34:5
2020-02-09T07:09:26.5003340Z +    |
2020-02-09T07:09:26.5003340Z +    |
2020-02-09T07:09:26.5003382Z + LL |     (*S): Z; // OK
2020-02-09T07:09:26.5003433Z +    |     ^^^^^^^ help: try surrounding the expression with parentheses: `((*S): Z)`
2020-02-09T07:09:26.5003532Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5003766Z +   --> $DIR/type-ascription-precedence.rs:35:7
2020-02-09T07:09:26.5003822Z +    |
2020-02-09T07:09:26.5003822Z +    |
2020-02-09T07:09:26.5003865Z + LL |     *(S: Z);
2020-02-09T07:09:26.5003914Z +    |       ^^^^ help: try surrounding the expression with parentheses: `(S: Z)`
2020-02-09T07:09:26.5004017Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5004241Z +   --> $DIR/type-ascription-precedence.rs:38:5
2020-02-09T07:09:26.5004287Z +    |
2020-02-09T07:09:26.5004287Z +    |
2020-02-09T07:09:26.5004492Z + LL |     -S: Z; // OK
2020-02-09T07:09:26.5004740Z +    |     ^^^^^ help: try surrounding the expression with parentheses: `(-S: Z)`
2020-02-09T07:09:26.5004855Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5005079Z +   --> $DIR/type-ascription-precedence.rs:39:5
2020-02-09T07:09:26.5005126Z +    |
2020-02-09T07:09:26.5005126Z +    |
2020-02-09T07:09:26.5005314Z + LL |     (-S): Z; // OK
2020-02-09T07:09:26.5005579Z +    |     ^^^^^^^ help: try surrounding the expression with parentheses: `((-S): Z)`
2020-02-09T07:09:26.5005680Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5005911Z +   --> $DIR/type-ascription-precedence.rs:40:7
2020-02-09T07:09:26.5005958Z +    |
2020-02-09T07:09:26.5005958Z +    |
2020-02-09T07:09:26.5006142Z + LL |     -(S: Z);
2020-02-09T07:09:26.5006202Z +    |       ^^^^ help: try surrounding the expression with parentheses: `(S: Z)`
2020-02-09T07:09:26.5006291Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5006523Z +   --> $DIR/type-ascription-precedence.rs:43:9
2020-02-09T07:09:26.5006569Z +    |
2020-02-09T07:09:26.5006569Z +    |
2020-02-09T07:09:26.5006618Z + LL |     S + Z: Z; // OK
2020-02-09T07:09:26.5006668Z +    |         ^^^^ help: try surrounding the expression with parentheses: `(Z: Z)`
2020-02-09T07:09:26.5006775Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5007003Z +   --> $DIR/type-ascription-precedence.rs:44:10
2020-02-09T07:09:26.5007071Z +    |
2020-02-09T07:09:26.5007071Z +    |
2020-02-09T07:09:26.5007113Z + LL |     S + (Z: Z); // OK
2020-02-09T07:09:26.5007163Z +    |          ^^^^ help: try surrounding the expression with parentheses: `(Z: Z)`
2020-02-09T07:09:26.5007263Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5007488Z +   --> $DIR/type-ascription-precedence.rs:45:5
2020-02-09T07:09:26.5007544Z +    |
2020-02-09T07:09:26.5007544Z +    |
2020-02-09T07:09:26.5007586Z + LL |     (S + Z): Z;
2020-02-09T07:09:26.5007637Z +    |     ^^^^^^^^^^ help: try surrounding the expression with parentheses: `((S + Z): Z)`
2020-02-09T07:09:26.5008009Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5008281Z +   --> $DIR/type-ascription-precedence.rs:47:9
2020-02-09T07:09:26.5008330Z +    |
2020-02-09T07:09:26.5008330Z +    |
2020-02-09T07:09:26.5008385Z + LL |     S * Z: Z; // OK
2020-02-09T07:09:26.5008437Z +    |         ^^^^ help: try surrounding the expression with parentheses: `(Z: Z)`
2020-02-09T07:09:26.5008632Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5008899Z +   --> $DIR/type-ascription-precedence.rs:48:10
2020-02-09T07:09:26.5008949Z +    |
2020-02-09T07:09:26.5008949Z +    |
2020-02-09T07:09:26.5009008Z + LL |     S * (Z: Z); // OK
2020-02-09T07:09:26.5009061Z +    |          ^^^^ help: try surrounding the expression with parentheses: `(Z: Z)`
2020-02-09T07:09:26.5009153Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5009399Z +   --> $DIR/type-ascription-precedence.rs:49:5
2020-02-09T07:09:26.5009447Z +    |
2020-02-09T07:09:26.5009447Z +    |
2020-02-09T07:09:26.5009499Z + LL |     (S * Z): Z;
2020-02-09T07:09:26.5009561Z +    |     ^^^^^^^^^^ help: try surrounding the expression with parentheses: `((S * Z): Z)`
2020-02-09T07:09:26.5009653Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5009898Z +   --> $DIR/type-ascription-precedence.rs:51:10
2020-02-09T07:09:26.5010107Z +    |
2020-02-09T07:09:26.5010107Z +    |
2020-02-09T07:09:26.5010157Z + LL |     S .. S: S; // OK
2020-02-09T07:09:26.5010222Z +    |          ^^^^ help: try surrounding the expression with parentheses: `(S: S)`
2020-02-09T07:09:26.5010311Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5010537Z +   --> $DIR/type-ascription-precedence.rs:52:11
2020-02-09T07:09:26.5010594Z +    |
2020-02-09T07:09:26.5010594Z +    |
2020-02-09T07:09:26.5010637Z + LL |     S .. (S: S); // OK
2020-02-09T07:09:26.5010687Z +    |           ^^^^ help: try surrounding the expression with parentheses: `(S: S)`
2020-02-09T07:09:26.5010791Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5011019Z +   --> $DIR/type-ascription-precedence.rs:53:5
2020-02-09T07:09:26.5011074Z +    |
2020-02-09T07:09:26.5011074Z +    |
2020-02-09T07:09:26.5011116Z + LL |     (S .. S): S;
2020-02-09T07:09:26.5011167Z +    |     ^^^^^^^^^^^ help: try surrounding the expression with parentheses: `((S .. S): S)`
2020-02-09T07:09:26.5011271Z 1 error[E0308]: mismatched types
2020-02-09T07:09:26.5011495Z 2   --> $DIR/type-ascription-precedence.rs:31:7
2020-02-09T07:09:26.5011540Z 3    |
2020-02-09T07:09:26.5011579Z 
---
2020-02-09T07:09:26.5012348Z 57 For more information about an error, try `rustc --explain E0308`.
2020-02-09T07:09:26.5012383Z 
2020-02-09T07:09:26.5012409Z 
2020-02-09T07:09:26.5012466Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.5012785Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-ascription-precedence/type-ascription-precedence.stderr
2020-02-09T07:09:26.5013040Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.5013497Z To only update this specific test, also pass `--test-args type/type-ascription-precedence.rs`
2020-02-09T07:09:26.5013580Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.5013633Z status: exit code: 1
2020-02-09T07:09:26.5013633Z status: exit code: 1
2020-02-09T07:09:26.5014556Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-ascription-precedence.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-ascription-precedence" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-ascription-precedence/auxiliary" "-A" "unused"
2020-02-09T07:09:26.5014980Z ------------------------------------------
2020-02-09T07:09:26.5015017Z 
2020-02-09T07:09:26.5015254Z ------------------------------------------
2020-02-09T07:09:26.5015303Z stderr:
2020-02-09T07:09:26.5015303Z stderr:
2020-02-09T07:09:26.5015520Z ------------------------------------------
2020-02-09T07:09:26.5015572Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5015838Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:29:5
2020-02-09T07:09:26.5015890Z    |
2020-02-09T07:09:26.5015936Z LL |     &S: &S; // OK
2020-02-09T07:09:26.5016012Z    |     ^^^^^^ help: try surrounding the expression with parentheses: `(&S: &S)`
2020-02-09T07:09:26.5016093Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5016363Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:30:5
2020-02-09T07:09:26.5016413Z    |
2020-02-09T07:09:26.5016413Z    |
2020-02-09T07:09:26.5016456Z LL |     (&S): &S; // OK
2020-02-09T07:09:26.5016527Z    |     ^^^^^^^^ help: try surrounding the expression with parentheses: `((&S): &S)`
2020-02-09T07:09:26.5016613Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5016887Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:31:7
2020-02-09T07:09:26.5016954Z    |
2020-02-09T07:09:26.5016954Z    |
2020-02-09T07:09:26.5017005Z LL |     &(S: &S); //~ ERROR mismatched types
2020-02-09T07:09:26.5017060Z    |       ^^^^^ help: try surrounding the expression with parentheses: `(S: &S)`
2020-02-09T07:09:26.5017172Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5017451Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:33:5
2020-02-09T07:09:26.5017504Z    |
2020-02-09T07:09:26.5017504Z    |
2020-02-09T07:09:26.5017564Z LL |     *S: Z; // OK
2020-02-09T07:09:26.5017619Z    |     ^^^^^ help: try surrounding the expression with parentheses: `(*S: Z)`
2020-02-09T07:09:26.5017719Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5018001Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:34:5
2020-02-09T07:09:26.5018052Z    |
2020-02-09T07:09:26.5018052Z    |
2020-02-09T07:09:26.5018100Z LL |     (*S): Z; // OK
2020-02-09T07:09:26.5018167Z    |     ^^^^^^^ help: try surrounding the expression with parentheses: `((*S): Z)`
2020-02-09T07:09:26.5018252Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5018535Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:35:7
2020-02-09T07:09:26.5018589Z    |
2020-02-09T07:09:26.5018589Z    |
2020-02-09T07:09:26.5018647Z LL |     *(S: Z); //~ ERROR mismatched types
2020-02-09T07:09:26.5018717Z    |       ^^^^ help: try surrounding the expression with parentheses: `(S: Z)`
2020-02-09T07:09:26.5018802Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5019075Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:38:5
2020-02-09T07:09:26.5019145Z    |
2020-02-09T07:09:26.5019145Z    |
2020-02-09T07:09:26.5019372Z LL |     -S: Z; // OK
2020-02-09T07:09:26.5019788Z    |     ^^^^^ help: try surrounding the expression with parentheses: `(-S: Z)`
2020-02-09T07:09:26.5019886Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5020127Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:39:5
2020-02-09T07:09:26.5020177Z    |
2020-02-09T07:09:26.5020177Z    |
2020-02-09T07:09:26.5020384Z LL |     (-S): Z; // OK
2020-02-09T07:09:26.5020636Z    |     ^^^^^^^ help: try surrounding the expression with parentheses: `((-S): Z)`
2020-02-09T07:09:26.5020799Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5021091Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:40:7
2020-02-09T07:09:26.5021140Z    |
2020-02-09T07:09:26.5021140Z    |
2020-02-09T07:09:26.5021351Z LL |     -(S: Z); //~ ERROR mismatched types
2020-02-09T07:09:26.5021416Z    |       ^^^^ help: try surrounding the expression with parentheses: `(S: Z)`
2020-02-09T07:09:26.5021577Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5022073Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:43:9
2020-02-09T07:09:26.5022124Z    |
2020-02-09T07:09:26.5022124Z    |
2020-02-09T07:09:26.5022167Z LL |     S + Z: Z; // OK
2020-02-09T07:09:26.5022219Z    |         ^^^^ help: try surrounding the expression with parentheses: `(Z: Z)`
2020-02-09T07:09:26.5022314Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5022568Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:44:10
2020-02-09T07:09:26.5022633Z    |
2020-02-09T07:09:26.5022633Z    |
2020-02-09T07:09:26.5022686Z LL |     S + (Z: Z); // OK
2020-02-09T07:09:26.5022738Z    |          ^^^^ help: try surrounding the expression with parentheses: `(Z: Z)`
2020-02-09T07:09:26.5022837Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5023091Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:45:5
2020-02-09T07:09:26.5023149Z    |
2020-02-09T07:09:26.5023149Z    |
2020-02-09T07:09:26.5023203Z LL |     (S + Z): Z; //~ ERROR mismatched types
2020-02-09T07:09:26.5023259Z    |     ^^^^^^^^^^ help: try surrounding the expression with parentheses: `((S + Z): Z)`
2020-02-09T07:09:26.5023355Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5023608Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:47:9
2020-02-09T07:09:26.5023657Z    |
2020-02-09T07:09:26.5023657Z    |
2020-02-09T07:09:26.5023699Z LL |     S * Z: Z; // OK
2020-02-09T07:09:26.5023765Z    |         ^^^^ help: try surrounding the expression with parentheses: `(Z: Z)`
2020-02-09T07:09:26.5023852Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5024304Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:48:10
2020-02-09T07:09:26.5024352Z    |
2020-02-09T07:09:26.5024352Z    |
2020-02-09T07:09:26.5024393Z LL |     S * (Z: Z); // OK
2020-02-09T07:09:26.5024459Z    |          ^^^^ help: try surrounding the expression with parentheses: `(Z: Z)`
2020-02-09T07:09:26.5024543Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5024786Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:49:5
2020-02-09T07:09:26.5024847Z    |
2020-02-09T07:09:26.5024847Z    |
2020-02-09T07:09:26.5024891Z LL |     (S * Z): Z; //~ ERROR mismatched types
2020-02-09T07:09:26.5024942Z    |     ^^^^^^^^^^ help: try surrounding the expression with parentheses: `((S * Z): Z)`
2020-02-09T07:09:26.5025029Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5025280Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:51:10
2020-02-09T07:09:26.5025328Z    |
2020-02-09T07:09:26.5025328Z    |
2020-02-09T07:09:26.5025380Z LL |     S .. S: S; // OK
2020-02-09T07:09:26.5025429Z    |          ^^^^ help: try surrounding the expression with parentheses: `(S: S)`
2020-02-09T07:09:26.5025512Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5025757Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:52:11
2020-02-09T07:09:26.5025813Z    |
2020-02-09T07:09:26.5025813Z    |
2020-02-09T07:09:26.5025856Z LL |     S .. (S: S); // OK
2020-02-09T07:09:26.5025918Z    |           ^^^^ help: try surrounding the expression with parentheses: `(S: S)`
2020-02-09T07:09:26.5025994Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5026496Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:53:5
2020-02-09T07:09:26.5026546Z    |
2020-02-09T07:09:26.5026546Z    |
2020-02-09T07:09:26.5026592Z LL |     (S .. S): S; //~ ERROR mismatched types
2020-02-09T07:09:26.5026741Z    |     ^^^^^^^^^^^ help: try surrounding the expression with parentheses: `((S .. S): S)`
2020-02-09T07:09:26.5026828Z error[E0308]: mismatched types
2020-02-09T07:09:26.5027110Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:31:7
2020-02-09T07:09:26.5027172Z    |
2020-02-09T07:09:26.5027172Z    |
2020-02-09T07:09:26.5027217Z LL |     &(S: &S); //~ ERROR mismatched types
2020-02-09T07:09:26.5027348Z    |       ^ expected `&S`, found struct `S`
2020-02-09T07:09:26.5027432Z error[E0308]: mismatched types
2020-02-09T07:09:26.5027714Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:35:7
2020-02-09T07:09:26.5027764Z    |
2020-02-09T07:09:26.5027764Z    |
2020-02-09T07:09:26.5027819Z LL |     *(S: Z); //~ ERROR mismatched types
2020-02-09T07:09:26.5027868Z    |       ^ expected struct `Z`, found struct `S`
2020-02-09T07:09:26.5027944Z error[E0614]: type `Z` cannot be dereferenced
2020-02-09T07:09:26.5028208Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:35:5
2020-02-09T07:09:26.5028266Z    |
2020-02-09T07:09:26.5028266Z    |
2020-02-09T07:09:26.5028311Z LL |     *(S: Z); //~ ERROR mismatched types
2020-02-09T07:09:26.5028403Z 
2020-02-09T07:09:26.5028445Z error[E0308]: mismatched types
2020-02-09T07:09:26.5028714Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:40:7
2020-02-09T07:09:26.5028764Z    |
2020-02-09T07:09:26.5028764Z    |
2020-02-09T07:09:26.5028995Z LL |     -(S: Z); //~ ERROR mismatched types
2020-02-09T07:09:26.5029047Z    |       ^ expected struct `Z`, found struct `S`
2020-02-09T07:09:26.5029326Z error[E0600]: cannot apply unary operator `-` to type `Z`
2020-02-09T07:09:26.5029573Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:40:5
2020-02-09T07:09:26.5029631Z    |
2020-02-09T07:09:26.5029631Z    |
2020-02-09T07:09:26.5029851Z LL |     -(S: Z); //~ ERROR mismatched types
2020-02-09T07:09:26.5030078Z    |     ^^^^^^^ cannot apply unary operator `-`
2020-02-09T07:09:26.5030184Z    |
2020-02-09T07:09:26.5030259Z    = note: an implementation of `std::ops::Neg` might be missing for `Z`
2020-02-09T07:09:26.5030335Z error[E0308]: mismatched types
2020-02-09T07:09:26.5030611Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:45:5
2020-02-09T07:09:26.5030662Z    |
2020-02-09T07:09:26.5030662Z    |
2020-02-09T07:09:26.5030707Z LL |     (S + Z): Z; //~ ERROR mismatched types
2020-02-09T07:09:26.5030757Z    |     ^^^^^^^ expected struct `Z`, found struct `S`
2020-02-09T07:09:26.5030853Z error[E0308]: mismatched types
2020-02-09T07:09:26.5031103Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:49:5
2020-02-09T07:09:26.5031166Z    |
2020-02-09T07:09:26.5031166Z    |
2020-02-09T07:09:26.5031212Z LL |     (S * Z): Z; //~ ERROR mismatched types
2020-02-09T07:09:26.5031261Z    |     ^^^^^^^ expected struct `Z`, found struct `S`
2020-02-09T07:09:26.5031345Z error[E0308]: mismatched types
2020-02-09T07:09:26.5031594Z   --> /checkout/src/test/ui/type/type-ascription-precedence.rs:53:5
2020-02-09T07:09:26.5031644Z    |
2020-02-09T07:09:26.5031644Z    |
2020-02-09T07:09:26.5031708Z LL |     (S .. S): S; //~ ERROR mismatched types
2020-02-09T07:09:26.5031759Z    |     ^^^^^^^^ expected struct `S`, found struct `std::ops::Range`
2020-02-09T07:09:26.5031858Z    = note: expected struct `S`
2020-02-09T07:09:26.5031904Z               found struct `std::ops::Range<S>`
2020-02-09T07:09:26.5031934Z 
2020-02-09T07:09:26.5031986Z error: aborting due to 26 previous errors
---
2020-02-09T07:09:26.5032974Z 
2020-02-09T07:09:26.5033111Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5033382Z +   --> $DIR/type-ascription-soundness.rs:7:17
2020-02-09T07:09:26.5033443Z +    |
2020-02-09T07:09:26.5033488Z + LL |     let ref x = arr: &[u8];
2020-02-09T07:09:26.5033543Z +    |                 ^^^^^^^^^^ help: try surrounding the expression with parentheses: `(arr: &[u8])`
2020-02-09T07:09:26.5034057Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5034358Z +   --> $DIR/type-ascription-soundness.rs:8:21
2020-02-09T07:09:26.5034420Z +    |
2020-02-09T07:09:26.5034420Z +    |
2020-02-09T07:09:26.5034465Z + LL |     let ref mut x = arr: &[u8];
2020-02-09T07:09:26.5034520Z +    |                     ^^^^^^^^^^ help: try surrounding the expression with parentheses: `(arr: &[u8])`
2020-02-09T07:09:26.5034625Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5034857Z +   --> $DIR/type-ascription-soundness.rs:9:11
2020-02-09T07:09:26.5034904Z +    |
2020-02-09T07:09:26.5034904Z +    |
2020-02-09T07:09:26.5034968Z + LL |     match arr: &[u8] {
2020-02-09T07:09:26.5035023Z +    |           ^^^^^^^^^^ help: try surrounding the expression with parentheses: `(arr: &[u8])`
2020-02-09T07:09:26.5035123Z + error: casts followed by expressions are not supported
2020-02-09T07:09:26.5035357Z +   --> $DIR/type-ascription-soundness.rs:12:17
2020-02-09T07:09:26.5035414Z +    |
2020-02-09T07:09:26.5035414Z +    |
2020-02-09T07:09:26.5035467Z + LL |     let _len = (arr: &[u8]).len();
2020-02-09T07:09:26.5035522Z +    |                 ^^^^^^^^^^ help: try surrounding the expression with parentheses: `(arr: &[u8])`
2020-02-09T07:09:26.5035624Z 1 error[E0308]: mismatched types
2020-02-09T07:09:26.5035858Z 2   --> $DIR/type-ascription-soundness.rs:7:17
2020-02-09T07:09:26.5035905Z 3    |
2020-02-09T07:09:26.5035932Z 
---
2020-02-09T07:09:26.5036723Z 40 
2020-02-09T07:09:26.5036751Z 
2020-02-09T07:09:26.5036779Z 
2020-02-09T07:09:26.5036833Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.5037174Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-ascription-soundness/type-ascription-soundness.stderr
2020-02-09T07:09:26.5037432Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.5037730Z To only update this specific test, also pass `--test-args type/type-ascription-soundness.rs`
2020-02-09T07:09:26.5037827Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.5037876Z status: exit code: 1
2020-02-09T07:09:26.5037876Z status: exit code: 1
2020-02-09T07:09:26.5038747Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-ascription-soundness.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-ascription-soundness" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-ascription-soundness/auxiliary" "-A" "unused"
2020-02-09T07:09:26.5039109Z ------------------------------------------
2020-02-09T07:09:26.5039146Z 
2020-02-09T07:09:26.5039387Z ------------------------------------------
2020-02-09T07:09:26.5039602Z stderr:
2020-02-09T07:09:26.5039602Z stderr:
2020-02-09T07:09:26.5039816Z ------------------------------------------
2020-02-09T07:09:26.5039961Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5040596Z   --> /checkout/src/test/ui/type/type-ascription-soundness.rs:7:17
2020-02-09T07:09:26.5040648Z    |
2020-02-09T07:09:26.5040697Z LL |     let ref x = arr: &[u8]; //~ ERROR mismatched types
2020-02-09T07:09:26.5040758Z    |                 ^^^^^^^^^^ help: try surrounding the expression with parentheses: `(arr: &[u8])`
2020-02-09T07:09:26.5040927Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5041206Z   --> /checkout/src/test/ui/type/type-ascription-soundness.rs:8:21
2020-02-09T07:09:26.5041264Z    |
2020-02-09T07:09:26.5041264Z    |
2020-02-09T07:09:26.5041310Z LL |     let ref mut x = arr: &[u8]; //~ ERROR mismatched types
2020-02-09T07:09:26.5041365Z    |                     ^^^^^^^^^^ help: try surrounding the expression with parentheses: `(arr: &[u8])`
2020-02-09T07:09:26.5041452Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5041702Z   --> /checkout/src/test/ui/type/type-ascription-soundness.rs:9:11
2020-02-09T07:09:26.5041757Z    |
2020-02-09T07:09:26.5041757Z    |
2020-02-09T07:09:26.5041802Z LL |     match arr: &[u8] { //~ ERROR mismatched types
2020-02-09T07:09:26.5041856Z    |           ^^^^^^^^^^ help: try surrounding the expression with parentheses: `(arr: &[u8])`
2020-02-09T07:09:26.5041940Z error: casts followed by expressions are not supported
2020-02-09T07:09:26.5042193Z   --> /checkout/src/test/ui/type/type-ascription-soundness.rs:12:17
2020-02-09T07:09:26.5042241Z    |
2020-02-09T07:09:26.5042241Z    |
2020-02-09T07:09:26.5042293Z LL |     let _len = (arr: &[u8]).len(); //~ ERROR mismatched types
2020-02-09T07:09:26.5042347Z    |                 ^^^^^^^^^^ help: try surrounding the expression with parentheses: `(arr: &[u8])`
2020-02-09T07:09:26.5042423Z error[E0308]: mismatched types
2020-02-09T07:09:26.5042668Z   --> /checkout/src/test/ui/type/type-ascription-soundness.rs:7:17
2020-02-09T07:09:26.5042715Z    |
2020-02-09T07:09:26.5042715Z    |
2020-02-09T07:09:26.5042769Z LL |     let ref x = arr: &[u8]; //~ ERROR mismatched types
2020-02-09T07:09:26.5044322Z    |                 ^^^ expected slice `[u8]`, found array `[u8; 3]`
2020-02-09T07:09:26.5044419Z    = note: expected reference `&[u8]`
2020-02-09T07:09:26.5044475Z               found reference `&[u8; 3]`
2020-02-09T07:09:26.5044506Z 
2020-02-09T07:09:26.5044549Z error[E0308]: mismatched types
2020-02-09T07:09:26.5044549Z error[E0308]: mismatched types
2020-02-09T07:09:26.5044893Z   --> /checkout/src/test/ui/type/type-ascription-soundness.rs:8:21
2020-02-09T07:09:26.5044952Z    |
2020-02-09T07:09:26.5045000Z LL |     let ref mut x = arr: &[u8]; //~ ERROR mismatched types
2020-02-09T07:09:26.5045053Z    |                     ^^^ expected slice `[u8]`, found array `[u8; 3]`
2020-02-09T07:09:26.5045148Z    = note: expected reference `&[u8]`
2020-02-09T07:09:26.5045195Z               found reference `&[u8; 3]`
2020-02-09T07:09:26.5045225Z 
2020-02-09T07:09:26.5045275Z error[E0308]: mismatched types
2020-02-09T07:09:26.5045275Z error[E0308]: mismatched types
2020-02-09T07:09:26.5045538Z   --> /checkout/src/test/ui/type/type-ascription-soundness.rs:9:11
2020-02-09T07:09:26.5045588Z    |
2020-02-09T07:09:26.5045641Z LL |     match arr: &[u8] { //~ ERROR mismatched types
2020-02-09T07:09:26.5045693Z    |           ^^^ expected slice `[u8]`, found array `[u8; 3]`
2020-02-09T07:09:26.5045788Z    = note: expected reference `&[u8]`
2020-02-09T07:09:26.5045834Z               found reference `&[u8; 3]`
2020-02-09T07:09:26.5045871Z 
2020-02-09T07:09:26.5045914Z error[E0308]: mismatched types
2020-02-09T07:09:26.5045914Z error[E0308]: mismatched types
2020-02-09T07:09:26.5046178Z   --> /checkout/src/test/ui/type/type-ascription-soundness.rs:12:17
2020-02-09T07:09:26.5046229Z    |
2020-02-09T07:09:26.5046276Z LL |     let _len = (arr: &[u8]).len(); //~ ERROR mismatched types
2020-02-09T07:09:26.5046336Z    |                 ^^^ expected slice `[u8]`, found array `[u8; 3]`
2020-02-09T07:09:26.5046424Z    = note: expected reference `&[u8]`
2020-02-09T07:09:26.5046477Z               found reference `&[u8; 3]`
2020-02-09T07:09:26.5046633Z 
2020-02-09T07:09:26.5046688Z error: aborting due to 8 previous errors
---
2020-02-09T07:09:26.5048158Z + LL | /     f()  :
2020-02-09T07:09:26.5048211Z + LL | |     f();
2020-02-09T07:09:26.5048255Z +    | |_______^
2020-02-09T07:09:26.5048297Z +    |
2020-02-09T07:09:26.5048351Z + help: try surrounding the expression with parentheses
2020-02-09T07:09:26.5048406Z +    |
2020-02-09T07:09:26.5048449Z + LL |     (f()  :
2020-02-09T07:09:26.5048493Z + LL |     f());
2020-02-09T07:09:26.5048583Z + 
2020-02-09T07:09:26.5048630Z 1 error[E0573]: expected type, found function `f`
2020-02-09T07:09:26.5048879Z 2   --> $DIR/type-ascription-with-fn-call.rs:5:5
2020-02-09T07:09:26.5048930Z 3    |
---
2020-02-09T07:09:26.5049741Z 15 
2020-02-09T07:09:26.5049768Z 
2020-02-09T07:09:26.5049795Z 
2020-02-09T07:09:26.5049842Z The actual stderr differed from the expected stderr.
2020-02-09T07:09:26.5050182Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-ascription-with-fn-call/type-ascription-with-fn-call.stderr
2020-02-09T07:09:26.5050457Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T07:09:26.5050760Z To only update this specific test, also pass `--test-args type/type-ascription-with-fn-call.rs`
2020-02-09T07:09:26.5050865Z error: 1 errors occurred comparing output.
2020-02-09T07:09:26.5050914Z status: exit code: 1
2020-02-09T07:09:26.5050914Z status: exit code: 1
2020-02-09T07:09:26.5051801Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-ascription-with-fn-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-ascription-with-fn-call" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-ascription-with-fn-call/auxiliary" "-A" "unused"
2020-02-09T07:09:26.5052153Z ------------------------------------------
2020-02-09T07:09:26.5052191Z 
2020-02-09T07:09:26.5052429Z ------------------------------------------
2020-02-09T07:09:26.5052490Z stderr:
---
2020-02-09T07:09:26.5053265Z    | |_______^
2020-02-09T07:09:26.5053311Z    |
2020-02-09T07:09:26.5053361Z help: try surrounding the expression with parentheses
2020-02-09T07:09:26.5053510Z    |
2020-02-09T07:09:26.5053563Z LL |     (f()  :
2020-02-09T07:09:26.5053614Z LL |     f()); //~ ERROR expected type, found function
2020-02-09T07:09:26.5053704Z 
2020-02-09T07:09:26.5053753Z error[E0573]: expected type, found function `f`
2020-02-09T07:09:26.5054064Z   --> /checkout/src/test/ui/type/type-ascription-with-fn-call.rs:5:5
2020-02-09T07:09:26.5054222Z    |
2020-02-09T07:09:26.5054222Z    |
2020-02-09T07:09:26.5054269Z LL |     f()  :
2020-02-09T07:09:26.5054560Z    |          - help: did you mean to use `;` here instead?
2020-02-09T07:09:26.5054620Z LL |     f(); //~ ERROR expected type, found function
2020-02-09T07:09:26.5054724Z    |     |
2020-02-09T07:09:26.5054769Z    |     not a type
2020-02-09T07:09:26.5054827Z    |     expecting a type here because of type ascription
2020-02-09T07:09:26.5054861Z 
---
2020-02-09T07:09:26.5061680Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-09T07:09:26.5061979Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-09T07:09:26.5062013Z 
2020-02-09T07:09:26.5062039Z 
2020-02-09T07:09:26.5063650Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-09T07:09:26.5063929Z 
2020-02-09T07:09:26.5063967Z 
2020-02-09T07:09:26.5064016Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-09T07:09:26.5064068Z Build completed unsuccessfully in 1:01:00
2020-02-09T07:09:26.5064068Z Build completed unsuccessfully in 1:01:00
2020-02-09T07:09:26.5064124Z == clock drift check ==
2020-02-09T07:09:26.5064180Z   local time: Sun Feb  9 07:09:26 UTC 2020
2020-02-09T07:09:27.0112174Z   network time: Sun, 09 Feb 2020 07:09:27 GMT
2020-02-09T07:09:27.0116757Z == end clock drift check ==
2020-02-09T07:09:27.4922970Z 
2020-02-09T07:09:27.5018768Z ##[error]Bash exited with code '1'.
2020-02-09T07:09:27.5031115Z ##[section]Finishing: Run build
2020-02-09T07:09:27.5055804Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68985/merge to s
2020-02-09T07:09:27.5057840Z Task         : Get sources
2020-02-09T07:09:27.5057886Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T07:09:27.5057950Z Version      : 1.0.0
2020-02-09T07:09:27.5057991Z Author       : Microsoft
2020-02-09T07:09:27.5057991Z Author       : Microsoft
2020-02-09T07:09:27.5058037Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-09T07:09:27.5058105Z ==============================================================================
2020-02-09T07:09:27.9392790Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-09T07:09:27.9432508Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68985/merge to s
2020-02-09T07:09:27.9544631Z Cleaning up task key
2020-02-09T07:09:27.9545386Z Start cleaning up orphan processes.
2020-02-09T07:09:27.9663634Z Terminate orphan process: pid (4326) (python)
2020-02-09T07:09:28.0402682Z ##[section]Finishing: Finalize Job
