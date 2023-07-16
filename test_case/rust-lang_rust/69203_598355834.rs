plain
2020-03-12T17:27:21.1538758Z ========================== Starting Command Output ===========================
2020-03-12T17:27:21.1541228Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/421e1097-9c06-40b4-ad0f-cb8eb3250b87.sh
2020-03-12T17:27:21.1541523Z 
2020-03-12T17:27:21.1545317Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-12T17:27:21.1564720Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69203/merge to s
2020-03-12T17:27:21.1567943Z Task         : Get sources
2020-03-12T17:27:21.1568253Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-12T17:27:21.1568555Z Version      : 1.0.0
2020-03-12T17:27:21.1568756Z Author       : Microsoft
---
2020-03-12T17:27:22.1474104Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-12T17:27:22.1479366Z ##[command]git config gc.auto 0
2020-03-12T17:27:22.1482864Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-12T17:27:22.1486629Z ##[command]git config --get-all http.proxy
2020-03-12T17:27:22.1492656Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69203/merge:refs/remotes/pull/69203/merge
---
2020-03-12T18:24:47.8086369Z .................................................................................................... 1700/9767
2020-03-12T18:24:52.2281932Z .................................................................................................... 1800/9767
2020-03-12T18:25:03.2676394Z ................................................................i................................... 1900/9767
2020-03-12T18:25:10.0207624Z .................................................................................................... 2000/9767
2020-03-12T18:25:24.2341494Z ......................................................iiiii......................................... 2100/9767
2020-03-12T18:25:34.3182628Z .................................................................................................... 2300/9767
2020-03-12T18:25:36.5203037Z .................................................................................................... 2400/9767
2020-03-12T18:25:39.5975929Z .................................................................................................... 2500/9767
2020-03-12T18:26:00.4966593Z .................................................................................................... 2600/9767
---
2020-03-12T18:28:33.6355036Z .........................i...............i.......................................................... 5000/9767
2020-03-12T18:28:43.1483912Z .................................................................................................... 5100/9767
2020-03-12T18:28:48.6974190Z ....................................................................i............................... 5200/9767
2020-03-12T18:28:54.9770853Z .................................................................................................... 5300/9767
2020-03-12T18:29:04.1720729Z .................................................ii.ii........i...i................................. 5400/9767
2020-03-12T18:29:12.8081325Z .................................................................................................... 5600/9767
2020-03-12T18:29:22.4553470Z .................................................................................................... 5700/9767
2020-03-12T18:29:28.9685062Z .........................................i.......................................................... 5800/9767
2020-03-12T18:29:35.2546457Z .................................................................................................... 5900/9767
2020-03-12T18:29:35.2546457Z .................................................................................................... 5900/9767
2020-03-12T18:29:45.4991791Z .................................................................................................... 6000/9767
2020-03-12T18:29:54.3834492Z ..................................ii...i..ii...........i............................................ 6100/9767
2020-03-12T18:30:11.8091572Z .................................................................................................... 6300/9767
2020-03-12T18:30:15.6487158Z .................................................................................................... 6400/9767
2020-03-12T18:30:15.6487158Z .................................................................................................... 6400/9767
2020-03-12T18:30:20.9073148Z .................................................................i..ii.............................. 6500/9767
2020-03-12T18:30:46.1529261Z .................................................................................................... 6700/9767
2020-03-12T18:30:51.9771251Z ...............................................................i.................................... 6800/9767
2020-03-12T18:30:54.2574301Z .................................................................................................... 6900/9767
2020-03-12T18:30:56.6358767Z .................................................................................................i.. 7000/9767
---
2020-03-12T18:32:34.8206444Z .................................................................................................... 7700/9767
2020-03-12T18:32:39.5229471Z .................................................................................................... 7800/9767
2020-03-12T18:32:45.6410746Z .................................................................................................... 7900/9767
2020-03-12T18:32:51.6355751Z ...............................................i.................................................... 8000/9767
2020-03-12T18:33:01.8479235Z ................................................................................................iiii 8100/9767
2020-03-12T18:33:07.7724861Z iiiiii.i............................................................................................ 8200/9767
2020-03-12T18:33:21.8154772Z .................................................................................................... 8400/9767
2020-03-12T18:33:32.7409096Z .................................................................................................... 8500/9767
2020-03-12T18:33:45.0499800Z .................................................................................................... 8600/9767
2020-03-12T18:33:50.5229553Z .................................................................................................... 8700/9767
---
2020-03-12T18:36:04.0875963Z  finished in 7.664
2020-03-12T18:36:04.1072727Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-12T18:36:04.2981459Z 
2020-03-12T18:36:04.2981821Z running 179 tests
2020-03-12T18:36:07.2006311Z iiii......i...........ii..iiii....i....i...........i............i..i..................i....i........ 100/179
2020-03-12T18:36:09.4036679Z ....i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-03-12T18:36:09.4043058Z 
2020-03-12T18:36:09.4048232Z  finished in 5.297
2020-03-12T18:36:09.4245068Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-12T18:36:09.5836995Z 
---
2020-03-12T18:36:11.4623858Z  finished in 2.038
2020-03-12T18:36:11.4818481Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-12T18:36:11.6367928Z 
2020-03-12T18:36:11.6368807Z running 9 tests
2020-03-12T18:36:11.6369910Z iiiiiiiii
2020-03-12T18:36:11.6370919Z 
2020-03-12T18:36:11.6371443Z  finished in 0.155
2020-03-12T18:36:11.6581962Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-12T18:36:11.8490386Z 
---
2020-03-12T18:36:31.0847983Z  finished in 19.426
2020-03-12T18:36:31.1062355Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-12T18:36:31.2808178Z 
2020-03-12T18:36:31.2808530Z running 115 tests
2020-03-12T18:36:44.1666492Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-12T18:36:45.7487099Z ...iiii.....ii.
2020-03-12T18:36:45.7488387Z 
2020-03-12T18:36:45.7494451Z  finished in 14.643
2020-03-12T18:36:45.7500536Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-12T18:36:45.7501261Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-03-12T18:40:42.3261967Z     Finished release [optimized] target(s) in 3m 17s
2020-03-12T18:40:42.3558937Z Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-12T18:40:42.5867003Z 
2020-03-12T18:40:42.5867380Z running 337 tests
2020-03-12T18:40:45.5873756Z FFFFFFFFF.FFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFiFFFFFFF..F.FFFFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF 100/337
2020-03-12T18:40:49.4372323Z FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF..FFFFFFFFFFFFFFF.FFF.FF.iFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFF.FFFF.. 200/337
2020-03-12T18:40:52.2922898Z FFFFFFFFFFFFFFF.FFFFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFFF.FFFFFFFFFFiFFFFFFFFFFFFFFFFFF 300/337
2020-03-12T18:40:53.3325551Z FFFFFFFFFFFFF.FF.FFFFFFFFFFFFFFFFFFFF
2020-03-12T18:40:53.3429488Z 
2020-03-12T18:40:53.3436942Z ---- [rustdoc] rustdoc/all.rs stdout ----
2020-03-12T18:40:53.3492379Z 
2020-03-12T18:40:53.3492965Z error: rustdoc failed!
2020-03-12T18:40:53.3492965Z error: rustdoc failed!
2020-03-12T18:40:53.3493558Z status: exit code: 1
2020-03-12T18:40:53.3495381Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/all/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/all" "/checkout/src/test/rustdoc/all.rs"
2020-03-12T18:40:53.3504171Z ------------------------------------------
2020-03-12T18:40:53.3521264Z 
2020-03-12T18:40:53.3522039Z ------------------------------------------
2020-03-12T18:40:53.3533401Z stderr:
---
2020-03-12T18:40:53.3552000Z ---- [rustdoc] rustdoc/assoc-consts-version.rs stdout ----
2020-03-12T18:40:53.3552362Z 
2020-03-12T18:40:53.3552636Z error: rustdoc failed!
2020-03-12T18:40:53.3552951Z status: exit code: 1
2020-03-12T18:40:53.3554687Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts-version/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts-version" "/checkout/src/test/rustdoc/assoc-consts-version.rs"
2020-03-12T18:40:53.3556198Z ------------------------------------------
2020-03-12T18:40:53.3556494Z 
2020-03-12T18:40:53.3557163Z ------------------------------------------
2020-03-12T18:40:53.3557510Z stderr:
---
2020-03-12T18:40:53.3631260Z ---- [rustdoc] rustdoc/async-fn.rs stdout ----
2020-03-12T18:40:53.3631573Z 
2020-03-12T18:40:53.3631845Z error: rustdoc failed!
2020-03-12T18:40:53.3632172Z status: exit code: 1
2020-03-12T18:40:53.3633637Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-fn/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-fn" "/checkout/src/test/rustdoc/async-fn.rs" "--edition=2018"
2020-03-12T18:40:53.3635066Z ------------------------------------------
2020-03-12T18:40:53.3635374Z 
2020-03-12T18:40:53.3635869Z ------------------------------------------
2020-03-12T18:40:53.3651787Z stderr:
---
2020-03-12T18:40:53.3669063Z ---- [rustdoc] rustdoc/auto-impl-primitive.rs stdout ----
2020-03-12T18:40:53.3669258Z 
2020-03-12T18:40:53.3669433Z error: rustdoc failed!
2020-03-12T18:40:53.3669633Z status: exit code: 1
2020-03-12T18:40:53.3671059Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto-impl-primitive/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto-impl-primitive" "/checkout/src/test/rustdoc/auto-impl-primitive.rs"
2020-03-12T18:40:53.3672500Z ------------------------------------------
2020-03-12T18:40:53.3672677Z 
2020-03-12T18:40:53.3673008Z ------------------------------------------
2020-03-12T18:40:53.3673443Z stderr:
---
2020-03-12T18:40:53.3704120Z ---- [rustdoc] rustdoc/bad-codeblock-syntax.rs stdout ----
2020-03-12T18:40:53.3704319Z 
2020-03-12T18:40:53.3704476Z error: rustdoc failed!
2020-03-12T18:40:53.3704694Z status: exit code: 1
2020-03-12T18:40:53.3706067Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/bad-codeblock-syntax/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/bad-codeblock-syntax" "/checkout/src/test/rustdoc/bad-codeblock-syntax.rs"
2020-03-12T18:40:53.3707291Z ------------------------------------------
2020-03-12T18:40:53.3707463Z 
2020-03-12T18:40:53.3707833Z ------------------------------------------
2020-03-12T18:40:53.3708032Z stderr:
---
2020-03-12T18:40:53.3710813Z ---- [rustdoc] rustdoc/blanket-reexport-item.rs stdout ----
2020-03-12T18:40:53.3711014Z 
2020-03-12T18:40:53.3711291Z error: rustdoc failed!
2020-03-12T18:40:53.3711491Z status: exit code: 1
2020-03-12T18:40:53.3712944Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item" "/checkout/src/test/rustdoc/blanket-reexport-item.rs"
2020-03-12T18:40:53.3714179Z ------------------------------------------
2020-03-12T18:40:53.3714352Z 
2020-03-12T18:40:53.3714726Z ------------------------------------------
2020-03-12T18:40:53.3714925Z stderr:
---
2020-03-12T18:40:53.3734119Z ---- [rustdoc] rustdoc/check-styled-link.rs stdout ----
2020-03-12T18:40:53.3734319Z 
2020-03-12T18:40:53.3734677Z error: rustdoc failed!
2020-03-12T18:40:53.3734899Z status: exit code: 1
2020-03-12T18:40:53.3750543Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/check-styled-link/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/check-styled-link" "/checkout/src/test/rustdoc/check-styled-link.rs"
2020-03-12T18:40:53.3752503Z ------------------------------------------
2020-03-12T18:40:53.3752687Z 
2020-03-12T18:40:53.3753060Z ------------------------------------------
2020-03-12T18:40:53.3753285Z stderr:
---
2020-03-12T18:40:53.3756169Z ---- [rustdoc] rustdoc/codeblock-title.rs stdout ----
2020-03-12T18:40:53.3756361Z 
2020-03-12T18:40:53.3756535Z error: rustdoc failed!
2020-03-12T18:40:53.3756741Z status: exit code: 1
2020-03-12T18:40:53.3758082Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/codeblock-title/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/codeblock-title" "/checkout/src/test/rustdoc/codeblock-title.rs"
2020-03-12T18:40:53.3759281Z ------------------------------------------
2020-03-12T18:40:53.3759454Z 
2020-03-12T18:40:53.3759837Z ------------------------------------------
2020-03-12T18:40:53.3760038Z stderr:
---
2020-03-12T18:40:53.3762809Z ---- [rustdoc] rustdoc/const-display.rs stdout ----
2020-03-12T18:40:53.3762999Z 
2020-03-12T18:40:53.3763342Z error: rustdoc failed!
2020-03-12T18:40:53.3763548Z status: exit code: 1
2020-03-12T18:40:53.3764888Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-display/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-display" "/checkout/src/test/rustdoc/const-display.rs"
2020-03-12T18:40:53.3769539Z ------------------------------------------
2020-03-12T18:40:53.3769778Z 
2020-03-12T18:40:53.3770147Z ------------------------------------------
2020-03-12T18:40:53.3770350Z stderr:
---
2020-03-12T18:40:53.3779932Z ---- [rustdoc] rustdoc/const-evalutation-ice.rs stdout ----
2020-03-12T18:40:53.3780132Z 
2020-03-12T18:40:53.3780287Z error: rustdoc failed!
2020-03-12T18:40:53.3780487Z status: exit code: 1
2020-03-12T18:40:53.3781874Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-evalutation-ice/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-evalutation-ice" "/checkout/src/test/rustdoc/const-evalutation-ice.rs"
2020-03-12T18:40:53.3783102Z ------------------------------------------
2020-03-12T18:40:53.3783274Z 
2020-03-12T18:40:53.3783639Z ------------------------------------------
2020-03-12T18:40:53.3783856Z stderr:
---
2020-03-12T18:40:53.3786620Z ---- [rustdoc] rustdoc/const-fn.rs stdout ----
2020-03-12T18:40:53.3786798Z 
2020-03-12T18:40:53.3786955Z error: rustdoc failed!
2020-03-12T18:40:53.3787171Z status: exit code: 1
2020-03-12T18:40:53.3788453Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-fn/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-fn" "/checkout/src/test/rustdoc/const-fn.rs"
2020-03-12T18:40:53.3789614Z ------------------------------------------
2020-03-12T18:40:53.3789788Z 
2020-03-12T18:40:53.3790143Z ------------------------------------------
2020-03-12T18:40:53.3790362Z stderr:
---
2020-03-12T18:40:53.3793192Z ---- [rustdoc] rustdoc/const-generics/add-impl.rs stdout ----
2020-03-12T18:40:53.3793398Z 
2020-03-12T18:40:53.3793632Z error: rustdoc failed!
2020-03-12T18:40:53.3793836Z status: exit code: 1
2020-03-12T18:40:53.3795285Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/add-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/add-impl" "/checkout/src/test/rustdoc/const-generics/add-impl.rs"
2020-03-12T18:40:53.3796537Z ------------------------------------------
2020-03-12T18:40:53.3796709Z 
2020-03-12T18:40:53.3797083Z ------------------------------------------
2020-03-12T18:40:53.3797283Z stderr:
---
2020-03-12T18:40:53.3802240Z ---- [rustdoc] rustdoc/const-generics/const-generic-slice.rs stdout ----
2020-03-12T18:40:53.3802464Z 
2020-03-12T18:40:53.3802622Z error: rustdoc failed!
2020-03-12T18:40:53.3802840Z status: exit code: 1
2020-03-12T18:40:53.3804472Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-generic-slice/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-generic-slice" "/checkout/src/test/rustdoc/const-generics/const-generic-slice.rs"
2020-03-12T18:40:53.3805758Z ------------------------------------------
2020-03-12T18:40:53.3805931Z 
2020-03-12T18:40:53.3806306Z ------------------------------------------
2020-03-12T18:40:53.3806508Z stderr:
---
2020-03-12T18:40:53.3812350Z ---- [rustdoc] rustdoc/const-underscore.rs stdout ----
2020-03-12T18:40:53.3812542Z 
2020-03-12T18:40:53.3812701Z error: rustdoc failed!
2020-03-12T18:40:53.3812901Z status: exit code: 1
2020-03-12T18:40:53.3814422Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-underscore/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-underscore" "/checkout/src/test/rustdoc/const-underscore.rs" "--document-private-items"
2020-03-12T18:40:53.3815728Z ------------------------------------------
2020-03-12T18:40:53.3815899Z 
2020-03-12T18:40:53.3816256Z ------------------------------------------
2020-03-12T18:40:53.3816476Z stderr:
---
2020-03-12T18:40:53.3819285Z ---- [rustdoc] rustdoc/const-generics/const-impl.rs stdout ----
2020-03-12T18:40:53.3819497Z 
2020-03-12T18:40:53.3819656Z error: rustdoc failed!
2020-03-12T18:40:53.3819874Z status: exit code: 1
2020-03-12T18:40:53.3821275Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-impl" "/checkout/src/test/rustdoc/const-generics/const-impl.rs"
2020-03-12T18:40:53.3822532Z ------------------------------------------
2020-03-12T18:40:53.3822705Z 
2020-03-12T18:40:53.3823075Z ------------------------------------------
2020-03-12T18:40:53.3823277Z stderr:
---
2020-03-12T18:40:53.3834798Z ---- [rustdoc] rustdoc/constructor-imports.rs stdout ----
2020-03-12T18:40:53.3834998Z 
2020-03-12T18:40:53.3835155Z error: rustdoc failed!
2020-03-12T18:40:53.3835372Z status: exit code: 1
2020-03-12T18:40:53.3836732Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/constructor-imports/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/constructor-imports" "/checkout/src/test/rustdoc/constructor-imports.rs"
2020-03-12T18:40:53.3837946Z ------------------------------------------
2020-03-12T18:40:53.3838116Z 
2020-03-12T18:40:53.3838483Z ------------------------------------------
2020-03-12T18:40:53.3838701Z stderr:
---
2020-03-12T18:40:53.3841540Z ---- [rustdoc] rustdoc/crate-version-escape.rs stdout ----
2020-03-12T18:40:53.3841739Z 
2020-03-12T18:40:53.3841911Z error: rustdoc failed!
2020-03-12T18:40:53.3842159Z status: exit code: 1
2020-03-12T18:40:53.3843899Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/crate-version-escape/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/crate-version-escape" "/checkout/src/test/rustdoc/crate-version-escape.rs" "--crate-version=<script>alert(\"hi\")</script>" "-Z" "unstable-options"
2020-03-12T18:40:53.3845270Z ------------------------------------------
2020-03-12T18:40:53.3845443Z 
2020-03-12T18:40:53.3845817Z ------------------------------------------
2020-03-12T18:40:53.3846017Z stderr:
---
2020-03-12T18:40:53.3848793Z ---- [rustdoc] rustdoc/crate-version.rs stdout ----
2020-03-12T18:40:53.3848999Z 
2020-03-12T18:40:53.3849162Z error: rustdoc failed!
2020-03-12T18:40:53.3849362Z status: exit code: 1
2020-03-12T18:40:53.3850819Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/crate-version/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/crate-version" "/checkout/src/test/rustdoc/crate-version.rs" "--crate-version=1.3.37" "-Z" "unstable-options"
2020-03-12T18:40:53.3852075Z ------------------------------------------
2020-03-12T18:40:53.3852265Z 
2020-03-12T18:40:53.3852625Z ------------------------------------------
2020-03-12T18:40:53.3852827Z stderr:
---
2020-03-12T18:40:53.3855785Z ---- [rustdoc] rustdoc/cross-crate-links.rs stdout ----
2020-03-12T18:40:53.3855980Z 
2020-03-12T18:40:53.3856138Z error: rustdoc failed!
2020-03-12T18:40:53.3856337Z status: exit code: 1
2020-03-12T18:40:53.3857773Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cross-crate-links/auxiliary/all-item-types/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cross-crate-links" "/checkout/src/test/rustdoc/auxiliary/all-item-types.rs"
2020-03-12T18:40:53.3859030Z ------------------------------------------
2020-03-12T18:40:53.3859202Z 
2020-03-12T18:40:53.3859559Z ------------------------------------------
2020-03-12T18:40:53.3859778Z stderr:
---
2020-03-12T18:40:53.3869211Z ---- [rustdoc] rustdoc/default-trait-method-link.rs stdout ----
2020-03-12T18:40:53.3869419Z 
2020-03-12T18:40:53.3869591Z error: rustdoc failed!
2020-03-12T18:40:53.3869790Z status: exit code: 1
2020-03-12T18:40:53.3871208Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/default-trait-method-link/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/default-trait-method-link" "/checkout/src/test/rustdoc/default-trait-method-link.rs"
2020-03-12T18:40:53.3872459Z ------------------------------------------
2020-03-12T18:40:53.3872631Z 
2020-03-12T18:40:53.3873007Z ------------------------------------------
2020-03-12T18:40:53.3873206Z stderr:
---
2020-03-12T18:40:53.3876141Z ---- [rustdoc] rustdoc/default-trait-method.rs stdout ----
2020-03-12T18:40:53.3876341Z 
2020-03-12T18:40:53.3876515Z error: rustdoc failed!
2020-03-12T18:40:53.3876715Z status: exit code: 1
2020-03-12T18:40:53.3878083Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/default-trait-method/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/default-trait-method" "/checkout/src/test/rustdoc/default-trait-method.rs"
2020-03-12T18:40:53.3879319Z ------------------------------------------
2020-03-12T18:40:53.3879507Z 
2020-03-12T18:40:53.3879864Z ------------------------------------------
2020-03-12T18:40:53.3880063Z stderr:
---
2020-03-12T18:40:53.3889624Z ---- [rustdoc] rustdoc/default-impl.rs stdout ----
2020-03-12T18:40:53.3889810Z 
2020-03-12T18:40:53.3889966Z error: rustdoc failed!
2020-03-12T18:40:53.3890179Z status: exit code: 1
2020-03-12T18:40:53.3891595Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/default-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/default-impl" "/checkout/src/test/rustdoc/default-impl.rs"
2020-03-12T18:40:53.3892785Z ------------------------------------------
2020-03-12T18:40:53.3892957Z 
2020-03-12T18:40:53.3893311Z ------------------------------------------
2020-03-12T18:40:53.3893601Z stderr:
---
2020-03-12T18:40:53.3896575Z ---- [rustdoc] rustdoc/deprecated-impls.rs stdout ----
2020-03-12T18:40:53.3896765Z 
2020-03-12T18:40:53.3896922Z error: rustdoc failed!
2020-03-12T18:40:53.3897140Z status: exit code: 1
2020-03-12T18:40:53.3898814Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deprecated-impls/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deprecated-impls" "/checkout/src/test/rustdoc/deprecated-impls.rs"
2020-03-12T18:40:53.3900028Z ------------------------------------------
2020-03-12T18:40:53.3900201Z 
2020-03-12T18:40:53.3900574Z ------------------------------------------
2020-03-12T18:40:53.3900774Z stderr:
---
2020-03-12T18:40:53.3916883Z ---- [rustdoc] rustdoc/deref-typedef.rs stdout ----
2020-03-12T18:40:53.3917073Z 
2020-03-12T18:40:53.3917231Z error: rustdoc failed!
2020-03-12T18:40:53.3917430Z status: exit code: 1
2020-03-12T18:40:53.3918759Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-typedef/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-typedef" "/checkout/src/test/rustdoc/deref-typedef.rs"
2020-03-12T18:40:53.3919953Z ------------------------------------------
2020-03-12T18:40:53.3920125Z 
2020-03-12T18:40:53.3920484Z ------------------------------------------
2020-03-12T18:40:53.3920701Z stderr:
---
2020-03-12T18:40:53.3923657Z ---- [rustdoc] rustdoc/doc-assoc-item.rs stdout ----
2020-03-12T18:40:53.3923848Z 
2020-03-12T18:40:53.3924004Z error: rustdoc failed!
2020-03-12T18:40:53.3924230Z status: exit code: 1
2020-03-12T18:40:53.3925562Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-assoc-item/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-assoc-item" "/checkout/src/test/rustdoc/doc-assoc-item.rs"
2020-03-12T18:40:53.3926750Z ------------------------------------------
2020-03-12T18:40:53.3926923Z 
2020-03-12T18:40:53.3927280Z ------------------------------------------
2020-03-12T18:40:53.3927499Z stderr:
---
2020-03-12T18:40:53.3930399Z ---- [rustdoc] rustdoc/doc-cfg.rs stdout ----
2020-03-12T18:40:53.3930575Z 
2020-03-12T18:40:53.3930748Z error: rustdoc failed!
2020-03-12T18:40:53.3930950Z status: exit code: 1
2020-03-12T18:40:53.3932207Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg" "/checkout/src/test/rustdoc/doc-cfg.rs"
2020-03-12T18:40:53.3933361Z ------------------------------------------
2020-03-12T18:40:53.3933533Z 
2020-03-12T18:40:53.3933906Z ------------------------------------------
2020-03-12T18:40:53.3934185Z stderr:
---
2020-03-12T18:40:53.3943643Z ---- [rustdoc] rustdoc/double-quote-escape.rs stdout ----
2020-03-12T18:40:53.3943860Z 
2020-03-12T18:40:53.3944019Z error: rustdoc failed!
2020-03-12T18:40:53.3944225Z status: exit code: 1
2020-03-12T18:40:53.3945611Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/double-quote-escape/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/double-quote-escape" "/checkout/src/test/rustdoc/double-quote-escape.rs"
2020-03-12T18:40:53.3946825Z ------------------------------------------
2020-03-12T18:40:53.3946999Z 
2020-03-12T18:40:53.3947357Z ------------------------------------------
2020-03-12T18:40:53.3947558Z stderr:
---
2020-03-12T18:40:53.3950355Z ---- [rustdoc] rustdoc/duplicate-cfg.rs stdout ----
2020-03-12T18:40:53.3950544Z 
2020-03-12T18:40:53.3950700Z error: rustdoc failed!
2020-03-12T18:40:53.3950899Z status: exit code: 1
2020-03-12T18:40:53.3952224Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/duplicate-cfg/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/duplicate-cfg" "/checkout/src/test/rustdoc/duplicate-cfg.rs"
2020-03-12T18:40:53.3953408Z ------------------------------------------
2020-03-12T18:40:53.3953581Z 
2020-03-12T18:40:53.3953937Z ------------------------------------------
2020-03-12T18:40:53.3954216Z stderr:
---
2020-03-12T18:40:53.3957271Z ---- [rustdoc] rustdoc/duplicate_impls/impls.rs stdout ----
2020-03-12T18:40:53.3957473Z 
2020-03-12T18:40:53.3957632Z error: rustdoc failed!
2020-03-12T18:40:53.3957849Z status: exit code: 1
2020-03-12T18:40:53.3959220Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/duplicate_impls/impls/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/duplicate_impls/impls" "/checkout/src/test/rustdoc/duplicate_impls/impls.rs"
2020-03-12T18:40:53.3960459Z ------------------------------------------
2020-03-12T18:40:53.3960632Z 
2020-03-12T18:40:53.3961006Z ------------------------------------------
2020-03-12T18:40:53.3961206Z stderr:
---
2020-03-12T18:40:53.3964259Z ---- [rustdoc] rustdoc/duplicate_impls/issue-33054.rs stdout ----
2020-03-12T18:40:53.3964472Z 
2020-03-12T18:40:53.3964654Z error: rustdoc failed!
2020-03-12T18:40:53.3964858Z status: exit code: 1
2020-03-12T18:40:53.3966293Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/duplicate_impls/issue-33054/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/duplicate_impls/issue-33054" "/checkout/src/test/rustdoc/duplicate_impls/issue-33054.rs"
2020-03-12T18:40:53.3969148Z ------------------------------------------
2020-03-12T18:40:53.3969330Z 
2020-03-12T18:40:53.3969710Z ------------------------------------------
2020-03-12T18:40:53.3969913Z stderr:
---
2020-03-12T18:40:53.3973141Z ---- [rustdoc] rustdoc/empty-mod-private.rs stdout ----
2020-03-12T18:40:53.3973356Z 
2020-03-12T18:40:53.3973516Z error: rustdoc failed!
2020-03-12T18:40:53.3973716Z status: exit code: 1
2020-03-12T18:40:53.3975167Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/empty-mod-private/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/empty-mod-private" "/checkout/src/test/rustdoc/empty-mod-private.rs" "--document-private-items"
2020-03-12T18:40:53.3976397Z ------------------------------------------
2020-03-12T18:40:53.3976712Z 
2020-03-12T18:40:53.3977083Z ------------------------------------------
2020-03-12T18:40:53.3977345Z stderr:
---
2020-03-12T18:40:53.3980160Z ---- [rustdoc] rustdoc/empty-mod-public.rs stdout ----
2020-03-12T18:40:53.3980352Z 
2020-03-12T18:40:53.3980511Z error: rustdoc failed!
2020-03-12T18:40:53.3980711Z status: exit code: 1
2020-03-12T18:40:53.3982078Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/empty-mod-public/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/empty-mod-public" "/checkout/src/test/rustdoc/empty-mod-public.rs"
2020-03-12T18:40:53.3983295Z ------------------------------------------
2020-03-12T18:40:53.3983469Z 
2020-03-12T18:40:53.3983828Z ------------------------------------------
2020-03-12T18:40:53.3984046Z stderr:
---
2020-03-12T18:40:53.3986822Z ---- [rustdoc] rustdoc/empty-section.rs stdout ----
2020-03-12T18:40:53.3987016Z 
2020-03-12T18:40:53.3987173Z error: rustdoc failed!
2020-03-12T18:40:53.3987394Z status: exit code: 1
2020-03-12T18:40:53.3988711Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/empty-section/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/empty-section" "/checkout/src/test/rustdoc/empty-section.rs"
2020-03-12T18:40:53.3989890Z ------------------------------------------
2020-03-12T18:40:53.3990062Z 
2020-03-12T18:40:53.3990418Z ------------------------------------------
2020-03-12T18:40:53.3990634Z stderr:
---
2020-03-12T18:40:53.4000306Z ---- [rustdoc] rustdoc/extern-html-root-url.rs stdout ----
2020-03-12T18:40:53.4000521Z 
2020-03-12T18:40:53.4000678Z error: rustdoc failed!
2020-03-12T18:40:53.4000877Z status: exit code: 1
2020-03-12T18:40:53.4002534Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-html-root-url/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-html-root-url" "/checkout/src/test/rustdoc/extern-html-root-url.rs" "-Z" "unstable-options" "--extern-html-root-url" "core=https://example.com/core/0.1.0"
2020-03-12T18:40:53.4004151Z ------------------------------------------
2020-03-12T18:40:53.4004342Z 
2020-03-12T18:40:53.4004698Z ------------------------------------------
2020-03-12T18:40:53.4004898Z stderr:
---
2020-03-12T18:40:53.4007718Z ---- [rustdoc] rustdoc/extern-default-method.rs stdout ----
2020-03-12T18:40:53.4007920Z 
2020-03-12T18:40:53.4008075Z error: rustdoc failed!
2020-03-12T18:40:53.4008275Z status: exit code: 1
2020-03-12T18:40:53.4009665Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-default-method/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-default-method" "/checkout/src/test/rustdoc/extern-default-method.rs"
2020-03-12T18:40:53.4010887Z ------------------------------------------
2020-03-12T18:40:53.4011058Z 
2020-03-12T18:40:53.4011413Z ------------------------------------------
2020-03-12T18:40:53.4011629Z stderr:
---
2020-03-12T18:40:53.4014408Z ---- [rustdoc] rustdoc/extern-impl.rs stdout ----
2020-03-12T18:40:53.4014594Z 
2020-03-12T18:40:53.4014750Z error: rustdoc failed!
2020-03-12T18:40:53.4014966Z status: exit code: 1
2020-03-12T18:40:53.4016269Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-impl" "/checkout/src/test/rustdoc/extern-impl.rs"
2020-03-12T18:40:53.4017589Z ------------------------------------------
2020-03-12T18:40:53.4017766Z 
2020-03-12T18:40:53.4018134Z ------------------------------------------
2020-03-12T18:40:53.4018352Z stderr:
---
2020-03-12T18:40:53.4027788Z ---- [rustdoc] rustdoc/extern-links.rs stdout ----
2020-03-12T18:40:53.4027973Z 
2020-03-12T18:40:53.4028145Z error: rustdoc failed!
2020-03-12T18:40:53.4028343Z status: exit code: 1
2020-03-12T18:40:53.4029652Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-links/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-links" "/checkout/src/test/rustdoc/extern-links.rs"
2020-03-12T18:40:53.4030827Z ------------------------------------------
2020-03-12T18:40:53.4031014Z 
2020-03-12T18:40:53.4031372Z ------------------------------------------
2020-03-12T18:40:53.4031573Z stderr:
---
2020-03-12T18:40:53.4034344Z ---- [rustdoc] rustdoc/extern-method.rs stdout ----
2020-03-12T18:40:53.4034547Z 
2020-03-12T18:40:53.4034703Z error: rustdoc failed!
2020-03-12T18:40:53.4034901Z status: exit code: 1
2020-03-12T18:40:53.4036227Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-method/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-method" "/checkout/src/test/rustdoc/extern-method.rs"
2020-03-12T18:40:53.4037523Z ------------------------------------------
2020-03-12T18:40:53.4037700Z 
2020-03-12T18:40:53.4038067Z ------------------------------------------
2020-03-12T18:40:53.4038265Z stderr:
---
2020-03-12T18:40:53.4047729Z ---- [rustdoc] rustdoc/external-cross.rs stdout ----
2020-03-12T18:40:53.4047919Z 
2020-03-12T18:40:53.4048076Z error: rustdoc failed!
2020-03-12T18:40:53.4048296Z status: exit code: 1
2020-03-12T18:40:53.4049627Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/external-cross/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/external-cross" "/checkout/src/test/rustdoc/external-cross.rs"
2020-03-12T18:40:53.4050820Z ------------------------------------------
2020-03-12T18:40:53.4050992Z 
2020-03-12T18:40:53.4051362Z ------------------------------------------
2020-03-12T18:40:53.4051562Z stderr:
---
2020-03-12T18:40:53.4054356Z ---- [rustdoc] rustdoc/fn-pointer-arg-name.rs stdout ----
2020-03-12T18:40:53.4054553Z 
2020-03-12T18:40:53.4054725Z error: rustdoc failed!
2020-03-12T18:40:53.4054926Z status: exit code: 1
2020-03-12T18:40:53.4056280Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/fn-pointer-arg-name/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/fn-pointer-arg-name" "/checkout/src/test/rustdoc/fn-pointer-arg-name.rs"
2020-03-12T18:40:53.4057616Z ------------------------------------------
2020-03-12T18:40:53.4057790Z 
2020-03-12T18:40:53.4058174Z ------------------------------------------
2020-03-12T18:40:53.4058377Z stderr:
---
2020-03-12T18:40:53.4061112Z ---- [rustdoc] rustdoc/ffi.rs stdout ----
2020-03-12T18:40:53.4061295Z 
2020-03-12T18:40:53.4061454Z error: rustdoc failed!
2020-03-12T18:40:53.4061654Z status: exit code: 1
2020-03-12T18:40:53.4062926Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/ffi/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/ffi" "/checkout/src/test/rustdoc/ffi.rs"
2020-03-12T18:40:53.4064046Z ------------------------------------------
2020-03-12T18:40:53.4064234Z 
2020-03-12T18:40:53.4064591Z ------------------------------------------
2020-03-12T18:40:53.4064791Z stderr:
---
2020-03-12T18:40:53.4067579Z ---- [rustdoc] rustdoc/fn-sidebar.rs stdout ----
2020-03-12T18:40:53.4067766Z 
2020-03-12T18:40:53.4067921Z error: rustdoc failed!
2020-03-12T18:40:53.4068123Z status: exit code: 1
2020-03-12T18:40:53.4069428Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/fn-sidebar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/fn-sidebar" "/checkout/src/test/rustdoc/fn-sidebar.rs"
2020-03-12T18:40:53.4070600Z ------------------------------------------
2020-03-12T18:40:53.4070773Z 
2020-03-12T18:40:53.4071132Z ------------------------------------------
2020-03-12T18:40:53.4071333Z stderr:
---
2020-03-12T18:40:53.4088208Z ---- [rustdoc] rustdoc/hidden-impls.rs stdout ----
2020-03-12T18:40:53.4088392Z 
2020-03-12T18:40:53.4088566Z error: rustdoc failed!
2020-03-12T18:40:53.4088767Z status: exit code: 1
2020-03-12T18:40:53.4090085Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-impls/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-impls" "/checkout/src/test/rustdoc/hidden-impls.rs"
2020-03-12T18:40:53.4091313Z ------------------------------------------
2020-03-12T18:40:53.4091485Z 
2020-03-12T18:40:53.4091860Z ------------------------------------------
2020-03-12T18:40:53.4092060Z stderr:
---
2020-03-12T18:40:53.4094830Z ---- [rustdoc] rustdoc/generic-impl.rs stdout ----
2020-03-12T18:40:53.4095029Z 
2020-03-12T18:40:53.4095188Z error: rustdoc failed!
2020-03-12T18:40:53.4095386Z status: exit code: 1
2020-03-12T18:40:53.4096702Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/generic-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/generic-impl" "/checkout/src/test/rustdoc/generic-impl.rs"
2020-03-12T18:40:53.4098012Z ------------------------------------------
2020-03-12T18:40:53.4098203Z 
2020-03-12T18:40:53.4098569Z ------------------------------------------
2020-03-12T18:40:53.4098770Z stderr:
---
2020-03-12T18:40:53.4114720Z ---- [rustdoc] rustdoc/hidden-trait-struct-impls.rs stdout ----
2020-03-12T18:40:53.4114929Z 
2020-03-12T18:40:53.4115085Z error: rustdoc failed!
2020-03-12T18:40:53.4115300Z status: exit code: 1
2020-03-12T18:40:53.4116712Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-trait-struct-impls/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-trait-struct-impls" "/checkout/src/test/rustdoc/hidden-trait-struct-impls.rs"
2020-03-12T18:40:53.4118083Z ------------------------------------------
2020-03-12T18:40:53.4118258Z 
2020-03-12T18:40:53.4118640Z ------------------------------------------
2020-03-12T18:40:53.4118840Z stderr:
---
2020-03-12T18:40:53.4121616Z ---- [rustdoc] rustdoc/impl-disambiguation.rs stdout ----
2020-03-12T18:40:53.4121815Z 
2020-03-12T18:40:53.4121988Z error: rustdoc failed!
2020-03-12T18:40:53.4122188Z status: exit code: 1
2020-03-12T18:40:53.4124347Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/impl-disambiguation/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/impl-disambiguation" "/checkout/src/test/rustdoc/impl-disambiguation.rs"
2020-03-12T18:40:53.4125604Z ------------------------------------------
2020-03-12T18:40:53.4125795Z 
2020-03-12T18:40:53.4126156Z ------------------------------------------
2020-03-12T18:40:53.4126359Z stderr:
---
2020-03-12T18:40:53.4129142Z ---- [rustdoc] rustdoc/impl-everywhere.rs stdout ----
2020-03-12T18:40:53.4129351Z 
2020-03-12T18:40:53.4129510Z error: rustdoc failed!
2020-03-12T18:40:53.4129710Z status: exit code: 1
2020-03-12T18:40:53.4131052Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/impl-everywhere/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/impl-everywhere" "/checkout/src/test/rustdoc/impl-everywhere.rs"
2020-03-12T18:40:53.4132235Z ------------------------------------------
2020-03-12T18:40:53.4132407Z 
2020-03-12T18:40:53.4132763Z ------------------------------------------
2020-03-12T18:40:53.4132967Z stderr:
---
2020-03-12T18:40:53.4135736Z ---- [rustdoc] rustdoc/impl-parts.rs stdout ----
2020-03-12T18:40:53.4135919Z 
2020-03-12T18:40:53.4136076Z error: rustdoc failed!
2020-03-12T18:40:53.4136275Z status: exit code: 1
2020-03-12T18:40:53.4137579Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/impl-parts/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/impl-parts" "/checkout/src/test/rustdoc/impl-parts.rs"
2020-03-12T18:40:53.4138902Z ------------------------------------------
2020-03-12T18:40:53.4139077Z 
2020-03-12T18:40:53.4139447Z ------------------------------------------
2020-03-12T18:40:53.4139663Z stderr:
---
2020-03-12T18:40:53.4142449Z ---- [rustdoc] rustdoc/impl-parts-crosscrate.rs stdout ----
2020-03-12T18:40:53.4142650Z 
2020-03-12T18:40:53.4142806Z error: rustdoc failed!
2020-03-12T18:40:53.4143030Z status: exit code: 1
2020-03-12T18:40:53.4144407Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/impl-parts-crosscrate/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/impl-parts-crosscrate" "/checkout/src/test/rustdoc/impl-parts-crosscrate.rs"
2020-03-12T18:40:53.4145636Z ------------------------------------------
2020-03-12T18:40:53.4145807Z 
2020-03-12T18:40:53.4146176Z ------------------------------------------
2020-03-12T18:40:53.4146377Z stderr:
---
2020-03-12T18:40:53.4149142Z ---- [rustdoc] rustdoc/index-page.rs stdout ----
2020-03-12T18:40:53.4149325Z 
2020-03-12T18:40:53.4149497Z error: rustdoc failed!
2020-03-12T18:40:53.4149697Z status: exit code: 1
2020-03-12T18:40:53.4151084Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/index-page/auxiliary/all-item-types/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/index-page" "/checkout/src/test/rustdoc/auxiliary/all-item-types.rs"
2020-03-12T18:40:53.4152314Z ------------------------------------------
2020-03-12T18:40:53.4152486Z 
2020-03-12T18:40:53.4152857Z ------------------------------------------
2020-03-12T18:40:53.4153064Z stderr:
---
2020-03-12T18:40:53.4162732Z ---- [rustdoc] rustdoc/inline_cross/add-docs.rs stdout ----
2020-03-12T18:40:53.4162940Z 
2020-03-12T18:40:53.4163095Z error: rustdoc failed!
2020-03-12T18:40:53.4163398Z status: exit code: 1
2020-03-12T18:40:53.4164799Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/add-docs/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/add-docs" "/checkout/src/test/rustdoc/inline_cross/add-docs.rs"
2020-03-12T18:40:53.4166018Z ------------------------------------------
2020-03-12T18:40:53.4166190Z 
2020-03-12T18:40:53.4166547Z ------------------------------------------
2020-03-12T18:40:53.4166763Z stderr:
---
2020-03-12T18:40:53.4169572Z ---- [rustdoc] rustdoc/inline_cross/assoc-items.rs stdout ----
2020-03-12T18:40:53.4169777Z 
2020-03-12T18:40:53.4169934Z error: rustdoc failed!
2020-03-12T18:40:53.4170149Z status: exit code: 1
2020-03-12T18:40:53.4171621Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items/auxiliary/assoc-items/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items" "/checkout/src/test/rustdoc/inline_cross/auxiliary/assoc-items.rs"
2020-03-12T18:40:53.4172912Z ------------------------------------------
2020-03-12T18:40:53.4173083Z 
2020-03-12T18:40:53.4173458Z ------------------------------------------
2020-03-12T18:40:53.4173661Z stderr:
---
2020-03-12T18:40:53.4176438Z ---- [rustdoc] rustdoc/inline_cross/cross-glob.rs stdout ----
2020-03-12T18:40:53.4176640Z 
2020-03-12T18:40:53.4176813Z error: rustdoc failed!
2020-03-12T18:40:53.4177013Z status: exit code: 1
2020-03-12T18:40:53.4178552Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/cross-glob/auxiliary/cross-glob/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/cross-glob" "/checkout/src/test/rustdoc/inline_cross/auxiliary/cross-glob.rs"
2020-03-12T18:40:53.4179889Z ------------------------------------------
2020-03-12T18:40:53.4180061Z 
2020-03-12T18:40:53.4180433Z ------------------------------------------
2020-03-12T18:40:53.4180636Z stderr:
---
2020-03-12T18:40:53.4183428Z ---- [rustdoc] rustdoc/inline_cross/hidden-use.rs stdout ----
2020-03-12T18:40:53.4183649Z 
2020-03-12T18:40:53.4183806Z error: rustdoc failed!
2020-03-12T18:40:53.4184007Z status: exit code: 1
2020-03-12T18:40:53.4185507Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/hidden-use/auxiliary/rustdoc-hidden/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/hidden-use" "/checkout/src/test/rustdoc/inline_cross/auxiliary/rustdoc-hidden.rs"
2020-03-12T18:40:53.4186810Z ------------------------------------------
2020-03-12T18:40:53.4186983Z 
2020-03-12T18:40:53.4187339Z ------------------------------------------
2020-03-12T18:40:53.4187538Z stderr:
---
2020-03-12T18:40:53.4190427Z ---- [rustdoc] rustdoc/inline_cross/impl-inline-without-trait.rs stdout ----
2020-03-12T18:40:53.4190658Z 
2020-03-12T18:40:53.4190816Z error: rustdoc failed!
2020-03-12T18:40:53.4191014Z status: exit code: 1
2020-03-12T18:40:53.4192668Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/impl-inline-without-trait/auxiliary/impl-inline-without-trait/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/impl-inline-without-trait" "/checkout/src/test/rustdoc/inline_cross/auxiliary/impl-inline-without-trait.rs"
2020-03-12T18:40:53.4194050Z ------------------------------------------
2020-03-12T18:40:53.4194224Z 
2020-03-12T18:40:53.4194578Z ------------------------------------------
2020-03-12T18:40:53.4194794Z stderr:
---
2020-03-12T18:40:53.4197602Z ---- [rustdoc] rustdoc/inline_cross/default-trait-method.rs stdout ----
2020-03-12T18:40:53.4197885Z 
2020-03-12T18:40:53.4198056Z error: rustdoc failed!
2020-03-12T18:40:53.4198297Z status: exit code: 1
2020-03-12T18:40:53.4199773Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/default-trait-method/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/default-trait-method" "/checkout/src/test/rustdoc/inline_cross/default-trait-method.rs"
2020-03-12T18:40:53.4201062Z ------------------------------------------
2020-03-12T18:40:53.4201233Z 
2020-03-12T18:40:53.4201606Z ------------------------------------------
2020-03-12T18:40:53.4201807Z stderr:
---
2020-03-12T18:40:53.4204709Z ---- [rustdoc] rustdoc/inline_cross/inline_hidden.rs stdout ----
2020-03-12T18:40:53.4204931Z 
2020-03-12T18:40:53.4205091Z error: rustdoc failed!
2020-03-12T18:40:53.4205293Z status: exit code: 1
2020-03-12T18:40:53.4206813Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/inline_hidden/auxiliary/rustdoc-hidden/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/inline_hidden" "/checkout/src/test/rustdoc/inline_cross/auxiliary/rustdoc-hidden.rs"
2020-03-12T18:40:53.4208114Z ------------------------------------------
2020-03-12T18:40:53.4208303Z 
2020-03-12T18:40:53.4208659Z ------------------------------------------
2020-03-12T18:40:53.4208859Z stderr:
---
2020-03-12T18:40:53.4211661Z ---- [rustdoc] rustdoc/inline_cross/issue-28480.rs stdout ----
2020-03-12T18:40:53.4211868Z 
2020-03-12T18:40:53.4212024Z error: rustdoc failed!
2020-03-12T18:40:53.4212223Z status: exit code: 1
2020-03-12T18:40:53.4213777Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-28480/auxiliary/rustdoc-hidden-sig/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-28480" "/checkout/src/test/rustdoc/inline_cross/auxiliary/rustdoc-hidden-sig.rs"
2020-03-12T18:40:53.4215095Z ------------------------------------------
2020-03-12T18:40:53.4215268Z 
2020-03-12T18:40:53.4215622Z ------------------------------------------
2020-03-12T18:40:53.4215839Z stderr:
---
2020-03-12T18:40:53.4225589Z ---- [rustdoc] rustdoc/inline_cross/issue-31948-1.rs stdout ----
2020-03-12T18:40:53.4225800Z 
2020-03-12T18:40:53.4225973Z error: rustdoc failed!
2020-03-12T18:40:53.4226172Z status: exit code: 1
2020-03-12T18:40:53.4227744Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-31948-1/auxiliary/rustdoc-nonreachable-impls/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-31948-1" "/checkout/src/test/rustdoc/inline_cross/auxiliary/rustdoc-nonreachable-impls.rs"
2020-03-12T18:40:53.4229104Z ------------------------------------------
2020-03-12T18:40:53.4229292Z 
2020-03-12T18:40:53.4229650Z ------------------------------------------
2020-03-12T18:40:53.4229851Z stderr:
---
2020-03-12T18:40:53.4232660Z ---- [rustdoc] rustdoc/inline_cross/issue-31948-2.rs stdout ----
2020-03-12T18:40:53.4232885Z 
2020-03-12T18:40:53.4233047Z error: rustdoc failed!
2020-03-12T18:40:53.4233249Z status: exit code: 1
2020-03-12T18:40:53.4234833Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-31948-2/auxiliary/rustdoc-nonreachable-impls/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-31948-2" "/checkout/src/test/rustdoc/inline_cross/auxiliary/rustdoc-nonreachable-impls.rs"
2020-03-12T18:40:53.4236184Z ------------------------------------------
2020-03-12T18:40:53.4236355Z 
2020-03-12T18:40:53.4236712Z ------------------------------------------
2020-03-12T18:40:53.4236911Z stderr:
---
2020-03-12T18:40:53.4239843Z ---- [rustdoc] rustdoc/inline_cross/issue-31948.rs stdout ----
2020-03-12T18:40:53.4240053Z 
2020-03-12T18:40:53.4240209Z error: rustdoc failed!
2020-03-12T18:40:53.4240426Z status: exit code: 1
2020-03-12T18:40:53.4241994Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-31948/auxiliary/rustdoc-nonreachable-impls/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-31948" "/checkout/src/test/rustdoc/inline_cross/auxiliary/rustdoc-nonreachable-impls.rs"
2020-03-12T18:40:53.4243434Z ------------------------------------------
2020-03-12T18:40:53.4243608Z 
2020-03-12T18:40:53.4243963Z ------------------------------------------
2020-03-12T18:40:53.4244184Z stderr:
---
2020-03-12T18:40:53.4246969Z ---- [rustdoc] rustdoc/inline_cross/issue-32881.rs stdout ----
2020-03-12T18:40:53.4247180Z 
2020-03-12T18:40:53.4247351Z error: rustdoc failed!
2020-03-12T18:40:53.4247549Z status: exit code: 1
2020-03-12T18:40:53.4249106Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-32881/auxiliary/rustdoc-trait-object-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-32881" "/checkout/src/test/rustdoc/inline_cross/auxiliary/rustdoc-trait-object-impl.rs"
2020-03-12T18:40:53.4250443Z ------------------------------------------
2020-03-12T18:40:53.4250614Z 
2020-03-12T18:40:53.4250985Z ------------------------------------------
2020-03-12T18:40:53.4251184Z stderr:
---
2020-03-12T18:40:53.4253969Z ---- [rustdoc] rustdoc/inline_cross/issue-33113.rs stdout ----
2020-03-12T18:40:53.4254187Z 
2020-03-12T18:40:53.4254346Z error: rustdoc failed!
2020-03-12T18:40:53.4254546Z status: exit code: 1
2020-03-12T18:40:53.4256040Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-33113/auxiliary/issue-33113/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-33113" "/checkout/src/test/rustdoc/inline_cross/auxiliary/issue-33113.rs"
2020-03-12T18:40:53.4257419Z ------------------------------------------
2020-03-12T18:40:53.4257646Z 
2020-03-12T18:40:53.4258019Z ------------------------------------------
2020-03-12T18:40:53.4258219Z stderr:
---
2020-03-12T18:40:53.4261014Z ---- [rustdoc] rustdoc/inline_cross/macro-vis.rs stdout ----
2020-03-12T18:40:53.4261218Z 
2020-03-12T18:40:53.4261374Z error: rustdoc failed!
2020-03-12T18:40:53.4261575Z status: exit code: 1
2020-03-12T18:40:53.4263050Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/macro-vis/auxiliary/macro-vis/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/macro-vis" "/checkout/src/test/rustdoc/inline_cross/auxiliary/macro-vis.rs"
2020-03-12T18:40:53.4264326Z ------------------------------------------
2020-03-12T18:40:53.4264498Z 
2020-03-12T18:40:53.4264855Z ------------------------------------------
2020-03-12T18:40:53.4265070Z stderr:
---
2020-03-12T18:40:53.4267845Z ---- [rustdoc] rustdoc/inline_cross/macros.rs stdout ----
2020-03-12T18:40:53.4268041Z 
2020-03-12T18:40:53.4268196Z error: rustdoc failed!
2020-03-12T18:40:53.4268411Z status: exit code: 1
2020-03-12T18:40:53.4269832Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/macros/auxiliary/macros/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/macros" "/checkout/src/test/rustdoc/inline_cross/auxiliary/macros.rs"
2020-03-12T18:40:53.4271088Z ------------------------------------------
2020-03-12T18:40:53.4271259Z 
2020-03-12T18:40:53.4271629Z ------------------------------------------
2020-03-12T18:40:53.4271834Z stderr:
---
2020-03-12T18:40:53.4274613Z ---- [rustdoc] rustdoc/inline_cross/proc_macro.rs stdout ----
2020-03-12T18:40:53.4274818Z 
2020-03-12T18:40:53.4274989Z error: rustdoc failed!
2020-03-12T18:40:53.4275190Z status: exit code: 1
2020-03-12T18:40:53.4276786Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/proc_macro/auxiliary/proc_macro/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/proc_macro" "/checkout/src/test/rustdoc/inline_cross/auxiliary/proc_macro.rs" "--crate-type" "proc-macro"
2020-03-12T18:40:53.4278166Z ------------------------------------------
2020-03-12T18:40:53.4278354Z 
2020-03-12T18:40:53.4278711Z ------------------------------------------
2020-03-12T18:40:53.4278910Z stderr:
---
2020-03-12T18:40:53.4281994Z ---- [rustdoc] rustdoc/inline_cross/renamed-via-module.rs stdout ----
2020-03-12T18:40:53.4282218Z 
2020-03-12T18:40:53.4282374Z error: rustdoc failed!
2020-03-12T18:40:53.4282575Z status: exit code: 1
2020-03-12T18:40:53.4284241Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/renamed-via-module/auxiliary/renamed-via-module/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/renamed-via-module" "/checkout/src/test/rustdoc/inline_cross/auxiliary/renamed-via-module.rs"
2020-03-12T18:40:53.4285584Z ------------------------------------------
2020-03-12T18:40:53.4285756Z 
2020-03-12T18:40:53.4286113Z ------------------------------------------
2020-03-12T18:40:53.4286327Z stderr:
---
2020-03-12T18:40:53.4289126Z ---- [rustdoc] rustdoc/inline_cross/use_crate.rs stdout ----
2020-03-12T18:40:53.4289358Z 
2020-03-12T18:40:53.4289514Z error: rustdoc failed!
2020-03-12T18:40:53.4289731Z status: exit code: 1
2020-03-12T18:40:53.4291182Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/use_crate/auxiliary/use_crate/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/use_crate" "/checkout/src/test/rustdoc/inline_cross/auxiliary/use_crate.rs"
2020-03-12T18:40:53.4292472Z ------------------------------------------
2020-03-12T18:40:53.4292646Z 
2020-03-12T18:40:53.4293016Z ------------------------------------------
2020-03-12T18:40:53.4293218Z stderr:
---
2020-03-12T18:40:53.4296043Z ---- [rustdoc] rustdoc/inline_local/glob-extern-no-defaults.rs stdout ----
2020-03-12T18:40:53.4296269Z 
2020-03-12T18:40:53.4296440Z error: rustdoc failed!
2020-03-12T18:40:53.4296640Z status: exit code: 1
2020-03-12T18:40:53.4298336Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/glob-extern-no-defaults/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/glob-extern-no-defaults" "/checkout/src/test/rustdoc/inline_local/glob-extern-no-defaults.rs" "--no-defaults"
2020-03-12T18:40:53.4299671Z ------------------------------------------
2020-03-12T18:40:53.4299843Z 
2020-03-12T18:40:53.4300214Z ------------------------------------------
2020-03-12T18:40:53.4300416Z stderr:
2020-03-12T18:40:53.4300416Z stderr:
2020-03-12T18:40:53.4300781Z ------------------------------------------
2020-03-12T18:40:53.4301271Z warning: the 'no-defaults' flag is considered deprecated
2020-03-12T18:40:53.4301495Z   |
2020-03-12T18:40:53.4302189Z   = warning: see issue #44136 <***/issues/44136> for more information
2020-03-12T18:40:53.4302729Z   = help: you may want to use --document-private-items
2020-03-12T18:40:53.4303505Z thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_interface/passes.rs:741:23
2020-03-12T18:40:53.4303999Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-12T18:40:53.4304232Z 
2020-03-12T18:40:53.4304592Z ------------------------------------------
2020-03-12T18:40:53.4304592Z ------------------------------------------
2020-03-12T18:40:53.4304781Z 
2020-03-12T18:40:53.4304879Z 
2020-03-12T18:40:53.4305283Z ---- [rustdoc] rustdoc/inline_cross/trait-vis.rs stdout ----
2020-03-12T18:40:53.4305486Z 
2020-03-12T18:40:53.4305660Z error: rustdoc failed!
2020-03-12T18:40:53.4305861Z status: exit code: 1
2020-03-12T18:40:53.4307250Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/trait-vis/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/trait-vis" "/checkout/src/test/rustdoc/inline_cross/trait-vis.rs"
2020-03-12T18:40:53.4308485Z ------------------------------------------
2020-03-12T18:40:53.4308657Z 
2020-03-12T18:40:53.4309031Z ------------------------------------------
2020-03-12T18:40:53.4309232Z stderr:
---
2020-03-12T18:40:53.4312023Z ---- [rustdoc] rustdoc/inline_local/glob-extern.rs stdout ----
2020-03-12T18:40:53.4312252Z 
2020-03-12T18:40:53.4312409Z error: rustdoc failed!
2020-03-12T18:40:53.4312611Z status: exit code: 1
2020-03-12T18:40:53.4314037Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/glob-extern/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/glob-extern" "/checkout/src/test/rustdoc/inline_local/glob-extern.rs"
2020-03-12T18:40:53.4315261Z ------------------------------------------
2020-03-12T18:40:53.4315447Z 
2020-03-12T18:40:53.4315803Z ------------------------------------------
2020-03-12T18:40:53.4316002Z stderr:
---
2020-03-12T18:40:53.4318981Z ---- [rustdoc] rustdoc/inline_local/glob-private-no-defaults.rs stdout ----
2020-03-12T18:40:53.4319210Z 
2020-03-12T18:40:53.4319367Z error: rustdoc failed!
2020-03-12T18:40:53.4319566Z status: exit code: 1
2020-03-12T18:40:53.4321120Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/glob-private-no-defaults/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/glob-private-no-defaults" "/checkout/src/test/rustdoc/inline_local/glob-private-no-defaults.rs" "--no-defaults"
2020-03-12T18:40:53.4322458Z ------------------------------------------
2020-03-12T18:40:53.4322629Z 
2020-03-12T18:40:53.4322991Z ------------------------------------------
2020-03-12T18:40:53.4323305Z stderr:
2020-03-12T18:40:53.4323305Z stderr:
2020-03-12T18:40:53.4323681Z ------------------------------------------
2020-03-12T18:40:53.4324151Z warning: the 'no-defaults' flag is considered deprecated
2020-03-12T18:40:53.4324392Z   |
2020-03-12T18:40:53.4324950Z   = warning: see issue #44136 <***/issues/44136> for more information
2020-03-12T18:40:53.4325478Z   = help: you may want to use --document-private-items
2020-03-12T18:40:53.4326264Z thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_interface/passes.rs:741:23
2020-03-12T18:40:53.4326739Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-12T18:40:53.4326987Z 
2020-03-12T18:40:53.4327344Z ------------------------------------------
2020-03-12T18:40:53.4327344Z ------------------------------------------
2020-03-12T18:40:53.4327523Z 
2020-03-12T18:40:53.4327619Z 
2020-03-12T18:40:53.4328051Z ---- [rustdoc] rustdoc/inline_local/glob-private.rs stdout ----
2020-03-12T18:40:53.4328268Z 
2020-03-12T18:40:53.4328428Z error: rustdoc failed!
2020-03-12T18:40:53.4328627Z status: exit code: 1
2020-03-12T18:40:53.4330052Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/glob-private/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/glob-private" "/checkout/src/test/rustdoc/inline_local/glob-private.rs"
2020-03-12T18:40:53.4331297Z ------------------------------------------
2020-03-12T18:40:53.4331468Z 
2020-03-12T18:40:53.4331825Z ------------------------------------------
2020-03-12T18:40:53.4332043Z stderr:
---
2020-03-12T18:40:53.4334842Z ---- [rustdoc] rustdoc/inline_local/hidden-use.rs stdout ----
2020-03-12T18:40:53.4335047Z 
2020-03-12T18:40:53.4335204Z error: rustdoc failed!
2020-03-12T18:40:53.4335421Z status: exit code: 1
2020-03-12T18:40:53.4336804Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/hidden-use/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/hidden-use" "/checkout/src/test/rustdoc/inline_local/hidden-use.rs"
2020-03-12T18:40:53.4338174Z ------------------------------------------
2020-03-12T18:40:53.4338351Z 
2020-03-12T18:40:53.4338732Z ------------------------------------------
2020-03-12T18:40:53.4338934Z stderr:
---
2020-03-12T18:40:53.4341721Z ---- [rustdoc] rustdoc/inline_local/issue-28537.rs stdout ----
2020-03-12T18:40:53.4341926Z 
2020-03-12T18:40:53.4342097Z error: rustdoc failed!
2020-03-12T18:40:53.4342298Z status: exit code: 1
2020-03-12T18:40:53.4343714Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/issue-28537/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/issue-28537" "/checkout/src/test/rustdoc/inline_local/issue-28537.rs"
2020-03-12T18:40:53.4344953Z ------------------------------------------
2020-03-12T18:40:53.4345124Z 
2020-03-12T18:40:53.4345494Z ------------------------------------------
2020-03-12T18:40:53.4345696Z stderr:
---
2020-03-12T18:40:53.4348486Z ---- [rustdoc] rustdoc/inline_local/issue-32343.rs stdout ----
2020-03-12T18:40:53.4348708Z 
2020-03-12T18:40:53.4348866Z error: rustdoc failed!
2020-03-12T18:40:53.4349068Z status: exit code: 1
2020-03-12T18:40:53.4350493Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/issue-32343/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/issue-32343" "/checkout/src/test/rustdoc/inline_local/issue-32343.rs"
2020-03-12T18:40:53.4351729Z ------------------------------------------
2020-03-12T18:40:53.4351902Z 
2020-03-12T18:40:53.4352259Z ------------------------------------------
2020-03-12T18:40:53.4352466Z stderr:
---
2020-03-12T18:40:53.4369126Z ---- [rustdoc] rustdoc/inline_local/trait-vis.rs stdout ----
2020-03-12T18:40:53.4369327Z 
2020-03-12T18:40:53.4369501Z error: rustdoc failed!
2020-03-12T18:40:53.4369702Z status: exit code: 1
2020-03-12T18:40:53.4371082Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/trait-vis/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/trait-vis" "/checkout/src/test/rustdoc/inline_local/trait-vis.rs"
2020-03-12T18:40:53.4372525Z ------------------------------------------
2020-03-12T18:40:53.4372714Z 
2020-03-12T18:40:53.4373075Z ------------------------------------------
2020-03-12T18:40:53.4373275Z stderr:
---
2020-03-12T18:40:53.4376022Z ---- [rustdoc] rustdoc/internal.rs stdout ----
2020-03-12T18:40:53.4376218Z 
2020-03-12T18:40:53.4376374Z error: rustdoc failed!
2020-03-12T18:40:53.4376572Z status: exit code: 1
2020-03-12T18:40:53.4378021Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/internal/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/internal" "/checkout/src/test/rustdoc/internal.rs" "-Z" "force-unstable-if-unmarked"
2020-03-12T18:40:53.4379293Z ------------------------------------------
2020-03-12T18:40:53.4379467Z 
2020-03-12T18:40:53.4379822Z ------------------------------------------
2020-03-12T18:40:53.4380024Z stderr:
---
2020-03-12T18:40:53.4382860Z ---- [rustdoc] rustdoc/intra-doc-link-enum-struct-field.rs stdout ----
2020-03-12T18:40:53.4383083Z 
2020-03-12T18:40:53.4383242Z error: rustdoc failed!
2020-03-12T18:40:53.4383440Z status: exit code: 1
2020-03-12T18:40:53.4384912Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc-link-enum-struct-field/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc-link-enum-struct-field" "/checkout/src/test/rustdoc/intra-doc-link-enum-struct-field.rs"
2020-03-12T18:40:53.4386188Z ------------------------------------------
2020-03-12T18:40:53.4386360Z 
2020-03-12T18:40:53.4386718Z ------------------------------------------
2020-03-12T18:40:53.4386936Z stderr:
---
2020-03-12T18:40:53.4396548Z ---- [rustdoc] rustdoc/intra-link-in-bodies.rs stdout ----
2020-03-12T18:40:53.4396745Z 
2020-03-12T18:40:53.4396920Z error: rustdoc failed!
2020-03-12T18:40:53.4397122Z status: exit code: 1
2020-03-12T18:40:53.4398959Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-in-bodies/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-in-bodies" "/checkout/src/test/rustdoc/intra-link-in-bodies.rs"
2020-03-12T18:40:53.4400202Z ------------------------------------------
2020-03-12T18:40:53.4400390Z 
2020-03-12T18:40:53.4400751Z ------------------------------------------
2020-03-12T18:40:53.4400951Z stderr:
---
2020-03-12T18:40:53.4410689Z ---- [rustdoc] rustdoc/intra-link-extern-crate.rs stdout ----
2020-03-12T18:40:53.4410894Z 
2020-03-12T18:40:53.4411050Z error: rustdoc failed!
2020-03-12T18:40:53.4411250Z status: exit code: 1
2020-03-12T18:40:53.4412658Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-extern-crate/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-extern-crate" "/checkout/src/test/rustdoc/intra-link-extern-crate.rs"
2020-03-12T18:40:53.4413900Z ------------------------------------------
2020-03-12T18:40:53.4414073Z 
2020-03-12T18:40:53.4414430Z ------------------------------------------
2020-03-12T18:40:53.4414649Z stderr:
---
2020-03-12T18:40:53.4424331Z ---- [rustdoc] rustdoc/intra-link-private.rs stdout ----
2020-03-12T18:40:53.4424528Z 
2020-03-12T18:40:53.4424702Z error: rustdoc failed!
2020-03-12T18:40:53.4424902Z status: exit code: 1
2020-03-12T18:40:53.4426266Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-private/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-private" "/checkout/src/test/rustdoc/intra-link-private.rs"
2020-03-12T18:40:53.4427471Z ------------------------------------------
2020-03-12T18:40:53.4427648Z 
2020-03-12T18:40:53.4428026Z ------------------------------------------
2020-03-12T18:40:53.4428232Z stderr:
---
2020-03-12T18:40:53.4431007Z ---- [rustdoc] rustdoc/intra-link-self.rs stdout ----
2020-03-12T18:40:53.4431217Z 
2020-03-12T18:40:53.4431374Z error: rustdoc failed!
2020-03-12T18:40:53.4431573Z status: exit code: 1
2020-03-12T18:40:53.4432921Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-self/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-self" "/checkout/src/test/rustdoc/intra-link-self.rs"
2020-03-12T18:40:53.4434103Z ------------------------------------------
2020-03-12T18:40:53.4434292Z 
2020-03-12T18:40:53.4434651Z ------------------------------------------
2020-03-12T18:40:53.4434851Z stderr:
---
2020-03-12T18:40:53.4437927Z ---- [rustdoc] rustdoc/intra-links-anchors.rs stdout ----
2020-03-12T18:40:53.4438207Z 
2020-03-12T18:40:53.4438364Z error: rustdoc failed!
2020-03-12T18:40:53.4438622Z status: exit code: 1
2020-03-12T18:40:53.4440012Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-links-anchors/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-links-anchors" "/checkout/src/test/rustdoc/intra-links-anchors.rs"
2020-03-12T18:40:53.4441223Z ------------------------------------------
2020-03-12T18:40:53.4441393Z 
2020-03-12T18:40:53.4441750Z ------------------------------------------
2020-03-12T18:40:53.4441963Z stderr:
---
2020-03-12T18:40:53.4444951Z ---- [rustdoc] rustdoc/intra-links.rs stdout ----
2020-03-12T18:40:53.4445137Z 
2020-03-12T18:40:53.4445294Z error: rustdoc failed!
2020-03-12T18:40:53.4445513Z status: exit code: 1
2020-03-12T18:40:53.4446812Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-links/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-links" "/checkout/src/test/rustdoc/intra-links.rs"
2020-03-12T18:40:53.4447987Z ------------------------------------------
2020-03-12T18:40:53.4448166Z 
2020-03-12T18:40:53.4448523Z ------------------------------------------
2020-03-12T18:40:53.4448746Z stderr:
---
2020-03-12T18:40:53.4451524Z ---- [rustdoc] rustdoc/invalid.crate.name.rs stdout ----
2020-03-12T18:40:53.4451718Z 
2020-03-12T18:40:53.4451892Z error: rustdoc failed!
2020-03-12T18:40:53.4452093Z status: exit code: 1
2020-03-12T18:40:53.4453496Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/invalid.crate.name/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/invalid.crate.name" "/checkout/src/test/rustdoc/invalid.crate.name.rs" "--crate-name" "foo"
2020-03-12T18:40:53.4454738Z ------------------------------------------
2020-03-12T18:40:53.4454910Z 
2020-03-12T18:40:53.4455285Z ------------------------------------------
2020-03-12T18:40:53.4455487Z stderr:
---
2020-03-12T18:40:53.4465253Z ---- [rustdoc] rustdoc/issue-12834.rs stdout ----
2020-03-12T18:40:53.4465439Z 
2020-03-12T18:40:53.4465595Z error: rustdoc failed!
2020-03-12T18:40:53.4465798Z status: exit code: 1
2020-03-12T18:40:53.4467118Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-12834/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-12834" "/checkout/src/test/rustdoc/issue-12834.rs"
2020-03-12T18:40:53.4468290Z ------------------------------------------
2020-03-12T18:40:53.4468468Z 
2020-03-12T18:40:53.4468825Z ------------------------------------------
2020-03-12T18:40:53.4469026Z stderr:
---
2020-03-12T18:40:53.4471803Z ---- [rustdoc] rustdoc/issue-15169.rs stdout ----
2020-03-12T18:40:53.4471989Z 
2020-03-12T18:40:53.4472144Z error: rustdoc failed!
2020-03-12T18:40:53.4472365Z status: exit code: 1
2020-03-12T18:40:53.4473671Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15169/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15169" "/checkout/src/test/rustdoc/issue-15169.rs"
2020-03-12T18:40:53.4474845Z ------------------------------------------
2020-03-12T18:40:53.4475017Z 
2020-03-12T18:40:53.4475371Z ------------------------------------------
2020-03-12T18:40:53.4475588Z stderr:
---
2020-03-12T18:40:53.4478419Z ---- [rustdoc] rustdoc/issue-13698.rs stdout ----
2020-03-12T18:40:53.4478651Z 
2020-03-12T18:40:53.4478811Z error: rustdoc failed!
2020-03-12T18:40:53.4479028Z status: exit code: 1
2020-03-12T18:40:53.4480341Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-13698/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-13698" "/checkout/src/test/rustdoc/issue-13698.rs"
2020-03-12T18:40:53.4481516Z ------------------------------------------
2020-03-12T18:40:53.4481686Z 
2020-03-12T18:40:53.4482060Z ------------------------------------------
2020-03-12T18:40:53.4482262Z stderr:
---
2020-03-12T18:40:53.4485148Z ---- [rustdoc] rustdoc/issue-15318-3.rs stdout ----
2020-03-12T18:40:53.4485336Z 
2020-03-12T18:40:53.4485510Z error: rustdoc failed!
2020-03-12T18:40:53.4485710Z status: exit code: 1
2020-03-12T18:40:53.4487027Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15318-3/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15318-3" "/checkout/src/test/rustdoc/issue-15318-3.rs"
2020-03-12T18:40:53.4488251Z ------------------------------------------
2020-03-12T18:40:53.4488425Z 
2020-03-12T18:40:53.4488804Z ------------------------------------------
2020-03-12T18:40:53.4489004Z stderr:
---
2020-03-12T18:40:53.4491772Z ---- [rustdoc] rustdoc/issue-15318-2.rs stdout ----
2020-03-12T18:40:53.4491979Z 
2020-03-12T18:40:53.4492135Z error: rustdoc failed!
2020-03-12T18:40:53.4492335Z status: exit code: 1
2020-03-12T18:40:53.4493681Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15318-2/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15318-2" "/checkout/src/test/rustdoc/issue-15318-2.rs"
2020-03-12T18:40:53.4494859Z ------------------------------------------
2020-03-12T18:40:53.4495051Z 
2020-03-12T18:40:53.4495406Z ------------------------------------------
2020-03-12T18:40:53.4495605Z stderr:
---
2020-03-12T18:40:53.4498488Z ---- [rustdoc] rustdoc/issue-15347.rs stdout ----
2020-03-12T18:40:53.4498726Z 
2020-03-12T18:40:53.4498888Z error: rustdoc failed!
2020-03-12T18:40:53.4499086Z status: exit code: 1
2020-03-12T18:40:53.4500602Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15347/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15347" "/checkout/src/test/rustdoc/issue-15347.rs" "--no-defaults" "--passes" "collapse-docs" "--passes" "unindent-comments"
2020-03-12T18:40:53.4501901Z ------------------------------------------
2020-03-12T18:40:53.4502074Z 
2020-03-12T18:40:53.4502428Z ------------------------------------------
2020-03-12T18:40:53.4502645Z stderr:
2020-03-12T18:40:53.4502645Z stderr:
2020-03-12T18:40:53.4503012Z ------------------------------------------
2020-03-12T18:40:53.4503493Z warning: the 'no-defaults' flag is considered deprecated
2020-03-12T18:40:53.4503741Z   |
2020-03-12T18:40:53.4504339Z   = warning: see issue #44136 <***/issues/44136> for more information
2020-03-12T18:40:53.4504865Z   = help: you may want to use --document-private-items
2020-03-12T18:40:53.4505467Z warning: the 'passes' flag is considered deprecated
2020-03-12T18:40:53.4505680Z   |
2020-03-12T18:40:53.4505680Z   |
2020-03-12T18:40:53.4506245Z   = warning: see issue #44136 <***/issues/44136> for more information
2020-03-12T18:40:53.4507049Z thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_interface/passes.rs:741:23
2020-03-12T18:40:53.4507543Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-12T18:40:53.4507775Z 
2020-03-12T18:40:53.4508132Z ------------------------------------------
2020-03-12T18:40:53.4508132Z ------------------------------------------
2020-03-12T18:40:53.4508304Z 
2020-03-12T18:40:53.4508421Z 
2020-03-12T18:40:53.4508809Z ---- [rustdoc] rustdoc/issue-16019.rs stdout ----
2020-03-12T18:40:53.4508994Z 
2020-03-12T18:40:53.4509155Z error: rustdoc failed!
2020-03-12T18:40:53.4509371Z status: exit code: 1
2020-03-12T18:40:53.4510688Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-16019/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-16019" "/checkout/src/test/rustdoc/issue-16019.rs"
2020-03-12T18:40:53.4511858Z ------------------------------------------
2020-03-12T18:40:53.4512030Z 
2020-03-12T18:40:53.4512403Z ------------------------------------------
2020-03-12T18:40:53.4512605Z stderr:
---
2020-03-12T18:40:53.4515379Z ---- [rustdoc] rustdoc/issue-15318.rs stdout ----
2020-03-12T18:40:53.4515563Z 
2020-03-12T18:40:53.4515736Z error: rustdoc failed!
2020-03-12T18:40:53.4515936Z status: exit code: 1
2020-03-12T18:40:53.4517233Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15318/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15318" "/checkout/src/test/rustdoc/issue-15318.rs"
2020-03-12T18:40:53.4518410Z ------------------------------------------
2020-03-12T18:40:53.4518641Z 
2020-03-12T18:40:53.4519031Z ------------------------------------------
2020-03-12T18:40:53.4519280Z stderr:
---
2020-03-12T18:40:53.4522065Z ---- [rustdoc] rustdoc/issue-16265-1.rs stdout ----
2020-03-12T18:40:53.4522254Z 
2020-03-12T18:40:53.4522430Z error: rustdoc failed!
2020-03-12T18:40:53.4522628Z status: exit code: 1
2020-03-12T18:40:53.4524043Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-16265-1/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-16265-1" "/checkout/src/test/rustdoc/issue-16265-1.rs"
2020-03-12T18:40:53.4525239Z ------------------------------------------
2020-03-12T18:40:53.4525430Z 
2020-03-12T18:40:53.4525789Z ------------------------------------------
2020-03-12T18:40:53.4525988Z stderr:
---
2020-03-12T18:40:53.4528773Z ---- [rustdoc] rustdoc/issue-16265-2.rs stdout ----
2020-03-12T18:40:53.4528967Z 
2020-03-12T18:40:53.4529126Z error: rustdoc failed!
2020-03-12T18:40:53.4529331Z status: exit code: 1
2020-03-12T18:40:53.4530671Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-16265-2/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-16265-2" "/checkout/src/test/rustdoc/issue-16265-2.rs"
2020-03-12T18:40:53.4531859Z ------------------------------------------
2020-03-12T18:40:53.4532032Z 
2020-03-12T18:40:53.4532389Z ------------------------------------------
2020-03-12T18:40:53.4532590Z stderr:
---
2020-03-12T18:40:53.4535382Z ---- [rustdoc] rustdoc/issue-17476.rs stdout ----
2020-03-12T18:40:53.4535567Z 
2020-03-12T18:40:53.4535725Z error: rustdoc failed!
2020-03-12T18:40:53.4535941Z status: exit code: 1
2020-03-12T18:40:53.4537239Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-17476/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-17476" "/checkout/src/test/rustdoc/issue-17476.rs"
2020-03-12T18:40:53.4538406Z ------------------------------------------
2020-03-12T18:40:53.4538652Z 
2020-03-12T18:40:53.4539017Z ------------------------------------------
2020-03-12T18:40:53.4539290Z stderr:
---
2020-03-12T18:40:53.4542063Z ---- [rustdoc] rustdoc/issue-19055.rs stdout ----
2020-03-12T18:40:53.4542248Z 
2020-03-12T18:40:53.4542406Z error: rustdoc failed!
2020-03-12T18:40:53.4542624Z status: exit code: 1
2020-03-12T18:40:53.4543926Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19055/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19055" "/checkout/src/test/rustdoc/issue-19055.rs"
2020-03-12T18:40:53.4545111Z ------------------------------------------
2020-03-12T18:40:53.4545282Z 
2020-03-12T18:40:53.4545654Z ------------------------------------------
2020-03-12T18:40:53.4545856Z stderr:
---
2020-03-12T18:40:53.4548617Z ---- [rustdoc] rustdoc/issue-19190-2.rs stdout ----
2020-03-12T18:40:53.4548814Z 
2020-03-12T18:40:53.4548988Z error: rustdoc failed!
2020-03-12T18:40:53.4549192Z status: exit code: 1
2020-03-12T18:40:53.4550514Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190-2/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190-2" "/checkout/src/test/rustdoc/issue-19190-2.rs"
2020-03-12T18:40:53.4551692Z ------------------------------------------
2020-03-12T18:40:53.4551862Z 
2020-03-12T18:40:53.4552238Z ------------------------------------------
2020-03-12T18:40:53.4552436Z stderr:
---
2020-03-12T18:40:53.4555200Z ---- [rustdoc] rustdoc/issue-19190.rs stdout ----
2020-03-12T18:40:53.4555400Z 
2020-03-12T18:40:53.4555558Z error: rustdoc failed!
2020-03-12T18:40:53.4555760Z status: exit code: 1
2020-03-12T18:40:53.4557209Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190" "/checkout/src/test/rustdoc/issue-19190.rs" "-Z" "unstable-options" "--generate-redirect-pages"
2020-03-12T18:40:53.4558518Z ------------------------------------------
2020-03-12T18:40:53.4558708Z 
2020-03-12T18:40:53.4559113Z ------------------------------------------
2020-03-12T18:40:53.4559318Z stderr:
---
2020-03-12T18:40:53.4562108Z ---- [rustdoc] rustdoc/issue-20175.rs stdout ----
2020-03-12T18:40:53.4562293Z 
2020-03-12T18:40:53.4562450Z error: rustdoc failed!
2020-03-12T18:40:53.4562649Z status: exit code: 1
2020-03-12T18:40:53.4564155Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20175/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20175" "/checkout/src/test/rustdoc/issue-20175.rs"
2020-03-12T18:40:53.4565345Z ------------------------------------------
2020-03-12T18:40:53.4565516Z 
2020-03-12T18:40:53.4565872Z ------------------------------------------
2020-03-12T18:40:53.4566088Z stderr:
---
2020-03-12T18:40:53.4568849Z ---- [rustdoc] rustdoc/issue-19190-3.rs stdout ----
2020-03-12T18:40:53.4569036Z 
2020-03-12T18:40:53.4569193Z error: rustdoc failed!
2020-03-12T18:40:53.4569415Z status: exit code: 1
2020-03-12T18:40:53.4570731Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190-3/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190-3" "/checkout/src/test/rustdoc/issue-19190-3.rs"
2020-03-12T18:40:53.4571918Z ------------------------------------------
2020-03-12T18:40:53.4572089Z 
2020-03-12T18:40:53.4572443Z ------------------------------------------
2020-03-12T18:40:53.4572661Z stderr:
---
2020-03-12T18:40:53.4575423Z ---- [rustdoc] rustdoc/issue-20646.rs stdout ----
2020-03-12T18:40:53.4575608Z 
2020-03-12T18:40:53.4575765Z error: rustdoc failed!
2020-03-12T18:40:53.4575984Z status: exit code: 1
2020-03-12T18:40:53.4577286Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20646/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20646" "/checkout/src/test/rustdoc/issue-20646.rs"
2020-03-12T18:40:53.4578450Z ------------------------------------------
2020-03-12T18:40:53.4578701Z 
2020-03-12T18:40:53.4579084Z ------------------------------------------
2020-03-12T18:40:53.4579356Z stderr:
---
2020-03-12T18:40:53.4582128Z ---- [rustdoc] rustdoc/issue-20727-2.rs stdout ----
2020-03-12T18:40:53.4582316Z 
2020-03-12T18:40:53.4582490Z error: rustdoc failed!
2020-03-12T18:40:53.4582689Z status: exit code: 1
2020-03-12T18:40:53.4584018Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-2/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-2" "/checkout/src/test/rustdoc/issue-20727-2.rs"
2020-03-12T18:40:53.4585211Z ------------------------------------------
2020-03-12T18:40:53.4585383Z 
2020-03-12T18:40:53.4585756Z ------------------------------------------
2020-03-12T18:40:53.4585954Z stderr:
---
2020-03-12T18:40:53.4588756Z ---- [rustdoc] rustdoc/issue-20727-3.rs stdout ----
2020-03-12T18:40:53.4588966Z 
2020-03-12T18:40:53.4589124Z error: rustdoc failed!
2020-03-12T18:40:53.4589327Z status: exit code: 1
2020-03-12T18:40:53.4590664Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-3/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-3" "/checkout/src/test/rustdoc/issue-20727-3.rs"
2020-03-12T18:40:53.4592079Z ------------------------------------------
2020-03-12T18:40:53.4592270Z 
2020-03-12T18:40:53.4592628Z ------------------------------------------
2020-03-12T18:40:53.4592830Z stderr:
---
2020-03-12T18:40:53.4595786Z ---- [rustdoc] rustdoc/issue-20727-4.rs stdout ----
2020-03-12T18:40:53.4595974Z 
2020-03-12T18:40:53.4596130Z error: rustdoc failed!
2020-03-12T18:40:53.4596330Z status: exit code: 1
2020-03-12T18:40:53.4597670Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-4/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-4" "/checkout/src/test/rustdoc/issue-20727-4.rs"
2020-03-12T18:40:53.4598851Z ------------------------------------------
2020-03-12T18:40:53.4599112Z 
2020-03-12T18:40:53.4599481Z ------------------------------------------
2020-03-12T18:40:53.4599750Z stderr:
---
2020-03-12T18:40:53.4602526Z ---- [rustdoc] rustdoc/issue-21092.rs stdout ----
2020-03-12T18:40:53.4602711Z 
2020-03-12T18:40:53.4602868Z error: rustdoc failed!
2020-03-12T18:40:53.4603085Z status: exit code: 1
2020-03-12T18:40:53.4604493Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-21092/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-21092" "/checkout/src/test/rustdoc/issue-21092.rs"
2020-03-12T18:40:53.4605671Z ------------------------------------------
2020-03-12T18:40:53.4605843Z 
2020-03-12T18:40:53.4606196Z ------------------------------------------
2020-03-12T18:40:53.4606416Z stderr:
---
2020-03-12T18:40:53.4609174Z ---- [rustdoc] rustdoc/issue-20727.rs stdout ----
2020-03-12T18:40:53.4609365Z 
2020-03-12T18:40:53.4609538Z error: rustdoc failed!
2020-03-12T18:40:53.4609742Z status: exit code: 1
2020-03-12T18:40:53.4611049Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727" "/checkout/src/test/rustdoc/issue-20727.rs"
2020-03-12T18:40:53.4612221Z ------------------------------------------
2020-03-12T18:40:53.4612393Z 
2020-03-12T18:40:53.4612765Z ------------------------------------------
2020-03-12T18:40:53.4612966Z stderr:
---
2020-03-12T18:40:53.4615734Z ---- [rustdoc] rustdoc/issue-21474.rs stdout ----
2020-03-12T18:40:53.4615919Z 
2020-03-12T18:40:53.4616092Z error: rustdoc failed!
2020-03-12T18:40:53.4616293Z status: exit code: 1
2020-03-12T18:40:53.4617601Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-21474/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-21474" "/checkout/src/test/rustdoc/issue-21474.rs"
2020-03-12T18:40:53.4618765Z ------------------------------------------
2020-03-12T18:40:53.4619027Z 
2020-03-12T18:40:53.4619396Z ------------------------------------------
2020-03-12T18:40:53.4619597Z stderr:
---
2020-03-12T18:40:53.4622422Z ---- [rustdoc] rustdoc/issue-21801.rs stdout ----
2020-03-12T18:40:53.4622625Z 
2020-03-12T18:40:53.4622783Z error: rustdoc failed!
2020-03-12T18:40:53.4622982Z status: exit code: 1
2020-03-12T18:40:53.4624312Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-21801/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-21801" "/checkout/src/test/rustdoc/issue-21801.rs"
2020-03-12T18:40:53.4625473Z ------------------------------------------
2020-03-12T18:40:53.4625663Z 
2020-03-12T18:40:53.4626019Z ------------------------------------------
2020-03-12T18:40:53.4626219Z stderr:
---
2020-03-12T18:40:53.4628987Z ---- [rustdoc] rustdoc/issue-22025.rs stdout ----
2020-03-12T18:40:53.4629180Z 
2020-03-12T18:40:53.4629339Z error: rustdoc failed!
2020-03-12T18:40:53.4629537Z status: exit code: 1
2020-03-12T18:40:53.4630869Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-22025/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-22025" "/checkout/src/test/rustdoc/issue-22025.rs"
2020-03-12T18:40:53.4632049Z ------------------------------------------
2020-03-12T18:40:53.4632220Z 
2020-03-12T18:40:53.4632573Z ------------------------------------------
2020-03-12T18:40:53.4632791Z stderr:
---
2020-03-12T18:40:53.4635565Z ---- [rustdoc] rustdoc/issue-22038.rs stdout ----
2020-03-12T18:40:53.4635748Z 
2020-03-12T18:40:53.4635907Z error: rustdoc failed!
2020-03-12T18:40:53.4636123Z status: exit code: 1
2020-03-12T18:40:53.4637430Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-22038/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-22038" "/checkout/src/test/rustdoc/issue-22038.rs"
2020-03-12T18:40:53.4638600Z ------------------------------------------
2020-03-12T18:40:53.4638773Z 
2020-03-12T18:40:53.4639191Z ------------------------------------------
2020-03-12T18:40:53.4639412Z stderr:
---
2020-03-12T18:40:53.4642230Z ---- [rustdoc] rustdoc/issue-23207.rs stdout ----
2020-03-12T18:40:53.4642414Z 
2020-03-12T18:40:53.4642589Z error: rustdoc failed!
2020-03-12T18:40:53.4642790Z status: exit code: 1
2020-03-12T18:40:53.4644192Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23207/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23207" "/checkout/src/test/rustdoc/issue-23207.rs"
2020-03-12T18:40:53.4645371Z ------------------------------------------
2020-03-12T18:40:53.4645545Z 
2020-03-12T18:40:53.4645919Z ------------------------------------------
2020-03-12T18:40:53.4646121Z stderr:
---
2020-03-12T18:40:53.4648869Z ---- [rustdoc] rustdoc/issue-23511.rs stdout ----
2020-03-12T18:40:53.4649055Z 
2020-03-12T18:40:53.4649233Z error: rustdoc failed!
2020-03-12T18:40:53.4649434Z status: exit code: 1
2020-03-12T18:40:53.4652621Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23511/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23511" "/checkout/src/test/rustdoc/issue-23511.rs"
2020-03-12T18:40:53.4653798Z ------------------------------------------
2020-03-12T18:40:53.4653986Z 
2020-03-12T18:40:53.4654341Z ------------------------------------------
2020-03-12T18:40:53.4654541Z stderr:
---
2020-03-12T18:40:53.4657321Z ---- [rustdoc] rustdoc/issue-23812.rs stdout ----
2020-03-12T18:40:53.4657506Z 
2020-03-12T18:40:53.4657662Z error: rustdoc failed!
2020-03-12T18:40:53.4657861Z status: exit code: 1
2020-03-12T18:40:53.4659184Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23812/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23812" "/checkout/src/test/rustdoc/issue-23812.rs"
2020-03-12T18:40:53.4660345Z ------------------------------------------
2020-03-12T18:40:53.4660518Z 
2020-03-12T18:40:53.4660874Z ------------------------------------------
2020-03-12T18:40:53.4661176Z stderr:
---
2020-03-12T18:40:53.4664036Z ---- [rustdoc] rustdoc/issue-25001.rs stdout ----
2020-03-12T18:40:53.4664219Z 
2020-03-12T18:40:53.4664376Z error: rustdoc failed!
2020-03-12T18:40:53.4664592Z status: exit code: 1
2020-03-12T18:40:53.4665913Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25001/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25001" "/checkout/src/test/rustdoc/issue-25001.rs"
2020-03-12T18:40:53.4667093Z ------------------------------------------
2020-03-12T18:40:53.4667267Z 
2020-03-12T18:40:53.4667621Z ------------------------------------------
2020-03-12T18:40:53.4667838Z stderr:
---
2020-03-12T18:40:53.4670590Z ---- [rustdoc] rustdoc/issue-26606.rs stdout ----
2020-03-12T18:40:53.4670775Z 
2020-03-12T18:40:53.4670929Z error: rustdoc failed!
2020-03-12T18:40:53.4671149Z status: exit code: 1
2020-03-12T18:40:53.4672564Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-26606/auxiliary/issue-26606-macro/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-26606" "/checkout/src/test/rustdoc/auxiliary/issue-26606-macro.rs"
2020-03-12T18:40:53.4673806Z ------------------------------------------
2020-03-12T18:40:53.4673977Z 
2020-03-12T18:40:53.4674349Z ------------------------------------------
2020-03-12T18:40:53.4674550Z stderr:
---
2020-03-12T18:40:53.4677313Z ---- [rustdoc] rustdoc/issue-26995.rs stdout ----
2020-03-12T18:40:53.4677498Z 
2020-03-12T18:40:53.4677670Z error: rustdoc failed!
2020-03-12T18:40:53.4677870Z status: exit code: 1
2020-03-12T18:40:53.4679208Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-26995/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-26995" "/checkout/src/test/rustdoc/issue-26995.rs" "--no-defaults"
2020-03-12T18:40:53.4680411Z ------------------------------------------
2020-03-12T18:40:53.4680582Z 
2020-03-12T18:40:53.4681025Z ------------------------------------------
2020-03-12T18:40:53.4681227Z stderr:
2020-03-12T18:40:53.4681227Z stderr:
2020-03-12T18:40:53.4681640Z ------------------------------------------
2020-03-12T18:40:53.4682142Z warning: the 'no-defaults' flag is considered deprecated
2020-03-12T18:40:53.4682368Z   |
2020-03-12T18:40:53.4682969Z   = warning: see issue #44136 <***/issues/44136> for more information
2020-03-12T18:40:53.4683615Z   = help: you may want to use --document-private-items
2020-03-12T18:40:53.4684381Z thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_interface/passes.rs:741:23
2020-03-12T18:40:53.4684874Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-12T18:40:53.4685107Z 
2020-03-12T18:40:53.4685462Z ------------------------------------------
2020-03-12T18:40:53.4685462Z ------------------------------------------
2020-03-12T18:40:53.4685650Z 
2020-03-12T18:40:53.4685747Z 
2020-03-12T18:40:53.4686165Z ---- [rustdoc] rustdoc/issue-27759.rs stdout ----
2020-03-12T18:40:53.4686350Z 
2020-03-12T18:40:53.4686529Z error: rustdoc failed!
2020-03-12T18:40:53.4686730Z status: exit code: 1
2020-03-12T18:40:53.4688047Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-27759/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-27759" "/checkout/src/test/rustdoc/issue-27759.rs"
2020-03-12T18:40:53.4689225Z ------------------------------------------
2020-03-12T18:40:53.4689397Z 
2020-03-12T18:40:53.4689772Z ------------------------------------------
2020-03-12T18:40:53.4689973Z stderr:
---
2020-03-12T18:40:53.4692737Z ---- [rustdoc] rustdoc/issue-27862.rs stdout ----
2020-03-12T18:40:53.4692923Z 
2020-03-12T18:40:53.4693095Z error: rustdoc failed!
2020-03-12T18:40:53.4693293Z status: exit code: 1
2020-03-12T18:40:53.4694597Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-27862/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-27862" "/checkout/src/test/rustdoc/issue-27862.rs"
2020-03-12T18:40:53.4695768Z ------------------------------------------
2020-03-12T18:40:53.4695954Z 
2020-03-12T18:40:53.4696311Z ------------------------------------------
2020-03-12T18:40:53.4696517Z stderr:
---
2020-03-12T18:40:53.4699278Z ---- [rustdoc] rustdoc/issue-27104.rs stdout ----
2020-03-12T18:40:53.4699480Z 
2020-03-12T18:40:53.4699636Z error: rustdoc failed!
2020-03-12T18:40:53.4699835Z status: exit code: 1
2020-03-12T18:40:53.4701349Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-27104/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-27104" "/checkout/src/test/rustdoc/issue-27104.rs" "--no-defaults" "--passes" "strip-priv-imports"
2020-03-12T18:40:53.4702689Z ------------------------------------------
2020-03-12T18:40:53.4702862Z 
2020-03-12T18:40:53.4703219Z ------------------------------------------
2020-03-12T18:40:53.4703421Z stderr:
2020-03-12T18:40:53.4703421Z stderr:
2020-03-12T18:40:53.4703802Z ------------------------------------------
2020-03-12T18:40:53.4704274Z warning: the 'no-defaults' flag is considered deprecated
2020-03-12T18:40:53.4704496Z   |
2020-03-12T18:40:53.4705067Z   = warning: see issue #44136 <***/issues/44136> for more information
2020-03-12T18:40:53.4705593Z   = help: you may want to use --document-private-items
2020-03-12T18:40:53.4706194Z warning: the 'passes' flag is considered deprecated
2020-03-12T18:40:53.4706409Z   |
2020-03-12T18:40:53.4706409Z   |
2020-03-12T18:40:53.4706965Z   = warning: see issue #44136 <***/issues/44136> for more information
2020-03-12T18:40:53.4707782Z thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_interface/passes.rs:741:23
2020-03-12T18:40:53.4708273Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-12T18:40:53.4708507Z 
2020-03-12T18:40:53.4708866Z ------------------------------------------
2020-03-12T18:40:53.4708866Z ------------------------------------------
2020-03-12T18:40:53.4709037Z 
2020-03-12T18:40:53.4709135Z 
2020-03-12T18:40:53.4709531Z ---- [rustdoc] rustdoc/issue-28478.rs stdout ----
2020-03-12T18:40:53.4709717Z 
2020-03-12T18:40:53.4709873Z error: rustdoc failed!
2020-03-12T18:40:53.4710088Z status: exit code: 1
2020-03-12T18:40:53.4711386Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-28478/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-28478" "/checkout/src/test/rustdoc/issue-28478.rs"
2020-03-12T18:40:53.4712573Z ------------------------------------------
2020-03-12T18:40:53.4712744Z 
2020-03-12T18:40:53.4713100Z ------------------------------------------
2020-03-12T18:40:53.4713316Z stderr:
---
2020-03-12T18:40:53.4716069Z ---- [rustdoc] rustdoc/issue-29449.rs stdout ----
2020-03-12T18:40:53.4716254Z 
2020-03-12T18:40:53.4716410Z error: rustdoc failed!
2020-03-12T18:40:53.4716626Z status: exit code: 1
2020-03-12T18:40:53.4717944Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29449/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29449" "/checkout/src/test/rustdoc/issue-29449.rs"
2020-03-12T18:40:53.4719117Z ------------------------------------------
2020-03-12T18:40:53.4719289Z 
2020-03-12T18:40:53.4719659Z ------------------------------------------
2020-03-12T18:40:53.4719859Z stderr:
---
2020-03-12T18:40:53.4723365Z ---- [rustdoc] rustdoc/issue-29503.rs stdout ----
2020-03-12T18:40:53.4723554Z 
2020-03-12T18:40:53.4723730Z error: rustdoc failed!
2020-03-12T18:40:53.4723931Z status: exit code: 1
2020-03-12T18:40:53.4725260Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29503/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29503" "/checkout/src/test/rustdoc/issue-29503.rs"
2020-03-12T18:40:53.4726437Z ------------------------------------------
2020-03-12T18:40:53.4726609Z 
2020-03-12T18:40:53.4726983Z ------------------------------------------
2020-03-12T18:40:53.4727185Z stderr:
---
2020-03-12T18:40:53.4729956Z ---- [rustdoc] rustdoc/issue-28927.rs stdout ----
2020-03-12T18:40:53.4730158Z 
2020-03-12T18:40:53.4730317Z error: rustdoc failed!
2020-03-12T18:40:53.4730517Z status: exit code: 1
2020-03-12T18:40:53.4731841Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-28927/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-28927" "/checkout/src/test/rustdoc/issue-28927.rs"
2020-03-12T18:40:53.4733014Z ------------------------------------------
2020-03-12T18:40:53.4733204Z 
2020-03-12T18:40:53.4733563Z ------------------------------------------
2020-03-12T18:40:53.4733763Z stderr:
---
2020-03-12T18:40:53.4736531Z ---- [rustdoc] rustdoc/issue-30109.rs stdout ----
2020-03-12T18:40:53.4736716Z 
2020-03-12T18:40:53.4736873Z error: rustdoc failed!
2020-03-12T18:40:53.4737073Z status: exit code: 1
2020-03-12T18:40:53.4738484Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30109/auxiliary/issue-30109-1/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30109" "/checkout/src/test/rustdoc/auxiliary/issue-30109-1.rs"
2020-03-12T18:40:53.4739721Z ------------------------------------------
2020-03-12T18:40:53.4739894Z 
2020-03-12T18:40:53.4740252Z ------------------------------------------
2020-03-12T18:40:53.4740470Z stderr:
---
2020-03-12T18:40:53.4743371Z ---- [rustdoc] rustdoc/issue-29584.rs stdout ----
2020-03-12T18:40:53.4743559Z 
2020-03-12T18:40:53.4743716Z error: rustdoc failed!
2020-03-12T18:40:53.4743929Z status: exit code: 1
2020-03-12T18:40:53.4745240Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29584/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29584" "/checkout/src/test/rustdoc/issue-29584.rs"
2020-03-12T18:40:53.4746409Z ------------------------------------------
2020-03-12T18:40:53.4746580Z 
2020-03-12T18:40:53.4746936Z ------------------------------------------
2020-03-12T18:40:53.4747151Z stderr:
---
2020-03-12T18:40:53.4750183Z ---- [rustdoc] rustdoc/issue-30366.rs stdout ----
2020-03-12T18:40:53.4750369Z 
2020-03-12T18:40:53.4750542Z error: rustdoc failed!
2020-03-12T18:40:53.4750742Z status: exit code: 1
2020-03-12T18:40:53.4752043Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30366/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30366" "/checkout/src/test/rustdoc/issue-30366.rs"
2020-03-12T18:40:53.4753223Z ------------------------------------------
2020-03-12T18:40:53.4753396Z 
2020-03-12T18:40:53.4753767Z ------------------------------------------
2020-03-12T18:40:53.4753968Z stderr:
---
2020-03-12T18:40:53.4756729Z ---- [rustdoc] rustdoc/issue-31808.rs stdout ----
2020-03-12T18:40:53.4756913Z 
2020-03-12T18:40:53.4757088Z error: rustdoc failed!
2020-03-12T18:40:53.4757287Z status: exit code: 1
2020-03-12T18:40:53.4758593Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-31808/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-31808" "/checkout/src/test/rustdoc/issue-31808.rs"
2020-03-12T18:40:53.4759772Z ------------------------------------------
2020-03-12T18:40:53.4759959Z 
2020-03-12T18:40:53.4760316Z ------------------------------------------
2020-03-12T18:40:53.4760519Z stderr:
---
2020-03-12T18:40:53.4763515Z ---- [rustdoc] rustdoc/issue-31899.rs stdout ----
2020-03-12T18:40:53.4763721Z 
2020-03-12T18:40:53.4763880Z error: rustdoc failed!
2020-03-12T18:40:53.4764078Z status: exit code: 1
2020-03-12T18:40:53.4765412Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-31899/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-31899" "/checkout/src/test/rustdoc/issue-31899.rs"
2020-03-12T18:40:53.4766568Z ------------------------------------------
2020-03-12T18:40:53.4766755Z 
2020-03-12T18:40:53.4767110Z ------------------------------------------
2020-03-12T18:40:53.4767312Z stderr:
---
2020-03-12T18:40:53.4770099Z ---- [rustdoc] rustdoc/issue-32374.rs stdout ----
2020-03-12T18:40:53.4770285Z 
2020-03-12T18:40:53.4770443Z error: rustdoc failed!
2020-03-12T18:40:53.4770643Z status: exit code: 1
2020-03-12T18:40:53.4771962Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-32374/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-32374" "/checkout/src/test/rustdoc/issue-32374.rs"
2020-03-12T18:40:53.4773142Z ------------------------------------------
2020-03-12T18:40:53.4773318Z 
2020-03-12T18:40:53.4773675Z ------------------------------------------
2020-03-12T18:40:53.4773893Z stderr:
---
2020-03-12T18:40:53.4776648Z ---- [rustdoc] rustdoc/issue-32395.rs stdout ----
2020-03-12T18:40:53.4776832Z 
2020-03-12T18:40:53.4776989Z error: rustdoc failed!
2020-03-12T18:40:53.4777205Z status: exit code: 1
2020-03-12T18:40:53.4778610Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-32395/auxiliary/variant-struct/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-32395" "/checkout/src/test/rustdoc/auxiliary/variant-struct.rs"
2020-03-12T18:40:53.4779850Z ------------------------------------------
2020-03-12T18:40:53.4780021Z 
2020-03-12T18:40:53.4780376Z ------------------------------------------
2020-03-12T18:40:53.4780592Z stderr:
---
2020-03-12T18:40:53.4783465Z ---- [rustdoc] rustdoc/issue-32556.rs stdout ----
2020-03-12T18:40:53.4783654Z 
2020-03-12T18:40:53.4783825Z error: rustdoc failed!
2020-03-12T18:40:53.4784026Z status: exit code: 1
2020-03-12T18:40:53.4785369Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-32556/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-32556" "/checkout/src/test/rustdoc/issue-32556.rs"
2020-03-12T18:40:53.4786545Z ------------------------------------------
2020-03-12T18:40:53.4786717Z 
2020-03-12T18:40:53.4787091Z ------------------------------------------
2020-03-12T18:40:53.4787291Z stderr:
---
2020-03-12T18:40:53.4790053Z ---- [rustdoc] rustdoc/issue-32890.rs stdout ----
2020-03-12T18:40:53.4790238Z 
2020-03-12T18:40:53.4790409Z error: rustdoc failed!
2020-03-12T18:40:53.4790610Z status: exit code: 1
2020-03-12T18:40:53.4791915Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-32890/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-32890" "/checkout/src/test/rustdoc/issue-32890.rs"
2020-03-12T18:40:53.4793091Z ------------------------------------------
2020-03-12T18:40:53.4793281Z 
2020-03-12T18:40:53.4793639Z ------------------------------------------
2020-03-12T18:40:53.4793839Z stderr:
---
2020-03-12T18:40:53.4796588Z ---- [rustdoc] rustdoc/issue-33069.rs stdout ----
2020-03-12T18:40:53.4796787Z 
2020-03-12T18:40:53.4796943Z error: rustdoc failed!
2020-03-12T18:40:53.4797142Z status: exit code: 1
2020-03-12T18:40:53.4798466Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33069/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33069" "/checkout/src/test/rustdoc/issue-33069.rs"
2020-03-12T18:40:53.4799642Z ------------------------------------------
2020-03-12T18:40:53.4799815Z 
2020-03-12T18:40:53.4800172Z ------------------------------------------
2020-03-12T18:40:53.4800374Z stderr:
---
2020-03-12T18:40:53.4803304Z ---- [rustdoc] rustdoc/issue-33178.rs stdout ----
2020-03-12T18:40:53.4803546Z 
2020-03-12T18:40:53.4803709Z error: rustdoc failed!
2020-03-12T18:40:53.4803906Z status: exit code: 1
2020-03-12T18:40:53.4805288Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33178/auxiliary/empty/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33178" "/checkout/src/test/rustdoc/auxiliary/empty.rs"
2020-03-12T18:40:53.4806488Z ------------------------------------------
2020-03-12T18:40:53.4806661Z 
2020-03-12T18:40:53.4807017Z ------------------------------------------
2020-03-12T18:40:53.4807236Z stderr:
---
2020-03-12T18:40:53.4809999Z ---- [rustdoc] rustdoc/issue-33302.rs stdout ----
2020-03-12T18:40:53.4810185Z 
2020-03-12T18:40:53.4810342Z error: rustdoc failed!
2020-03-12T18:40:53.4810560Z status: exit code: 1
2020-03-12T18:40:53.4811860Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33302/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33302" "/checkout/src/test/rustdoc/issue-33302.rs"
2020-03-12T18:40:53.4813030Z ------------------------------------------
2020-03-12T18:40:53.4813207Z 
2020-03-12T18:40:53.4813583Z ------------------------------------------
2020-03-12T18:40:53.4813784Z stderr:
---
2020-03-12T18:40:53.4816542Z ---- [rustdoc] rustdoc/issue-33592.rs stdout ----
2020-03-12T18:40:53.4816726Z 
2020-03-12T18:40:53.4816900Z error: rustdoc failed!
2020-03-12T18:40:53.4817100Z status: exit code: 1
2020-03-12T18:40:53.4818401Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33592/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33592" "/checkout/src/test/rustdoc/issue-33592.rs"
2020-03-12T18:40:53.4819576Z ------------------------------------------
2020-03-12T18:40:53.4819749Z 
2020-03-12T18:40:53.4820119Z ------------------------------------------
2020-03-12T18:40:53.4820318Z stderr:
---
2020-03-12T18:40:53.4823137Z ---- [rustdoc] rustdoc/issue-33178-1.rs stdout ----
2020-03-12T18:40:53.4823387Z 
2020-03-12T18:40:53.4823549Z error: rustdoc failed!
2020-03-12T18:40:53.4823749Z status: exit code: 1
2020-03-12T18:40:53.4825091Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33178-1/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33178-1" "/checkout/src/test/rustdoc/issue-33178-1.rs"
2020-03-12T18:40:53.4826257Z ------------------------------------------
2020-03-12T18:40:53.4826448Z 
2020-03-12T18:40:53.4826804Z ------------------------------------------
2020-03-12T18:40:53.4827006Z stderr:
---
2020-03-12T18:40:53.4829789Z ---- [rustdoc] rustdoc/issue-34025.rs stdout ----
2020-03-12T18:40:53.4829974Z 
2020-03-12T18:40:53.4830130Z error: rustdoc failed!
2020-03-12T18:40:53.4830328Z status: exit code: 1
2020-03-12T18:40:53.4831641Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34025/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34025" "/checkout/src/test/rustdoc/issue-34025.rs"
2020-03-12T18:40:53.4832813Z ------------------------------------------
2020-03-12T18:40:53.4832988Z 
2020-03-12T18:40:53.4833347Z ------------------------------------------
2020-03-12T18:40:53.4833549Z stderr:
---
2020-03-12T18:40:53.4836316Z ---- [rustdoc] rustdoc/issue-34423.rs stdout ----
2020-03-12T18:40:53.4836501Z 
2020-03-12T18:40:53.4836657Z error: rustdoc failed!
2020-03-12T18:40:53.4836870Z status: exit code: 1
2020-03-12T18:40:53.4838171Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34423/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34423" "/checkout/src/test/rustdoc/issue-34423.rs"
2020-03-12T18:40:53.4839345Z ------------------------------------------
2020-03-12T18:40:53.4839516Z 
2020-03-12T18:40:53.4839869Z ------------------------------------------
2020-03-12T18:40:53.4840084Z stderr:
---
2020-03-12T18:40:53.4842906Z ---- [rustdoc] rustdoc/issue-34274.rs stdout ----
2020-03-12T18:40:53.4843092Z 
2020-03-12T18:40:53.4843402Z error: rustdoc failed!
2020-03-12T18:40:53.4843623Z status: exit code: 1
2020-03-12T18:40:53.4845025Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34274/auxiliary/issue-34274/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34274" "/checkout/src/test/rustdoc/auxiliary/issue-34274.rs"
2020-03-12T18:40:53.4846256Z ------------------------------------------
2020-03-12T18:40:53.4846426Z 
2020-03-12T18:40:53.4846797Z ------------------------------------------
2020-03-12T18:40:53.4846997Z stderr:
---
2020-03-12T18:40:53.4849765Z ---- [rustdoc] rustdoc/issue-34473.rs stdout ----
2020-03-12T18:40:53.4849950Z 
2020-03-12T18:40:53.4850123Z error: rustdoc failed!
2020-03-12T18:40:53.4850324Z status: exit code: 1
2020-03-12T18:40:53.4851621Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34473/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34473" "/checkout/src/test/rustdoc/issue-34473.rs"
2020-03-12T18:40:53.4852834Z ------------------------------------------
2020-03-12T18:40:53.4853005Z 
2020-03-12T18:40:53.4853389Z ------------------------------------------
2020-03-12T18:40:53.4853591Z stderr:
---
2020-03-12T18:40:53.4856339Z ---- [rustdoc] rustdoc/issue-34928.rs stdout ----
2020-03-12T18:40:53.4856541Z 
2020-03-12T18:40:53.4856699Z error: rustdoc failed!
2020-03-12T18:40:53.4856899Z status: exit code: 1
2020-03-12T18:40:53.4858223Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34928/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34928" "/checkout/src/test/rustdoc/issue-34928.rs"
2020-03-12T18:40:53.4859382Z ------------------------------------------
2020-03-12T18:40:53.4859571Z 
2020-03-12T18:40:53.4859927Z ------------------------------------------
2020-03-12T18:40:53.4860128Z stderr:
---
2020-03-12T18:40:53.4862987Z ---- [rustdoc] rustdoc/issue-35169-2.rs stdout ----
2020-03-12T18:40:53.4863176Z 
2020-03-12T18:40:53.4863399Z error: rustdoc failed!
2020-03-12T18:40:53.4863603Z status: exit code: 1
2020-03-12T18:40:53.4864944Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-35169-2/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-35169-2" "/checkout/src/test/rustdoc/issue-35169-2.rs"
2020-03-12T18:40:53.4866122Z ------------------------------------------
2020-03-12T18:40:53.4866294Z 
2020-03-12T18:40:53.4866651Z ------------------------------------------
2020-03-12T18:40:53.4866853Z stderr:
---
2020-03-12T18:40:53.4869631Z ---- [rustdoc] rustdoc/issue-35169.rs stdout ----
2020-03-12T18:40:53.4869816Z 
2020-03-12T18:40:53.4869973Z error: rustdoc failed!
2020-03-12T18:40:53.4870186Z status: exit code: 1
2020-03-12T18:40:53.4871489Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-35169/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-35169" "/checkout/src/test/rustdoc/issue-35169.rs"
2020-03-12T18:40:53.4872668Z ------------------------------------------
2020-03-12T18:40:53.4872839Z 
2020-03-12T18:40:53.4873199Z ------------------------------------------
2020-03-12T18:40:53.4873417Z stderr:
---
2020-03-12T18:40:53.4876174Z ---- [rustdoc] rustdoc/issue-35488.rs stdout ----
2020-03-12T18:40:53.4876358Z 
2020-03-12T18:40:53.4876515Z error: rustdoc failed!
2020-03-12T18:40:53.4876732Z status: exit code: 1
2020-03-12T18:40:53.4878031Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-35488/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-35488" "/checkout/src/test/rustdoc/issue-35488.rs"
2020-03-12T18:40:53.4879203Z ------------------------------------------
2020-03-12T18:40:53.4879375Z 
2020-03-12T18:40:53.4879746Z ------------------------------------------
2020-03-12T18:40:53.4879944Z stderr:
---
2020-03-12T18:40:53.4882767Z ---- [rustdoc] rustdoc/issue-36031.rs stdout ----
2020-03-12T18:40:53.4882952Z 
2020-03-12T18:40:53.4883266Z error: rustdoc failed!
2020-03-12T18:40:53.4883473Z status: exit code: 1
2020-03-12T18:40:53.4884914Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-36031/auxiliary/issue-36031/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-36031" "/checkout/src/test/rustdoc/auxiliary/issue-36031.rs"
2020-03-12T18:40:53.4886143Z ------------------------------------------
2020-03-12T18:40:53.4886316Z 
2020-03-12T18:40:53.4886689Z ------------------------------------------
2020-03-12T18:40:53.4886889Z stderr:
---
2020-03-12T18:40:53.4889662Z ---- [rustdoc] rustdoc/issue-40936.rs stdout ----
2020-03-12T18:40:53.4889864Z 
2020-03-12T18:40:53.4890021Z error: rustdoc failed!
2020-03-12T18:40:53.4890221Z status: exit code: 1
2020-03-12T18:40:53.4891616Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-40936/auxiliary/issue-40936/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-40936" "/checkout/src/test/rustdoc/auxiliary/issue-40936.rs"
2020-03-12T18:40:53.4892846Z ------------------------------------------
2020-03-12T18:40:53.4893018Z 
2020-03-12T18:40:53.4893379Z ------------------------------------------
2020-03-12T18:40:53.4893580Z stderr:
---
2020-03-12T18:40:53.4896348Z ---- [rustdoc] rustdoc/issue-41783.rs stdout ----
2020-03-12T18:40:53.4896533Z 
2020-03-12T18:40:53.4896690Z error: rustdoc failed!
2020-03-12T18:40:53.4896888Z status: exit code: 1
2020-03-12T18:40:53.4898214Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-41783/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-41783" "/checkout/src/test/rustdoc/issue-41783.rs"
2020-03-12T18:40:53.4899389Z ------------------------------------------
2020-03-12T18:40:53.4899561Z 
2020-03-12T18:40:53.4899915Z ------------------------------------------
2020-03-12T18:40:53.4900131Z stderr:
---
2020-03-12T18:40:53.4902962Z ---- [rustdoc] rustdoc/issue-42760.rs stdout ----
2020-03-12T18:40:53.4903201Z 
2020-03-12T18:40:53.4903363Z error: rustdoc failed!
2020-03-12T18:40:53.4903577Z status: exit code: 1
2020-03-12T18:40:53.4905577Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-42760/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-42760" "/checkout/src/test/rustdoc/issue-42760.rs"
2020-03-12T18:40:53.4906770Z ------------------------------------------
2020-03-12T18:40:53.4906947Z 
2020-03-12T18:40:53.4907305Z ------------------------------------------
2020-03-12T18:40:53.4907523Z stderr:
---
2020-03-12T18:40:53.4910297Z ---- [rustdoc] rustdoc/issue-42875.rs stdout ----
2020-03-12T18:40:53.4910482Z 
2020-03-12T18:40:53.4910654Z error: rustdoc failed!
2020-03-12T18:40:53.4910855Z status: exit code: 1
2020-03-12T18:40:53.4912203Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-42875/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-42875" "/checkout/src/test/rustdoc/issue-42875.rs" "--no-defaults"
2020-03-12T18:40:53.4913416Z ------------------------------------------
2020-03-12T18:40:53.4913588Z 
2020-03-12T18:40:53.4913966Z ------------------------------------------
2020-03-12T18:40:53.4914167Z stderr:
2020-03-12T18:40:53.4914167Z stderr:
2020-03-12T18:40:53.4914533Z ------------------------------------------
2020-03-12T18:40:53.4915022Z warning: the 'no-defaults' flag is considered deprecated
2020-03-12T18:40:53.4915248Z   |
2020-03-12T18:40:53.4915846Z   = warning: see issue #44136 <***/issues/44136> for more information
2020-03-12T18:40:53.4916394Z   = help: you may want to use --document-private-items
2020-03-12T18:40:53.4917163Z thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_interface/passes.rs:741:23
2020-03-12T18:40:53.4917657Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-12T18:40:53.4917888Z 
2020-03-12T18:40:53.4918245Z ------------------------------------------
2020-03-12T18:40:53.4918245Z ------------------------------------------
2020-03-12T18:40:53.4918415Z 
2020-03-12T18:40:53.4918535Z 
2020-03-12T18:40:53.4918917Z ---- [rustdoc] rustdoc/issue-43701.rs stdout ----
2020-03-12T18:40:53.4919103Z 
2020-03-12T18:40:53.4919266Z error: rustdoc failed!
2020-03-12T18:40:53.4919484Z status: exit code: 1
2020-03-12T18:40:53.4920785Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43701/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43701" "/checkout/src/test/rustdoc/issue-43701.rs"
2020-03-12T18:40:53.4921957Z ------------------------------------------
2020-03-12T18:40:53.4922130Z 
2020-03-12T18:40:53.4922503Z ------------------------------------------
2020-03-12T18:40:53.4922705Z stderr:
---
2020-03-12T18:40:53.4925837Z ---- [rustdoc] rustdoc/issue-43869.rs stdout ----
2020-03-12T18:40:53.4926026Z 
2020-03-12T18:40:53.4926199Z error: rustdoc failed!
2020-03-12T18:40:53.4926401Z status: exit code: 1
2020-03-12T18:40:53.4927706Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43869/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43869" "/checkout/src/test/rustdoc/issue-43869.rs"
2020-03-12T18:40:53.4928878Z ------------------------------------------
2020-03-12T18:40:53.4929052Z 
2020-03-12T18:40:53.4929430Z ------------------------------------------
2020-03-12T18:40:53.4929631Z stderr:
2020-03-12T18:40:53.4929631Z stderr:
2020-03-12T18:40:53.4929996Z ------------------------------------------
2020-03-12T18:40:53.4930283Z warning: unnecessary parentheses around type
2020-03-12T18:40:53.4930762Z  --> /checkout/src/test/rustdoc/issue-43869.rs:5:15
2020-03-12T18:40:53.4930984Z   |
2020-03-12T18:40:53.4931372Z 5 | pub fn h() -> (impl Iterator<Item=u8>) {
2020-03-12T18:40:53.4931984Z   |
2020-03-12T18:40:53.4932213Z   = note: `#[warn(unused_parens)]` on by default
2020-03-12T18:40:53.4932399Z 
2020-03-12T18:40:53.4932974Z thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_interface/passes.rs:741:23
---
2020-03-12T18:40:53.4934742Z ---- [rustdoc] rustdoc/issue-43893.rs stdout ----
2020-03-12T18:40:53.4934926Z 
2020-03-12T18:40:53.4935082Z error: rustdoc failed!
2020-03-12T18:40:53.4935298Z status: exit code: 1
2020-03-12T18:40:53.4936594Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43893/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43893" "/checkout/src/test/rustdoc/issue-43893.rs"
2020-03-12T18:40:53.4937766Z ------------------------------------------
2020-03-12T18:40:53.4937938Z 
2020-03-12T18:40:53.4938294Z ------------------------------------------
2020-03-12T18:40:53.4938515Z stderr:
---
2020-03-12T18:40:53.4941274Z ---- [rustdoc] rustdoc/issue-45584.rs stdout ----
2020-03-12T18:40:53.4941460Z 
2020-03-12T18:40:53.4941633Z error: rustdoc failed!
2020-03-12T18:40:53.4941834Z status: exit code: 1
2020-03-12T18:40:53.4943138Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-45584/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-45584" "/checkout/src/test/rustdoc/issue-45584.rs"
2020-03-12T18:40:53.4944419Z ------------------------------------------
2020-03-12T18:40:53.4944595Z 
2020-03-12T18:40:53.4944979Z ------------------------------------------
2020-03-12T18:40:53.4945181Z stderr:
---
2020-03-12T18:40:53.4947931Z ---- [rustdoc] rustdoc/issue-46271.rs stdout ----
2020-03-12T18:40:53.4948116Z 
2020-03-12T18:40:53.4948288Z error: rustdoc failed!
2020-03-12T18:40:53.4948489Z status: exit code: 1
2020-03-12T18:40:53.4949805Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-46271/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-46271" "/checkout/src/test/rustdoc/issue-46271.rs"
2020-03-12T18:40:53.4950980Z ------------------------------------------
2020-03-12T18:40:53.4951166Z 
2020-03-12T18:40:53.4951523Z ------------------------------------------
2020-03-12T18:40:53.4951725Z stderr:
---
2020-03-12T18:40:53.4954477Z ---- [rustdoc] rustdoc/issue-46377.rs stdout ----
2020-03-12T18:40:53.4954678Z 
2020-03-12T18:40:53.4954834Z error: rustdoc failed!
2020-03-12T18:40:53.4955035Z status: exit code: 1
2020-03-12T18:40:53.4956347Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-46377/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-46377" "/checkout/src/test/rustdoc/issue-46377.rs"
2020-03-12T18:40:53.4957518Z ------------------------------------------
2020-03-12T18:40:53.4957690Z 
2020-03-12T18:40:53.4958046Z ------------------------------------------
2020-03-12T18:40:53.4958246Z stderr:
---
2020-03-12T18:40:53.4961189Z ---- [rustdoc] rustdoc/issue-46380-2.rs stdout ----
2020-03-12T18:40:53.4961378Z 
2020-03-12T18:40:53.4961534Z error: rustdoc failed!
2020-03-12T18:40:53.4961734Z status: exit code: 1
2020-03-12T18:40:53.4963068Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-46380-2/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-46380-2" "/checkout/src/test/rustdoc/issue-46380-2.rs"
2020-03-12T18:40:53.4964480Z ------------------------------------------
2020-03-12T18:40:53.4964655Z 
2020-03-12T18:40:53.4965023Z ------------------------------------------
2020-03-12T18:40:53.4965241Z stderr:
---
2020-03-12T18:40:53.4968003Z ---- [rustdoc] rustdoc/issue-46766.rs stdout ----
2020-03-12T18:40:53.4968187Z 
2020-03-12T18:40:53.4968344Z error: rustdoc failed!
2020-03-12T18:40:53.4968559Z status: exit code: 1
2020-03-12T18:40:53.4969868Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-46766/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-46766" "/checkout/src/test/rustdoc/issue-46766.rs"
2020-03-12T18:40:53.4971039Z ------------------------------------------
2020-03-12T18:40:53.4971212Z 
2020-03-12T18:40:53.4971569Z ------------------------------------------
2020-03-12T18:40:53.4971787Z stderr:
---
2020-03-12T18:40:53.4974558Z ---- [rustdoc] rustdoc/issue-46767.rs stdout ----
2020-03-12T18:40:53.4974743Z 
2020-03-12T18:40:53.4974918Z error: rustdoc failed!
2020-03-12T18:40:53.4975119Z status: exit code: 1
2020-03-12T18:40:53.4976414Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-46767/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-46767" "/checkout/src/test/rustdoc/issue-46767.rs"
2020-03-12T18:40:53.4977588Z ------------------------------------------
2020-03-12T18:40:53.4977761Z 
2020-03-12T18:40:53.4978134Z ------------------------------------------
2020-03-12T18:40:53.4978333Z stderr:
---
2020-03-12T18:40:53.4981093Z ---- [rustdoc] rustdoc/issue-46727.rs stdout ----
2020-03-12T18:40:53.4981278Z 
2020-03-12T18:40:53.4981451Z error: rustdoc failed!
2020-03-12T18:40:53.4981649Z status: exit code: 1
2020-03-12T18:40:53.4982949Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-46727/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-46727" "/checkout/src/test/rustdoc/issue-46727.rs"
2020-03-12T18:40:53.4984278Z ------------------------------------------
2020-03-12T18:40:53.4984467Z 
2020-03-12T18:40:53.4984838Z ------------------------------------------
2020-03-12T18:40:53.4985040Z stderr:
---
2020-03-12T18:40:53.4987788Z ---- [rustdoc] rustdoc/issue-46976.rs stdout ----
2020-03-12T18:40:53.4987987Z 
2020-03-12T18:40:53.4988144Z error: rustdoc failed!
2020-03-12T18:40:53.4988344Z status: exit code: 1
2020-03-12T18:40:53.4989669Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-46976/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-46976" "/checkout/src/test/rustdoc/issue-46976.rs"
2020-03-12T18:40:53.4990842Z ------------------------------------------
2020-03-12T18:40:53.4991016Z 
2020-03-12T18:40:53.4991371Z ------------------------------------------
2020-03-12T18:40:53.4991572Z stderr:
---
2020-03-12T18:40:53.4994349Z ---- [rustdoc] rustdoc/issue-47038.rs stdout ----
2020-03-12T18:40:53.4994535Z 
2020-03-12T18:40:53.4994691Z error: rustdoc failed!
2020-03-12T18:40:53.4994889Z status: exit code: 1
2020-03-12T18:40:53.4996202Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-47038/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-47038" "/checkout/src/test/rustdoc/issue-47038.rs"
2020-03-12T18:40:53.4997368Z ------------------------------------------
2020-03-12T18:40:53.4997541Z 
2020-03-12T18:40:53.4997897Z ------------------------------------------
2020-03-12T18:40:53.4998112Z stderr:
---
2020-03-12T18:40:53.5000944Z ---- [rustdoc] rustdoc/issue-47197-blank-line-in-doc-block.rs stdout ----
2020-03-12T18:40:53.5001169Z 
2020-03-12T18:40:53.5001325Z error: rustdoc failed!
2020-03-12T18:40:53.5001539Z status: exit code: 1
2020-03-12T18:40:53.5003033Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-47197-blank-line-in-doc-block/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-47197-blank-line-in-doc-block" "/checkout/src/test/rustdoc/issue-47197-blank-line-in-doc-block.rs"
2020-03-12T18:40:53.5004635Z ------------------------------------------
2020-03-12T18:40:53.5004809Z 
2020-03-12T18:40:53.5005182Z ------------------------------------------
2020-03-12T18:40:53.5005384Z stderr:
---
2020-03-12T18:40:53.5008141Z ---- [rustdoc] rustdoc/issue-47639.rs stdout ----
2020-03-12T18:40:53.5008326Z 
2020-03-12T18:40:53.5008500Z error: rustdoc failed!
2020-03-12T18:40:53.5008706Z status: exit code: 1
2020-03-12T18:40:53.5010014Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-47639/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-47639" "/checkout/src/test/rustdoc/issue-47639.rs"
2020-03-12T18:40:53.5011190Z ------------------------------------------
2020-03-12T18:40:53.5011363Z 
2020-03-12T18:40:53.5011736Z ------------------------------------------
2020-03-12T18:40:53.5011939Z stderr:
---
2020-03-12T18:40:53.5014707Z ---- [rustdoc] rustdoc/issue-48414.rs stdout ----
2020-03-12T18:40:53.5014908Z 
2020-03-12T18:40:53.5015067Z error: rustdoc failed!
2020-03-12T18:40:53.5015268Z status: exit code: 1
2020-03-12T18:40:53.5016589Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-48414/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-48414" "/checkout/src/test/rustdoc/issue-48414.rs"
2020-03-12T18:40:53.5017745Z ------------------------------------------
2020-03-12T18:40:53.5017937Z 
2020-03-12T18:40:53.5018295Z ------------------------------------------
2020-03-12T18:40:53.5018496Z stderr:
---
2020-03-12T18:40:53.5021276Z ---- [rustdoc] rustdoc/issue-50159.rs stdout ----
2020-03-12T18:40:53.5021460Z 
2020-03-12T18:40:53.5021618Z error: rustdoc failed!
2020-03-12T18:40:53.5021818Z status: exit code: 1
2020-03-12T18:40:53.5023128Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-50159/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-50159" "/checkout/src/test/rustdoc/issue-50159.rs"
2020-03-12T18:40:53.5024446Z ------------------------------------------
2020-03-12T18:40:53.5024621Z 
2020-03-12T18:40:53.5024990Z ------------------------------------------
2020-03-12T18:40:53.5025207Z stderr:
---
2020-03-12T18:40:53.5027956Z ---- [rustdoc] rustdoc/issue-51236.rs stdout ----
2020-03-12T18:40:53.5028141Z 
2020-03-12T18:40:53.5028297Z error: rustdoc failed!
2020-03-12T18:40:53.5028516Z status: exit code: 1
2020-03-12T18:40:53.5029829Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-51236/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-51236" "/checkout/src/test/rustdoc/issue-51236.rs"
2020-03-12T18:40:53.5031002Z ------------------------------------------
2020-03-12T18:40:53.5031175Z 
2020-03-12T18:40:53.5031530Z ------------------------------------------
2020-03-12T18:40:53.5031748Z stderr:
---
2020-03-12T18:40:53.5034511Z ---- [rustdoc] rustdoc/issue-52873.rs stdout ----
2020-03-12T18:40:53.5034697Z 
2020-03-12T18:40:53.5034854Z error: rustdoc failed!
2020-03-12T18:40:53.5035069Z status: exit code: 1
2020-03-12T18:40:53.5036366Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-52873/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-52873" "/checkout/src/test/rustdoc/issue-52873.rs"
2020-03-12T18:40:53.5037533Z ------------------------------------------
2020-03-12T18:40:53.5037706Z 
2020-03-12T18:40:53.5038075Z ------------------------------------------
2020-03-12T18:40:53.5038277Z stderr:
---
2020-03-12T18:40:53.5041040Z ---- [rustdoc] rustdoc/issue-53812.rs stdout ----
2020-03-12T18:40:53.5041225Z 
2020-03-12T18:40:53.5041396Z error: rustdoc failed!
2020-03-12T18:40:53.5041595Z status: exit code: 1
2020-03-12T18:40:53.5043411Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-53812/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-53812" "/checkout/src/test/rustdoc/issue-53812.rs"
2020-03-12T18:40:53.5044849Z ------------------------------------------
2020-03-12T18:40:53.5045026Z 
2020-03-12T18:40:53.5045416Z ------------------------------------------
2020-03-12T18:40:53.5045618Z stderr:
---
2020-03-12T18:40:53.5048382Z ---- [rustdoc] rustdoc/issue-53689.rs stdout ----
2020-03-12T18:40:53.5048584Z 
2020-03-12T18:40:53.5048741Z error: rustdoc failed!
2020-03-12T18:40:53.5048941Z status: exit code: 1
2020-03-12T18:40:53.5050267Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-53689/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-53689" "/checkout/src/test/rustdoc/issue-53689.rs"
2020-03-12T18:40:53.5051428Z ------------------------------------------
2020-03-12T18:40:53.5051617Z 
2020-03-12T18:40:53.5051973Z ------------------------------------------
2020-03-12T18:40:53.5052175Z stderr:
---
2020-03-12T18:40:53.5054959Z ---- [rustdoc] rustdoc/issue-54705.rs stdout ----
2020-03-12T18:40:53.5055143Z 
2020-03-12T18:40:53.5055300Z error: rustdoc failed!
2020-03-12T18:40:53.5055501Z status: exit code: 1
2020-03-12T18:40:53.5056817Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-54705/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-54705" "/checkout/src/test/rustdoc/issue-54705.rs"
2020-03-12T18:40:53.5057986Z ------------------------------------------
2020-03-12T18:40:53.5058158Z 
2020-03-12T18:40:53.5058514Z ------------------------------------------
2020-03-12T18:40:53.5058730Z stderr:
---
2020-03-12T18:40:53.5062038Z ---- [rustdoc] rustdoc/issue-55001.rs stdout ----
2020-03-12T18:40:53.5062223Z 
2020-03-12T18:40:53.5062380Z error: rustdoc failed!
2020-03-12T18:40:53.5062596Z status: exit code: 1
2020-03-12T18:40:53.5063904Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-55001/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-55001" "/checkout/src/test/rustdoc/issue-55001.rs"
2020-03-12T18:40:53.5065151Z ------------------------------------------
2020-03-12T18:40:53.5065372Z 
2020-03-12T18:40:53.5065742Z ------------------------------------------
2020-03-12T18:40:53.5065959Z stderr:
---
2020-03-12T18:40:53.5068714Z ---- [rustdoc] rustdoc/issue-55321.rs stdout ----
2020-03-12T18:40:53.5068899Z 
2020-03-12T18:40:53.5069069Z error: rustdoc failed!
2020-03-12T18:40:53.5069271Z status: exit code: 1
2020-03-12T18:40:53.5070573Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-55321/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-55321" "/checkout/src/test/rustdoc/issue-55321.rs"
2020-03-12T18:40:53.5071750Z ------------------------------------------
2020-03-12T18:40:53.5071923Z 
2020-03-12T18:40:53.5072295Z ------------------------------------------
2020-03-12T18:40:53.5072495Z stderr:
---
2020-03-12T18:40:53.5076714Z ---- [rustdoc] rustdoc/issue-55364.rs stdout ----
2020-03-12T18:40:53.5076905Z 
2020-03-12T18:40:53.5077078Z error: rustdoc failed!
2020-03-12T18:40:53.5077279Z status: exit code: 1
2020-03-12T18:40:53.5078580Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-55364/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-55364" "/checkout/src/test/rustdoc/issue-55364.rs"
2020-03-12T18:40:53.5079755Z ------------------------------------------
2020-03-12T18:40:53.5079926Z 
2020-03-12T18:40:53.5080303Z ------------------------------------------
2020-03-12T18:40:53.5080503Z stderr:
---
2020-03-12T18:40:53.5083425Z ---- [rustdoc] rustdoc/issue-56701.rs stdout ----
2020-03-12T18:40:53.5083630Z 
2020-03-12T18:40:53.5083790Z error: rustdoc failed!
2020-03-12T18:40:53.5083990Z status: exit code: 1
2020-03-12T18:40:53.5085313Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-56701/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-56701" "/checkout/src/test/rustdoc/issue-56701.rs"
2020-03-12T18:40:53.5086591Z ------------------------------------------
2020-03-12T18:40:53.5086779Z 
2020-03-12T18:40:53.5087199Z ------------------------------------------
2020-03-12T18:40:53.5087404Z stderr:
---
2020-03-12T18:40:53.5090194Z ---- [rustdoc] rustdoc/issue-56822.rs stdout ----
2020-03-12T18:40:53.5090380Z 
2020-03-12T18:40:53.5090538Z error: rustdoc failed!
2020-03-12T18:40:53.5090737Z status: exit code: 1
2020-03-12T18:40:53.5092061Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-56822/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-56822" "/checkout/src/test/rustdoc/issue-56822.rs"
2020-03-12T18:40:53.5093239Z ------------------------------------------
2020-03-12T18:40:53.5093413Z 
2020-03-12T18:40:53.5093769Z ------------------------------------------
2020-03-12T18:40:53.5093983Z stderr:
---
2020-03-12T18:40:53.5096747Z ---- [rustdoc] rustdoc/issue-60482.rs stdout ----
2020-03-12T18:40:53.5096932Z 
2020-03-12T18:40:53.5097092Z error: rustdoc failed!
2020-03-12T18:40:53.5097310Z status: exit code: 1
2020-03-12T18:40:53.5098617Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-60482/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-60482" "/checkout/src/test/rustdoc/issue-60482.rs"
2020-03-12T18:40:53.5099790Z ------------------------------------------
2020-03-12T18:40:53.5099963Z 
2020-03-12T18:40:53.5100318Z ------------------------------------------
2020-03-12T18:40:53.5100536Z stderr:
---
2020-03-12T18:40:53.5103306Z ---- [rustdoc] rustdoc/issue-57180.rs stdout ----
2020-03-12T18:40:53.5103492Z 
2020-03-12T18:40:53.5103666Z error: rustdoc failed!
2020-03-12T18:40:53.5103868Z status: exit code: 1
2020-03-12T18:40:53.5105175Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-57180/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-57180" "/checkout/src/test/rustdoc/issue-57180.rs"
2020-03-12T18:40:53.5106412Z ------------------------------------------
2020-03-12T18:40:53.5106584Z 
2020-03-12T18:40:53.5107015Z ------------------------------------------
2020-03-12T18:40:53.5107222Z stderr:
---
2020-03-12T18:40:53.5109986Z ---- [rustdoc] rustdoc/issue-60726.rs stdout ----
2020-03-12T18:40:53.5110171Z 
2020-03-12T18:40:53.5110343Z error: rustdoc failed!
2020-03-12T18:40:53.5110543Z status: exit code: 1
2020-03-12T18:40:53.5111853Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-60726/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-60726" "/checkout/src/test/rustdoc/issue-60726.rs"
2020-03-12T18:40:53.5113032Z ------------------------------------------
2020-03-12T18:40:53.5113218Z 
2020-03-12T18:40:53.5113577Z ------------------------------------------
2020-03-12T18:40:53.5113777Z stderr:
---
2020-03-12T18:40:53.5116543Z ---- [rustdoc] rustdoc/issue-67851-both.rs stdout ----
2020-03-12T18:40:53.5116751Z 
2020-03-12T18:40:53.5116912Z error: rustdoc failed!
2020-03-12T18:40:53.5117113Z status: exit code: 1
2020-03-12T18:40:53.5118667Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-67851-both/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-67851-both" "/checkout/src/test/rustdoc/issue-67851-both.rs" "-Zunstable-options" "--document-private-items" "--document-hidden-items"
2020-03-12T18:40:53.5119993Z ------------------------------------------
2020-03-12T18:40:53.5120166Z 
2020-03-12T18:40:53.5120523Z ------------------------------------------
2020-03-12T18:40:53.5120725Z stderr:
---
2020-03-12T18:40:53.5123617Z ---- [rustdoc] rustdoc/issue-67851-hidden.rs stdout ----
2020-03-12T18:40:53.5123816Z 
2020-03-12T18:40:53.5123972Z error: rustdoc failed!
2020-03-12T18:40:53.5124186Z status: exit code: 1
2020-03-12T18:40:53.5125672Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-67851-hidden/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-67851-hidden" "/checkout/src/test/rustdoc/issue-67851-hidden.rs" "-Zunstable-options" "--document-hidden-items"
2020-03-12T18:40:53.5127093Z ------------------------------------------
2020-03-12T18:40:53.5127269Z 
2020-03-12T18:40:53.5127636Z ------------------------------------------
2020-03-12T18:40:53.5127853Z stderr:
---
2020-03-12T18:40:53.5130635Z ---- [rustdoc] rustdoc/issue-67851-neither.rs stdout ----
2020-03-12T18:40:53.5130836Z 
2020-03-12T18:40:53.5130992Z error: rustdoc failed!
2020-03-12T18:40:53.5131214Z status: exit code: 1
2020-03-12T18:40:53.5132587Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-67851-neither/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-67851-neither" "/checkout/src/test/rustdoc/issue-67851-neither.rs"
2020-03-12T18:40:53.5133807Z ------------------------------------------
2020-03-12T18:40:53.5133979Z 
2020-03-12T18:40:53.5134350Z ------------------------------------------
2020-03-12T18:40:53.5134552Z stderr:
---
2020-03-12T18:40:53.5137323Z ---- [rustdoc] rustdoc/issue-66159.rs stdout ----
2020-03-12T18:40:53.5137509Z 
2020-03-12T18:40:53.5137683Z error: rustdoc failed!
2020-03-12T18:40:53.5137885Z status: exit code: 1
2020-03-12T18:40:53.5139625Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-66159/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-66159" "/checkout/src/test/rustdoc/issue-66159.rs" "-Z" "unstable-options" "--extern" "priv:issue_66159_1=/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-66159/auxiliary/libissue_66159_1.so"
2020-03-12T18:40:53.5141098Z ------------------------------------------
2020-03-12T18:40:53.5141291Z 
2020-03-12T18:40:53.5141649Z ------------------------------------------
2020-03-12T18:40:53.5141856Z stderr:
---
2020-03-12T18:40:53.5144646Z ---- [rustdoc] rustdoc/issue-67851-private.rs stdout ----
2020-03-12T18:40:53.5144845Z 
2020-03-12T18:40:53.5145002Z error: rustdoc failed!
2020-03-12T18:40:53.5145202Z status: exit code: 1
2020-03-12T18:40:53.5146721Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-67851-private/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-67851-private" "/checkout/src/test/rustdoc/issue-67851-private.rs" "--document-private-items"
2020-03-12T18:40:53.5148057Z ------------------------------------------
2020-03-12T18:40:53.5148230Z 
2020-03-12T18:40:53.5148586Z ------------------------------------------
2020-03-12T18:40:53.5148802Z stderr:
---
2020-03-12T18:40:53.5151555Z ---- [rustdoc] rustdoc/keyword.rs stdout ----
2020-03-12T18:40:53.5151733Z 
2020-03-12T18:40:53.5151896Z error: rustdoc failed!
2020-03-12T18:40:53.5152112Z status: exit code: 1
2020-03-12T18:40:53.5153378Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/keyword/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/keyword" "/checkout/src/test/rustdoc/keyword.rs"
2020-03-12T18:40:53.5154534Z ------------------------------------------
2020-03-12T18:40:53.5154706Z 
2020-03-12T18:40:53.5155059Z ------------------------------------------
2020-03-12T18:40:53.5155277Z stderr:
---
2020-03-12T18:40:53.5164743Z ---- [rustdoc] rustdoc/line-breaks.rs stdout ----
2020-03-12T18:40:53.5164928Z 
2020-03-12T18:40:53.5165100Z error: rustdoc failed!
2020-03-12T18:40:53.5165300Z status: exit code: 1
2020-03-12T18:40:53.5166661Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/line-breaks/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/line-breaks" "/checkout/src/test/rustdoc/line-breaks.rs"
2020-03-12T18:40:53.5167882Z ------------------------------------------
2020-03-12T18:40:53.5168069Z 
2020-03-12T18:40:53.5168426Z ------------------------------------------
2020-03-12T18:40:53.5168627Z stderr:
---
2020-03-12T18:40:53.5178038Z ---- [rustdoc] rustdoc/link-title-escape.rs stdout ----
2020-03-12T18:40:53.5178234Z 
2020-03-12T18:40:53.5178392Z error: rustdoc failed!
2020-03-12T18:40:53.5178593Z status: exit code: 1
2020-03-12T18:40:53.5179957Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/link-title-escape/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/link-title-escape" "/checkout/src/test/rustdoc/link-title-escape.rs"
2020-03-12T18:40:53.5181169Z ------------------------------------------
2020-03-12T18:40:53.5181341Z 
2020-03-12T18:40:53.5181733Z ------------------------------------------
2020-03-12T18:40:53.5181951Z stderr:
---
2020-03-12T18:40:53.5191566Z ---- [rustdoc] rustdoc/manual_impl.rs stdout ----
2020-03-12T18:40:53.5191750Z 
2020-03-12T18:40:53.5191929Z error: rustdoc failed!
2020-03-12T18:40:53.5192132Z status: exit code: 1
2020-03-12T18:40:53.5193427Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/manual_impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/manual_impl" "/checkout/src/test/rustdoc/manual_impl.rs"
2020-03-12T18:40:53.5194593Z ------------------------------------------
2020-03-12T18:40:53.5194763Z 
2020-03-12T18:40:53.5195126Z ------------------------------------------
2020-03-12T18:40:53.5230098Z stderr:
---
2020-03-12T18:40:53.5240334Z ---- [rustdoc] rustdoc/masked.rs stdout ----
2020-03-12T18:40:53.5240511Z 
2020-03-12T18:40:53.5240686Z error: rustdoc failed!
2020-03-12T18:40:53.5240887Z status: exit code: 1
2020-03-12T18:40:53.5242340Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/masked/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/masked" "/checkout/src/test/rustdoc/masked.rs"
2020-03-12T18:40:53.5243699Z ------------------------------------------
2020-03-12T18:40:53.5243889Z 
2020-03-12T18:40:53.5244250Z ------------------------------------------
2020-03-12T18:40:53.5244452Z stderr:
---
2020-03-12T18:40:53.5247240Z ---- [rustdoc] rustdoc/mod-stackoverflow.rs stdout ----
2020-03-12T18:40:53.5247460Z 
2020-03-12T18:40:53.5247618Z error: rustdoc failed!
2020-03-12T18:40:53.5247824Z status: exit code: 1
2020-03-12T18:40:53.5249193Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/mod-stackoverflow/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/mod-stackoverflow" "/checkout/src/test/rustdoc/mod-stackoverflow.rs"
2020-03-12T18:40:53.5250400Z ------------------------------------------
2020-03-12T18:40:53.5250574Z 
2020-03-12T18:40:53.5250929Z ------------------------------------------
2020-03-12T18:40:53.5251131Z stderr:
---
2020-03-12T18:40:53.5253922Z ---- [rustdoc] rustdoc/module-impls.rs stdout ----
2020-03-12T18:40:53.5254109Z 
2020-03-12T18:40:53.5254266Z error: rustdoc failed!
2020-03-12T18:40:53.5254464Z status: exit code: 1
2020-03-12T18:40:53.5255792Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/module-impls/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/module-impls" "/checkout/src/test/rustdoc/module-impls.rs"
2020-03-12T18:40:53.5256968Z ------------------------------------------
2020-03-12T18:40:53.5257146Z 
2020-03-12T18:40:53.5257503Z ------------------------------------------
2020-03-12T18:40:53.5257719Z stderr:
---
2020-03-12T18:40:53.5260473Z ---- [rustdoc] rustdoc/must-use.rs stdout ----
2020-03-12T18:40:53.5260651Z 
2020-03-12T18:40:53.5260807Z error: rustdoc failed!
2020-03-12T18:40:53.5261023Z status: exit code: 1
2020-03-12T18:40:53.5262364Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/must-use/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/must-use" "/checkout/src/test/rustdoc/must-use.rs"
2020-03-12T18:40:53.5263574Z ------------------------------------------
2020-03-12T18:40:53.5263745Z 
2020-03-12T18:40:53.5264101Z ------------------------------------------
2020-03-12T18:40:53.5264319Z stderr:
---
2020-03-12T18:40:53.5273631Z ---- [rustdoc] rustdoc/negative-impl-sidebar.rs stdout ----
2020-03-12T18:40:53.5273833Z 
2020-03-12T18:40:53.5274005Z error: rustdoc failed!
2020-03-12T18:40:53.5274204Z status: exit code: 1
2020-03-12T18:40:53.5275578Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/negative-impl-sidebar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/negative-impl-sidebar" "/checkout/src/test/rustdoc/negative-impl-sidebar.rs"
2020-03-12T18:40:53.5276797Z ------------------------------------------
2020-03-12T18:40:53.5276990Z 
2020-03-12T18:40:53.5277350Z ------------------------------------------
2020-03-12T18:40:53.5277547Z stderr:
---
2020-03-12T18:40:53.5280333Z ---- [rustdoc] rustdoc/negative-impl.rs stdout ----
2020-03-12T18:40:53.5280524Z 
2020-03-12T18:40:53.5280680Z error: rustdoc failed!
2020-03-12T18:40:53.5280919Z status: exit code: 1
2020-03-12T18:40:53.5282313Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/negative-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/negative-impl" "/checkout/src/test/rustdoc/negative-impl.rs"
2020-03-12T18:40:53.5284245Z ------------------------------------------
2020-03-12T18:40:53.5284424Z 
2020-03-12T18:40:53.5284810Z ------------------------------------------
2020-03-12T18:40:53.5285010Z stderr:
---
2020-03-12T18:40:53.5287802Z ---- [rustdoc] rustdoc/no-crate-filter.rs stdout ----
2020-03-12T18:40:53.5288002Z 
2020-03-12T18:40:53.5288158Z error: rustdoc failed!
2020-03-12T18:40:53.5288378Z status: exit code: 1
2020-03-12T18:40:53.5289846Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/no-crate-filter/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/no-crate-filter" "/checkout/src/test/rustdoc/no-crate-filter.rs" "-Z" "unstable-options" "--disable-per-crate-search"
2020-03-12T18:40:53.5291131Z ------------------------------------------
2020-03-12T18:40:53.5291301Z 
2020-03-12T18:40:53.5291660Z ------------------------------------------
2020-03-12T18:40:53.5291875Z stderr:
---
2020-03-12T18:40:53.5301382Z ---- [rustdoc] rustdoc/no-stack-overflow-25295.rs stdout ----
2020-03-12T18:40:53.5301587Z 
2020-03-12T18:40:53.5301761Z error: rustdoc failed!
2020-03-12T18:40:53.5301960Z status: exit code: 1
2020-03-12T18:40:53.5303439Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/no-stack-overflow-25295/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/no-stack-overflow-25295" "/checkout/src/test/rustdoc/no-stack-overflow-25295.rs"
2020-03-12T18:40:53.5304726Z ------------------------------------------
2020-03-12T18:40:53.5304913Z 
2020-03-12T18:40:53.5305270Z ------------------------------------------
2020-03-12T18:40:53.5305471Z stderr:
---
2020-03-12T18:40:53.5308238Z ---- [rustdoc] rustdoc/playground-arg.rs stdout ----
2020-03-12T18:40:53.5308446Z 
2020-03-12T18:40:53.5308608Z error: rustdoc failed!
2020-03-12T18:40:53.5308807Z status: exit code: 1
2020-03-12T18:40:53.5310348Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-arg/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-arg" "/checkout/src/test/rustdoc/playground-arg.rs" "--playground-url=https://example.com/" "-Z" "unstable-options"
2020-03-12T18:40:53.5311670Z ------------------------------------------
2020-03-12T18:40:53.5311844Z 
2020-03-12T18:40:53.5312199Z ------------------------------------------
2020-03-12T18:40:53.5312397Z stderr:
---
2020-03-12T18:40:53.5315193Z ---- [rustdoc] rustdoc/playground-empty.rs stdout ----
2020-03-12T18:40:53.5315386Z 
2020-03-12T18:40:53.5315543Z error: rustdoc failed!
2020-03-12T18:40:53.5315758Z status: exit code: 1
2020-03-12T18:40:53.5317321Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-empty/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-empty" "/checkout/src/test/rustdoc/playground-empty.rs" "-Z" "unstable-options" "--playground-url" "https://play.rust-lang.org/"
2020-03-12T18:40:53.5318680Z ------------------------------------------
2020-03-12T18:40:53.5318853Z 
2020-03-12T18:40:53.5319209Z ------------------------------------------
2020-03-12T18:40:53.5319427Z stderr:
---
2020-03-12T18:40:53.5322187Z ---- [rustdoc] rustdoc/playground-none.rs stdout ----
2020-03-12T18:40:53.5322376Z 
2020-03-12T18:40:53.5322548Z error: rustdoc failed!
2020-03-12T18:40:53.5322800Z status: exit code: 1
2020-03-12T18:40:53.5324334Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-none/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-none" "/checkout/src/test/rustdoc/playground-none.rs"
2020-03-12T18:40:53.5325543Z ------------------------------------------
2020-03-12T18:40:53.5325715Z 
2020-03-12T18:40:53.5326086Z ------------------------------------------
2020-03-12T18:40:53.5326288Z stderr:
---
2020-03-12T18:40:53.5342339Z ---- [rustdoc] rustdoc/prim-title.rs stdout ----
2020-03-12T18:40:53.5342521Z 
2020-03-12T18:40:53.5342677Z error: rustdoc failed!
2020-03-12T18:40:53.5342940Z status: exit code: 1
2020-03-12T18:40:53.5344309Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/prim-title/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/prim-title" "/checkout/src/test/rustdoc/prim-title.rs"
2020-03-12T18:40:53.5345479Z ------------------------------------------
2020-03-12T18:40:53.5345650Z 
2020-03-12T18:40:53.5346005Z ------------------------------------------
2020-03-12T18:40:53.5346223Z stderr:
---
2020-03-12T18:40:53.5349022Z ---- [rustdoc] rustdoc/primitive-generic-impl.rs stdout ----
2020-03-12T18:40:53.5349223Z 
2020-03-12T18:40:53.5349379Z error: rustdoc failed!
2020-03-12T18:40:53.5349595Z status: exit code: 1
2020-03-12T18:40:53.5350975Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive-generic-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive-generic-impl" "/checkout/src/test/rustdoc/primitive-generic-impl.rs"
2020-03-12T18:40:53.5352201Z ------------------------------------------
2020-03-12T18:40:53.5352374Z 
2020-03-12T18:40:53.5352744Z ------------------------------------------
2020-03-12T18:40:53.5352951Z stderr:
---
2020-03-12T18:40:53.5355724Z ---- [rustdoc] rustdoc/primitive-link.rs stdout ----
2020-03-12T18:40:53.5355915Z 
2020-03-12T18:40:53.5356087Z error: rustdoc failed!
2020-03-12T18:40:53.5356287Z status: exit code: 1
2020-03-12T18:40:53.5357619Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive-link/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive-link" "/checkout/src/test/rustdoc/primitive-link.rs"
2020-03-12T18:40:53.5358812Z ------------------------------------------
2020-03-12T18:40:53.5358984Z 
2020-03-12T18:40:53.5359356Z ------------------------------------------
2020-03-12T18:40:53.5359554Z stderr:
---
2020-03-12T18:40:53.5362351Z ---- [rustdoc] rustdoc/primitive/primitive-generic-impl.rs stdout ----
2020-03-12T18:40:53.5362586Z 
2020-03-12T18:40:53.5362801Z error: rustdoc failed!
2020-03-12T18:40:53.5363001Z status: exit code: 1
2020-03-12T18:40:53.5364669Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive/primitive-generic-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive/primitive-generic-impl" "/checkout/src/test/rustdoc/primitive/primitive-generic-impl.rs"
2020-03-12T18:40:53.5366237Z ------------------------------------------
2020-03-12T18:40:53.5366433Z 
2020-03-12T18:40:53.5366794Z ------------------------------------------
2020-03-12T18:40:53.5366993Z stderr:
---
2020-03-12T18:40:53.5369808Z ---- [rustdoc] rustdoc/primitive-reexport.rs stdout ----
2020-03-12T18:40:53.5370005Z 
2020-03-12T18:40:53.5370162Z error: rustdoc failed!
2020-03-12T18:40:53.5370363Z status: exit code: 1
2020-03-12T18:40:53.5371836Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive-reexport/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive-reexport" "/checkout/src/test/rustdoc/primitive-reexport.rs" "--extern" "foo" "--edition" "2018"
2020-03-12T18:40:53.5373103Z ------------------------------------------
2020-03-12T18:40:53.5373276Z 
2020-03-12T18:40:53.5373641Z ------------------------------------------
2020-03-12T18:40:53.5374205Z stderr:
---
2020-03-12T18:40:53.5377404Z ---- [rustdoc] rustdoc/private-type-alias.rs stdout ----
2020-03-12T18:40:53.5377601Z 
2020-03-12T18:40:53.5377757Z error: rustdoc failed!
2020-03-12T18:40:53.5377975Z status: exit code: 1
2020-03-12T18:40:53.5379338Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/private-type-alias/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/private-type-alias" "/checkout/src/test/rustdoc/private-type-alias.rs"
2020-03-12T18:40:53.5380598Z ------------------------------------------
2020-03-12T18:40:53.5380771Z 
2020-03-12T18:40:53.5381124Z ------------------------------------------
2020-03-12T18:40:53.5381345Z stderr:
---
2020-03-12T18:40:53.5384223Z ---- [rustdoc] rustdoc/proc-macro.rs stdout ----
2020-03-12T18:40:53.5384406Z 
2020-03-12T18:40:53.5384630Z error: rustdoc failed!
2020-03-12T18:40:53.5384836Z status: exit code: 1
2020-03-12T18:40:53.5386290Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/proc-macro/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/proc-macro" "/checkout/src/test/rustdoc/proc-macro.rs" "--crate-type" "proc-macro" "--document-private-items"
2020-03-12T18:40:53.5387555Z ------------------------------------------
2020-03-12T18:40:53.5387727Z 
2020-03-12T18:40:53.5388102Z ------------------------------------------
2020-03-12T18:40:53.5388302Z stderr:
---
2020-03-12T18:40:53.5391082Z ---- [rustdoc] rustdoc/pub-extern-crate.rs stdout ----
2020-03-12T18:40:53.5391291Z 
2020-03-12T18:40:53.5391449Z error: rustdoc failed!
2020-03-12T18:40:53.5391650Z status: exit code: 1
2020-03-12T18:40:53.5393009Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/pub-extern-crate/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/pub-extern-crate" "/checkout/src/test/rustdoc/pub-extern-crate.rs"
2020-03-12T18:40:53.5394199Z ------------------------------------------
2020-03-12T18:40:53.5394394Z 
2020-03-12T18:40:53.5394753Z ------------------------------------------
2020-03-12T18:40:53.5394954Z stderr:
---
2020-03-12T18:40:53.5397724Z ---- [rustdoc] rustdoc/pub-method.rs stdout ----
2020-03-12T18:40:53.5397908Z 
2020-03-12T18:40:53.5398063Z error: rustdoc failed!
2020-03-12T18:40:53.5398264Z status: exit code: 1
2020-03-12T18:40:53.5399649Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/pub-method/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/pub-method" "/checkout/src/test/rustdoc/pub-method.rs" "--document-private-items"
2020-03-12T18:40:53.5400869Z ------------------------------------------
2020-03-12T18:40:53.5401042Z 
2020-03-12T18:40:53.5401395Z ------------------------------------------
2020-03-12T18:40:53.5401611Z stderr:
---
2020-03-12T18:40:53.5404635Z ---- [rustdoc] rustdoc/pub-restricted.rs stdout ----
2020-03-12T18:40:53.5404829Z 
2020-03-12T18:40:53.5404987Z error: rustdoc failed!
2020-03-12T18:40:53.5405208Z status: exit code: 1
2020-03-12T18:40:53.5406612Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/pub-restricted/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/pub-restricted" "/checkout/src/test/rustdoc/pub-restricted.rs" "--document-private-items"
2020-03-12T18:40:53.5407840Z ------------------------------------------
2020-03-12T18:40:53.5408012Z 
2020-03-12T18:40:53.5408365Z ------------------------------------------
2020-03-12T18:40:53.5408582Z stderr:
---
2020-03-12T18:40:53.5411350Z ---- [rustdoc] rustdoc/recursion1.rs stdout ----
2020-03-12T18:40:53.5411533Z 
2020-03-12T18:40:53.5411705Z error: rustdoc failed!
2020-03-12T18:40:53.5411907Z status: exit code: 1
2020-03-12T18:40:53.5413200Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/recursion1/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/recursion1" "/checkout/src/test/rustdoc/recursion1.rs"
2020-03-12T18:40:53.5414384Z ------------------------------------------
2020-03-12T18:40:53.5414555Z 
2020-03-12T18:40:53.5414927Z ------------------------------------------
2020-03-12T18:40:53.5415128Z stderr:
---
2020-03-12T18:40:53.5417879Z ---- [rustdoc] rustdoc/recursion2.rs stdout ----
2020-03-12T18:40:53.5418061Z 
2020-03-12T18:40:53.5418232Z error: rustdoc failed!
2020-03-12T18:40:53.5418434Z status: exit code: 1
2020-03-12T18:40:53.5419736Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/recursion2/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/recursion2" "/checkout/src/test/rustdoc/recursion2.rs"
2020-03-12T18:40:53.5420908Z ------------------------------------------
2020-03-12T18:40:53.5421080Z 
2020-03-12T18:40:53.5421456Z ------------------------------------------
2020-03-12T18:40:53.5421656Z stderr:
---
2020-03-12T18:40:53.5431454Z ---- [rustdoc] rustdoc/recursion3.rs stdout ----
2020-03-12T18:40:53.5431635Z 
2020-03-12T18:40:53.5431793Z error: rustdoc failed!
2020-03-12T18:40:53.5431990Z status: exit code: 1
2020-03-12T18:40:53.5433299Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/recursion3/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/recursion3" "/checkout/src/test/rustdoc/recursion3.rs"
2020-03-12T18:40:53.5434477Z ------------------------------------------
2020-03-12T18:40:53.5434650Z 
2020-03-12T18:40:53.5435006Z ------------------------------------------
2020-03-12T18:40:53.5435222Z stderr:
---
2020-03-12T18:40:53.5444821Z ---- [rustdoc] rustdoc/redirect-rename.rs stdout ----
2020-03-12T18:40:53.5445017Z 
2020-03-12T18:40:53.5445191Z error: rustdoc failed!
2020-03-12T18:40:53.5445392Z status: exit code: 1
2020-03-12T18:40:53.5446737Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/redirect-rename/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/redirect-rename" "/checkout/src/test/rustdoc/redirect-rename.rs"
2020-03-12T18:40:53.5447918Z ------------------------------------------
2020-03-12T18:40:53.5448089Z 
2020-03-12T18:40:53.5448463Z ------------------------------------------
2020-03-12T18:40:53.5448663Z stderr:
---
2020-03-12T18:40:53.5451417Z ---- [rustdoc] rustdoc/redirect.rs stdout ----
2020-03-12T18:40:53.5451595Z 
2020-03-12T18:40:53.5451768Z error: rustdoc failed!
2020-03-12T18:40:53.5451968Z status: exit code: 1
2020-03-12T18:40:53.5453344Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/redirect/auxiliary/reexp-stripped/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/redirect" "/checkout/src/test/rustdoc/auxiliary/reexp-stripped.rs"
2020-03-12T18:40:53.5454583Z ------------------------------------------
2020-03-12T18:40:53.5454773Z 
2020-03-12T18:40:53.5455130Z ------------------------------------------
2020-03-12T18:40:53.5455332Z stderr:
---
2020-03-12T18:40:53.5471681Z ---- [rustdoc] rustdoc/rustc-macro-crate.rs stdout ----
2020-03-12T18:40:53.5471877Z 
2020-03-12T18:40:53.5472032Z error: rustdoc failed!
2020-03-12T18:40:53.5472249Z status: exit code: 1
2020-03-12T18:40:53.5473679Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/rustc-macro-crate/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/rustc-macro-crate" "/checkout/src/test/rustdoc/rustc-macro-crate.rs" "--crate-type" "proc-macro"
2020-03-12T18:40:53.5474939Z ------------------------------------------
2020-03-12T18:40:53.5475111Z 
2020-03-12T18:40:53.5475482Z ------------------------------------------
2020-03-12T18:40:53.5475683Z stderr:
---
2020-03-12T18:40:53.5492214Z ---- [rustdoc] rustdoc/search-index.rs stdout ----
2020-03-12T18:40:53.5492400Z 
2020-03-12T18:40:53.5492556Z error: rustdoc failed!
2020-03-12T18:40:53.5492771Z status: exit code: 1
2020-03-12T18:40:53.5494091Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/search-index/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/search-index" "/checkout/src/test/rustdoc/search-index.rs"
2020-03-12T18:40:53.5495275Z ------------------------------------------
2020-03-12T18:40:53.5495448Z 
2020-03-12T18:40:53.5495802Z ------------------------------------------
2020-03-12T18:40:53.5496020Z stderr:
---
2020-03-12T18:40:53.5498797Z ---- [rustdoc] rustdoc/short-docblock-codeblock.rs stdout ----
2020-03-12T18:40:53.5499010Z 
2020-03-12T18:40:53.5499165Z error: rustdoc failed!
2020-03-12T18:40:53.5499384Z status: exit code: 1
2020-03-12T18:40:53.5500783Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/short-docblock-codeblock/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/short-docblock-codeblock" "/checkout/src/test/rustdoc/short-docblock-codeblock.rs"
2020-03-12T18:40:53.5502022Z ------------------------------------------
2020-03-12T18:40:53.5502194Z 
2020-03-12T18:40:53.5502564Z ------------------------------------------
2020-03-12T18:40:53.5502766Z stderr:
---
2020-03-12T18:40:53.5505632Z ---- [rustdoc] rustdoc/short-dockblock.rs stdout ----
2020-03-12T18:40:53.5505824Z 
2020-03-12T18:40:53.5505996Z error: rustdoc failed!
2020-03-12T18:40:53.5506196Z status: exit code: 1
2020-03-12T18:40:53.5507532Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/short-dockblock/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/short-dockblock" "/checkout/src/test/rustdoc/short-dockblock.rs"
2020-03-12T18:40:53.5508732Z ------------------------------------------
2020-03-12T18:40:53.5508903Z 
2020-03-12T18:40:53.5509281Z ------------------------------------------
2020-03-12T18:40:53.5509483Z stderr:
---
2020-03-12T18:40:53.5530692Z ---- [rustdoc] rustdoc/sidebar-link-generation.rs stdout ----
2020-03-12T18:40:53.5530896Z 
2020-03-12T18:40:53.5531075Z error: rustdoc failed!
2020-03-12T18:40:53.5531275Z status: exit code: 1
2020-03-12T18:40:53.5532670Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/sidebar-link-generation/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/sidebar-link-generation" "/checkout/src/test/rustdoc/sidebar-link-generation.rs"
2020-03-12T18:40:53.5533906Z ------------------------------------------
2020-03-12T18:40:53.5534086Z 
2020-03-12T18:40:53.5534462Z ------------------------------------------
2020-03-12T18:40:53.5534662Z stderr:
---
2020-03-12T18:40:53.5537475Z ---- [rustdoc] rustdoc/sidebar-links-to-foreign-impl.rs stdout ----
2020-03-12T18:40:53.5537706Z 
2020-03-12T18:40:53.5537865Z error: rustdoc failed!
2020-03-12T18:40:53.5538065Z status: exit code: 1
2020-03-12T18:40:53.5539519Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/sidebar-links-to-foreign-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/sidebar-links-to-foreign-impl" "/checkout/src/test/rustdoc/sidebar-links-to-foreign-impl.rs"
2020-03-12T18:40:53.5540776Z ------------------------------------------
2020-03-12T18:40:53.5540966Z 
2020-03-12T18:40:53.5541328Z ------------------------------------------
2020-03-12T18:40:53.5541529Z stderr:
---
2020-03-12T18:40:53.5544391Z ---- [rustdoc] rustdoc/smoke.rs stdout ----
2020-03-12T18:40:53.5544566Z 
2020-03-12T18:40:53.5544722Z error: rustdoc failed!
2020-03-12T18:40:53.5544970Z status: exit code: 1
2020-03-12T18:40:53.5546251Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/smoke/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/smoke" "/checkout/src/test/rustdoc/smoke.rs"
2020-03-12T18:40:53.5547395Z ------------------------------------------
2020-03-12T18:40:53.5547567Z 
2020-03-12T18:40:53.5547924Z ------------------------------------------
2020-03-12T18:40:53.5548142Z stderr:
---
2020-03-12T18:40:53.5550937Z ---- [rustdoc] rustdoc/sort-modules-by-appearance.rs stdout ----
2020-03-12T18:40:53.5551148Z 
2020-03-12T18:40:53.5551330Z error: rustdoc failed!
2020-03-12T18:40:53.5551528Z status: exit code: 1
2020-03-12T18:40:53.5553090Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/sort-modules-by-appearance/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/sort-modules-by-appearance" "/checkout/src/test/rustdoc/sort-modules-by-appearance.rs" "-Z" "unstable-options" "--sort-modules-by-appearance"
2020-03-12T18:40:53.5554439Z ------------------------------------------
2020-03-12T18:40:53.5554629Z 
2020-03-12T18:40:53.5554992Z ------------------------------------------
2020-03-12T18:40:53.5555192Z stderr:
---
2020-03-12T18:40:53.5564729Z ---- [rustdoc] rustdoc/src-links-external.rs stdout ----
2020-03-12T18:40:53.5565005Z 
2020-03-12T18:40:53.5565169Z error: rustdoc failed!
2020-03-12T18:40:53.5565386Z status: exit code: 1
2020-03-12T18:40:53.5566855Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/src-links-external/auxiliary/src-links-external/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/src-links-external" "/checkout/src/test/rustdoc/auxiliary/src-links-external.rs"
2020-03-12T18:40:53.5568129Z ------------------------------------------
2020-03-12T18:40:53.5568301Z 
2020-03-12T18:40:53.5568658Z ------------------------------------------
2020-03-12T18:40:53.5568875Z stderr:
---
2020-03-12T18:40:53.5571637Z ---- [rustdoc] rustdoc/src-links.rs stdout ----
2020-03-12T18:40:53.5571817Z 
2020-03-12T18:40:53.5571990Z error: rustdoc failed!
2020-03-12T18:40:53.5572191Z status: exit code: 1
2020-03-12T18:40:53.5573475Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/src-links/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/src-links" "/checkout/src/test/rustdoc/src-links.rs"
2020-03-12T18:40:53.5574640Z ------------------------------------------
2020-03-12T18:40:53.5574818Z 
2020-03-12T18:40:53.5575191Z ------------------------------------------
2020-03-12T18:40:53.5575393Z stderr:
---
2020-03-12T18:40:53.5578144Z ---- [rustdoc] rustdoc/stability.rs stdout ----
2020-03-12T18:40:53.5578324Z 
2020-03-12T18:40:53.5578499Z error: rustdoc failed!
2020-03-12T18:40:53.5578727Z status: exit code: 1
2020-03-12T18:40:53.5580026Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/stability/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/stability" "/checkout/src/test/rustdoc/stability.rs"
2020-03-12T18:40:53.5581193Z ------------------------------------------
2020-03-12T18:40:53.5581363Z 
2020-03-12T18:40:53.5581739Z ------------------------------------------
2020-03-12T18:40:53.5581938Z stderr:
---
2020-03-12T18:40:53.5584825Z ---- [rustdoc] rustdoc/static-root-path.rs stdout ----
2020-03-12T18:40:53.5585037Z 
2020-03-12T18:40:53.5585194Z error: rustdoc failed!
2020-03-12T18:40:53.5585394Z status: exit code: 1
2020-03-12T18:40:53.5586910Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/static-root-path/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/static-root-path" "/checkout/src/test/rustdoc/static-root-path.rs" "-Z" "unstable-options" "--static-root-path" "/cache/"
2020-03-12T18:40:53.5588195Z ------------------------------------------
2020-03-12T18:40:53.5588365Z 
2020-03-12T18:40:53.5588722Z ------------------------------------------
2020-03-12T18:40:53.5588923Z stderr:
---
2020-03-12T18:40:53.5591707Z ---- [rustdoc] rustdoc/struct-field.rs stdout ----
2020-03-12T18:40:53.5591891Z 
2020-03-12T18:40:53.5592049Z error: rustdoc failed!
2020-03-12T18:40:53.5592248Z status: exit code: 1
2020-03-12T18:40:53.5593573Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/struct-field/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/struct-field" "/checkout/src/test/rustdoc/struct-field.rs"
2020-03-12T18:40:53.5594759Z ------------------------------------------
2020-03-12T18:40:53.5594931Z 
2020-03-12T18:40:53.5595287Z ------------------------------------------
2020-03-12T18:40:53.5595505Z stderr:
---
2020-03-12T18:40:53.5598259Z ---- [rustdoc] rustdoc/structfields.rs stdout ----
2020-03-12T18:40:53.5598443Z 
2020-03-12T18:40:53.5598601Z error: rustdoc failed!
2020-03-12T18:40:53.5598817Z status: exit code: 1
2020-03-12T18:40:53.5600266Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/structfields/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/structfields" "/checkout/src/test/rustdoc/structfields.rs" "-Z" "unstable-options" "--generate-redirect-pages"
2020-03-12T18:40:53.5601528Z ------------------------------------------
2020-03-12T18:40:53.5601701Z 
2020-03-12T18:40:53.5602071Z ------------------------------------------
2020-03-12T18:40:53.5602271Z stderr:
---
2020-03-12T18:40:53.5605424Z ---- [rustdoc] rustdoc/synthetic_auto/basic.rs stdout ----
2020-03-12T18:40:53.5605623Z 
2020-03-12T18:40:53.5605799Z error: rustdoc failed!
2020-03-12T18:40:53.5605998Z status: exit code: 1
2020-03-12T18:40:53.5607372Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/basic/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/basic" "/checkout/src/test/rustdoc/synthetic_auto/basic.rs"
2020-03-12T18:40:53.5608588Z ------------------------------------------
2020-03-12T18:40:53.5608760Z 
2020-03-12T18:40:53.5609136Z ------------------------------------------
2020-03-12T18:40:53.5609343Z stderr:
---
2020-03-12T18:40:53.5612132Z ---- [rustdoc] rustdoc/synthetic_auto/complex.rs stdout ----
2020-03-12T18:40:53.5612353Z 
2020-03-12T18:40:53.5612508Z error: rustdoc failed!
2020-03-12T18:40:53.5612710Z status: exit code: 1
2020-03-12T18:40:53.5614126Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/complex/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/complex" "/checkout/src/test/rustdoc/synthetic_auto/complex.rs"
2020-03-12T18:40:53.5615339Z ------------------------------------------
2020-03-12T18:40:53.5615529Z 
2020-03-12T18:40:53.5615885Z ------------------------------------------
2020-03-12T18:40:53.5616088Z stderr:
---
2020-03-12T18:40:53.5618909Z ---- [rustdoc] rustdoc/synthetic_auto/crate-local.rs stdout ----
2020-03-12T18:40:53.5619127Z 
2020-03-12T18:40:53.5619283Z error: rustdoc failed!
2020-03-12T18:40:53.5619484Z status: exit code: 1
2020-03-12T18:40:53.5620915Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/crate-local/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/crate-local" "/checkout/src/test/rustdoc/synthetic_auto/crate-local.rs"
2020-03-12T18:40:53.5622164Z ------------------------------------------
2020-03-12T18:40:53.5622337Z 
2020-03-12T18:40:53.5622689Z ------------------------------------------
2020-03-12T18:40:53.5622906Z stderr:
---
2020-03-12T18:40:53.5632549Z ---- [rustdoc] rustdoc/synthetic_auto/manual.rs stdout ----
2020-03-12T18:40:53.5632752Z 
2020-03-12T18:40:53.5632926Z error: rustdoc failed!
2020-03-12T18:40:53.5633126Z status: exit code: 1
2020-03-12T18:40:53.5634509Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/manual/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/manual" "/checkout/src/test/rustdoc/synthetic_auto/manual.rs"
2020-03-12T18:40:53.5635739Z ------------------------------------------
2020-03-12T18:40:53.5635910Z 
2020-03-12T18:40:53.5636283Z ------------------------------------------
2020-03-12T18:40:53.5636484Z stderr:
---
2020-03-12T18:40:53.5639274Z ---- [rustdoc] rustdoc/synthetic_auto/negative.rs stdout ----
2020-03-12T18:40:53.5639496Z 
2020-03-12T18:40:53.5639658Z error: rustdoc failed!
2020-03-12T18:40:53.5639859Z status: exit code: 1
2020-03-12T18:40:53.5641263Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/negative/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/negative" "/checkout/src/test/rustdoc/synthetic_auto/negative.rs"
2020-03-12T18:40:53.5642478Z ------------------------------------------
2020-03-12T18:40:53.5642665Z 
2020-03-12T18:40:53.5643020Z ------------------------------------------
2020-03-12T18:40:53.5643308Z stderr:
---
2020-03-12T18:40:53.5646257Z ---- [rustdoc] rustdoc/synthetic_auto/nested.rs stdout ----
2020-03-12T18:40:53.5646459Z 
2020-03-12T18:40:53.5646616Z error: rustdoc failed!
2020-03-12T18:40:53.5646816Z status: exit code: 1
2020-03-12T18:40:53.5648206Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/nested/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/nested" "/checkout/src/test/rustdoc/synthetic_auto/nested.rs"
2020-03-12T18:40:53.5649439Z ------------------------------------------
2020-03-12T18:40:53.5649617Z 
2020-03-12T18:40:53.5649974Z ------------------------------------------
2020-03-12T18:40:53.5650192Z stderr:
---
2020-03-12T18:40:53.5652990Z ---- [rustdoc] rustdoc/synthetic_auto/no-redundancy.rs stdout ----
2020-03-12T18:40:53.5653203Z 
2020-03-12T18:40:53.5653359Z error: rustdoc failed!
2020-03-12T18:40:53.5653576Z status: exit code: 1
2020-03-12T18:40:53.5655017Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/no-redundancy/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/no-redundancy" "/checkout/src/test/rustdoc/synthetic_auto/no-redundancy.rs"
2020-03-12T18:40:53.5656279Z ------------------------------------------
2020-03-12T18:40:53.5656453Z 
2020-03-12T18:40:53.5656825Z ------------------------------------------
2020-03-12T18:40:53.5657029Z stderr:
---
2020-03-12T18:40:53.5666687Z ---- [rustdoc] rustdoc/synthetic_auto/self-referential.rs stdout ----
2020-03-12T18:40:53.5666921Z 
2020-03-12T18:40:53.5667080Z error: rustdoc failed!
2020-03-12T18:40:53.5667278Z status: exit code: 1
2020-03-12T18:40:53.5668763Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/self-referential/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/self-referential" "/checkout/src/test/rustdoc/synthetic_auto/self-referential.rs"
2020-03-12T18:40:53.5670019Z ------------------------------------------
2020-03-12T18:40:53.5670207Z 
2020-03-12T18:40:53.5670563Z ------------------------------------------
2020-03-12T18:40:53.5670763Z stderr:
---
2020-03-12T18:40:53.5673571Z ---- [rustdoc] rustdoc/synthetic_auto/static-region.rs stdout ----
2020-03-12T18:40:53.5673790Z 
2020-03-12T18:40:53.5673946Z error: rustdoc failed!
2020-03-12T18:40:53.5674146Z status: exit code: 1
2020-03-12T18:40:53.5675595Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/static-region/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/static-region" "/checkout/src/test/rustdoc/synthetic_auto/static-region.rs"
2020-03-12T18:40:53.5676847Z ------------------------------------------
2020-03-12T18:40:53.5677018Z 
2020-03-12T18:40:53.5677374Z ------------------------------------------
2020-03-12T18:40:53.5677627Z stderr:
---
2020-03-12T18:40:53.5680392Z ---- [rustdoc] rustdoc/test-lists.rs stdout ----
2020-03-12T18:40:53.5680574Z 
2020-03-12T18:40:53.5680729Z error: rustdoc failed!
2020-03-12T18:40:53.5680945Z status: exit code: 1
2020-03-12T18:40:53.5682244Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test-lists/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test-lists" "/checkout/src/test/rustdoc/test-lists.rs"
2020-03-12T18:40:53.5683499Z ------------------------------------------
2020-03-12T18:40:53.5683774Z 
2020-03-12T18:40:53.5684159Z ------------------------------------------
2020-03-12T18:40:53.5684362Z stderr:
---
2020-03-12T18:40:53.5687803Z ---- [rustdoc] rustdoc/test-parens.rs stdout ----
2020-03-12T18:40:53.5688000Z 
2020-03-12T18:40:53.5688187Z error: rustdoc failed!
2020-03-12T18:40:53.5688386Z status: exit code: 1
2020-03-12T18:40:53.5689767Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test-parens/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test-parens" "/checkout/src/test/rustdoc/test-parens.rs"
2020-03-12T18:40:53.5690945Z ------------------------------------------
2020-03-12T18:40:53.5691117Z 
2020-03-12T18:40:53.5691789Z ------------------------------------------
2020-03-12T18:40:53.5691992Z stderr:
---
2020-03-12T18:40:53.5694770Z ---- [rustdoc] rustdoc/through-proc-macro.rs stdout ----
2020-03-12T18:40:53.5694987Z 
2020-03-12T18:40:53.5695147Z error: rustdoc failed!
2020-03-12T18:40:53.5695352Z status: exit code: 1
2020-03-12T18:40:53.5696860Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/through-proc-macro/auxiliary/through-proc-macro-aux/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/through-proc-macro" "/checkout/src/test/rustdoc/auxiliary/through-proc-macro-aux.rs"
2020-03-12T18:40:53.5698138Z ------------------------------------------
2020-03-12T18:40:53.5698329Z 
2020-03-12T18:40:53.5698685Z ------------------------------------------
2020-03-12T18:40:53.5698886Z stderr:
---
2020-03-12T18:40:53.5716207Z ---- [rustdoc] rustdoc/trait-self-link.rs stdout ----
2020-03-12T18:40:53.5716397Z 
2020-03-12T18:40:53.5716573Z error: rustdoc failed!
2020-03-12T18:40:53.5716772Z status: exit code: 1
2020-03-12T18:40:53.5718109Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/trait-self-link/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/trait-self-link" "/checkout/src/test/rustdoc/trait-self-link.rs"
2020-03-12T18:40:53.5719302Z ------------------------------------------
2020-03-12T18:40:53.5719475Z 
2020-03-12T18:40:53.5719855Z ------------------------------------------
2020-03-12T18:40:53.5720056Z stderr:
---
2020-03-12T18:40:53.5722809Z ---- [rustdoc] rustdoc/trait_alias.rs stdout ----
2020-03-12T18:40:53.5723009Z 
2020-03-12T18:40:53.5723267Z error: rustdoc failed!
2020-03-12T18:40:53.5723469Z status: exit code: 1
2020-03-12T18:40:53.5724853Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/trait_alias/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/trait_alias" "/checkout/src/test/rustdoc/trait_alias.rs"
2020-03-12T18:40:53.5726065Z ------------------------------------------
2020-03-12T18:40:53.5726255Z 
2020-03-12T18:40:53.5726611Z ------------------------------------------
2020-03-12T18:40:53.5726810Z stderr:
---
2020-03-12T18:40:53.5729610Z ---- [rustdoc] rustdoc/traits-in-bodies-private.rs stdout ----
2020-03-12T18:40:53.5729822Z 
2020-03-12T18:40:53.5729980Z error: rustdoc failed!
2020-03-12T18:40:53.5730178Z status: exit code: 1
2020-03-12T18:40:53.5731670Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/traits-in-bodies-private/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/traits-in-bodies-private" "/checkout/src/test/rustdoc/traits-in-bodies-private.rs" "--document-private-items"
2020-03-12T18:40:53.5732963Z ------------------------------------------
2020-03-12T18:40:53.5733135Z 
2020-03-12T18:40:53.5733492Z ------------------------------------------
2020-03-12T18:40:53.5733710Z stderr:
---
2020-03-12T18:40:53.5736489Z ---- [rustdoc] rustdoc/traits-in-bodies.rs stdout ----
2020-03-12T18:40:53.5736682Z 
2020-03-12T18:40:53.5736840Z error: rustdoc failed!
2020-03-12T18:40:53.5737056Z status: exit code: 1
2020-03-12T18:40:53.5738400Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/traits-in-bodies/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/traits-in-bodies" "/checkout/src/test/rustdoc/traits-in-bodies.rs"
2020-03-12T18:40:53.5739596Z ------------------------------------------
2020-03-12T18:40:53.5739774Z 
2020-03-12T18:40:53.5740129Z ------------------------------------------
2020-03-12T18:40:53.5740353Z stderr:
---
2020-03-12T18:40:53.5749655Z ---- [rustdoc] rustdoc/typedef.rs stdout ----
2020-03-12T18:40:53.5749839Z 
2020-03-12T18:40:53.5750012Z error: rustdoc failed!
2020-03-12T18:40:53.5750216Z status: exit code: 1
2020-03-12T18:40:53.5751488Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/typedef/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/typedef" "/checkout/src/test/rustdoc/typedef.rs"
2020-03-12T18:40:53.5752640Z ------------------------------------------
2020-03-12T18:40:53.5752829Z 
2020-03-12T18:40:53.5753185Z ------------------------------------------
2020-03-12T18:40:53.5753384Z stderr:
---
2020-03-12T18:40:53.5756125Z ---- [rustdoc] rustdoc/union.rs stdout ----
2020-03-12T18:40:53.5756316Z 
2020-03-12T18:40:53.5756471Z error: rustdoc failed!
2020-03-12T18:40:53.5756670Z status: exit code: 1
2020-03-12T18:40:53.5757936Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/union/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/union" "/checkout/src/test/rustdoc/union.rs"
2020-03-12T18:40:53.5759060Z ------------------------------------------
2020-03-12T18:40:53.5759249Z 
2020-03-12T18:40:53.5759613Z ------------------------------------------
2020-03-12T18:40:53.5759813Z stderr:
---
2020-03-12T18:40:53.5769533Z ---- [rustdoc] rustdoc/unneeded-trait-implementations-title.rs stdout ----
2020-03-12T18:40:53.5769763Z 
2020-03-12T18:40:53.5769919Z error: rustdoc failed!
2020-03-12T18:40:53.5770141Z status: exit code: 1
2020-03-12T18:40:53.5771637Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/unneeded-trait-implementations-title/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/unneeded-trait-implementations-title" "/checkout/src/test/rustdoc/unneeded-trait-implementations-title.rs"
2020-03-12T18:40:53.5772931Z ------------------------------------------
2020-03-12T18:40:53.5773102Z 
2020-03-12T18:40:53.5773475Z ------------------------------------------
2020-03-12T18:40:53.5773677Z stderr:
---
2020-03-12T18:40:53.5776440Z ---- [rustdoc] rustdoc/unit-return.rs stdout ----
2020-03-12T18:40:53.5776624Z 
2020-03-12T18:40:53.5776798Z error: rustdoc failed!
2020-03-12T18:40:53.5777028Z status: exit code: 1
2020-03-12T18:40:53.5778329Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/unit-return/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/unit-return" "/checkout/src/test/rustdoc/unit-return.rs"
2020-03-12T18:40:53.5779503Z ------------------------------------------
2020-03-12T18:40:53.5779675Z 
2020-03-12T18:40:53.5780052Z ------------------------------------------
2020-03-12T18:40:53.5780252Z stderr:
---
2020-03-12T18:40:53.5782988Z ---- [rustdoc] rustdoc/use-attr.rs stdout ----
2020-03-12T18:40:53.5783183Z 
2020-03-12T18:40:53.5783339Z error: rustdoc failed!
2020-03-12T18:40:53.5783539Z status: exit code: 1
2020-03-12T18:40:53.5784936Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/use-attr/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/use-attr" "/checkout/src/test/rustdoc/use-attr.rs" "--edition=2018"
2020-03-12T18:40:53.5786156Z ------------------------------------------
2020-03-12T18:40:53.5786344Z 
2020-03-12T18:40:53.5786704Z ------------------------------------------
2020-03-12T18:40:53.5786903Z stderr:
---
2020-03-12T18:40:53.5796397Z ---- [rustdoc] rustdoc/variadic.rs stdout ----
2020-03-12T18:40:53.5796575Z 
2020-03-12T18:40:53.5796732Z error: rustdoc failed!
2020-03-12T18:40:53.5796947Z status: exit code: 1
2020-03-12T18:40:53.5798484Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/variadic/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/variadic" "/checkout/src/test/rustdoc/variadic.rs"
2020-03-12T18:40:53.5799650Z ------------------------------------------
2020-03-12T18:40:53.5799822Z 
2020-03-12T18:40:53.5800184Z ------------------------------------------
2020-03-12T18:40:53.5800401Z stderr:
---
2020-03-12T18:40:53.5803246Z ---- [rustdoc] rustdoc/viewpath-rename.rs stdout ----
2020-03-12T18:40:53.5803439Z 
2020-03-12T18:40:53.5803595Z error: rustdoc failed!
2020-03-12T18:40:53.5803811Z status: exit code: 1
2020-03-12T18:40:53.5805210Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/viewpath-rename/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/viewpath-rename" "/checkout/src/test/rustdoc/viewpath-rename.rs"
2020-03-12T18:40:53.5806460Z ------------------------------------------
2020-03-12T18:40:53.5806633Z 
2020-03-12T18:40:53.5807005Z ------------------------------------------
2020-03-12T18:40:53.5807205Z stderr:
---
2020-03-12T18:40:53.5809963Z ---- [rustdoc] rustdoc/viewpath-self.rs stdout ----
2020-03-12T18:40:53.5810151Z 
2020-03-12T18:40:53.5810330Z error: rustdoc failed!
2020-03-12T18:40:53.5810531Z status: exit code: 1
2020-03-12T18:40:53.5811841Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/viewpath-self/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/viewpath-self" "/checkout/src/test/rustdoc/viewpath-self.rs"
2020-03-12T18:40:53.5813029Z ------------------------------------------
2020-03-12T18:40:53.5813202Z 
2020-03-12T18:40:53.5813576Z ------------------------------------------
2020-03-12T18:40:53.5813776Z stderr:
---
2020-03-12T18:40:53.5816544Z ---- [rustdoc] rustdoc/where-sized.rs stdout ----
2020-03-12T18:40:53.5816746Z 
2020-03-12T18:40:53.5816903Z error: rustdoc failed!
2020-03-12T18:40:53.5817102Z status: exit code: 1
2020-03-12T18:40:53.5818418Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/where-sized/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/where-sized" "/checkout/src/test/rustdoc/where-sized.rs"
2020-03-12T18:40:53.5819616Z ------------------------------------------
2020-03-12T18:40:53.5819805Z 
2020-03-12T18:40:53.5820174Z ------------------------------------------
2020-03-12T18:40:53.5820377Z stderr:
---
2020-03-12T18:40:53.5823132Z ---- [rustdoc] rustdoc/where.rs stdout ----
2020-03-12T18:40:53.5823306Z 
2020-03-12T18:40:53.5823462Z error: rustdoc failed!
2020-03-12T18:40:53.5823663Z status: exit code: 1
2020-03-12T18:40:53.5824984Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/where/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/where" "/checkout/src/test/rustdoc/where.rs"
2020-03-12T18:40:53.5826181Z ------------------------------------------
2020-03-12T18:40:53.5826353Z 
2020-03-12T18:40:53.5826708Z ------------------------------------------
2020-03-12T18:40:53.5826925Z stderr:
---
2020-03-12T18:40:53.6004347Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-12T18:40:53.6004883Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-12T18:40:53.6005164Z 
2020-03-12T18:40:53.6005280Z 
2020-03-12T18:40:53.6010246Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-12T18:40:53.6013603Z 
2020-03-12T18:40:53.6013729Z 
2020-03-12T18:40:53.6014008Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-12T18:40:53.6014398Z Build completed unsuccessfully in 1:07:29
2020-03-12T18:40:53.6014398Z Build completed unsuccessfully in 1:07:29
2020-03-12T18:40:53.6014712Z == clock drift check ==
2020-03-12T18:40:53.6015009Z   local time: Thu Mar 12 18:40:53 UTC 2020
2020-03-12T18:40:53.9135588Z   network time: Thu, 12 Mar 2020 18:40:53 GMT
2020-03-12T18:40:53.9136642Z == end clock drift check ==
2020-03-12T18:40:54.8477050Z 
2020-03-12T18:40:54.8545043Z ##[error]Bash exited with code '1'.
2020-03-12T18:40:54.8558264Z ##[section]Finishing: Run build
2020-03-12T18:40:54.8609703Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69203/merge to s
2020-03-12T18:40:54.8614739Z Task         : Get sources
2020-03-12T18:40:54.8615241Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-12T18:40:54.8615557Z Version      : 1.0.0
2020-03-12T18:40:54.8615796Z Author       : Microsoft
2020-03-12T18:40:54.8615796Z Author       : Microsoft
2020-03-12T18:40:54.8616157Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-12T18:40:54.8616570Z ==============================================================================
2020-03-12T18:40:55.1965736Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-12T18:40:55.2010176Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69203/merge to s
2020-03-12T18:40:55.2100713Z Cleaning up task key
2020-03-12T18:40:55.2101975Z Start cleaning up orphan processes.
2020-03-12T18:40:55.2271252Z Terminate orphan process: pid (3706) (python)
2020-03-12T18:40:55.2529789Z ##[section]Finishing: Finalize Job
