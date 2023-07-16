plain
2019-08-14T17:39:15.0637064Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-14T17:39:15.0823380Z ##[command]git config gc.auto 0
2019-08-14T17:39:15.0892228Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-14T17:39:15.0940830Z ##[command]git config --get-all http.proxy
2019-08-14T17:39:15.1082482Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63565/merge:refs/remotes/pull/63565/merge
---
2019-08-14T17:39:51.1675003Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-14T17:39:51.1675039Z 
2019-08-14T17:39:51.1675292Z   git checkout -b <new-branch-name>
2019-08-14T17:39:51.1675326Z 
2019-08-14T17:39:51.1675378Z HEAD is now at 10fa022cb Merge 00d60a5cad12e62411b34bdf293b1b8c4a47b3d3 into c43d03a19f326f4a323569328cc501e86eb6d22e
2019-08-14T17:39:51.1851306Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-14T17:39:51.1854302Z ==============================================================================
2019-08-14T17:39:51.1854351Z Task         : Bash
2019-08-14T17:39:51.1854405Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-14T18:43:51.8334697Z .................................................................................................... 1400/8906
2019-08-14T18:43:58.5667058Z .................................................................................................... 1500/8906
2019-08-14T18:44:08.5531185Z ..F.F.......................................................................................i....... 1600/8906
2019-08-14T18:44:17.0788847Z ........i........................................................................................... 1700/8906
2019-08-14T18:44:24.5484352Z ...................................................................................iiiii............ 1800/8906
2019-08-14T18:44:47.4845285Z .................................................................................................... 2000/8906
2019-08-14T18:44:50.0952045Z .................................................................................................... 2100/8906
2019-08-14T18:44:52.8904127Z .................................................................................................... 2200/8906
2019-08-14T18:45:00.5737935Z .................................................................................................... 2300/8906
---
2019-08-14T18:49:04.8128068Z .................................................................................................... 5300/8906
2019-08-14T18:49:12.8964035Z .................i.................................................................................. 5400/8906
2019-08-14T18:49:18.7549957Z .................................................................................................... 5500/8906
2019-08-14T18:49:31.7573505Z .................................................................................................... 5600/8906
2019-08-14T18:49:45.8330063Z ............ii...i..ii...........i.................................................................. 5700/8906
2019-08-14T18:50:03.2040177Z .................................................................................................... 5900/8906
2019-08-14T18:50:08.2433172Z .................................................................................................... 6000/8906
2019-08-14T18:50:08.2433172Z .................................................................................................... 6000/8906
2019-08-14T18:50:22.6852805Z .............i..ii.................................................................................. 6100/8906
2019-08-14T18:50:42.7992156Z ........................................................i........................................... 6300/8906
2019-08-14T18:50:45.0806124Z .................................................................................................... 6400/8906
2019-08-14T18:50:47.6439037Z ............................i....................................................................... 6500/8906
2019-08-14T18:50:52.1290776Z .............................................................F...................................... 6600/8906
---
2019-08-14T18:55:00.4421948Z 
2019-08-14T18:55:00.4422777Z ---- [ui] ui/borrowck/borrowck-anon-fields-variant.rs stdout ----
2019-08-14T18:55:00.4422839Z diff of stderr:
2019-08-14T18:55:00.4422869Z 
2019-08-14T18:55:00.4423506Z - warning[E0503]: cannot use `y` because it was mutably borrowed
2019-08-14T18:55:00.4423604Z + error[E0503]: cannot use `y` because it was mutably borrowed
2019-08-14T18:55:00.4423928Z 3    |
2019-08-14T18:55:00.4423928Z 3    |
2019-08-14T18:55:00.4423973Z 4 LL |       Foo::Y(ref mut a, _) => a,
2019-08-14T18:55:00.4424041Z 9 ...
2019-08-14T18:55:00.4424104Z 10 LL |     *a += 1;
2019-08-14T18:55:00.4424104Z 10 LL |     *a += 1;
2019-08-14T18:55:00.4424334Z 11    |     ------- borrow later used here
2019-08-14T18:55:00.4425386Z -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
2019-08-14T18:55:00.4425774Z -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
2019-08-14T18:55:00.4426036Z -    = note: for more information, try `rustc --explain E0729`
2019-08-14T18:55:00.4426105Z 16 
2019-08-14T18:55:00.4426105Z 16 
2019-08-14T18:55:00.4426151Z 17 error[E0503]: cannot use `y` because it was mutably borrowed
2019-08-14T18:55:00.4426511Z 
2019-08-14T18:55:00.4426554Z 38 LL |     *a += 1;
2019-08-14T18:55:00.4426554Z 38 LL |     *a += 1;
2019-08-14T18:55:00.4426780Z 39    |     ------- first borrow later used here
2019-08-14T18:55:00.4427059Z - error: aborting due to 2 previous errors
2019-08-14T18:55:00.4427107Z + error: aborting due to 3 previous errors
2019-08-14T18:55:00.4427147Z 42 
2019-08-14T18:55:00.4427208Z 43 Some errors have detailed explanations: E0499, E0503.
2019-08-14T18:55:00.4427208Z 43 Some errors have detailed explanations: E0499, E0503.
2019-08-14T18:55:00.4427466Z 44 For more information about an error, try `rustc --explain E0499`.
2019-08-14T18:55:00.4427501Z 
2019-08-14T18:55:00.4427527Z 
2019-08-14T18:55:00.4427591Z The actual stderr differed from the expected stderr.
2019-08-14T18:55:00.4427917Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-anon-fields-variant/borrowck-anon-fields-variant.stderr
2019-08-14T18:55:00.4428169Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T18:55:00.4428670Z To only update this specific test, also pass `--test-args borrowck/borrowck-anon-fields-variant.rs`
2019-08-14T18:55:00.4428758Z error: 1 errors occurred comparing output.
2019-08-14T18:55:00.4428825Z status: exit code: 1
2019-08-14T18:55:00.4428825Z status: exit code: 1
2019-08-14T18:55:00.4429618Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-anon-fields-variant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-anon-fields-variant" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-anon-fields-variant/auxiliary" "-A" "unused"
2019-08-14T18:55:00.4429979Z ------------------------------------------
2019-08-14T18:55:00.4430024Z 
2019-08-14T18:55:00.4430284Z ------------------------------------------
2019-08-14T18:55:00.4430331Z stderr:
2019-08-14T18:55:00.4430331Z stderr:
2019-08-14T18:55:00.4430566Z ------------------------------------------
2019-08-14T18:55:00.4430634Z error[E0503]: cannot use `y` because it was mutably borrowed
2019-08-14T18:55:00.4430967Z    |
2019-08-14T18:55:00.4430967Z    |
2019-08-14T18:55:00.4431027Z LL |       Foo::Y(ref mut a, _) => a,
2019-08-14T18:55:00.4431293Z    |              --------- borrow of `y.0` occurs here
2019-08-14T18:55:00.4431342Z ...
2019-08-14T18:55:00.4431402Z LL |       Foo::Y(_, ref mut b) => b,
2019-08-14T18:55:00.4431453Z    |       ^^^^^^^^^^^^^^^^^^^^ use of borrowed `y.0`
2019-08-14T18:55:00.4431541Z LL |     *a += 1;
2019-08-14T18:55:00.4431541Z LL |     *a += 1;
2019-08-14T18:55:00.4431795Z    |     ------- borrow later used here
2019-08-14T18:55:00.4431828Z 
2019-08-14T18:55:00.4431876Z error[E0503]: cannot use `y` because it was mutably borrowed
2019-08-14T18:55:00.4432223Z    |
2019-08-14T18:55:00.4432223Z    |
2019-08-14T18:55:00.4432266Z LL |       Foo::Y(ref mut a, _) => a,
2019-08-14T18:55:00.4432514Z    |              --------- borrow of `y.0` occurs here
2019-08-14T18:55:00.4433382Z ...
2019-08-14T18:55:00.4433440Z LL |       Foo::Y(ref mut b, _) => b, //~ ERROR cannot use `y`
2019-08-14T18:55:00.4433491Z    |       ^^^^^^^^^^^^^^^^^^^^ use of borrowed `y.0`
2019-08-14T18:55:00.4433918Z LL |     *a += 1;
2019-08-14T18:55:00.4433918Z LL |     *a += 1;
2019-08-14T18:55:00.4434277Z    |     ------- borrow later used here
2019-08-14T18:55:00.4434333Z 
2019-08-14T18:55:00.4434382Z error[E0499]: cannot borrow `y.0` as mutable more than once at a time
2019-08-14T18:55:00.4434696Z    |
2019-08-14T18:55:00.4434696Z    |
2019-08-14T18:55:00.4434756Z LL |       Foo::Y(ref mut a, _) => a,
2019-08-14T18:55:00.4435008Z    |              --------- first mutable borrow occurs here
2019-08-14T18:55:00.4435054Z ...
2019-08-14T18:55:00.4435115Z LL |       Foo::Y(ref mut b, _) => b, //~ ERROR cannot use `y`
2019-08-14T18:55:00.4435163Z    |              ^^^^^^^^^ second mutable borrow occurs here
2019-08-14T18:55:00.4435261Z LL |     *a += 1;
2019-08-14T18:55:00.4435261Z LL |     *a += 1;
2019-08-14T18:55:00.4435484Z    |     ------- first borrow later used here
2019-08-14T18:55:00.4435560Z error: aborting due to 3 previous errors
2019-08-14T18:55:00.4435603Z 
2019-08-14T18:55:00.4435655Z Some errors have detailed explanations: E0499, E0503.
2019-08-14T18:55:00.4435903Z For more information about an error, try `rustc --explain E0499`.
2019-08-14T18:55:00.4435903Z For more information about an error, try `rustc --explain E0499`.
2019-08-14T18:55:00.4435937Z 
2019-08-14T18:55:00.4436168Z ------------------------------------------
2019-08-14T18:55:00.4436199Z 
2019-08-14T18:55:00.4436224Z 
2019-08-14T18:55:00.4436575Z ---- [ui] ui/borrowck/borrowck-describe-lvalue.rs stdout ----
2019-08-14T18:55:00.4436724Z diff of stderr:
2019-08-14T18:55:00.4436751Z 
2019-08-14T18:55:00.4436791Z 328 LL |         drop(x);
2019-08-14T18:55:00.4437050Z 329    |              - mutable borrow later used here
2019-08-14T18:55:00.4437111Z 330 
2019-08-14T18:55:00.4437376Z - warning[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4437437Z + error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4437749Z 333    |
2019-08-14T18:55:00.4437791Z 334 LL |             let x = &mut block;
2019-08-14T18:55:00.4437958Z 
2019-08-14T18:55:00.4437995Z 338 ...
2019-08-14T18:55:00.4437995Z 338 ...
2019-08-14T18:55:00.4438034Z 339 LL |             drop(x);
2019-08-14T18:55:00.4438256Z 340    |                  - mutable borrow later used here
2019-08-14T18:55:00.4438710Z -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
2019-08-14T18:55:00.4439007Z -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
2019-08-14T18:55:00.4439258Z -    = note: for more information, try `rustc --explain E0729`
2019-08-14T18:55:00.4439303Z 345 
2019-08-14T18:55:00.4439303Z 345 
2019-08-14T18:55:00.4439795Z - warning[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4439865Z + error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4440146Z 348    |
2019-08-14T18:55:00.4440187Z 349 LL |             let x = &mut block;
2019-08-14T18:55:00.4440214Z 
2019-08-14T18:55:00.4440250Z 353 ...
2019-08-14T18:55:00.4440250Z 353 ...
2019-08-14T18:55:00.4440303Z 354 LL |             drop(x);
2019-08-14T18:55:00.4440525Z 355    |                  - mutable borrow later used here
2019-08-14T18:55:00.4440985Z -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
2019-08-14T18:55:00.4441274Z -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
2019-08-14T18:55:00.4441506Z -    = note: for more information, try `rustc --explain E0729`
2019-08-14T18:55:00.4441564Z 360 
2019-08-14T18:55:00.4441564Z 360 
2019-08-14T18:55:00.4441604Z 361 error[E0382]: use of moved value: `x`
2019-08-14T18:55:00.4441937Z 
2019-08-14T18:55:00.4442003Z 368    |
2019-08-14T18:55:00.4442003Z 368    |
2019-08-14T18:55:00.4442052Z 369    = note: move occurs because `x` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
2019-08-14T18:55:00.4442359Z - error: aborting due to 30 previous errors
2019-08-14T18:55:00.4442406Z + error: aborting due to 32 previous errors
2019-08-14T18:55:00.4442446Z 372 
2019-08-14T18:55:00.4442497Z 373 Some errors have detailed explanations: E0382, E0499, E0502, E0503.
2019-08-14T18:55:00.4442497Z 373 Some errors have detailed explanations: E0382, E0499, E0502, E0503.
2019-08-14T18:55:00.4442873Z 374 For more information about an error, try `rustc --explain E0382`.
2019-08-14T18:55:00.4442907Z 
2019-08-14T18:55:00.4442931Z 
2019-08-14T18:55:00.4443389Z The actual stderr differed from the expected stderr.
2019-08-14T18:55:00.4443796Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue/borrowck-describe-lvalue.stderr
2019-08-14T18:55:00.4444061Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T18:55:00.4444353Z To only update this specific test, also pass `--test-args borrowck/borrowck-describe-lvalue.rs`
2019-08-14T18:55:00.4444433Z error: 1 errors occurred comparing output.
2019-08-14T18:55:00.4444477Z status: exit code: 1
2019-08-14T18:55:00.4444477Z status: exit code: 1
2019-08-14T18:55:00.4445238Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue/auxiliary" "-A" "unused"
2019-08-14T18:55:00.4445719Z ------------------------------------------
2019-08-14T18:55:00.4445763Z 
2019-08-14T18:55:00.4445988Z ------------------------------------------
2019-08-14T18:55:00.4446050Z stderr:
2019-08-14T18:55:00.4446050Z stderr:
2019-08-14T18:55:00.4446267Z ------------------------------------------
2019-08-14T18:55:00.4446319Z error[E0499]: cannot borrow `x` as mutable more than once at a time
2019-08-14T18:55:00.4446893Z    |
2019-08-14T18:55:00.4446893Z    |
2019-08-14T18:55:00.4446933Z LL |             let y = &mut x;
2019-08-14T18:55:00.4447175Z    |                     ------ first mutable borrow occurs here
2019-08-14T18:55:00.4447229Z LL |             &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
2019-08-14T18:55:00.4447278Z    |             ^^^^^^ second mutable borrow occurs here
2019-08-14T18:55:00.4447342Z LL |             *y = 1;
2019-08-14T18:55:00.4447557Z    |             ------ first borrow later used here
2019-08-14T18:55:00.4447588Z 
2019-08-14T18:55:00.4447639Z error[E0499]: cannot borrow `x` as mutable more than once at a time
2019-08-14T18:55:00.4447934Z    |
2019-08-14T18:55:00.4447934Z    |
2019-08-14T18:55:00.4447973Z LL |                    let y = &mut x;
2019-08-14T18:55:00.4448215Z    |                            ------ first mutable borrow occurs here
2019-08-14T18:55:00.4448269Z LL |                    &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
2019-08-14T18:55:00.4448326Z    |                    ^^^^^^ second mutable borrow occurs here
2019-08-14T18:55:00.4448385Z LL |                    *y = 1;
2019-08-14T18:55:00.4448605Z    |                    ------ first borrow later used here
2019-08-14T18:55:00.4448637Z 
2019-08-14T18:55:00.4448679Z error: captured variable cannot escape `FnMut` closure body
2019-08-14T18:55:00.4448971Z    |
2019-08-14T18:55:00.4449009Z LL |              || {
2019-08-14T18:55:00.4449009Z LL |              || {
2019-08-14T18:55:00.4449307Z    |               - inferred to be a `FnMut` closure
2019-08-14T18:55:00.4449367Z LL | /                || { //~ ERROR captured variable cannot escape `FnMut` closure body
2019-08-14T18:55:00.4449414Z LL | |                    let y = &mut x;
2019-08-14T18:55:00.4449481Z LL | |                    &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
2019-08-14T18:55:00.4449527Z LL | |                    *y = 1;
2019-08-14T18:55:00.4449575Z LL | |                    drop(y);
2019-08-14T18:55:00.4449631Z LL | |                 }
2019-08-14T18:55:00.4449683Z    | |_________________^ returns a closure that contains a reference to a captured variable, which then escapes the closure body
2019-08-14T18:55:00.4449731Z    |
2019-08-14T18:55:00.4449797Z    = note: `FnMut` closures only have access to their captured variables while they are executing...
2019-08-14T18:55:00.4449850Z    = note: ...therefore, they cannot allow references to captured variables to escape
2019-08-14T18:55:00.4449888Z 
2019-08-14T18:55:00.4449945Z error[E0503]: cannot use `f.x` because it was mutably borrowed
2019-08-14T18:55:00.4450261Z    |
2019-08-14T18:55:00.4450261Z    |
2019-08-14T18:55:00.4450299Z LL |         let x = f.x();
2019-08-14T18:55:00.4450528Z    |                 - borrow of `f` occurs here
2019-08-14T18:55:00.4450579Z LL |         f.x; //~ ERROR cannot use `f.x` because it was mutably borrowed
2019-08-14T18:55:00.4450695Z    |         ^^^ use of borrowed `f`
2019-08-14T18:55:00.4450754Z LL |         drop(x);
2019-08-14T18:55:00.4450986Z    |              - borrow later used here
2019-08-14T18:55:00.4451017Z 
2019-08-14T18:55:00.4451076Z error[E0503]: cannot use `g.0` because it was mutably borrowed
2019-08-14T18:55:00.4451553Z    |
2019-08-14T18:55:00.4451591Z LL |         let x = g.x();
2019-08-14T18:55:00.4451876Z    |                 - borrow of `g` occurs here
2019-08-14T18:55:00.4451876Z    |                 - borrow of `g` occurs here
2019-08-14T18:55:00.4451928Z LL |         g.0; //~ ERROR cannot use `g.0` because it was mutably borrowed
2019-08-14T18:55:00.4451972Z    |         ^^^ use of borrowed `g`
2019-08-14T18:55:00.4452028Z LL |         drop(x);
2019-08-14T18:55:00.4452237Z    |              - borrow later used here
2019-08-14T18:55:00.4452267Z 
2019-08-14T18:55:00.4452309Z error[E0503]: cannot use `h.0` because it was mutably borrowed
2019-08-14T18:55:00.4452613Z    |
2019-08-14T18:55:00.4452767Z LL |         let x = &mut h.0;
2019-08-14T18:55:00.4452767Z LL |         let x = &mut h.0;
2019-08-14T18:55:00.4453413Z    |                 -------- borrow of `h.0` occurs here
2019-08-14T18:55:00.4453479Z LL |         h.0; //~ ERROR cannot use `h.0` because it was mutably borrowed
2019-08-14T18:55:00.4453527Z    |         ^^^ use of borrowed `h.0`
2019-08-14T18:55:00.4453590Z LL |         drop(x);
2019-08-14T18:55:00.4453862Z    |              - borrow later used here
2019-08-14T18:55:00.4453896Z 
2019-08-14T18:55:00.4453941Z error[E0503]: cannot use `e.0` because it was mutably borrowed
2019-08-14T18:55:00.4454264Z    |
2019-08-14T18:55:00.4454264Z    |
2019-08-14T18:55:00.4454304Z LL |         let x = e.x();
2019-08-14T18:55:00.4454545Z    |                 - borrow of `e` occurs here
2019-08-14T18:55:00.4454602Z LL |         match e {
2019-08-14T18:55:00.4454652Z LL |             Baz::X(value) => value //~ ERROR cannot use `e.0` because it was mutably borrowed
2019-08-14T18:55:00.4454721Z    |                    ^^^^^ use of borrowed `e`
2019-08-14T18:55:00.4454808Z LL |         drop(x);
2019-08-14T18:55:00.4454808Z LL |         drop(x);
2019-08-14T18:55:00.4455045Z    |              - borrow later used here
2019-08-14T18:55:00.4455077Z 
2019-08-14T18:55:00.4455122Z error[E0503]: cannot use `u.a` because it was mutably borrowed
2019-08-14T18:55:00.4456837Z    |
2019-08-14T18:55:00.4456837Z    |
2019-08-14T18:55:00.4456878Z LL |         let x = &mut u.a;
2019-08-14T18:55:00.4457485Z    |                 -------- borrow of `u.a` occurs here
2019-08-14T18:55:00.4457569Z LL |         u.a; //~ ERROR cannot use `u.a` because it was mutably borrowed
2019-08-14T18:55:00.4457618Z    |         ^^^ use of borrowed `u.a`
2019-08-14T18:55:00.4457661Z LL |         drop(x);
2019-08-14T18:55:00.4458154Z    |              - borrow later used here
2019-08-14T18:55:00.4458197Z 
2019-08-14T18:55:00.4458244Z error[E0503]: cannot use `f.x` because it was mutably borrowed
2019-08-14T18:55:00.4458562Z    |
2019-08-14T18:55:00.4458562Z    |
2019-08-14T18:55:00.4458764Z LL |         let x = f.x();
2019-08-14T18:55:00.4459165Z    |                 - borrow of `*f` occurs here
2019-08-14T18:55:00.4459243Z LL |         f.x; //~ ERROR cannot use `f.x` because it was mutably borrowed
2019-08-14T18:55:00.4459570Z    |         ^^^ use of borrowed `*f`
2019-08-14T18:55:00.4459620Z LL |         drop(x);
2019-08-14T18:55:00.4459918Z    |              - borrow later used here
2019-08-14T18:55:00.4459951Z 
2019-08-14T18:55:00.4459997Z error[E0503]: cannot use `g.0` because it was mutably borrowed
2019-08-14T18:55:00.4460510Z    |
2019-08-14T18:55:00.4460670Z LL |         let x = g.x();
2019-08-14T18:55:00.4460670Z LL |         let x = g.x();
2019-08-14T18:55:00.4461112Z    |                 - borrow of `*g` occurs here
2019-08-14T18:55:00.4461222Z LL |         g.0; //~ ERROR cannot use `g.0` because it was mutably borrowed
2019-08-14T18:55:00.4461270Z    |         ^^^ use of borrowed `*g`
2019-08-14T18:55:00.4461360Z LL |         drop(x);
2019-08-14T18:55:00.4461657Z    |              - borrow later used here
2019-08-14T18:55:00.4461690Z 
2019-08-14T18:55:00.4461736Z error[E0503]: cannot use `h.0` because it was mutably borrowed
2019-08-14T18:55:00.4500405Z    |
2019-08-14T18:55:00.4500451Z LL |         let x = &mut h.0;
2019-08-14T18:55:00.4500451Z LL |         let x = &mut h.0;
2019-08-14T18:55:00.4501071Z    |                 -------- borrow of `h.0` occurs here
2019-08-14T18:55:00.4501160Z LL |         h.0; //~ ERROR cannot use `h.0` because it was mutably borrowed
2019-08-14T18:55:00.4501212Z    |         ^^^ use of borrowed `h.0`
2019-08-14T18:55:00.4501270Z LL |         drop(x);
2019-08-14T18:55:00.4501529Z    |              - borrow later used here
2019-08-14T18:55:00.4501564Z 
2019-08-14T18:55:00.4501609Z error[E0503]: cannot use `e.0` because it was mutably borrowed
2019-08-14T18:55:00.4501920Z    |
2019-08-14T18:55:00.4501920Z    |
2019-08-14T18:55:00.4501963Z LL |         let x = e.x();
2019-08-14T18:55:00.4502190Z    |                 - borrow of `*e` occurs here
2019-08-14T18:55:00.4502246Z LL |         match *e {
2019-08-14T18:55:00.4502298Z LL |             Baz::X(value) => value
2019-08-14T18:55:00.4502344Z    |                    ^^^^^ use of borrowed `*e`
2019-08-14T18:55:00.4502435Z LL |         drop(x);
2019-08-14T18:55:00.4502435Z LL |         drop(x);
2019-08-14T18:55:00.4502660Z    |              - borrow later used here
2019-08-14T18:55:00.4502692Z 
2019-08-14T18:55:00.4502745Z error[E0503]: cannot use `u.a` because it was mutably borrowed
2019-08-14T18:55:00.4503304Z    |
2019-08-14T18:55:00.4503304Z    |
2019-08-14T18:55:00.4503363Z LL |         let x = &mut u.a;
2019-08-14T18:55:00.4503652Z    |                 -------- borrow of `u.a` occurs here
2019-08-14T18:55:00.4503707Z LL |         u.a; //~ ERROR cannot use `u.a` because it was mutably borrowed
2019-08-14T18:55:00.4503772Z    |         ^^^ use of borrowed `u.a`
2019-08-14T18:55:00.4503815Z LL |         drop(x);
2019-08-14T18:55:00.4504035Z    |              - borrow later used here
2019-08-14T18:55:00.4504067Z 
2019-08-14T18:55:00.4504275Z error[E0503]: cannot use `v[..]` because it was mutably borrowed
2019-08-14T18:55:00.4511178Z    |
2019-08-14T18:55:00.4511261Z LL |         let x = &mut v;
2019-08-14T18:55:00.4511261Z LL |         let x = &mut v;
2019-08-14T18:55:00.4511599Z    |                 ------ borrow of `v` occurs here
2019-08-14T18:55:00.4511650Z LL |         match v {
2019-08-14T18:55:00.4511712Z LL |             &[x, _, .., _, _] => println!("{}", x),
2019-08-14T18:55:00.4511779Z    |               ^ use of borrowed `v`
2019-08-14T18:55:00.4511863Z LL |         drop(x);
2019-08-14T18:55:00.4511863Z LL |         drop(x);
2019-08-14T18:55:00.4512202Z    |              - borrow later used here
2019-08-14T18:55:00.4512238Z 
2019-08-14T18:55:00.4512285Z error[E0503]: cannot use `v[..]` because it was mutably borrowed
2019-08-14T18:55:00.4512602Z    |
2019-08-14T18:55:00.4512644Z LL |         let x = &mut v;
2019-08-14T18:55:00.4512644Z LL |         let x = &mut v;
2019-08-14T18:55:00.4512897Z    |                 ------ borrow of `v` occurs here
2019-08-14T18:55:00.4513206Z ...
2019-08-14T18:55:00.4513251Z LL |             &[_, x, .., _, _] => println!("{}", x),
2019-08-14T18:55:00.4513298Z    |                  ^ use of borrowed `v`
2019-08-14T18:55:00.4513392Z LL |         drop(x);
2019-08-14T18:55:00.4513392Z LL |         drop(x);
2019-08-14T18:55:00.4513688Z    |              - borrow later used here
2019-08-14T18:55:00.4513913Z 
2019-08-14T18:55:00.4513961Z error[E0503]: cannot use `v[..]` because it was mutably borrowed
2019-08-14T18:55:00.4514316Z    |
2019-08-14T18:55:00.4514368Z LL |         let x = &mut v;
2019-08-14T18:55:00.4514368Z LL |         let x = &mut v;
2019-08-14T18:55:00.4514602Z    |                 ------ borrow of `v` occurs here
2019-08-14T18:55:00.4514649Z ...
2019-08-14T18:55:00.4514708Z LL |             &[_, _, .., x, _] => println!("{}", x),
2019-08-14T18:55:00.4514755Z    |                         ^ use of borrowed `v`
2019-08-14T18:55:00.4514864Z LL |         drop(x);
2019-08-14T18:55:00.4514864Z LL |         drop(x);
2019-08-14T18:55:00.4515089Z    |              - borrow later used here
2019-08-14T18:55:00.4515121Z 
2019-08-14T18:55:00.4515167Z error[E0503]: cannot use `v[..]` because it was mutably borrowed
2019-08-14T18:55:00.4515479Z    |
2019-08-14T18:55:00.4515530Z LL |         let x = &mut v;
2019-08-14T18:55:00.4515530Z LL |         let x = &mut v;
2019-08-14T18:55:00.4515771Z    |                 ------ borrow of `v` occurs here
2019-08-14T18:55:00.4515816Z ...
2019-08-14T18:55:00.4515859Z LL |             &[_, _, .., _, x] => println!("{}", x),
2019-08-14T18:55:00.4515917Z    |                            ^ use of borrowed `v`
2019-08-14T18:55:00.4515998Z LL |         drop(x);
2019-08-14T18:55:00.4515998Z LL |         drop(x);
2019-08-14T18:55:00.4516217Z    |              - borrow later used here
2019-08-14T18:55:00.4516259Z 
2019-08-14T18:55:00.4516313Z error[E0503]: cannot use `v[..]` because it was mutably borrowed
2019-08-14T18:55:00.4516626Z    |
2019-08-14T18:55:00.4516667Z LL |         let x = &mut v;
2019-08-14T18:55:00.4516667Z LL |         let x = &mut v;
2019-08-14T18:55:00.4516898Z    |                 ------ borrow of `v` occurs here
2019-08-14T18:55:00.4516945Z LL |         match v {
2019-08-14T18:55:00.4517005Z LL |             &[x @ ..] => println!("{:?}", x),
2019-08-14T18:55:00.4517059Z    |               ^^^^^^ use of borrowed `v`
2019-08-14T18:55:00.4517156Z LL |         drop(x);
2019-08-14T18:55:00.4517156Z LL |         drop(x);
2019-08-14T18:55:00.4517379Z    |              - borrow later used here
2019-08-14T18:55:00.4517411Z 
2019-08-14T18:55:00.4517456Z error[E0503]: cannot use `v[..]` because it was mutably borrowed
2019-08-14T18:55:00.4517768Z    |
2019-08-14T18:55:00.4517809Z LL |         let x = &mut v;
2019-08-14T18:55:00.4517809Z LL |         let x = &mut v;
2019-08-14T18:55:00.4518146Z    |                 ------ borrow of `v` occurs here
2019-08-14T18:55:00.4518201Z ...
2019-08-14T18:55:00.4518244Z LL |             &[_, x @ ..] => println!("{:?}", x),
2019-08-14T18:55:00.4518432Z    |                  ^^^^^^ use of borrowed `v`
2019-08-14T18:55:00.4518512Z LL |         drop(x);
2019-08-14T18:55:00.4518512Z LL |         drop(x);
2019-08-14T18:55:00.4518765Z    |              - borrow later used here
2019-08-14T18:55:00.4518811Z 
2019-08-14T18:55:00.4518864Z error[E0503]: cannot use `v[..]` because it was mutably borrowed
2019-08-14T18:55:00.4527351Z    |
2019-08-14T18:55:00.4527402Z LL |         let x = &mut v;
2019-08-14T18:55:00.4527402Z LL |         let x = &mut v;
2019-08-14T18:55:00.4527915Z    |                 ------ borrow of `v` occurs here
2019-08-14T18:55:00.4527984Z ...
2019-08-14T18:55:00.4528027Z LL |             &[x @ .., _] => println!("{:?}", x),
2019-08-14T18:55:00.4528072Z    |               ^^^^^^ use of borrowed `v`
2019-08-14T18:55:00.4528180Z LL |         drop(x);
2019-08-14T18:55:00.4528180Z LL |         drop(x);
2019-08-14T18:55:00.4528412Z    |              - borrow later used here
2019-08-14T18:55:00.4528445Z 
2019-08-14T18:55:00.4528503Z error[E0503]: cannot use `v[..]` because it was mutably borrowed
2019-08-14T18:55:00.4528802Z    |
2019-08-14T18:55:00.4528842Z LL |         let x = &mut v;
2019-08-14T18:55:00.4528842Z LL |         let x = &mut v;
2019-08-14T18:55:00.4529284Z    |                 ------ borrow of `v` occurs here
2019-08-14T18:55:00.4529328Z ...
2019-08-14T18:55:00.4529370Z LL |             &[_, x @ .., _] => println!("{:?}", x),
2019-08-14T18:55:00.4529432Z    |                  ^^^^^^ use of borrowed `v`
2019-08-14T18:55:00.4529510Z LL |         drop(x);
2019-08-14T18:55:00.4529510Z LL |         drop(x);
2019-08-14T18:55:00.4529740Z    |              - borrow later used here
2019-08-14T18:55:00.4529773Z 
2019-08-14T18:55:00.4529816Z error[E0503]: cannot use `e` because it was mutably borrowed
2019-08-14T18:55:00.4530130Z    |
2019-08-14T18:55:00.4530171Z LL |         let x = &mut e;
2019-08-14T18:55:00.4530398Z    |                 ------ borrow of `e` occurs here
2019-08-14T18:55:00.4530454Z LL |         match e {
2019-08-14T18:55:00.4530454Z LL |         match e {
2019-08-14T18:55:00.4530496Z LL |             E::A(ref ax) =>
2019-08-14T18:55:00.4530543Z    |             ^^^^^^^^^^^^ use of borrowed `e`
2019-08-14T18:55:00.4530761Z LL |         drop(x);
2019-08-14T18:55:00.4530761Z LL |         drop(x);
2019-08-14T18:55:00.4530988Z    |              - borrow later used here
2019-08-14T18:55:00.4531019Z 
2019-08-14T18:55:00.4531078Z error[E0502]: cannot borrow `e.0` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4531883Z    |
2019-08-14T18:55:00.4531942Z LL |         let x = &mut e;
2019-08-14T18:55:00.4532232Z    |                 ------ mutable borrow occurs here
2019-08-14T18:55:00.4532291Z LL |         match e {
2019-08-14T18:55:00.4532291Z LL |         match e {
2019-08-14T18:55:00.4532333Z LL |             E::A(ref ax) =>
2019-08-14T18:55:00.4532395Z    |                  ^^^^^^ immutable borrow occurs here
2019-08-14T18:55:00.4532476Z LL |         drop(x);
2019-08-14T18:55:00.4532476Z LL |         drop(x);
2019-08-14T18:55:00.4532725Z    |              - mutable borrow later used here
2019-08-14T18:55:00.4532758Z 
2019-08-14T18:55:00.4532806Z error[E0502]: cannot borrow `e.x` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4533456Z    |
2019-08-14T18:55:00.4533497Z LL |         let x = &mut e;
2019-08-14T18:55:00.4533734Z    |                 ------ mutable borrow occurs here
2019-08-14T18:55:00.4533796Z ...
2019-08-14T18:55:00.4533796Z ...
2019-08-14T18:55:00.4533837Z LL |             E::B { x: ref bx } =>
2019-08-14T18:55:00.4533885Z    |                       ^^^^^^ immutable borrow occurs here
2019-08-14T18:55:00.4534139Z LL |         drop(x);
2019-08-14T18:55:00.4534139Z LL |         drop(x);
2019-08-14T18:55:00.4534419Z    |              - mutable borrow later used here
2019-08-14T18:55:00.4534454Z 
2019-08-14T18:55:00.4534517Z error[E0502]: cannot borrow `s.y.0` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4534819Z    |
2019-08-14T18:55:00.4534872Z LL |         let x = &mut s;
2019-08-14T18:55:00.4535113Z    |                 ------ mutable borrow occurs here
2019-08-14T18:55:00.4535161Z LL |         match s {
2019-08-14T18:55:00.4535161Z LL |         match s {
2019-08-14T18:55:00.4535216Z LL |             S  { y: (ref y0, _), .. } =>
2019-08-14T18:55:00.4535264Z    |                      ^^^^^^ immutable borrow occurs here
2019-08-14T18:55:00.4535346Z LL |         drop(x);
2019-08-14T18:55:00.4535346Z LL |         drop(x);
2019-08-14T18:55:00.4535584Z    |              - mutable borrow later used here
2019-08-14T18:55:00.4535616Z 
2019-08-14T18:55:00.4535672Z error[E0502]: cannot borrow `s.x.y` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4535983Z    |
2019-08-14T18:55:00.4536024Z LL |         let x = &mut s;
2019-08-14T18:55:00.4536267Z    |                 ------ mutable borrow occurs here
2019-08-14T18:55:00.4536313Z ...
2019-08-14T18:55:00.4536313Z ...
2019-08-14T18:55:00.4536356Z LL |             S  { x: F { y: ref x0, .. }, .. } =>
2019-08-14T18:55:00.4536496Z    |                            ^^^^^^ immutable borrow occurs here
2019-08-14T18:55:00.4536595Z LL |         drop(x);
2019-08-14T18:55:00.4536595Z LL |         drop(x);
2019-08-14T18:55:00.4536856Z    |              - mutable borrow later used here
2019-08-14T18:55:00.4536904Z 
2019-08-14T18:55:00.4536948Z error[E0503]: cannot use `*v` because it was mutably borrowed
2019-08-14T18:55:00.4537377Z    |
2019-08-14T18:55:00.4537417Z LL |         let x = &mut v;
2019-08-14T18:55:00.4537417Z LL |         let x = &mut v;
2019-08-14T18:55:00.4537764Z    |                 ------ borrow of `v` occurs here
2019-08-14T18:55:00.4537809Z LL |         v[0].y;
2019-08-14T18:55:00.4537861Z    |         ^^^^ use of borrowed `v`
2019-08-14T18:55:00.4537936Z LL |         drop(x);
2019-08-14T18:55:00.4537936Z LL |         drop(x);
2019-08-14T18:55:00.4538156Z    |              - borrow later used here
2019-08-14T18:55:00.4538187Z 
2019-08-14T18:55:00.4538229Z error[E0503]: cannot use `v[_].y` because it was mutably borrowed
2019-08-14T18:55:00.4538526Z    |
2019-08-14T18:55:00.4538564Z LL |         let x = &mut v;
2019-08-14T18:55:00.4538564Z LL |         let x = &mut v;
2019-08-14T18:55:00.4538775Z    |                 ------ borrow of `v` occurs here
2019-08-14T18:55:00.4538835Z LL |         v[0].y;
2019-08-14T18:55:00.4538876Z    |         ^^^^^^ use of borrowed `v`
2019-08-14T18:55:00.4538966Z LL |         drop(x);
2019-08-14T18:55:00.4538966Z LL |         drop(x);
2019-08-14T18:55:00.4539168Z    |              - borrow later used here
2019-08-14T18:55:00.4539197Z 
2019-08-14T18:55:00.4539250Z error[E0502]: cannot borrow `v[..].x` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4539547Z    |
2019-08-14T18:55:00.4539583Z LL |         let x = &mut v;
2019-08-14T18:55:00.4539811Z    |                 ------ mutable borrow occurs here
2019-08-14T18:55:00.4539855Z LL |         match v {
2019-08-14T18:55:00.4539855Z LL |         match v {
2019-08-14T18:55:00.4539906Z LL |             &[_, F {x: ref xf, ..}] => println!("{}", xf),
2019-08-14T18:55:00.4539966Z    |                        ^^^^^^ immutable borrow occurs here
2019-08-14T18:55:00.4540044Z LL |         drop(x);
2019-08-14T18:55:00.4540044Z LL |         drop(x);
2019-08-14T18:55:00.4540256Z    |              - mutable borrow later used here
2019-08-14T18:55:00.4540301Z 
2019-08-14T18:55:00.4540347Z error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4540723Z    |
2019-08-14T18:55:00.4540763Z LL |             let x = &mut block;
2019-08-14T18:55:00.4541018Z    |                     ---------- mutable borrow occurs here
2019-08-14T18:55:00.4541018Z    |                     ---------- mutable borrow occurs here
2019-08-14T18:55:00.4541236Z LL |             let p: &'a u8 = &*block.current;
2019-08-14T18:55:00.4541303Z    |                             ^^^^^^^^^^^^^^^ immutable borrow occurs here
2019-08-14T18:55:00.4541390Z LL |             drop(x);
2019-08-14T18:55:00.4541390Z LL |             drop(x);
2019-08-14T18:55:00.4541623Z    |                  - mutable borrow later used here
2019-08-14T18:55:00.4541653Z 
2019-08-14T18:55:00.4541698Z error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4541990Z    |
2019-08-14T18:55:00.4542029Z LL |             let x = &mut block;
2019-08-14T18:55:00.4542251Z    |                     ---------- mutable borrow occurs here
2019-08-14T18:55:00.4542251Z    |                     ---------- mutable borrow occurs here
2019-08-14T18:55:00.4542319Z LL |             let p : *const u8 = &*(*block).current;
2019-08-14T18:55:00.4542366Z    |                                 ^^^^^^^^^^^^^^^^^^ immutable borrow occurs here
2019-08-14T18:55:00.4542455Z LL |             drop(x);
2019-08-14T18:55:00.4542455Z LL |             drop(x);
---
2019-08-14T18:55:00.4559843Z 35 For more information about an error, try `rustc --explain E0716`.
2019-08-14T18:55:00.4559993Z 
2019-08-14T18:55:00.4560019Z 
2019-08-14T18:55:00.4560146Z The actual stderr differed from the expected stderr.
2019-08-14T18:55:00.4560635Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn/min_const_fn_dyn.stderr
2019-08-14T18:55:00.4561002Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T18:55:00.4561281Z To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn_dyn.rs`
2019-08-14T18:55:00.4561373Z error: 1 errors occurred comparing output.
2019-08-14T18:55:00.4561424Z status: exit code: 1
2019-08-14T18:55:00.4561424Z status: exit code: 1
2019-08-14T18:55:00.4562206Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn/auxiliary" "-A" "unused"
2019-08-14T18:55:00.4562547Z ------------------------------------------
2019-08-14T18:55:00.4562580Z 
2019-08-14T18:55:00.4562805Z ------------------------------------------
2019-08-14T18:55:00.4562864Z stderr:
2019-08-14T18:55:00.4562864Z stderr:
2019-08-14T18:55:00.4565587Z ------------------------------------------
2019-08-14T18:55:00.4565672Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-14T18:55:00.4566018Z    |
2019-08-14T18:55:00.4566060Z LL |     x.0.field;
2019-08-14T18:55:00.4566101Z    |     ^^^^^^^^^
2019-08-14T18:55:00.4566157Z    |
2019-08-14T18:55:00.4566157Z    |
2019-08-14T18:55:00.4566477Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4566561Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4566594Z 
2019-08-14T18:55:00.4566641Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-14T18:55:00.4567262Z    |
2019-08-14T18:55:00.4567262Z    |
2019-08-14T18:55:00.4567522Z LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }
2019-08-14T18:55:00.4567754Z    |
2019-08-14T18:55:00.4567754Z    |
2019-08-14T18:55:00.4568114Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4568176Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4568227Z 
2019-08-14T18:55:00.4568273Z error[E0716]: temporary value dropped while borrowed
2019-08-14T18:55:00.4570793Z    |
2019-08-14T18:55:00.4570793Z    |
2019-08-14T18:55:00.4571252Z LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }
2019-08-14T18:55:00.4571859Z    |                                                                  -^    - temporary value is freed at the end of this statement
2019-08-14T18:55:00.4571945Z    |                                                                  ||
2019-08-14T18:55:00.4572008Z    |                                                                  |creates a temporary which is freed while still in use
2019-08-14T18:55:00.4572331Z    |                                                                  cast requires that borrow lasts for `'static`
2019-08-14T18:55:00.4572430Z error: aborting due to 3 previous errors
2019-08-14T18:55:00.4572459Z 
2019-08-14T18:55:00.4572502Z Some errors have detailed explanations: E0716, E0723.
2019-08-14T18:55:00.4573038Z For more information about an error, try `rustc --explain E0716`.
2019-08-14T18:55:00.4573038Z For more information about an error, try `rustc --explain E0716`.
2019-08-14T18:55:00.4573238Z 
2019-08-14T18:55:00.4573536Z ------------------------------------------
2019-08-14T18:55:00.4573569Z 
2019-08-14T18:55:00.4573612Z 
2019-08-14T18:55:00.4573854Z ---- [ui] ui/consts/min_const_fn/min_const_fn.rs stdout ----
2019-08-14T18:55:00.4573902Z diff of stderr:
2019-08-14T18:55:00.4573930Z 
2019-08-14T18:55:00.4574265Z 286    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4574326Z 287    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4574682Z - warning[E0515]: cannot return reference to temporary value
2019-08-14T18:55:00.4574740Z + error[E0515]: cannot return reference to temporary value
2019-08-14T18:55:00.4574989Z 290   --> $DIR/min_const_fn.rs:137:63
2019-08-14T18:55:00.4575051Z 291    |
2019-08-14T18:55:00.4575051Z 291    |
2019-08-14T18:55:00.4575330Z 292 LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
2019-08-14T18:55:00.4575428Z 294    |                                                               ||
2019-08-14T18:55:00.4575495Z 295    |                                                               |temporary value created here
2019-08-14T18:55:00.4575557Z 296    |                                                               returns a reference to data owned by the current function
2019-08-14T18:55:00.4575792Z -    |
2019-08-14T18:55:00.4575792Z -    |
2019-08-14T18:55:00.4576102Z -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
2019-08-14T18:55:00.4576666Z -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
2019-08-14T18:55:00.4576919Z -    = note: for more information, try `rustc --explain E0729`
2019-08-14T18:55:00.4576963Z 301 
2019-08-14T18:55:00.4577009Z 302 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-14T18:55:00.4577264Z 
2019-08-14T18:55:00.4577264Z 
2019-08-14T18:55:00.4577550Z 326    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4577620Z 327    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4577903Z - error: aborting due to 36 previous errors
2019-08-14T18:55:00.4577952Z + error: aborting due to 37 previous errors
2019-08-14T18:55:00.4578008Z 330 
2019-08-14T18:55:00.4578053Z 331 Some errors have detailed explanations: E0515, E0723.
2019-08-14T18:55:00.4578053Z 331 Some errors have detailed explanations: E0515, E0723.
2019-08-14T18:55:00.4578313Z 332 For more information about an error, try `rustc --explain E0515`.
2019-08-14T18:55:00.4578359Z 
2019-08-14T18:55:00.4578478Z 
2019-08-14T18:55:00.4578529Z The actual stderr differed from the expected stderr.
2019-08-14T18:55:00.4578875Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/min_const_fn.stderr
2019-08-14T18:55:00.4579153Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T18:55:00.4579433Z To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn.rs`
2019-08-14T18:55:00.4579535Z error: 1 errors occurred comparing output.
2019-08-14T18:55:00.4579578Z status: exit code: 1
2019-08-14T18:55:00.4579578Z status: exit code: 1
2019-08-14T18:55:00.4580308Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/auxiliary" "-A" "unused"
2019-08-14T18:55:00.4580645Z ------------------------------------------
2019-08-14T18:55:00.4580813Z 
2019-08-14T18:55:00.4581040Z ------------------------------------------
2019-08-14T18:55:00.4581166Z stderr:
2019-08-14T18:55:00.4581166Z stderr:
2019-08-14T18:55:00.4581408Z ------------------------------------------
2019-08-14T18:55:00.4581785Z error[E0493]: destructors cannot be evaluated at compile-time
2019-08-14T18:55:00.4582044Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:37:25
2019-08-14T18:55:00.4582096Z    |
2019-08-14T18:55:00.4582377Z LL |     const fn into_inner(self) -> T { self.0 } //~ destructors cannot be evaluated
2019-08-14T18:55:00.4582436Z    |                         ^^^^ constant functions cannot evaluate destructors
2019-08-14T18:55:00.4582540Z error[E0723]: mutable references in const fn are unstable
2019-08-14T18:55:00.4582797Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:39:36
2019-08-14T18:55:00.4582846Z    |
2019-08-14T18:55:00.4582846Z    |
2019-08-14T18:55:00.4583374Z LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
2019-08-14T18:55:00.4583492Z    |
2019-08-14T18:55:00.4583492Z    |
2019-08-14T18:55:00.4583848Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4583909Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4584288Z error[E0493]: destructors cannot be evaluated at compile-time
2019-08-14T18:55:00.4584573Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:44:28
2019-08-14T18:55:00.4584625Z    |
2019-08-14T18:55:00.4584625Z    |
2019-08-14T18:55:00.4584913Z LL |     const fn into_inner_lt(self) -> T { self.0 } //~ destructors cannot be evaluated
2019-08-14T18:55:00.4584999Z    |                            ^^^^ constant functions cannot evaluate destructors
2019-08-14T18:55:00.4585080Z error[E0723]: mutable references in const fn are unstable
2019-08-14T18:55:00.4585372Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:46:42
2019-08-14T18:55:00.4585424Z    |
2019-08-14T18:55:00.4585424Z    |
2019-08-14T18:55:00.4585690Z LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
2019-08-14T18:55:00.4585818Z    |
2019-08-14T18:55:00.4585818Z    |
2019-08-14T18:55:00.4586502Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4586579Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4586888Z error[E0493]: destructors cannot be evaluated at compile-time
2019-08-14T18:55:00.4587180Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:51:27
2019-08-14T18:55:00.4587351Z    |
2019-08-14T18:55:00.4587351Z    |
2019-08-14T18:55:00.4587956Z LL |     const fn into_inner_s(self) -> T { self.0 } //~ ERROR destructors
2019-08-14T18:55:00.4588040Z    |                           ^^^^ constant functions cannot evaluate destructors
2019-08-14T18:55:00.4588115Z error[E0723]: mutable references in const fn are unstable
2019-08-14T18:55:00.4588521Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:53:38
2019-08-14T18:55:00.4588596Z    |
2019-08-14T18:55:00.4588596Z    |
2019-08-14T18:55:00.4588840Z LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
2019-08-14T18:55:00.4588949Z    |
2019-08-14T18:55:00.4588949Z    |
2019-08-14T18:55:00.4589246Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4589302Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4589394Z error[E0723]: mutable references in const fn are unstable
2019-08-14T18:55:00.4589666Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:58:39
2019-08-14T18:55:00.4589730Z    |
2019-08-14T18:55:00.4589730Z    |
2019-08-14T18:55:00.4589974Z LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
2019-08-14T18:55:00.4590067Z    |
2019-08-14T18:55:00.4590067Z    |
2019-08-14T18:55:00.4590369Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4590426Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4590549Z 
2019-08-14T18:55:00.4590617Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-14T18:55:00.4590986Z    |
2019-08-14T18:55:00.4590986Z    |
2019-08-14T18:55:00.4591259Z LL | const fn foo11<T: std::fmt::Display>(t: T) -> T { t }
2019-08-14T18:55:00.4591350Z    |
2019-08-14T18:55:00.4591350Z    |
2019-08-14T18:55:00.4591681Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4591748Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4591781Z 
2019-08-14T18:55:00.4591845Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-14T18:55:00.4592183Z    |
2019-08-14T18:55:00.4592183Z    |
2019-08-14T18:55:00.4592499Z LL | const fn foo11_2<T: Send>(t: T) -> T { t }
2019-08-14T18:55:00.4592603Z    |
2019-08-14T18:55:00.4592603Z    |
2019-08-14T18:55:00.4593141Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4593222Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4593254Z 
2019-08-14T18:55:00.4593301Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-08-14T18:55:00.4593696Z    |
2019-08-14T18:55:00.4593696Z    |
2019-08-14T18:55:00.4593939Z LL | const fn foo19(f: f32) -> f32 { f * 2.0 }
2019-08-14T18:55:00.4594052Z    |
2019-08-14T18:55:00.4594052Z    |
2019-08-14T18:55:00.4594354Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4594423Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4594455Z 
2019-08-14T18:55:00.4594501Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-08-14T18:55:00.4594841Z    |
2019-08-14T18:55:00.4594841Z    |
2019-08-14T18:55:00.4595072Z LL | const fn foo19_2(f: f32) -> f32 { 2.0 - f }
2019-08-14T18:55:00.4595176Z    |
2019-08-14T18:55:00.4595176Z    |
2019-08-14T18:55:00.4595461Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4595532Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4595564Z 
2019-08-14T18:55:00.4595716Z error[E0723]: only int and `bool` operations are stable in const fn
2019-08-14T18:55:00.4596091Z    |
2019-08-14T18:55:00.4596091Z    |
2019-08-14T18:55:00.4596319Z LL | const fn foo19_3(f: f32) -> f32 { -f }
2019-08-14T18:55:00.4596428Z    |
2019-08-14T18:55:00.4596428Z    |
2019-08-14T18:55:00.4596721Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4596787Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4596830Z 
2019-08-14T18:55:00.4596876Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-08-14T18:55:00.4597202Z    |
2019-08-14T18:55:00.4597202Z    |
2019-08-14T18:55:00.4597438Z LL | const fn foo19_4(f: f32, g: f32) -> f32 { f / g }
2019-08-14T18:55:00.4597555Z    |
2019-08-14T18:55:00.4597555Z    |
2019-08-14T18:55:00.4597844Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4597901Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4597948Z 
2019-08-14T18:55:00.4597994Z error[E0723]: cannot access `static` items in const fn
2019-08-14T18:55:00.4598419Z    |
2019-08-14T18:55:00.4598419Z    |
2019-08-14T18:55:00.4598759Z LL | const fn foo25() -> u32 { BAR } //~ ERROR cannot access `static` items in const fn
2019-08-14T18:55:00.4598858Z    |
2019-08-14T18:55:00.4598858Z    |
2019-08-14T18:55:00.4599409Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4599460Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4599489Z 
2019-08-14T18:55:00.4599542Z error[E0723]: cannot access `static` items in const fn
2019-08-14T18:55:00.4599843Z    |
2019-08-14T18:55:00.4599843Z    |
2019-08-14T18:55:00.4600104Z LL | const fn foo26() -> &'static u32 { &BAR } //~ ERROR cannot access `static` items
2019-08-14T18:55:00.4600193Z    |
2019-08-14T18:55:00.4600193Z    |
2019-08-14T18:55:00.4600473Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4600535Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4600605Z error[E0723]: casting pointers to ints is unstable in const fn
2019-08-14T18:55:00.4600865Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:92:42
2019-08-14T18:55:00.4600909Z    |
2019-08-14T18:55:00.4600909Z    |
2019-08-14T18:55:00.4601128Z LL | const fn foo30(x: *const u32) -> usize { x as usize }
2019-08-14T18:55:00.4601229Z    |
2019-08-14T18:55:00.4601229Z    |
2019-08-14T18:55:00.4601515Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4601566Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4601636Z error[E0723]: casting pointers to ints is unstable in const fn
2019-08-14T18:55:00.4601894Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:94:63
2019-08-14T18:55:00.4601938Z    |
2019-08-14T18:55:00.4601938Z    |
2019-08-14T18:55:00.4602177Z LL | const fn foo30_with_unsafe(x: *const u32) -> usize { unsafe { x as usize } }
2019-08-14T18:55:00.4602294Z    |
2019-08-14T18:55:00.4602294Z    |
2019-08-14T18:55:00.4602559Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4602625Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4602812Z error[E0723]: casting pointers to ints is unstable in const fn
2019-08-14T18:55:00.4603596Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:96:42
2019-08-14T18:55:00.4603660Z    |
2019-08-14T18:55:00.4603660Z    |
2019-08-14T18:55:00.4603942Z LL | const fn foo30_2(x: *mut u32) -> usize { x as usize }
2019-08-14T18:55:00.4604052Z    |
2019-08-14T18:55:00.4604052Z    |
2019-08-14T18:55:00.4604353Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4604422Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4604509Z error[E0723]: casting pointers to ints is unstable in const fn
2019-08-14T18:55:00.4604776Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:98:63
2019-08-14T18:55:00.4604837Z    |
2019-08-14T18:55:00.4604837Z    |
2019-08-14T18:55:00.4605096Z LL | const fn foo30_2_with_unsafe(x: *mut u32) -> usize { unsafe { x as usize } }
2019-08-14T18:55:00.4605213Z    |
2019-08-14T18:55:00.4605213Z    |
2019-08-14T18:55:00.4605506Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4605561Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4605609Z 
2019-08-14T18:55:00.4605654Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-08-14T18:55:00.4605981Z    |
2019-08-14T18:55:00.4605981Z    |
2019-08-14T18:55:00.4606470Z LL | const fn foo30_4(b: bool) -> usize { if b { 1 } else { 42 } }
2019-08-14T18:55:00.4606700Z    |
2019-08-14T18:55:00.4606700Z    |
2019-08-14T18:55:00.4606977Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4607028Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4607112Z error[E0723]: loops are not allowed in const fn
2019-08-14T18:55:00.4607358Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:102:29
2019-08-14T18:55:00.4607410Z    |
2019-08-14T18:55:00.4607410Z    |
2019-08-14T18:55:00.4607465Z LL | const fn foo30_5(b: bool) { while b { } }
2019-08-14T18:55:00.4607546Z    |
2019-08-14T18:55:00.4607546Z    |
2019-08-14T18:55:00.4607829Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4607880Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4607918Z 
2019-08-14T18:55:00.4607978Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-08-14T18:55:00.4608269Z    |
2019-08-14T18:55:00.4608269Z    |
2019-08-14T18:55:00.4608502Z LL | const fn foo36(a: bool, b: bool) -> bool { a && b }
2019-08-14T18:55:00.4608588Z    |
2019-08-14T18:55:00.4608588Z    |
2019-08-14T18:55:00.4608864Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4608923Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4608954Z 
2019-08-14T18:55:00.4608997Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-08-14T18:55:00.4609298Z    |
2019-08-14T18:55:00.4609298Z    |
2019-08-14T18:55:00.4609514Z LL | const fn foo37(a: bool, b: bool) -> bool { a || b }
2019-08-14T18:55:00.4609620Z    |
2019-08-14T18:55:00.4609620Z    |
2019-08-14T18:55:00.4609897Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4609948Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4610017Z error[E0723]: mutable references in const fn are unstable
2019-08-14T18:55:00.4610274Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:109:14
2019-08-14T18:55:00.4610318Z    |
2019-08-14T18:55:00.4610318Z    |
2019-08-14T18:55:00.4610430Z LL | const fn inc(x: &mut i32) { *x += 1 }
2019-08-14T18:55:00.4610531Z    |
2019-08-14T18:55:00.4610531Z    |
2019-08-14T18:55:00.4610837Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4610904Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4610934Z 
2019-08-14T18:55:00.4610978Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-14T18:55:00.4611295Z    |
2019-08-14T18:55:00.4611295Z    |
2019-08-14T18:55:00.4611624Z LL | impl<T: std::fmt::Debug> Foo<T> {
2019-08-14T18:55:00.4611724Z    |
2019-08-14T18:55:00.4611724Z    |
2019-08-14T18:55:00.4612056Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4612108Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4612153Z 
2019-08-14T18:55:00.4612205Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-14T18:55:00.4612512Z    |
2019-08-14T18:55:00.4612512Z    |
2019-08-14T18:55:00.4612551Z LL | impl<T: std::fmt::Debug + Sized> Foo<T> {
2019-08-14T18:55:00.4612752Z    |
2019-08-14T18:55:00.4612752Z    |
2019-08-14T18:55:00.4613490Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4613555Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4613717Z 
2019-08-14T18:55:00.4613783Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-14T18:55:00.4614164Z    |
2019-08-14T18:55:00.4614164Z    |
2019-08-14T18:55:00.4614222Z LL | impl<T: Sync + Sized> Foo<T> {
2019-08-14T18:55:00.4614303Z    |
2019-08-14T18:55:00.4614303Z    |
2019-08-14T18:55:00.4614617Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4614687Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4614720Z 
2019-08-14T18:55:00.4614781Z error[E0723]: `impl Trait` in const fn is unstable
2019-08-14T18:55:00.4615121Z    |
2019-08-14T18:55:00.4615121Z    |
2019-08-14T18:55:00.4615411Z LL | const fn no_rpit2() -> AlanTuring<impl std::fmt::Debug> { AlanTuring(0) }
2019-08-14T18:55:00.4615522Z    |
2019-08-14T18:55:00.4615522Z    |
2019-08-14T18:55:00.4615853Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4615912Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4615945Z 
2019-08-14T18:55:00.4615995Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-14T18:55:00.4616350Z    |
2019-08-14T18:55:00.4616350Z    |
2019-08-14T18:55:00.4616405Z LL | const fn no_apit2(_x: AlanTuring<impl std::fmt::Debug>) {}
2019-08-14T18:55:00.4616519Z    |
2019-08-14T18:55:00.4616519Z    |
2019-08-14T18:55:00.4616930Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4617001Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4617033Z 
2019-08-14T18:55:00.4617089Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-14T18:55:00.4617434Z    |
2019-08-14T18:55:00.4617434Z    |
2019-08-14T18:55:00.4617482Z LL | const fn no_apit(_x: impl std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
2019-08-14T18:55:00.4617592Z    |
2019-08-14T18:55:00.4617592Z    |
2019-08-14T18:55:00.4617894Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4618068Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4618108Z 
2019-08-14T18:55:00.4618153Z error[E0723]: `impl Trait` in const fn is unstable
2019-08-14T18:55:00.4618643Z    |
2019-08-14T18:55:00.4618643Z    |
2019-08-14T18:55:00.4618904Z LL | const fn no_rpit() -> impl std::fmt::Debug {} //~ ERROR `impl Trait` in const fn is unstable
2019-08-14T18:55:00.4619020Z    |
2019-08-14T18:55:00.4619020Z    |
2019-08-14T18:55:00.4619299Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4619367Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4619397Z 
2019-08-14T18:55:00.4619548Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-14T18:55:00.4619951Z    |
2019-08-14T18:55:00.4619951Z    |
2019-08-14T18:55:00.4620007Z LL | const fn no_dyn_trait(_x: &dyn std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
2019-08-14T18:55:00.4620105Z    |
2019-08-14T18:55:00.4620105Z    |
2019-08-14T18:55:00.4620377Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4620429Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4620554Z 
2019-08-14T18:55:00.4620597Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-14T18:55:00.4621049Z    |
2019-08-14T18:55:00.4621049Z    |
2019-08-14T18:55:00.4621289Z LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
2019-08-14T18:55:00.4621395Z    |
2019-08-14T18:55:00.4621395Z    |
2019-08-14T18:55:00.4621676Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4621738Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4621826Z error[E0515]: cannot return reference to temporary value
2019-08-14T18:55:00.4622082Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:137:63
2019-08-14T18:55:00.4622127Z    |
2019-08-14T18:55:00.4622127Z    |
2019-08-14T18:55:00.4622377Z LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
2019-08-14T18:55:00.4622629Z    |                                                               ^--
2019-08-14T18:55:00.4623138Z    |                                                               |temporary value created here
2019-08-14T18:55:00.4623197Z    |                                                               returns a reference to data owned by the current function
2019-08-14T18:55:00.4623236Z 
2019-08-14T18:55:00.4623236Z 
2019-08-14T18:55:00.4623318Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-14T18:55:00.4623685Z    |
2019-08-14T18:55:00.4623685Z    |
2019-08-14T18:55:00.4623748Z LL | const fn really_no_traits_i_mean_it() { (&() as &dyn std::fmt::Debug, ()).1 }
2019-08-14T18:55:00.4623845Z    |
2019-08-14T18:55:00.4623845Z    |
2019-08-14T18:55:00.4624169Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4624225Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4624317Z error[E0723]: function pointers in const fn are unstable
2019-08-14T18:55:00.4624581Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:148:21
2019-08-14T18:55:00.4624629Z    |
2019-08-14T18:55:00.4624629Z    |
2019-08-14T18:55:00.4624685Z LL | const fn no_fn_ptrs(_x: fn()) {}
2019-08-14T18:55:00.4624769Z    |
2019-08-14T18:55:00.4624769Z    |
2019-08-14T18:55:00.4625177Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4625244Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4625318Z error[E0723]: function pointers in const fn are unstable
2019-08-14T18:55:00.4625636Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:150:27
2019-08-14T18:55:00.4625683Z    |
2019-08-14T18:55:00.4625683Z    |
2019-08-14T18:55:00.4625929Z LL | const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }
2019-08-14T18:55:00.4626036Z    |
2019-08-14T18:55:00.4626036Z    |
2019-08-14T18:55:00.4626645Z    = note: for more information, see issue ***/issues/57563
2019-08-14T18:55:00.4626708Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-14T18:55:00.4626772Z error: aborting due to 37 previous errors
2019-08-14T18:55:00.4626798Z 
2019-08-14T18:55:00.4626849Z Some errors have detailed explanations: E0515, E0723.
2019-08-14T18:55:00.4627083Z For more information about an error, try `rustc --explain E0515`.
2019-08-14T18:55:00.4627083Z For more information about an error, try `rustc --explain E0515`.
2019-08-14T18:55:00.4627113Z 
2019-08-14T18:55:00.4627320Z ------------------------------------------
2019-08-14T18:55:00.4627348Z 
2019-08-14T18:55:00.4627370Z 
2019-08-14T18:55:00.4627574Z ---- [ui] ui/feature-gates/feature-gate-nll.rs stdout ----
2019-08-14T18:55:00.4627629Z diff of stderr:
2019-08-14T18:55:00.4627654Z 
2019-08-14T18:55:00.4628015Z - warning[E0502]: cannot borrow `*x.1` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4628067Z + error[E0502]: cannot borrow `*x.1` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4628325Z 3    |
2019-08-14T18:55:00.4628325Z 3    |
2019-08-14T18:55:00.4628361Z 4 LL |     let m = &mut x;
2019-08-14T18:55:00.4628432Z 8 ...
2019-08-14T18:55:00.4628466Z 9 LL |     m;
2019-08-14T18:55:00.4628466Z 9 LL |     m;
2019-08-14T18:55:00.4628665Z 10    |     - mutable borrow later used here
2019-08-14T18:55:00.4629105Z -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
2019-08-14T18:55:00.4629383Z -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
2019-08-14T18:55:00.4629619Z -    = note: for more information, try `rustc --explain E0729`
2019-08-14T18:55:00.4629783Z - 
2019-08-14T18:55:00.4629783Z - 
2019-08-14T18:55:00.4629979Z - error: compilation successful
2019-08-14T18:55:00.4630188Z -   --> $DIR/feature-gate-nll.rs:11:1
2019-08-14T18:55:00.4630354Z -    |
2019-08-14T18:55:00.4630529Z - LL | / fn main() {
2019-08-14T18:55:00.4630720Z - LL | |     let mut x = (33, &0);
2019-08-14T18:55:00.4630905Z - LL | |
2019-08-14T18:55:00.4631088Z - LL | |     let m = &mut x;
2019-08-14T18:55:00.4631254Z - ...  |
2019-08-14T18:55:00.4631439Z - LL | |     m;
2019-08-14T18:55:00.4631608Z - LL | | }
2019-08-14T18:55:00.4631810Z 27 
2019-08-14T18:55:00.4631868Z 28 error: aborting due to previous error
2019-08-14T18:55:00.4631904Z 29 
2019-08-14T18:55:00.4631928Z 
2019-08-14T18:55:00.4631928Z 
2019-08-14T18:55:00.4631950Z 
2019-08-14T18:55:00.4632003Z The actual stderr differed from the expected stderr.
2019-08-14T18:55:00.4632280Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-nll/feature-gate-nll.stderr
2019-08-14T18:55:00.4632501Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T18:55:00.4632896Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-nll.rs`
2019-08-14T18:55:00.4633309Z error: 1 errors occurred comparing output.
2019-08-14T18:55:00.4633371Z status: exit code: 1
2019-08-14T18:55:00.4633371Z status: exit code: 1
2019-08-14T18:55:00.4634278Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-nll.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-nll" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-nll/auxiliary" "-A" "unused"
2019-08-14T18:55:00.4634668Z ------------------------------------------
2019-08-14T18:55:00.4634714Z 
2019-08-14T18:55:00.4634958Z ------------------------------------------
2019-08-14T18:55:00.4635005Z stderr:
2019-08-14T18:55:00.4635005Z stderr:
2019-08-14T18:55:00.4635225Z ------------------------------------------
2019-08-14T18:55:00.4635279Z error[E0502]: cannot borrow `*x.1` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4635605Z    |
2019-08-14T18:55:00.4635605Z    |
2019-08-14T18:55:00.4635647Z LL |     let m = &mut x;
2019-08-14T18:55:00.4635901Z    |             ------ mutable borrow occurs here
2019-08-14T18:55:00.4635949Z LL |     let p = &*x.1;
2019-08-14T18:55:00.4635994Z    |             ^^^^^ immutable borrow occurs here
2019-08-14T18:55:00.4636091Z LL |     m;
2019-08-14T18:55:00.4636091Z LL |     m;
2019-08-14T18:55:00.4636314Z    |     - mutable borrow later used here
2019-08-14T18:55:00.4636404Z error: aborting due to previous error
2019-08-14T18:55:00.4636432Z 
2019-08-14T18:55:00.4636797Z For more information about this error, try `rustc --explain E0502`.
2019-08-14T18:55:00.4636954Z 
2019-08-14T18:55:00.4636954Z 
2019-08-14T18:55:00.4637307Z ------------------------------------------
2019-08-14T18:55:00.4637337Z 
2019-08-14T18:55:00.4637360Z 
2019-08-14T18:55:00.4637580Z ---- [ui] ui/issues/issue-40510-1.rs#migrate stdout ----
2019-08-14T18:55:00.4637641Z diff of stderr:
2019-08-14T18:55:00.4637667Z 
2019-08-14T18:55:00.4637889Z - warning: captured variable cannot escape `FnMut` closure body
---
2019-08-14T18:55:00.4654647Z - warning[E0713]: borrow may still be in use when destructor runs
2019-08-14T18:55:00.4654700Z + error[E0713]: borrow may still be in use when destructor runs
2019-08-14T18:55:00.4654953Z 2   --> $DIR/issue-45696-scribble-on-boxed-borrow.rs:52:5
2019-08-14T18:55:00.4655000Z 3    |
2019-08-14T18:55:00.4655234Z 4 LL | fn scribbled<'a>(s: Scribble<'a>) -> &'a mut u32 {
2019-08-14T18:55:00.4655322Z 8 ...
2019-08-14T18:55:00.4655362Z 9 LL | }
2019-08-14T18:55:00.4655362Z 9 LL | }
2019-08-14T18:55:00.4655666Z 10    | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait
2019-08-14T18:55:00.4656171Z -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
2019-08-14T18:55:00.4656486Z -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
2019-08-14T18:55:00.4656757Z -    = note: for more information, try `rustc --explain E0729`
2019-08-14T18:55:00.4656813Z 15 
2019-08-14T18:55:00.4656813Z 15 
2019-08-14T18:55:00.4657055Z - warning[E0713]: borrow may still be in use when destructor runs
2019-08-14T18:55:00.4657124Z + error[E0713]: borrow may still be in use when destructor runs
2019-08-14T18:55:00.4657366Z 17   --> $DIR/issue-45696-scribble-on-boxed-borrow.rs:63:5
2019-08-14T18:55:00.4657412Z 18    |
2019-08-14T18:55:00.4657673Z 19 LL | fn boxed_scribbled<'a>(s: Box<Scribble<'a>>) -> &'a mut u32 {
2019-08-14T18:55:00.4657746Z 23 ...
2019-08-14T18:55:00.4657786Z 24 LL | }
2019-08-14T18:55:00.4657786Z 24 LL | }
2019-08-14T18:55:00.4658204Z 25    | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait
2019-08-14T18:55:00.4658739Z -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
2019-08-14T18:55:00.4659192Z -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
2019-08-14T18:55:00.4659449Z -    = note: for more information, try `rustc --explain E0729`
2019-08-14T18:55:00.4659493Z 30 
2019-08-14T18:55:00.4659493Z 30 
2019-08-14T18:55:00.4659960Z - warning[E0713]: borrow may still be in use when destructor runs
2019-08-14T18:55:00.4660011Z + error[E0713]: borrow may still be in use when destructor runs
2019-08-14T18:55:00.4660342Z 32   --> $DIR/issue-45696-scribble-on-boxed-borrow.rs:74:5
2019-08-14T18:55:00.4660400Z 33    |
2019-08-14T18:55:00.4660648Z 34 LL | fn boxed_boxed_scribbled<'a>(s: Box<Box<Scribble<'a>>>) -> &'a mut u32 {
2019-08-14T18:55:00.4660717Z 38 ...
2019-08-14T18:55:00.4660881Z 39 LL | }
2019-08-14T18:55:00.4660881Z 39 LL | }
2019-08-14T18:55:00.4661170Z 40    | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait
2019-08-14T18:55:00.4661766Z -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
2019-08-14T18:55:00.4662204Z -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
2019-08-14T18:55:00.4662456Z -    = note: for more information, try `rustc --explain E0729`
2019-08-14T18:55:00.4662519Z 45 
2019-08-14T18:55:00.4662519Z 45 
2019-08-14T18:55:00.4662729Z - error: compilation successful
2019-08-14T18:55:00.4663197Z -   --> $DIR/issue-45696-scribble-on-boxed-borrow.rs:81:1
2019-08-14T18:55:00.4663463Z -    |
2019-08-14T18:55:00.4663668Z - LL | / fn main() {
2019-08-14T18:55:00.4663876Z - LL | |     let mut x = 1;
2019-08-14T18:55:00.4664085Z - LL | |     {
2019-08-14T18:55:00.4664341Z - LL | |         let mut long_lived = Scribble(&mut x);
2019-08-14T18:55:00.4664533Z - ...  |
2019-08-14T18:55:00.4664788Z - LL | |     *boxed_boxed_scribbled(Box::new(Box::new(Scribble(&mut x)))) += 10;
2019-08-14T18:55:00.4665002Z - LL | | }
2019-08-14T18:55:00.4665369Z - 
2019-08-14T18:55:00.4665602Z - error: aborting due to previous error
2019-08-14T18:55:00.4665661Z + error: aborting due to 3 previous errors
2019-08-14T18:55:00.4665701Z 59 
2019-08-14T18:55:00.4665701Z 59 
2019-08-14T18:55:00.4665955Z 60 For more information about this error, try `rustc --explain E0713`.
2019-08-14T18:55:00.4666017Z 61 
2019-08-14T18:55:00.4666044Z 
2019-08-14T18:55:00.4666069Z 
2019-08-14T18:55:00.4666113Z The actual stderr differed from the expected stderr.
2019-08-14T18:55:00.4666497Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45696-scribble-on-boxed-borrow.migrate/issue-45696-scribble-on-boxed-borrow.migrate.stderr
2019-08-14T18:55:00.4666875Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T18:55:00.4667169Z To only update this specific test, also pass `--test-args issues/issue-45696-scribble-on-boxed-borrow.rs`
2019-08-14T18:55:00.4667210Z 
2019-08-14T18:55:00.4667255Z error in revision `migrate`: 1 errors occurred comparing output.
2019-08-14T18:55:00.4667312Z status: exit code: 1
2019-08-14T18:55:00.4668369Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-45696-scribble-on-boxed-borrow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45696-scribble-on-boxed-borrow.migrate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45696-scribble-on-boxed-borrow.migrate/auxiliary" "-A" "unused"
2019-08-14T18:55:00.4668764Z ------------------------------------------
2019-08-14T18:55:00.4668799Z 
2019-08-14T18:55:00.4669039Z ------------------------------------------
2019-08-14T18:55:00.4669084Z stderr:
2019-08-14T18:55:00.4669084Z stderr:
2019-08-14T18:55:00.4669304Z ------------------------------------------
2019-08-14T18:55:00.4669365Z error[E0713]: borrow may still be in use when destructor runs
2019-08-14T18:55:00.4669650Z   --> /checkout/src/test/ui/issues/issue-45696-scribble-on-boxed-borrow.rs:52:5
2019-08-14T18:55:00.4669702Z    |
2019-08-14T18:55:00.4669934Z LL | fn scribbled<'a>(s: Scribble<'a>) -> &'a mut u32 {
2019-08-14T18:55:00.4670175Z    |              -- lifetime `'a` defined here
2019-08-14T18:55:00.4670231Z LL |     &mut *s.0 //[nll]~ ERROR borrow may still be in use when destructor runs [E0713]
2019-08-14T18:55:00.4670503Z    |     ^^^^^^^^^ returning this value requires that `*s.0` is borrowed for `'a`
2019-08-14T18:55:00.4670607Z LL | }
2019-08-14T18:55:00.4670607Z LL | }
2019-08-14T18:55:00.4670902Z    | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait
2019-08-14T18:55:00.4671006Z error[E0713]: borrow may still be in use when destructor runs
2019-08-14T18:55:00.4671271Z   --> /checkout/src/test/ui/issues/issue-45696-scribble-on-boxed-borrow.rs:63:5
2019-08-14T18:55:00.4671431Z    |
2019-08-14T18:55:00.4671431Z    |
2019-08-14T18:55:00.4671708Z LL | fn boxed_scribbled<'a>(s: Box<Scribble<'a>>) -> &'a mut u32 {
2019-08-14T18:55:00.4671947Z    |                    -- lifetime `'a` defined here
2019-08-14T18:55:00.4672019Z LL |     &mut *(*s).0 //[nll]~ ERROR borrow may still be in use when destructor runs [E0713]
2019-08-14T18:55:00.4672287Z    |     ^^^^^^^^^^^^ returning this value requires that `*s.0` is borrowed for `'a`
2019-08-14T18:55:00.4672390Z LL | }
2019-08-14T18:55:00.4672390Z LL | }
2019-08-14T18:55:00.4672692Z    | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait
2019-08-14T18:55:00.4672777Z error[E0713]: borrow may still be in use when destructor runs
2019-08-14T18:55:00.4673686Z   --> /checkout/src/test/ui/issues/issue-45696-scribble-on-boxed-borrow.rs:74:5
2019-08-14T18:55:00.4673744Z    |
2019-08-14T18:55:00.4673744Z    |
2019-08-14T18:55:00.4674002Z LL | fn boxed_boxed_scribbled<'a>(s: Box<Box<Scribble<'a>>>) -> &'a mut u32 {
2019-08-14T18:55:00.4674286Z    |                          -- lifetime `'a` defined here
2019-08-14T18:55:00.4674343Z LL |     &mut *(**s).0 //[nll]~ ERROR borrow may still be in use when destructor runs [E0713]
2019-08-14T18:55:00.4674608Z    |     ^^^^^^^^^^^^^ returning this value requires that `*s.0` is borrowed for `'a`
2019-08-14T18:55:00.4674709Z LL | }
2019-08-14T18:55:00.4674709Z LL | }
2019-08-14T18:55:00.4675001Z    | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait
2019-08-14T18:55:00.4675105Z error: aborting due to 3 previous errors
2019-08-14T18:55:00.4675134Z 
2019-08-14T18:55:00.4675381Z For more information about this error, try `rustc --explain E0713`.
2019-08-14T18:55:00.4675470Z 
2019-08-14T18:55:00.4675470Z 
2019-08-14T18:55:00.4675706Z ------------------------------------------
2019-08-14T18:55:00.4675737Z 
2019-08-14T18:55:00.4675763Z 
2019-08-14T18:55:00.4675988Z ---- [ui] ui/issues/issue-49824.rs stdout ----
2019-08-14T18:55:00.4676059Z diff of stderr:
2019-08-14T18:55:00.4676086Z 
2019-08-14T18:55:00.4676325Z - warning: captured variable cannot escape `FnMut` closure body
2019-08-14T18:55:00.4676394Z + error: captured variable cannot escape `FnMut` closure body
2019-08-14T18:55:00.4676612Z 2   --> $DIR/issue-49824.rs:10:9
2019-08-14T18:55:00.4676697Z 4 LL |       || {
2019-08-14T18:55:00.4676742Z 
2019-08-14T18:55:00.4676779Z 13    |
2019-08-14T18:55:00.4676779Z 13    |
2019-08-14T18:55:00.4676933Z 14    = note: `FnMut` closures only have access to their captured variables while they are executing...
2019-08-14T18:55:00.4677015Z 15    = note: ...therefore, they cannot allow references to captured variables to escape
2019-08-14T18:55:00.4677666Z -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
2019-08-14T18:55:00.4677947Z -    = note: for more information, try `rustc --explain E0729`
2019-08-14T18:55:00.4678134Z - 
2019-08-14T18:55:00.4678346Z - error: compilation successful
2019-08-14T18:55:00.4678346Z - error: compilation successful
2019-08-14T18:55:00.4678578Z -   --> $DIR/issue-49824.rs:6:1
2019-08-14T18:55:00.4678769Z -    |
2019-08-14T18:55:00.4678967Z - LL | / fn main() {
2019-08-14T18:55:00.4679157Z - LL | |
2019-08-14T18:55:00.4679503Z - LL | |     let mut x = 0;
2019-08-14T18:55:00.4679940Z - LL | |     || {
2019-08-14T18:55:00.4680123Z - ...  |
2019-08-14T18:55:00.4680326Z - LL | |     };
2019-08-14T18:55:00.4680519Z - LL | | }
2019-08-14T18:55:00.4680865Z 31 
2019-08-14T18:55:00.4680926Z 32 error: aborting due to previous error
2019-08-14T18:55:00.4680966Z 33 
2019-08-14T18:55:00.4680992Z 
2019-08-14T18:55:00.4680992Z 
2019-08-14T18:55:00.4681017Z 
2019-08-14T18:55:00.4681074Z The actual stderr differed from the expected stderr.
2019-08-14T18:55:00.4681372Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49824/issue-49824.stderr
2019-08-14T18:55:00.4681746Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T18:55:00.4682030Z To only update this specific test, also pass `--test-args issues/issue-49824.rs`
2019-08-14T18:55:00.4682109Z error: 1 errors occurred comparing output.
2019-08-14T18:55:00.4682168Z status: exit code: 1
2019-08-14T18:55:00.4682168Z status: exit code: 1
2019-08-14T18:55:00.4682901Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-49824.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49824" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49824/auxiliary" "-A" "unused"
2019-08-14T18:55:00.4683513Z ------------------------------------------
2019-08-14T18:55:00.4683547Z 
2019-08-14T18:55:00.4683773Z ------------------------------------------
2019-08-14T18:55:00.4683836Z stderr:
2019-08-14T18:55:00.4683836Z stderr:
2019-08-14T18:55:00.4684056Z ------------------------------------------
2019-08-14T18:55:00.4684107Z error: captured variable cannot escape `FnMut` closure body
2019-08-14T18:55:00.4684368Z   --> /checkout/src/test/ui/issues/issue-49824.rs:10:9
2019-08-14T18:55:00.4684456Z LL |       || {
2019-08-14T18:55:00.4684456Z LL |       || {
2019-08-14T18:55:00.4684709Z    |        - inferred to be a `FnMut` closure
2019-08-14T18:55:00.4684757Z LL | /         || {
2019-08-14T18:55:00.4684806Z LL | |         //~^ WARNING captured variable cannot escape `FnMut` closure body
2019-08-14T18:55:00.4684877Z LL | |         //~| WARNING this error has been downgraded to a warning
2019-08-14T18:55:00.4684932Z LL | |         //~| WARNING this warning will become a hard error in the future
2019-08-14T18:55:00.4684980Z LL | |             let _y = &mut x;
2019-08-14T18:55:00.4685045Z LL | |         }
2019-08-14T18:55:00.4685098Z    | |_________^ returns a closure that contains a reference to a captured variable, which then escapes the closure body
2019-08-14T18:55:00.4685147Z    |
2019-08-14T18:55:00.4685213Z    = note: `FnMut` closures only have access to their captured variables while they are executing...
2019-08-14T18:55:00.4685268Z    = note: ...therefore, they cannot allow references to captured variables to escape
2019-08-14T18:55:00.4685357Z error: aborting due to previous error
2019-08-14T18:55:00.4685488Z 
2019-08-14T18:55:00.4685522Z 
2019-08-14T18:55:00.4685787Z ------------------------------------------
2019-08-14T18:55:00.4685787Z ------------------------------------------
2019-08-14T18:55:00.4685819Z 
2019-08-14T18:55:00.4685862Z 
2019-08-14T18:55:00.4686101Z ---- [ui] ui/pattern/pattern-bindings-after-at.rs stdout ----
2019-08-14T18:55:00.4686149Z diff of stderr:
2019-08-14T18:55:00.4686175Z 
2019-08-14T18:55:00.4686234Z 4 LL |         ref mut z @ &mut Some(ref a) => {
2019-08-14T18:55:00.4686293Z 5    |                               ^^^^^ not allowed after `@`
2019-08-14T18:55:00.4686336Z 6 
2019-08-14T18:55:00.4686624Z - warning[E0502]: cannot borrow `_` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4686680Z + error[E0502]: cannot borrow `_` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4686974Z 9    |
2019-08-14T18:55:00.4686974Z 9    |
2019-08-14T18:55:00.4687019Z 10 LL |         ref mut z @ &mut Some(ref a) => {
2019-08-14T18:55:00.4687094Z 15 ...
2019-08-14T18:55:00.4687094Z 15 ...
2019-08-14T18:55:00.4687150Z 16 LL |             **z = None;
2019-08-14T18:55:00.4687393Z 17    |             ---------- mutable borrow later used here
2019-08-14T18:55:00.4687884Z -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
2019-08-14T18:55:00.4688199Z -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
2019-08-14T18:55:00.4688595Z -    = note: for more information, try `rustc --explain E0729`
2019-08-14T18:55:00.4688661Z 22 
---
2019-08-14T18:55:00.4689281Z 26 For more information about an error, try `rustc --explain E0303`.
2019-08-14T18:55:00.4689324Z 
2019-08-14T18:55:00.4689349Z 
2019-08-14T18:55:00.4689411Z The actual stderr differed from the expected stderr.
2019-08-14T18:55:00.4689735Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-bindings-after-at/pattern-bindings-after-at.stderr
2019-08-14T18:55:00.4689987Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T18:55:00.4690280Z To only update this specific test, also pass `--test-args pattern/pattern-bindings-after-at.rs`
2019-08-14T18:55:00.4690368Z error: 1 errors occurred comparing output.
2019-08-14T18:55:00.4690429Z status: exit code: 1
2019-08-14T18:55:00.4690429Z status: exit code: 1
2019-08-14T18:55:00.4691192Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pattern-bindings-after-at.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-bindings-after-at" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-bindings-after-at/auxiliary" "-A" "unused"
2019-08-14T18:55:00.4691917Z ------------------------------------------
2019-08-14T18:55:00.4691953Z 
2019-08-14T18:55:00.4692213Z ------------------------------------------
2019-08-14T18:55:00.4692258Z stderr:
2019-08-14T18:55:00.4692258Z stderr:
2019-08-14T18:55:00.4692477Z ------------------------------------------
2019-08-14T18:55:00.4692528Z error[E0303]: pattern bindings are not allowed after an `@`
2019-08-14T18:55:00.4692850Z    |
2019-08-14T18:55:00.4692850Z    |
2019-08-14T18:55:00.4692893Z LL |         ref mut z @ &mut Some(ref a) => {
2019-08-14T18:55:00.4693164Z    |                               ^^^^^ not allowed after `@`
2019-08-14T18:55:00.4693199Z 
2019-08-14T18:55:00.4693367Z error[E0502]: cannot borrow `_` as immutable because it is also borrowed as mutable
2019-08-14T18:55:00.4694049Z    |
2019-08-14T18:55:00.4694049Z    |
2019-08-14T18:55:00.4694092Z LL |         ref mut z @ &mut Some(ref a) => {
2019-08-14T18:55:00.4694337Z    |         ----------------------^^^^^-
2019-08-14T18:55:00.4694443Z    |         |                     immutable borrow occurs here
2019-08-14T18:55:00.4694503Z    |         mutable borrow occurs here
2019-08-14T18:55:00.4694543Z ...
2019-08-14T18:55:00.4694543Z ...
2019-08-14T18:55:00.4694585Z LL |             **z = None;
2019-08-14T18:55:00.4694827Z    |             ---------- mutable borrow later used here
2019-08-14T18:55:00.4694918Z error: aborting due to 2 previous errors
2019-08-14T18:55:00.4694946Z 
2019-08-14T18:55:00.4694990Z Some errors have detailed explanations: E0303, E0502.
2019-08-14T18:55:00.4695261Z For more information about an error, try `rustc --explain E0303`.
2019-08-14T18:55:00.4695261Z For more information about an error, try `rustc --explain E0303`.
2019-08-14T18:55:00.4695295Z 
2019-08-14T18:55:00.4695515Z ------------------------------------------
2019-08-14T18:55:00.4695547Z 
2019-08-14T18:55:00.4695589Z 
2019-08-14T18:55:00.4695815Z ---- [ui] ui/thread-local-in-ctfe.rs stdout ----
2019-08-14T18:55:00.4695862Z diff of stderr:
2019-08-14T18:55:00.4695889Z 
2019-08-14T18:55:00.4695946Z 10 LL | static C: &u32 = &A;
2019-08-14T18:55:00.4696137Z 12 
2019-08-14T18:55:00.4696443Z - warning[E0712]: thread-local variable borrowed past end of function
2019-08-14T18:55:00.4696443Z - warning[E0712]: thread-local variable borrowed past end of function
2019-08-14T18:55:00.4696700Z + error[E0712]: thread-local variable borrowed past end of function
2019-08-14T18:55:00.4696990Z 15    |
2019-08-14T18:55:00.4696990Z 15    |
2019-08-14T18:55:00.4697030Z 16 LL | static C: &u32 = &A;
2019-08-14T18:55:00.4697058Z 
2019-08-14T18:55:00.4697298Z 17    |                  ^^- end of enclosing function is here
2019-08-14T18:55:00.4697649Z 19    |                  thread-local variables cannot be borrowed beyond the end of the function
2019-08-14T18:55:00.4697844Z -    |
2019-08-14T18:55:00.4698143Z -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
2019-08-14T18:55:00.4698461Z -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
2019-08-14T18:55:00.4698461Z -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
2019-08-14T18:55:00.4698852Z -    = note: for more information, try `rustc --explain E0729`
2019-08-14T18:55:00.4698913Z 24 
2019-08-14T18:55:00.4699156Z 25 error[E0625]: thread-local statics cannot be accessed at compile-time
2019-08-14T18:55:00.4699377Z 26   --> $DIR/thread-local-in-ctfe.rs:15:16
2019-08-14T18:55:00.4699423Z 
2019-08-14T18:55:00.4699580Z 34 LL | const E: &u32 = &A;
2019-08-14T18:55:00.4699661Z 36 
2019-08-14T18:55:00.4699936Z - warning[E0712]: thread-local variable borrowed past end of function
2019-08-14T18:55:00.4699936Z - warning[E0712]: thread-local variable borrowed past end of function
2019-08-14T18:55:00.4700188Z + error[E0712]: thread-local variable borrowed past end of function
2019-08-14T18:55:00.4700479Z 39    |
2019-08-14T18:55:00.4700479Z 39    |
2019-08-14T18:55:00.4700519Z 40 LL | const E: &u32 = &A;
2019-08-14T18:55:00.4700547Z 
2019-08-14T18:55:00.4700786Z 41    |                 ^^- end of enclosing function is here
2019-08-14T18:55:00.4701134Z 43    |                 thread-local variables cannot be borrowed beyond the end of the function
2019-08-14T18:55:00.4701327Z -    |
2019-08-14T18:55:00.4701629Z -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
2019-08-14T18:55:00.4701941Z -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
---
2019-08-14T18:55:00.4703973Z 58 
2019-08-14T18:55:00.4704000Z 
2019-08-14T18:55:00.4704025Z 
2019-08-14T18:55:00.4704083Z The actual stderr differed from the expected stderr.
2019-08-14T18:55:00.4704388Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-in-ctfe/thread-local-in-ctfe.stderr
2019-08-14T18:55:00.4704647Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T18:55:00.4704928Z To only update this specific test, also pass `--test-args thread-local-in-ctfe.rs`
2019-08-14T18:55:00.4705006Z error: 1 errors occurred comparing output.
2019-08-14T18:55:00.4705048Z status: exit code: 1
2019-08-14T18:55:00.4705048Z status: exit code: 1
2019-08-14T18:55:00.4705785Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/thread-local-in-ctfe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-in-ctfe" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-in-ctfe/auxiliary" "-A" "unused"
2019-08-14T18:55:00.4706272Z ------------------------------------------
2019-08-14T18:55:00.4706316Z 
2019-08-14T18:55:00.4706546Z ------------------------------------------
2019-08-14T18:55:00.4706610Z stderr:
2019-08-14T18:55:00.4706610Z stderr:
2019-08-14T18:55:00.4706831Z ------------------------------------------
2019-08-14T18:55:00.4707084Z error[E0625]: thread-local statics cannot be accessed at compile-time
2019-08-14T18:55:00.4707344Z   --> /checkout/src/test/ui/thread-local-in-ctfe.rs:6:17
2019-08-14T18:55:00.4707394Z    |
2019-08-14T18:55:00.4707443Z LL | static B: u32 = A;
2019-08-14T18:55:00.4707528Z 
2019-08-14T18:55:00.4707778Z error[E0625]: thread-local statics cannot be accessed at compile-time
2019-08-14T18:55:00.4708021Z   --> /checkout/src/test/ui/thread-local-in-ctfe.rs:9:18
2019-08-14T18:55:00.4708083Z    |
2019-08-14T18:55:00.4708083Z    |
2019-08-14T18:55:00.4708123Z LL | static C: &u32 = &A;
2019-08-14T18:55:00.4708192Z 
2019-08-14T18:55:00.4708451Z error[E0712]: thread-local variable borrowed past end of function
2019-08-14T18:55:00.4708702Z   --> /checkout/src/test/ui/thread-local-in-ctfe.rs:9:18
2019-08-14T18:55:00.4708748Z    |
2019-08-14T18:55:00.4708748Z    |
2019-08-14T18:55:00.4708804Z LL | static C: &u32 = &A;
2019-08-14T18:55:00.4709042Z    |                  ^^- end of enclosing function is here
2019-08-14T18:55:00.4709377Z    |                  thread-local variables cannot be borrowed beyond the end of the function
2019-08-14T18:55:00.4709424Z 
2019-08-14T18:55:00.4709677Z error[E0625]: thread-local statics cannot be accessed at compile-time
2019-08-14T18:55:00.4709918Z   --> /checkout/src/test/ui/thread-local-in-ctfe.rs:15:16
2019-08-14T18:55:00.4709918Z   --> /checkout/src/test/ui/thread-local-in-ctfe.rs:15:16
2019-08-14T18:55:00.4709981Z    |
2019-08-14T18:55:00.4710022Z LL | const D: u32 = A;
2019-08-14T18:55:00.4710107Z 
2019-08-14T18:55:00.4710354Z error[E0625]: thread-local statics cannot be accessed at compile-time
2019-08-14T18:55:00.4710711Z   --> /checkout/src/test/ui/thread-local-in-ctfe.rs:18:17
2019-08-14T18:55:00.4710832Z    |
2019-08-14T18:55:00.4710832Z    |
2019-08-14T18:55:00.4711009Z LL | const E: &u32 = &A;
2019-08-14T18:55:00.4711077Z 
2019-08-14T18:55:00.4711372Z error[E0712]: thread-local variable borrowed past end of function
2019-08-14T18:55:00.4711616Z   --> /checkout/src/test/ui/thread-local-in-ctfe.rs:18:17
2019-08-14T18:55:00.4711661Z    |
2019-08-14T18:55:00.4711661Z    |
2019-08-14T18:55:00.4711700Z LL | const E: &u32 = &A;
2019-08-14T18:55:00.4711965Z    |                 ^^- end of enclosing function is here
2019-08-14T18:55:00.4712280Z    |                 thread-local variables cannot be borrowed beyond the end of the function
2019-08-14T18:55:00.4712333Z 
2019-08-14T18:55:00.4712578Z error[E0625]: thread-local statics cannot be accessed at compile-time
2019-08-14T18:55:00.4712818Z   --> /checkout/src/test/ui/thread-local-in-ctfe.rs:25:5
---
2019-08-14T18:55:00.4717700Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-14T18:55:00.4717772Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-14T18:55:00.4717804Z 
2019-08-14T18:55:00.4717829Z 
2019-08-14T18:55:00.4719405Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-14T18:55:00.4719772Z 
2019-08-14T18:55:00.4719800Z 
2019-08-14T18:55:00.4719859Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-14T18:55:00.4719913Z Build completed unsuccessfully in 1:08:40
2019-08-14T18:55:00.4719913Z Build completed unsuccessfully in 1:08:40
2019-08-14T18:55:00.4720073Z == clock drift check ==
2019-08-14T18:55:00.4720135Z   local time: Wed Aug 14 18:55:00 UTC 2019
2019-08-14T18:55:00.6051510Z   network time: Wed, 14 Aug 2019 18:55:00 GMT
2019-08-14T18:55:00.6052591Z == end clock drift check ==
2019-08-14T18:55:01.3645166Z ##[error]Bash exited with code '1'.
2019-08-14T18:55:01.3703203Z ##[section]Starting: Checkout
2019-08-14T18:55:01.3705365Z ==============================================================================
2019-08-14T18:55:01.3705427Z Task         : Get sources
2019-08-14T18:55:01.3705499Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
