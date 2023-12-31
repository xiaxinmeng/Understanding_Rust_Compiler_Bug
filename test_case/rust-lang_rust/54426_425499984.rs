plain
travis_fold:end:system_info

Network availability confirmed.
Setting APT mirror in /etc/apt/sources.list: http://us-central1.gce.archive.ubuntu.com/ubuntu/
travis_fold:start:update_heroku
Updating Heroku
$ heroku version
heroku/7.16.0 linux-x64 node-v10.10.0
travis_fold:end:update_heroku
Installing APT Packages
travis_time:start:0491b5b6
$ travis_apt_get_update
travis_time:end:0491b5b6:start=1538150885093476171,finish=1538150890306030621,duration=5212554450
---
[00:46:00] 
[00:46:00] running 4309 tests
[00:46:03] .F..................................................................................................
[00:46:06] ....................................................................................................
[00:46:09] F........F.........................F.........F..................F.....F.................F..F....FF..
[00:46:11] .................................................F......F...FF...F.......FF...F...............F.....
[00:46:14] .........................F....F.F...........F.......................................................
[00:46:17] .......................i........F...................................................................
[00:46:27] ................................i...........i.......................................................
[00:46:30] ...................................................iiiii............................................
[00:46:33] ....................................................................................................
[00:46:35] ....................................................................................................
[00:46:35] ....................................................................................................
[00:46:37] ....................................................................................................
[00:46:39] ....................................................................................................
[00:46:41] ....................................................................................................
[00:46:44] ...........................................i........................................................
[00:46:47] ..........i.............................................F...........................................
[00:46:53] ....................................................................................................
[00:46:55] i...................................................................................................
[00:46:58] ....................................................................................................
[00:46:58] ....................................................................................................
[00:47:01] ................................F....F..............................................................
[00:47:07] ....................................................................................................
[00:47:07] ....................................................................................................
[00:47:10] ...........F..........F.FF.F....F............F......................................................
[00:47:13] ..................F.................................................................................
[00:47:20] .................................................................................................i..
[00:47:22] ....................................................................................................
[00:47:22] ....................................................................................................
[00:47:25] .........................................................i.i..ii..............................F..F..
[00:47:29] ...............F.......F..F.....................F....FFF.F...F..F..........FFF.F.......F.F.....F....
[00:47:32] .......F.F.....F.......F...................................................................i........
[00:47:38] ....................................................................................................
[00:47:40] ....................................................................................................
[00:47:44] ....................................................................................................
[00:47:47] ................................i...................................................................
[00:47:47] ................................i...................................................................
[00:47:52] ....................................................................................................
[00:47:55] ....................................................................................................
[00:47:59] ......................................................................F.......................i.....
[00:48:05] .................................................................................................F..
[00:48:07] ....................................................................................................
[00:48:10] ................................................i...................................................
[00:48:10] .........
[00:48:10] .........
[00:48:10] failures:
[00:48:10] 
[00:48:10] ---- [ui] ui/E0501.rs#mir stdout ----
[00:48:10] diff of stderr:
[00:48:10] 
[00:48:10] - error[E0501]: cannot borrow `*a` as mutable because previous closure requires unique access
[00:48:10] + error[E0501]: cannot borrow `a*` as mutable because previous closure requires unique access
[00:48:10] 2   --> $DIR/E0501.rs:28:23
[00:48:10] 3    |
[00:48:10] 4 LL |     let bar = || {
[00:48:10] 
[00:48:10] 12 LL |     drop(bar);
[00:48:10] 14 
[00:48:10] 14 
[00:48:10] - error[E0501]: cannot borrow `*a` as immutable because previous closure requires unique access
[00:48:10] + error[E0501]: cannot borrow `a*` as immutable because previous closure requires unique access
[00:48:10] 16   --> $DIR/E0501.rs:31:23
[00:48:10] 17    |
[00:48:10] 18 LL |     let bar = || {
[00:48:10] 
[00:48:10] The actual stderr differed from the expected stderr.
[00:48:10] The actual stderr differed from the expected stderr.
[00:48:10] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0501.mir/E0501.mir.stderr
[00:48:10] To update references, rerun the tests and pass the `--bless` flag
[00:48:10] To only update this specific test, also pass `--test-args E0501.rs`
[00:48:10] 
[00:48:10] error in revision `mir`: 1 errors occurred comparing output.
[00:48:10] status: exit code: 1
[00:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/E0501.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0501.mir/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0501.mir/auxiliary" "-A" "unused"
[00:48:10] ------------------------------------------
[00:48:10] 
[00:48:10] ------------------------------------------
[00:48:10] stderr:
[00:48:10] stderr:
[00:48:10] ------------------------------------------
[00:48:10] {"message":"cannot borrow `a*` as mutable because previous closure requires unique access","code":{"code":"E0501","explanation":"\nThis error indicates that a mutable variable is being used while it is still\ncaptured by a closure. Because the closure has borrowed the variable, it is not\navailable for use until the closure goes out of scope.\n\nNote that a capture will either move or borrow a variable, but in this\nsituation, the closure is borrowing the variable. Take a look at\nhttp://rustbyexample.com/fn/closures/capture.html for more information about\ncapturing.\n\nExample of erroneous code:\n\n