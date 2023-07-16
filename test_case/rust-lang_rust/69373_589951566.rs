plain
2020-02-22T12:19:50.1307040Z ========================== Starting Command Output ===========================
2020-02-22T12:19:50.1314109Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/83441b1f-4b91-446a-91b0-159099627d88.sh
2020-02-22T12:19:50.1314625Z 
2020-02-22T12:19:50.1318819Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T12:19:50.1339515Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69373/merge to s
2020-02-22T12:19:50.1343233Z Task         : Get sources
2020-02-22T12:19:50.1343559Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T12:19:50.1343879Z Version      : 1.0.0
2020-02-22T12:19:50.1344112Z Author       : Microsoft
---
2020-02-22T12:19:51.7500873Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T12:19:51.7508137Z ##[command]git config gc.auto 0
2020-02-22T12:19:51.7511839Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T12:19:51.7515462Z ##[command]git config --get-all http.proxy
2020-02-22T12:19:51.7524196Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69373/merge:refs/remotes/pull/69373/merge
---
2020-02-22T12:24:48.5220683Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-02-22T12:24:52.4657155Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:52.4658536Z     --> src/libcore/num/mod.rs:2218:68
2020-02-22T12:24:52.4659548Z      |
2020-02-22T12:24:52.4662137Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:52.4663462Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:52.4664869Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:52.4666047Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:52.4668288Z 2218 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:52.4670023Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:52.4670896Z ...    |
2020-02-22T12:24:52.4672481Z 2370 | |     }
2020-02-22T12:24:52.4672481Z 2370 | |     }
2020-02-22T12:24:52.4673346Z 2371 | | }
2020-02-22T12:24:52.4674542Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:52.4675204Z ...
2020-02-22T12:24:52.4676411Z 2375 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2020-02-22T12:24:52.4677476Z 2376 | |     "[0x12]", "[0x12]", "", "" }
2020-02-22T12:24:52.4678765Z 
2020-02-22T12:24:52.5375365Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:52.5376059Z     --> src/libcore/num/mod.rs:2253:68
2020-02-22T12:24:52.5376594Z      |
2020-02-22T12:24:52.5376594Z      |
2020-02-22T12:24:52.5377279Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:52.5378382Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:52.5379715Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:52.5380893Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:52.5382540Z 2253 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:52.5384011Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:52.5384873Z ...    |
2020-02-22T12:24:52.5385527Z 2370 | |     }
2020-02-22T12:24:52.5385527Z 2370 | |     }
2020-02-22T12:24:52.5386387Z 2371 | | }
2020-02-22T12:24:52.5387226Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:52.5387778Z ...
2020-02-22T12:24:52.5392395Z 2375 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2020-02-22T12:24:52.5393725Z 2376 | |     "[0x12]", "[0x12]", "", "" }
2020-02-22T12:24:52.5395015Z 
2020-02-22T12:24:52.6137974Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:52.6138701Z     --> src/libcore/num/mod.rs:2287:68
2020-02-22T12:24:52.6139242Z      |
2020-02-22T12:24:52.6139242Z      |
2020-02-22T12:24:52.6139897Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:52.6140984Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:52.6143153Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:52.6144350Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:52.6146005Z 2287 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:52.6147204Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:52.6148050Z ...    |
2020-02-22T12:24:52.6148715Z 2370 | |     }
2020-02-22T12:24:52.6148715Z 2370 | |     }
2020-02-22T12:24:52.6149434Z 2371 | | }
2020-02-22T12:24:52.6150205Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:52.6150765Z ...
2020-02-22T12:24:52.6151581Z 2375 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2020-02-22T12:24:52.6152551Z 2376 | |     "[0x12]", "[0x12]", "", "" }
2020-02-22T12:24:52.6153997Z 
2020-02-22T12:24:52.6862822Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:52.6863522Z     --> src/libcore/num/mod.rs:2320:68
2020-02-22T12:24:52.6864022Z      |
2020-02-22T12:24:52.6864022Z      |
2020-02-22T12:24:52.6864864Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:52.6865999Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:52.6867316Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:52.6868505Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:52.6873285Z 2320 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:52.6874580Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:52.6875440Z ...    |
2020-02-22T12:24:52.6876082Z 2370 | |     }
2020-02-22T12:24:52.6876082Z 2370 | |     }
2020-02-22T12:24:52.6876795Z 2371 | | }
2020-02-22T12:24:52.6877572Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:52.6878116Z ...
2020-02-22T12:24:52.6878925Z 2375 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2020-02-22T12:24:52.6880062Z 2376 | |     "[0x12]", "[0x12]", "", "" }
2020-02-22T12:24:52.6881337Z 
2020-02-22T12:24:52.7579113Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:52.7579877Z     --> src/libcore/num/mod.rs:2363:68
2020-02-22T12:24:52.7580419Z      |
2020-02-22T12:24:52.7580419Z      |
2020-02-22T12:24:52.7581090Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:52.7582291Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:52.7583553Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:52.7584835Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:52.7586688Z 2363 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:52.7587895Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:52.7588728Z ...    |
2020-02-22T12:24:52.7589457Z 2370 | |     }
2020-02-22T12:24:52.7589457Z 2370 | |     }
2020-02-22T12:24:52.7590196Z 2371 | | }
2020-02-22T12:24:52.7590981Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:52.7591530Z ...
2020-02-22T12:24:52.7592369Z 2375 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2020-02-22T12:24:52.7593325Z 2376 | |     "[0x12]", "[0x12]", "", "" }
2020-02-22T12:24:52.7594579Z 
2020-02-22T12:24:53.8274056Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:53.8281883Z     --> src/libcore/num/mod.rs:2218:68
2020-02-22T12:24:53.8282831Z      |
2020-02-22T12:24:53.8282831Z      |
2020-02-22T12:24:53.8283716Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:53.8285123Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:53.8286817Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:53.8288318Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:53.8289955Z 2218 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:53.8291191Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:53.8291987Z ...    |
2020-02-22T12:24:53.8292729Z 2370 | |     }
2020-02-22T12:24:53.8292729Z 2370 | |     }
2020-02-22T12:24:53.8293441Z 2371 | | }
2020-02-22T12:24:53.8294195Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:53.8294741Z ...
2020-02-22T12:24:53.8295678Z 2381 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2020-02-22T12:24:53.8297015Z 2382 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2020-02-22T12:24:53.8298616Z 
2020-02-22T12:24:53.8299088Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:53.8299835Z     --> src/libcore/num/mod.rs:2253:68
2020-02-22T12:24:53.8300489Z      |
2020-02-22T12:24:53.8300489Z      |
2020-02-22T12:24:53.8301124Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:53.8302427Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:53.8303697Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:53.8304942Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:53.8306761Z 2253 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:53.8308064Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:53.8308939Z ...    |
2020-02-22T12:24:53.8309608Z 2370 | |     }
2020-02-22T12:24:53.8309608Z 2370 | |     }
2020-02-22T12:24:53.8310557Z 2371 | | }
2020-02-22T12:24:53.8311337Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:53.8311905Z ...
2020-02-22T12:24:53.8312769Z 2381 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2020-02-22T12:24:53.8313806Z 2382 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2020-02-22T12:24:53.8315228Z 
2020-02-22T12:24:53.8315805Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:53.8316528Z     --> src/libcore/num/mod.rs:2287:68
2020-02-22T12:24:53.8316991Z      |
2020-02-22T12:24:53.8316991Z      |
2020-02-22T12:24:53.8317715Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:53.8318791Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:53.8320202Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:53.8321618Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:53.8323277Z 2287 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:53.8324583Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:53.8325556Z ...    |
2020-02-22T12:24:53.8326314Z 2370 | |     }
2020-02-22T12:24:53.8326314Z 2370 | |     }
2020-02-22T12:24:53.8327009Z 2371 | | }
2020-02-22T12:24:53.8327892Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:53.8328460Z ...
2020-02-22T12:24:53.8329315Z 2381 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2020-02-22T12:24:53.8330357Z 2382 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2020-02-22T12:24:53.8331774Z 
2020-02-22T12:24:53.8332243Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:53.8332887Z     --> src/libcore/num/mod.rs:2320:68
2020-02-22T12:24:53.8333380Z      |
2020-02-22T12:24:53.8333380Z      |
2020-02-22T12:24:53.8334035Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:53.8335148Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:53.8336545Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:53.8337786Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:53.8339468Z 2320 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:53.8340663Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:53.8341566Z ...    |
2020-02-22T12:24:53.8342192Z 2370 | |     }
2020-02-22T12:24:53.8342192Z 2370 | |     }
2020-02-22T12:24:53.8342867Z 2371 | | }
2020-02-22T12:24:53.8343731Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:53.8344391Z ...
2020-02-22T12:24:53.8345218Z 2381 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2020-02-22T12:24:53.8346350Z 2382 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2020-02-22T12:24:53.8347728Z 
2020-02-22T12:24:53.8348336Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:53.8348938Z     --> src/libcore/num/mod.rs:2363:68
2020-02-22T12:24:53.8349415Z      |
2020-02-22T12:24:53.8349415Z      |
2020-02-22T12:24:53.8350061Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:53.8351286Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:53.8352587Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:53.8353778Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:53.8355485Z 2363 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:53.8356821Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:53.8357658Z ...    |
2020-02-22T12:24:53.8358327Z 2370 | |     }
2020-02-22T12:24:53.8358327Z 2370 | |     }
2020-02-22T12:24:53.8359049Z 2371 | | }
2020-02-22T12:24:53.8400408Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:53.8404416Z ...
2020-02-22T12:24:53.8405332Z 2381 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2020-02-22T12:24:53.8406547Z 2382 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2020-02-22T12:24:53.8407988Z 
2020-02-22T12:24:53.8408483Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:53.8409347Z     --> src/libcore/num/mod.rs:2218:68
2020-02-22T12:24:53.8409853Z      |
2020-02-22T12:24:53.8409853Z      |
2020-02-22T12:24:53.8410531Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:53.8411707Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:53.8413262Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:53.8414947Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:53.8421091Z 2218 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:53.8422774Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:53.8426722Z ...    |
2020-02-22T12:24:53.8427416Z 2370 | |     }
2020-02-22T12:24:53.8427416Z 2370 | |     }
2020-02-22T12:24:53.8428349Z 2371 | | }
2020-02-22T12:24:53.8429202Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:53.8429756Z ...
2020-02-22T12:24:53.8430608Z 2387 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2020-02-22T12:24:53.8431716Z 2388 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:24:53.8432679Z 2389 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
2020-02-22T12:24:53.8434010Z 
2020-02-22T12:24:53.8434696Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:53.8435301Z     --> src/libcore/num/mod.rs:2253:68
2020-02-22T12:24:53.8435813Z      |
2020-02-22T12:24:53.8435813Z      |
2020-02-22T12:24:53.8436470Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:53.8437655Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:53.8438941Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:53.8440587Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:53.8442203Z 2253 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:53.8443498Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:53.8444284Z ...    |
2020-02-22T12:24:53.8445190Z 2370 | |     }
2020-02-22T12:24:53.8445190Z 2370 | |     }
2020-02-22T12:24:53.8445904Z 2371 | | }
2020-02-22T12:24:53.8446653Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:53.8447179Z ...
2020-02-22T12:24:53.8448196Z 2387 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2020-02-22T12:24:53.8449241Z 2388 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:24:53.8450411Z 2389 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
2020-02-22T12:24:53.8451735Z 
2020-02-22T12:24:53.8452223Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:53.8452950Z     --> src/libcore/num/mod.rs:2287:68
2020-02-22T12:24:53.8453430Z      |
2020-02-22T12:24:53.8453430Z      |
2020-02-22T12:24:53.8454082Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:53.8455370Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:53.8456626Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:53.8457901Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:53.8459555Z 2287 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:53.8460892Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:53.8461729Z ...    |
2020-02-22T12:24:53.8462397Z 2370 | |     }
2020-02-22T12:24:53.8462397Z 2370 | |     }
2020-02-22T12:24:53.8463122Z 2371 | | }
2020-02-22T12:24:53.8464001Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:53.8464669Z ...
2020-02-22T12:24:53.8465717Z 2387 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2020-02-22T12:24:53.8466818Z 2388 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:24:53.8467769Z 2389 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
2020-02-22T12:24:53.8469279Z 
2020-02-22T12:24:53.8469732Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:53.8470456Z     --> src/libcore/num/mod.rs:2320:68
2020-02-22T12:24:53.8470972Z      |
2020-02-22T12:24:53.8470972Z      |
2020-02-22T12:24:53.8471700Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:53.8472826Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:53.8474152Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:53.8475535Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:53.8477181Z 2320 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:53.8478600Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:53.8479400Z ...    |
2020-02-22T12:24:53.8480160Z 2370 | |     }
2020-02-22T12:24:53.8480160Z 2370 | |     }
2020-02-22T12:24:53.8481118Z 2371 | | }
2020-02-22T12:24:53.8481899Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:53.8482445Z ...
2020-02-22T12:24:53.8483403Z 2387 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2020-02-22T12:24:53.8484568Z 2388 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:24:53.8485521Z 2389 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
2020-02-22T12:24:53.8486855Z 
2020-02-22T12:24:53.8487320Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:53.8487956Z     --> src/libcore/num/mod.rs:2363:68
2020-02-22T12:24:53.8488451Z      |
2020-02-22T12:24:53.8488451Z      |
2020-02-22T12:24:53.8489107Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:53.8490222Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:53.8491527Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:53.8492852Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:53.8494474Z 2363 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:53.8495786Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:53.8496658Z ...    |
2020-02-22T12:24:53.8497330Z 2370 | |     }
2020-02-22T12:24:53.8497330Z 2370 | |     }
2020-02-22T12:24:53.8498050Z 2371 | | }
2020-02-22T12:24:53.8498837Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:53.8499401Z ...
2020-02-22T12:24:53.8500238Z 2387 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2020-02-22T12:24:53.8501416Z 2388 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:24:53.8502375Z 2389 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
2020-02-22T12:24:53.8503710Z 
2020-02-22T12:24:53.8504179Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:53.8504796Z     --> src/libcore/num/mod.rs:2218:68
2020-02-22T12:24:53.8505309Z      |
2020-02-22T12:24:53.8505309Z      |
2020-02-22T12:24:53.8505965Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:53.8507055Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:53.8508379Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:53.8509545Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:53.8511188Z 2218 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:53.8512408Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:53.8513267Z ...    |
2020-02-22T12:24:53.8513918Z 2370 | |     }
2020-02-22T12:24:53.8513918Z 2370 | |     }
2020-02-22T12:24:53.8514635Z 2371 | | }
2020-02-22T12:24:53.8515564Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:53.8516114Z ...
2020-02-22T12:24:53.8516935Z 2394 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2020-02-22T12:24:53.8518131Z 2395 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2020-02-22T12:24:53.8519242Z 2396 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:24:53.8520439Z 2397 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2020-02-22T12:24:53.8521920Z 
2020-02-22T12:24:53.8522403Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:53.8523025Z     --> src/libcore/num/mod.rs:2253:68
2020-02-22T12:24:53.8523523Z      |
2020-02-22T12:24:53.8523523Z      |
2020-02-22T12:24:53.8524198Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:53.8525293Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:53.8526597Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:53.8527789Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:53.8529427Z 2253 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:53.8530653Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:53.8531491Z ...    |
2020-02-22T12:24:53.8532157Z 2370 | |     }
2020-02-22T12:24:53.8532157Z 2370 | |     }
2020-02-22T12:24:53.8532880Z 2371 | | }
2020-02-22T12:24:53.8533671Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:53.8534215Z ...
2020-02-22T12:24:53.8535038Z 2394 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2020-02-22T12:24:53.8536155Z 2395 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2020-02-22T12:24:53.8537228Z 2396 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:24:53.8538386Z 2397 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2020-02-22T12:24:53.8539863Z 
2020-02-22T12:24:53.8540331Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:53.8541041Z     --> src/libcore/num/mod.rs:2287:68
2020-02-22T12:24:53.8541571Z      |
2020-02-22T12:24:53.8541571Z      |
2020-02-22T12:24:53.8542225Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:53.8543338Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:53.8544648Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:53.8545832Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:53.8547461Z 2287 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:53.8548778Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:53.8549626Z ...    |
2020-02-22T12:24:53.8550296Z 2370 | |     }
2020-02-22T12:24:53.8550296Z 2370 | |     }
2020-02-22T12:24:53.8551011Z 2371 | | }
2020-02-22T12:24:53.8551786Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:53.8552349Z ...
2020-02-22T12:24:53.8553175Z 2394 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2020-02-22T12:24:53.8554287Z 2395 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2020-02-22T12:24:53.8555366Z 2396 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:24:53.8556390Z 2397 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2020-02-22T12:24:53.8558055Z 
2020-02-22T12:24:53.8558648Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:53.8559272Z     --> src/libcore/num/mod.rs:2320:68
2020-02-22T12:24:53.8600349Z      |
2020-02-22T12:24:53.8600349Z      |
2020-02-22T12:24:53.8601269Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:53.8602864Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:53.8604469Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:53.8606139Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:53.8607994Z 2320 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:53.8609324Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:53.8610150Z ...    |
2020-02-22T12:24:53.8611009Z 2370 | |     }
2020-02-22T12:24:53.8611009Z 2370 | |     }
2020-02-22T12:24:53.8611711Z 2371 | | }
2020-02-22T12:24:53.8612445Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:53.8613147Z ...
2020-02-22T12:24:53.8613952Z 2394 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2020-02-22T12:24:53.8615008Z 2395 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2020-02-22T12:24:53.8616218Z 2396 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:24:53.8617175Z 2397 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2020-02-22T12:24:53.8618745Z 
2020-02-22T12:24:53.9094593Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:53.9095727Z     --> src/libcore/num/mod.rs:2363:68
2020-02-22T12:24:53.9096642Z      |
2020-02-22T12:24:53.9096642Z      |
2020-02-22T12:24:53.9097750Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:53.9099234Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:53.9102703Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:53.9104460Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:53.9106800Z 2363 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:53.9108541Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:53.9109735Z ...    |
2020-02-22T12:24:53.9110528Z 2370 | |     }
2020-02-22T12:24:53.9110528Z 2370 | |     }
2020-02-22T12:24:53.9185747Z 2371 | | }
2020-02-22T12:24:53.9235503Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:53.9244224Z ...
2020-02-22T12:24:53.9245114Z 2394 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2020-02-22T12:24:53.9246269Z 2395 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2020-02-22T12:24:53.9247352Z 2396 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:24:53.9248409Z 2397 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2020-02-22T12:24:53.9249953Z 
2020-02-22T12:24:54.0057300Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:54.0057998Z     --> src/libcore/num/mod.rs:2218:68
2020-02-22T12:24:54.0058518Z      |
2020-02-22T12:24:54.0058518Z      |
2020-02-22T12:24:54.0059161Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:54.0060332Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:54.0061712Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:54.0062852Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:54.0065337Z 2218 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:54.0066585Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:54.0067436Z ...    |
2020-02-22T12:24:54.0068117Z 2370 | |     }
2020-02-22T12:24:54.0068117Z 2370 | |     }
2020-02-22T12:24:54.0068846Z 2371 | | }
2020-02-22T12:24:54.0069630Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:54.0070415Z ...
2020-02-22T12:24:54.0071223Z 2402 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2020-02-22T12:24:54.0072955Z 2403 | |     170141183460469231731687303715884105727, "", "", 16,
2020-02-22T12:24:54.0074141Z 2404 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2020-02-22T12:24:54.0075375Z 2405 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2020-02-22T12:24:54.0076075Z ...    |
2020-02-22T12:24:54.0076945Z 2408 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2020-02-22T12:24:54.0078046Z 2409 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2020-02-22T12:24:54.0079511Z 
2020-02-22T12:24:54.0938612Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:54.0940296Z     --> src/libcore/num/mod.rs:2253:68
2020-02-22T12:24:54.0940881Z      |
2020-02-22T12:24:54.0940881Z      |
2020-02-22T12:24:54.0941692Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:54.0943423Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:54.0944878Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:54.0946028Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:54.0947794Z 2253 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:54.0949018Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:54.0950003Z ...    |
2020-02-22T12:24:54.0950642Z 2370 | |     }
2020-02-22T12:24:54.0950642Z 2370 | |     }
2020-02-22T12:24:54.0951501Z 2371 | | }
2020-02-22T12:24:54.0952296Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:54.0952850Z ...
2020-02-22T12:24:54.0953670Z 2402 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2020-02-22T12:24:54.0954683Z 2403 | |     170141183460469231731687303715884105727, "", "", 16,
2020-02-22T12:24:54.0956424Z 2404 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2020-02-22T12:24:54.0957541Z 2405 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2020-02-22T12:24:54.0958384Z ...    |
2020-02-22T12:24:54.0959253Z 2408 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2020-02-22T12:24:54.0960422Z 2409 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2020-02-22T12:24:54.0961925Z 
2020-02-22T12:24:54.1780512Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:54.1781337Z     --> src/libcore/num/mod.rs:2287:68
2020-02-22T12:24:54.1781835Z      |
2020-02-22T12:24:54.1781835Z      |
2020-02-22T12:24:54.1782639Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:54.1783859Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:54.1785244Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:54.1787271Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:54.1788957Z 2287 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:54.1790123Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:54.1791058Z ...    |
2020-02-22T12:24:54.1791830Z 2370 | |     }
2020-02-22T12:24:54.1791830Z 2370 | |     }
2020-02-22T12:24:54.1792579Z 2371 | | }
2020-02-22T12:24:54.1793759Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:54.1794363Z ...
2020-02-22T12:24:54.1795172Z 2402 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2020-02-22T12:24:54.1796287Z 2403 | |     170141183460469231731687303715884105727, "", "", 16,
2020-02-22T12:24:54.1797340Z 2404 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2020-02-22T12:24:54.1798538Z 2405 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2020-02-22T12:24:54.1799444Z ...    |
2020-02-22T12:24:54.1800341Z 2408 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2020-02-22T12:24:54.1801577Z 2409 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2020-02-22T12:24:54.1803041Z 
2020-02-22T12:24:54.2500400Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:54.2501056Z     --> src/libcore/num/mod.rs:2320:68
2020-02-22T12:24:54.2501559Z      |
2020-02-22T12:24:54.2501559Z      |
2020-02-22T12:24:54.2502327Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:54.2503394Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:54.2504687Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:54.2505969Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:54.2508477Z 2320 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
2020-02-22T12:24:54.2509716Z      | |                                                                    ^^^^^^^^^^^^^^^^ expected one of `since`, `note`
2020-02-22T12:24:54.2510907Z ...    |
2020-02-22T12:24:54.2512063Z 2370 | |     }
2020-02-22T12:24:54.2512063Z 2370 | |     }
2020-02-22T12:24:54.2512841Z 2371 | | }
2020-02-22T12:24:54.2514178Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:24:54.2514938Z ...
2020-02-22T12:24:54.2515896Z 2402 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2020-02-22T12:24:54.2516914Z 2403 | |     170141183460469231731687303715884105727, "", "", 16,
2020-02-22T12:24:54.2518011Z 2404 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2020-02-22T12:24:54.2519122Z 2405 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2020-02-22T12:24:54.2520257Z ...    |
2020-02-22T12:24:54.2520985Z 2408 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2020-02-22T12:24:54.2522101Z 2409 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2020-02-22T12:24:54.2523633Z 
2020-02-22T12:24:54.3416452Z error[E0541]: unknown meta item 'issue'
2020-02-22T12:24:54.3417608Z     --> src/libcore/num/mod.rs:2363:68
2020-02-22T12:24:54.3422011Z      |
2020-02-22T12:24:54.3422011Z      |
2020-02-22T12:24:54.3423003Z 256  | / macro_rules! int_impl {
2020-02-22T12:24:54.3424230Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:24:54.3425852Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:24:54.3427499Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:24:54.3429658Z 2363 | |             #[rustc_const_stable(feature = "const_int_conversion", issue = "1.43.0")]
---
2020-02-22T12:25:04.3880521Z   local time: Sat Feb 22 12:25:04 UTC 2020
2020-02-22T12:25:04.6767618Z   network time: Sat, 22 Feb 2020 12:25:04 GMT
2020-02-22T12:25:04.6768549Z == end clock drift check ==
2020-02-22T12:25:05.4562606Z 
2020-02-22T12:25:05.4657008Z ##[error]Bash exited with code '1'.
2020-02-22T12:25:05.4674695Z ##[section]Finishing: Run build
2020-02-22T12:25:05.4723745Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69373/merge to s
2020-02-22T12:25:05.4730422Z Task         : Get sources
2020-02-22T12:25:05.4730773Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T12:25:05.4731097Z Version      : 1.0.0
2020-02-22T12:25:05.4731335Z Author       : Microsoft
2020-02-22T12:25:05.4731335Z Author       : Microsoft
2020-02-22T12:25:05.4731764Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-22T12:25:05.4732166Z ==============================================================================
2020-02-22T12:25:05.8373368Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-22T12:25:05.8425518Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69373/merge to s
2020-02-22T12:25:05.8519377Z Cleaning up task key
2020-02-22T12:25:05.8520954Z Start cleaning up orphan processes.
2020-02-22T12:25:05.8726943Z Terminate orphan process: pid (3596) (python)
2020-02-22T12:25:05.8869316Z ##[section]Finishing: Finalize Job
