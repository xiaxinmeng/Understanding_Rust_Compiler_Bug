plain
2019-11-11T22:21:50.8221408Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T22:21:50.8488440Z ##[command]git config gc.auto 0
2019-11-11T22:21:50.8579926Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T22:21:50.8640762Z ##[command]git config --get-all http.proxy
2019-11-11T22:21:50.8789968Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64588/merge:refs/remotes/pull/64588/merge
---
2019-11-11T23:17:44.5882078Z .................................................................................................... 1500/9242
2019-11-11T23:17:50.7425285Z .................................................................................................... 1600/9242
2019-11-11T23:17:58.7088092Z .................................................................................................... 1700/9242
2019-11-11T23:18:07.8444526Z ...........i........................................................................................ 1800/9242
2019-11-11T23:18:14.2221843Z ...............................................................................................iiiii 1900/9242
2019-11-11T23:18:35.0039116Z .................................................................................................... 2100/9242
2019-11-11T23:18:37.2588960Z .................................................................................................... 2200/9242
2019-11-11T23:18:39.7175452Z .................................................................................................... 2300/9242
2019-11-11T23:18:45.6412264Z .................................................................................................... 2400/9242
---
2019-11-11T23:21:39.7398404Z ...........................................................................................i........ 4700/9242
2019-11-11T23:21:46.3525007Z .......i............................................................................................ 4800/9242
2019-11-11T23:21:55.3847142Z .................................................................................................... 4900/9242
2019-11-11T23:22:00.7288081Z .................................................................................................... 5000/9242
2019-11-11T23:22:10.9641988Z ..............................................................................................ii.ii. 5100/9242
2019-11-11T23:22:19.9510154Z .............................i...................................................................... 5300/9242
2019-11-11T23:22:26.1852338Z .................................................................................................... 5400/9242
2019-11-11T23:22:36.3265127Z ............................................................................i....................... 5500/9242
2019-11-11T23:22:43.8174152Z .................................................................................................... 5600/9242
2019-11-11T23:22:43.8174152Z .................................................................................................... 5600/9242
2019-11-11T23:22:50.2534854Z .................................................................................................... 5700/9242
2019-11-11T23:22:59.8034887Z .............................................................ii...i..ii...........i................. 5800/9242
2019-11-11T23:23:21.7600010Z .................................................................................................... 6000/9242
2019-11-11T23:23:29.9834007Z .................................................................................................... 6100/9242
2019-11-11T23:23:29.9834007Z .................................................................................................... 6100/9242
2019-11-11T23:23:34.9582418Z ................................................................................i..ii............... 6200/9242
2019-11-11T23:24:01.1535345Z .................................................................................................... 6400/9242
2019-11-11T23:24:05.7062631Z .................................................i.................................................. 6500/9242
2019-11-11T23:24:07.9459534Z .................................................................................................... 6600/9242
2019-11-11T23:24:10.2809620Z .................................i.................................................................. 6700/9242
---
2019-11-11T23:28:28.5525978Z ..............................................i..................................................... 9200/9242
2019-11-11T23:28:38.1861059Z ..........................................
2019-11-11T23:28:38.1861702Z failures:
2019-11-11T23:28:38.1904413Z 
2019-11-11T23:28:38.1905674Z ---- [ui] ui/raw-ref-op/raw-ref-temp.rs stdout ----
2019-11-11T23:28:38.1906028Z 
2019-11-11T23:28:38.1906701Z 2   --> $DIR/raw-ref-temp.rs:12:20
2019-11-11T23:28:38.1907055Z 3    |
2019-11-11T23:28:38.1907055Z 3    |
2019-11-11T23:28:38.1907200Z 4 LL |     let ref_expr = &raw const 2;
2019-11-11T23:28:38.1907758Z -    |                    ^^^^^^^^^^^^ temporary value
2019-11-11T23:28:38.1908678Z +    |                               |
2019-11-11T23:28:38.1908859Z +    |                               temporary value
2019-11-11T23:28:38.1909008Z 6 
2019-11-11T23:28:38.1909148Z 7 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1909148Z 7 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1909515Z 8   --> $DIR/raw-ref-temp.rs:13:24
2019-11-11T23:28:38.1909673Z 
2019-11-11T23:28:38.1909817Z 9    |
2019-11-11T23:28:38.1910080Z 10 LL |     let mut_ref_expr = &raw mut 3;
2019-11-11T23:28:38.1910626Z -    |                        ^^^^^^^^^^ temporary value
2019-11-11T23:28:38.1911402Z +    |                                 |
2019-11-11T23:28:38.1911967Z +    |                                 temporary value
2019-11-11T23:28:38.1912118Z 12 
2019-11-11T23:28:38.1912253Z 13 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1912253Z 13 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1912583Z 14   --> $DIR/raw-ref-temp.rs:14:21
2019-11-11T23:28:38.1912733Z 
2019-11-11T23:28:38.1912885Z 15    |
2019-11-11T23:28:38.1913014Z 16 LL |     let ref_const = &raw const 4;
2019-11-11T23:28:38.1913364Z -    |                     ^^^^^^^^^^^^ temporary value
2019-11-11T23:28:38.1913918Z +    |                                |
2019-11-11T23:28:38.1914320Z +    |                                temporary value
2019-11-11T23:28:38.1914567Z 18 
2019-11-11T23:28:38.1914707Z 19 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1914707Z 19 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1915110Z 20   --> $DIR/raw-ref-temp.rs:15:25
2019-11-11T23:28:38.1916345Z 
2019-11-11T23:28:38.1916512Z 21    |
2019-11-11T23:28:38.1916682Z 22 LL |     let mut_ref_const = &raw mut 5;
2019-11-11T23:28:38.1917130Z -    |                         ^^^^^^^^^^ temporary value
2019-11-11T23:28:38.1917749Z +    |                                  |
2019-11-11T23:28:38.1917904Z +    |                                  temporary value
2019-11-11T23:28:38.1918047Z 24 
2019-11-11T23:28:38.1918213Z 25 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1918213Z 25 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1918740Z 26   --> $DIR/raw-ref-temp.rs:17:26
2019-11-11T23:28:38.1918935Z 
2019-11-11T23:28:38.1919271Z 27    |
2019-11-11T23:28:38.1919405Z 28 LL |     let field_ref_expr = &raw const (1, 2).0;
2019-11-11T23:28:38.1919785Z -    |                          ^^^^^^^^^^^^^^^^^^^ temporary value
2019-11-11T23:28:38.1920353Z +    |                                     |
2019-11-11T23:28:38.1920490Z +    |                                     temporary value
2019-11-11T23:28:38.1920644Z 30 
2019-11-11T23:28:38.1920787Z 31 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1920787Z 31 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1921138Z 32   --> $DIR/raw-ref-temp.rs:18:30
2019-11-11T23:28:38.1921289Z 
2019-11-11T23:28:38.1921422Z 33    |
2019-11-11T23:28:38.1921552Z 34 LL |     let mut_field_ref_expr = &raw mut (1, 2).0;
2019-11-11T23:28:38.1921927Z -    |                              ^^^^^^^^^^^^^^^^^ temporary value
2019-11-11T23:28:38.1922500Z +    |                                       |
2019-11-11T23:28:38.1922641Z +    |                                       temporary value
2019-11-11T23:28:38.1923011Z 36 
2019-11-11T23:28:38.1923185Z 37 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1923185Z 37 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1923684Z 38   --> $DIR/raw-ref-temp.rs:19:21
2019-11-11T23:28:38.1923915Z 
2019-11-11T23:28:38.1924096Z 39    |
2019-11-11T23:28:38.1924235Z 40 LL |     let field_ref = &raw const PAIR.0;
2019-11-11T23:28:38.1924590Z -    |                     ^^^^^^^^^^^^^^^^^ temporary value
2019-11-11T23:28:38.1925318Z +    |                                |
2019-11-11T23:28:38.1925456Z +    |                                temporary value
2019-11-11T23:28:38.1925603Z 42 
2019-11-11T23:28:38.1925735Z 43 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1925735Z 43 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1926060Z 44   --> $DIR/raw-ref-temp.rs:20:25
2019-11-11T23:28:38.1926241Z 
2019-11-11T23:28:38.1926375Z 45    |
2019-11-11T23:28:38.1926504Z 46 LL |     let mut_field_ref = &raw mut PAIR.0;
2019-11-11T23:28:38.1926883Z -    |                         ^^^^^^^^^^^^^^^ temporary value
2019-11-11T23:28:38.1927640Z +    |                                  |
2019-11-11T23:28:38.1927806Z +    |                                  temporary value
2019-11-11T23:28:38.1928154Z 48 
2019-11-11T23:28:38.1928928Z 49 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1928928Z 49 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1929728Z 50   --> $DIR/raw-ref-temp.rs:22:26
2019-11-11T23:28:38.1931215Z 
2019-11-11T23:28:38.1931278Z 51    |
2019-11-11T23:28:38.1931321Z 52 LL |     let field_ref_expr = &raw const [1, 2][0];
2019-11-11T23:28:38.1931672Z -    |                          ^^^^^^^^^^^^^^^^^^^^ temporary value
2019-11-11T23:28:38.1931963Z +    |                                     |
2019-11-11T23:28:38.1932028Z +    |                                     temporary value
2019-11-11T23:28:38.1932071Z 54 
2019-11-11T23:28:38.1932114Z 55 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1932114Z 55 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1932351Z 56   --> $DIR/raw-ref-temp.rs:23:30
2019-11-11T23:28:38.1932381Z 
2019-11-11T23:28:38.1932419Z 57    |
2019-11-11T23:28:38.1932463Z 58 LL |     let mut_field_ref_expr = &raw mut [1, 2][0];
2019-11-11T23:28:38.1932779Z -    |                              ^^^^^^^^^^^^^^^^^^ temporary value
2019-11-11T23:28:38.1933063Z +    |                                       |
2019-11-11T23:28:38.1933128Z +    |                                       temporary value
2019-11-11T23:28:38.1933170Z 60 
2019-11-11T23:28:38.1933214Z 61 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1933214Z 61 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1933447Z 62   --> $DIR/raw-ref-temp.rs:24:21
2019-11-11T23:28:38.1933478Z 
2019-11-11T23:28:38.1933621Z 63    |
2019-11-11T23:28:38.1933672Z 64 LL |     let field_ref = &raw const ARRAY[0];
2019-11-11T23:28:38.1933958Z -    |                     ^^^^^^^^^^^^^^^^^^^ temporary value
2019-11-11T23:28:38.1934228Z +    |                                |
2019-11-11T23:28:38.1934292Z +    |                                temporary value
2019-11-11T23:28:38.1934334Z 66 
2019-11-11T23:28:38.1934377Z 67 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1934377Z 67 error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1934589Z 68   --> $DIR/raw-ref-temp.rs:25:25
2019-11-11T23:28:38.1934646Z 
2019-11-11T23:28:38.1934684Z 69    |
2019-11-11T23:28:38.1934727Z 70 LL |     let mut_field_ref = &raw mut ARRAY[1];
2019-11-11T23:28:38.1934985Z -    |                         ^^^^^^^^^^^^^^^^^ temporary value
2019-11-11T23:28:38.1935256Z +    |                                  |
2019-11-11T23:28:38.1935329Z +    |                                  temporary value
2019-11-11T23:28:38.1935370Z 72 
2019-11-11T23:28:38.1935413Z 73 error: aborting due to 12 previous errors
2019-11-11T23:28:38.1935413Z 73 error: aborting due to 12 previous errors
2019-11-11T23:28:38.1935454Z 74 
2019-11-11T23:28:38.1935499Z 
2019-11-11T23:28:38.1935524Z 
2019-11-11T23:28:38.1935567Z The actual stderr differed from the expected stderr.
2019-11-11T23:28:38.1935867Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-ref-op/raw-ref-temp/raw-ref-temp.stderr
2019-11-11T23:28:38.1936136Z To update references, rerun the tests and pass the `--bless` flag
2019-11-11T23:28:38.1936667Z To only update this specific test, also pass `--test-args raw-ref-op/raw-ref-temp.rs`
2019-11-11T23:28:38.1936762Z error: 1 errors occurred comparing output.
2019-11-11T23:28:38.1936804Z status: exit code: 1
2019-11-11T23:28:38.1936804Z status: exit code: 1
2019-11-11T23:28:38.1937753Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-ref-op/raw-ref-temp" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-ref-op/raw-ref-temp/auxiliary" "-A" "unused"
2019-11-11T23:28:38.1938233Z ------------------------------------------
2019-11-11T23:28:38.1938282Z 
2019-11-11T23:28:38.1938487Z ------------------------------------------
2019-11-11T23:28:38.1938536Z stderr:
2019-11-11T23:28:38.1938536Z stderr:
2019-11-11T23:28:38.1938756Z ------------------------------------------
2019-11-11T23:28:38.1938802Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1939202Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:12:20
2019-11-11T23:28:38.1939250Z    |
2019-11-11T23:28:38.1939326Z LL |     let ref_expr = &raw const 2;                    //~ ERROR cannot take address
2019-11-11T23:28:38.1939607Z    |                               |
2019-11-11T23:28:38.1939653Z    |                               temporary value
2019-11-11T23:28:38.1939682Z 
2019-11-11T23:28:38.1939723Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1939723Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1939978Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:13:24
2019-11-11T23:28:38.1940024Z    |
2019-11-11T23:28:38.1940072Z LL |     let mut_ref_expr = &raw mut 3;                  //~ ERROR cannot take address
2019-11-11T23:28:38.1940560Z    |                                 |
2019-11-11T23:28:38.1940760Z    |                                 temporary value
2019-11-11T23:28:38.1940788Z 
2019-11-11T23:28:38.1940846Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1940846Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1941400Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:14:21
2019-11-11T23:28:38.1941441Z    |
2019-11-11T23:28:38.1941594Z LL |     let ref_const = &raw const 4;                   //~ ERROR cannot take address
2019-11-11T23:28:38.1941872Z    |                                |
2019-11-11T23:28:38.1941932Z    |                                temporary value
2019-11-11T23:28:38.1941961Z 
2019-11-11T23:28:38.1942001Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1942001Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1942222Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:15:25
2019-11-11T23:28:38.1942281Z    |
2019-11-11T23:28:38.1942334Z LL |     let mut_ref_const = &raw mut 5;                 //~ ERROR cannot take address
2019-11-11T23:28:38.1942598Z    |                                  |
2019-11-11T23:28:38.1942640Z    |                                  temporary value
2019-11-11T23:28:38.1942668Z 
2019-11-11T23:28:38.1942706Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1942706Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1942954Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:17:26
2019-11-11T23:28:38.1942997Z    |
2019-11-11T23:28:38.1943040Z LL |     let field_ref_expr = &raw const (1, 2).0;       //~ ERROR cannot take address
2019-11-11T23:28:38.1943314Z    |                                     |
2019-11-11T23:28:38.1943357Z    |                                     temporary value
2019-11-11T23:28:38.1943401Z 
2019-11-11T23:28:38.1943440Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1943440Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1943751Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:18:30
2019-11-11T23:28:38.1943793Z    |
2019-11-11T23:28:38.1943855Z LL |     let mut_field_ref_expr = &raw mut (1, 2).0;     //~ ERROR cannot take address
2019-11-11T23:28:38.1944112Z    |                                       |
2019-11-11T23:28:38.1944182Z    |                                       temporary value
2019-11-11T23:28:38.1944210Z 
2019-11-11T23:28:38.1944249Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1944249Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1944486Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:19:21
2019-11-11T23:28:38.1944528Z    |
2019-11-11T23:28:38.1944572Z LL |     let field_ref = &raw const PAIR.0;              //~ ERROR cannot take address
2019-11-11T23:28:38.1944834Z    |                                |
2019-11-11T23:28:38.1944877Z    |                                temporary value
2019-11-11T23:28:38.1944911Z 
2019-11-11T23:28:38.1944973Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1944973Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1945195Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:20:25
2019-11-11T23:28:38.1945237Z    |
2019-11-11T23:28:38.1945298Z LL |     let mut_field_ref = &raw mut PAIR.0;            //~ ERROR cannot take address
2019-11-11T23:28:38.1945554Z    |                                  |
2019-11-11T23:28:38.1945597Z    |                                  temporary value
2019-11-11T23:28:38.1945645Z 
2019-11-11T23:28:38.1945683Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1945683Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1945902Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:22:26
2019-11-11T23:28:38.1945962Z    |
2019-11-11T23:28:38.1946007Z LL |     let field_ref_expr = &raw const [1, 2][0];       //~ ERROR cannot take address
2019-11-11T23:28:38.1946286Z    |                                     |
2019-11-11T23:28:38.1946329Z    |                                     temporary value
2019-11-11T23:28:38.1946357Z 
2019-11-11T23:28:38.1946395Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1946395Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1946634Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:23:30
2019-11-11T23:28:38.1946676Z    |
2019-11-11T23:28:38.1946786Z LL |     let mut_field_ref_expr = &raw mut [1, 2][0];     //~ ERROR cannot take address
2019-11-11T23:28:38.1947091Z    |                                       |
2019-11-11T23:28:38.1947135Z    |                                       temporary value
2019-11-11T23:28:38.1947163Z 
2019-11-11T23:28:38.1947223Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1947223Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1947443Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:24:21
2019-11-11T23:28:38.1947484Z    |
2019-11-11T23:28:38.1947547Z LL |     let field_ref = &raw const ARRAY[0];             //~ ERROR cannot take address
2019-11-11T23:28:38.1947804Z    |                                |
2019-11-11T23:28:38.1948036Z    |                                temporary value
2019-11-11T23:28:38.1948064Z 
2019-11-11T23:28:38.1948105Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1948105Z error[E0744]: cannot take address of a temporary
2019-11-11T23:28:38.1948337Z   --> /checkout/src/test/ui/raw-ref-op/raw-ref-temp.rs:25:25
2019-11-11T23:28:38.1948398Z    |
2019-11-11T23:28:38.1948444Z LL |     let mut_field_ref = &raw mut ARRAY[1];           //~ ERROR cannot take address
2019-11-11T23:28:38.1948720Z    |                                  |
2019-11-11T23:28:38.1948763Z    |                                  temporary value
2019-11-11T23:28:38.1948791Z 
2019-11-11T23:28:38.1948831Z error: aborting due to 12 previous errors
---
2019-11-11T23:28:38.1949664Z 
2019-11-11T23:28:38.1949688Z 
2019-11-11T23:28:38.1949712Z 
2019-11-11T23:28:38.1949751Z failures:
2019-11-11T23:28:38.1949979Z     [ui] ui/raw-ref-op/raw-ref-temp.rs
2019-11-11T23:28:38.1950284Z test result: FAILED. 9198 passed; 1 failed; 43 ignored; 0 measured; 0 filtered out
2019-11-11T23:28:38.1950320Z 
2019-11-11T23:28:38.1955596Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-11T23:28:38.1955668Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-11T23:28:38.1955668Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-11T23:28:38.1970208Z 
2019-11-11T23:28:38.1970286Z 
2019-11-11T23:28:38.1972220Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-11T23:28:38.1972537Z 
2019-11-11T23:28:38.1972568Z 
2019-11-11T23:28:38.1978090Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-11T23:28:38.1978158Z Build completed unsuccessfully in 1:00:30
2019-11-11T23:28:38.1978158Z Build completed unsuccessfully in 1:00:30
2019-11-11T23:28:38.2028004Z == clock drift check ==
2019-11-11T23:28:38.2042750Z   local time: Mon Nov 11 23:28:38 UTC 2019
2019-11-11T23:28:38.4839892Z   network time: Mon, 11 Nov 2019 23:28:38 GMT
2019-11-11T23:28:38.4844238Z == end clock drift check ==
2019-11-11T23:28:39.3366907Z 
2019-11-11T23:28:39.3436841Z ##[error]Bash exited with code '1'.
2019-11-11T23:28:39.3482896Z ##[section]Starting: Checkout
2019-11-11T23:28:39.3484805Z ==============================================================================
2019-11-11T23:28:39.3484851Z Task         : Get sources
2019-11-11T23:28:39.3484922Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
