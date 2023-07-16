plain
2020-02-08T18:08:32.1161508Z - error[E0623]: lifetime mismatch
2020-02-08T18:08:32.1161704Z + error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-02-08T18:08:32.1162400Z +   --> $DIR/ref-self-async.rs:22:42
2020-02-08T18:08:32.1162576Z +    |
2020-02-08T18:08:32.1163140Z + LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2020-02-08T18:08:32.1163678Z +    |
2020-02-08T18:08:32.1164047Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2020-02-08T18:08:32.1164238Z + 
2020-02-08T18:08:32.1164400Z + error: lifetime may not live long enough
2020-02-08T18:08:32.1164400Z + error: lifetime may not live long enough
2020-02-08T18:08:32.1164727Z 2   --> $DIR/ref-self-async.rs:23:9
2020-02-08T18:08:32.1164895Z 3    |
2020-02-08T18:08:32.1165226Z 4 LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2020-02-08T18:08:32.1165708Z -    |                       -----              ----
2020-02-08T18:08:32.1166624Z +    |                       -         - let's call the lifetime of this reference `'1`
2020-02-08T18:08:32.1166822Z 6    |                       |
2020-02-08T18:08:32.1167189Z -    |                       this parameter and the return type are declared with different lifetimes...
2020-02-08T18:08:32.1167189Z -    |                       this parameter and the return type are declared with different lifetimes...
2020-02-08T18:08:32.1169077Z +    |                       let's call the lifetime of this reference `'2`
2020-02-08T18:08:32.1169786Z 8 LL |         f
2020-02-08T18:08:32.1170260Z -    |         ^ ...but data from `f` is returned here
2020-02-08T18:08:32.1170798Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2020-02-08T18:08:32.1171706Z - error[E0623]: lifetime mismatch
2020-02-08T18:08:32.1172120Z + error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-02-08T18:08:32.1173485Z +   --> $DIR/ref-self-async.rs:28:48
2020-02-08T18:08:32.1173719Z +    |
2020-02-08T18:08:32.1173719Z +    |
2020-02-08T18:08:32.1174108Z + LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2020-02-08T18:08:32.1174471Z +    |
2020-02-08T18:08:32.1174838Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2020-02-08T18:08:32.1175033Z + 
2020-02-08T18:08:32.1175175Z + error: lifetime may not live long enough
2020-02-08T18:08:32.1175175Z + error: lifetime may not live long enough
2020-02-08T18:08:32.1175711Z 12   --> $DIR/ref-self-async.rs:29:9
2020-02-08T18:08:32.1175910Z 13    |
2020-02-08T18:08:32.1176258Z 14 LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2020-02-08T18:08:32.1177110Z -    |                             -----              ----
2020-02-08T18:08:32.1177503Z +    |                             -         - let's call the lifetime of this reference `'1`
2020-02-08T18:08:32.1177702Z 16    |                             |
2020-02-08T18:08:32.1178249Z -    |                             this parameter and the return type are declared with different lifetimes...
2020-02-08T18:08:32.1178249Z -    |                             this parameter and the return type are declared with different lifetimes...
2020-02-08T18:08:32.1178670Z +    |                             let's call the lifetime of this reference `'2`
2020-02-08T18:08:32.1181409Z 18 LL |         f
2020-02-08T18:08:32.1181861Z -    |         ^ ...but data from `f` is returned here
2020-02-08T18:08:32.1182296Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2020-02-08T18:08:32.1183173Z - error[E0623]: lifetime mismatch
2020-02-08T18:08:32.1183407Z + error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-02-08T18:08:32.1183917Z +   --> $DIR/ref-self-async.rs:32:57
2020-02-08T18:08:32.1184117Z +    |
2020-02-08T18:08:32.1184117Z +    |
2020-02-08T18:08:32.1184454Z + LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2020-02-08T18:08:32.1184836Z +    |
2020-02-08T18:08:32.1185354Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2020-02-08T18:08:32.1185546Z + 
2020-02-08T18:08:32.1185710Z + error: lifetime may not live long enough
2020-02-08T18:08:32.1185710Z + error: lifetime may not live long enough
2020-02-08T18:08:32.1186480Z 22   --> $DIR/ref-self-async.rs:33:9
2020-02-08T18:08:32.1186687Z 23    |
2020-02-08T18:08:32.1187652Z 24 LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2020-02-08T18:08:32.1188396Z -    |                                     -----               ----
2020-02-08T18:08:32.1188866Z +    |                                     -          - let's call the lifetime of this reference `'1`
2020-02-08T18:08:32.1189138Z 26    |                                     |
2020-02-08T18:08:32.1189741Z -    |                                     this parameter and the return type are declared with different lifetimes...
2020-02-08T18:08:32.1189741Z -    |                                     this parameter and the return type are declared with different lifetimes...
2020-02-08T18:08:32.1190349Z +    |                                     let's call the lifetime of this reference `'2`
2020-02-08T18:08:32.1192365Z 28 LL |         f
2020-02-08T18:08:32.1192780Z -    |         ^ ...but data from `f` is returned here
2020-02-08T18:08:32.1193433Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2020-02-08T18:08:32.1194125Z - error[E0623]: lifetime mismatch
2020-02-08T18:08:32.1194330Z + error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-02-08T18:08:32.1194669Z +   --> $DIR/ref-self-async.rs:36:57
2020-02-08T18:08:32.1194863Z +    |
2020-02-08T18:08:32.1194863Z +    |
2020-02-08T18:08:32.1195573Z + LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2020-02-08T18:08:32.1195962Z +    |
2020-02-08T18:08:32.1196655Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2020-02-08T18:08:32.1200435Z + 
2020-02-08T18:08:32.1200874Z + error: lifetime may not live long enough
2020-02-08T18:08:32.1200874Z + error: lifetime may not live long enough
2020-02-08T18:08:32.1201324Z 32   --> $DIR/ref-self-async.rs:37:9
2020-02-08T18:08:32.1201504Z 33    |
2020-02-08T18:08:32.1202027Z 34 LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2020-02-08T18:08:32.1204087Z -    |                                     -----               ----
2020-02-08T18:08:32.1204372Z +    |                                     -          - let's call the lifetime of this reference `'1`
2020-02-08T18:08:32.1204468Z 36    |                                     |
2020-02-08T18:08:32.1204754Z -    |                                     this parameter and the return type are declared with different lifetimes...
2020-02-08T18:08:32.1204754Z -    |                                     this parameter and the return type are declared with different lifetimes...
2020-02-08T18:08:32.1205047Z +    |                                     let's call the lifetime of this reference `'2`
2020-02-08T18:08:32.1205119Z 38 LL |         f
2020-02-08T18:08:32.1205345Z -    |         ^ ...but data from `f` is returned here
2020-02-08T18:08:32.1205616Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2020-02-08T18:08:32.1205909Z - error[E0623]: lifetime mismatch
2020-02-08T18:08:32.1205986Z + error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-02-08T18:08:32.1206370Z +   --> $DIR/ref-self-async.rs:40:66
2020-02-08T18:08:32.1206427Z +    |
2020-02-08T18:08:32.1206427Z +    |
2020-02-08T18:08:32.1206668Z + LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2020-02-08T18:08:32.1206961Z +    |
2020-02-08T18:08:32.1207270Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2020-02-08T18:08:32.1207353Z + 
2020-02-08T18:08:32.1207404Z + error: lifetime may not live long enough
2020-02-08T18:08:32.1207404Z + error: lifetime may not live long enough
2020-02-08T18:08:32.1207612Z 42   --> $DIR/ref-self-async.rs:41:9
2020-02-08T18:08:32.1207684Z 43    |
2020-02-08T18:08:32.1208138Z 44 LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2020-02-08T18:08:32.1208461Z -    |                                             -----                ----
2020-02-08T18:08:32.1209654Z +    |                                             -           - let's call the lifetime of this reference `'1`
2020-02-08T18:08:32.1209777Z 46    |                                             |
2020-02-08T18:08:32.1210151Z -    |                                             this parameter and the return type are declared with different lifetimes...
2020-02-08T18:08:32.1210151Z -    |                                             this parameter and the return type are declared with different lifetimes...
2020-02-08T18:08:32.1210439Z +    |                                             let's call the lifetime of this reference `'2`
2020-02-08T18:08:32.1210537Z 48 LL |         f
2020-02-08T18:08:32.1210743Z -    |         ^ ...but data from `f` is returned here
2020-02-08T18:08:32.1211027Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2020-02-08T18:08:32.1211297Z - error[E0623]: lifetime mismatch
2020-02-08T18:08:32.1211386Z + error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-02-08T18:08:32.1211587Z +   --> $DIR/ref-self-async.rs:44:66
2020-02-08T18:08:32.1211660Z +    |
2020-02-08T18:08:32.1211660Z +    |
2020-02-08T18:08:32.1212221Z + LL |     async fn box_pin_ref_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2020-02-08T18:08:32.1212376Z +    |
2020-02-08T18:08:32.1212617Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2020-02-08T18:08:32.1212681Z + 
2020-02-08T18:08:32.1212754Z + error: lifetime may not live long enough
2020-02-08T18:08:32.1212754Z + error: lifetime may not live long enough
2020-02-08T18:08:32.1213126Z 52   --> $DIR/ref-self-async.rs:45:9
2020-02-08T18:08:32.1213202Z 53    |
2020-02-08T18:08:32.1213444Z 54 LL |     async fn box_pin_ref_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2020-02-08T18:08:32.1213716Z -    |                                             -----                ----
2020-02-08T18:08:32.1214003Z +    |                                             -           - let's call the lifetime of this reference `'1`
2020-02-08T18:08:32.1214102Z 56    |                                             |
2020-02-08T18:08:32.1214564Z -    |                                             this parameter and the return type are declared with different lifetimes...
2020-02-08T18:08:32.1214564Z -    |                                             this parameter and the return type are declared with different lifetimes...
2020-02-08T18:08:32.1214875Z +    |                                             let's call the lifetime of this reference `'2`
2020-02-08T18:08:32.1215383Z 58 LL |         f
2020-02-08T18:08:32.1215672Z -    |         ^ ...but data from `f` is returned here
2020-02-08T18:08:32.1216147Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2020-02-08T18:08:32.1216607Z - error[E0623]: lifetime mismatch
2020-02-08T18:08:32.1216682Z + error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-02-08T18:08:32.1216904Z +   --> $DIR/ref-self-async.rs:48:69
2020-02-08T18:08:32.1216962Z +    |
2020-02-08T18:08:32.1216962Z +    |
2020-02-08T18:08:32.1217205Z + LL |     async fn wrap_ref_Self_Self(self: Wrap<&Self, Self>, f: &u8) -> &u8 {
2020-02-08T18:08:32.1217367Z +    |
2020-02-08T18:08:32.1217590Z +    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2020-02-08T18:08:32.1217672Z + 
2020-02-08T18:08:32.1217848Z + error: lifetime may not live long enough
2020-02-08T18:08:32.1217848Z + error: lifetime may not live long enough
2020-02-08T18:08:32.1218265Z 62   --> $DIR/ref-self-async.rs:49:9
2020-02-08T18:08:32.1218412Z 63    |
2020-02-08T18:08:32.1218658Z 64 LL |     async fn wrap_ref_Self_Self(self: Wrap<&Self, Self>, f: &u8) -> &u8 {
2020-02-08T18:08:32.1218945Z -    |                                            -----                    ---
2020-02-08T18:08:32.1219212Z +    |                                            -                - let's call the lifetime of this reference `'1`
2020-02-08T18:08:32.1219313Z 66    |                                            |
2020-02-08T18:08:32.1219601Z -    |                                            this parameter and the return type are declared with different lifetimes...
2020-02-08T18:08:32.1219601Z -    |                                            this parameter and the return type are declared with different lifetimes...
2020-02-08T18:08:32.1219869Z +    |                                            let's call the lifetime of this reference `'2`
2020-02-08T18:08:32.1219955Z 68 LL |         f
2020-02-08T18:08:32.1220154Z -    |         ^ ...but data from `f` is returned here
2020-02-08T18:08:32.1220443Z +    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2020-02-08T18:08:32.1220729Z - error: aborting due to 7 previous errors
2020-02-08T18:08:32.1220806Z + error: aborting due to 14 previous errors
2020-02-08T18:08:32.1220861Z 72 
2020-02-08T18:08:32.1221088Z - For more information about this error, try `rustc --explain E0623`.
2020-02-08T18:08:32.1221088Z - For more information about this error, try `rustc --explain E0623`.
2020-02-08T18:08:32.1221314Z + For more information about this error, try `rustc --explain E0700`.
2020-02-08T18:08:32.1221392Z 74 
2020-02-08T18:08:32.1221420Z 
2020-02-08T18:08:32.1221448Z 
2020-02-08T18:08:32.1221519Z The actual stderr differed from the expected stderr.
2020-02-08T18:08:32.1222137Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self-async.nll/ref-self-async.nll.stderr
2020-02-08T18:08:32.1222390Z To update references, rerun the tests and pass the `--bless` flag
2020-02-08T18:08:32.1222652Z To only update this specific test, also pass `--test-args self/elision/ref-self-async.rs`
2020-02-08T18:08:32.1222760Z error: 1 errors occurred comparing output.
2020-02-08T18:08:32.1222832Z status: exit code: 1
2020-02-08T18:08:32.1222832Z status: exit code: 1
2020-02-08T18:08:32.1223863Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self-async.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self-async.nll/auxiliary" "-A" "unused"
2020-02-08T18:08:32.1224310Z ------------------------------------------
2020-02-08T18:08:32.1224541Z 
2020-02-08T18:08:32.1224743Z ------------------------------------------
2020-02-08T18:08:32.1224822Z stderr:
2020-02-08T18:08:32.1224822Z stderr:
2020-02-08T18:08:32.1225017Z ------------------------------------------
2020-02-08T18:08:32.1225114Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-02-08T18:08:32.1225371Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:22:42
2020-02-08T18:08:32.1225439Z    |
2020-02-08T18:08:32.1225667Z LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2020-02-08T18:08:32.1225813Z    |
2020-02-08T18:08:32.1226041Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2020-02-08T18:08:32.1226264Z 
2020-02-08T18:08:32.1226315Z error: lifetime may not live long enough
2020-02-08T18:08:32.1226315Z error: lifetime may not live long enough
2020-02-08T18:08:32.1226704Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:23:9
2020-02-08T18:08:32.1226765Z    |
2020-02-08T18:08:32.1227051Z LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2020-02-08T18:08:32.1227469Z    |                       |
2020-02-08T18:08:32.1227710Z    |                       let's call the lifetime of this reference `'2`
2020-02-08T18:08:32.1227795Z LL |         f //~ ERROR lifetime mismatch
2020-02-08T18:08:32.1227795Z LL |         f //~ ERROR lifetime mismatch
2020-02-08T18:08:32.1228253Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2020-02-08T18:08:32.1228374Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-02-08T18:08:32.1228624Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:28:48
2020-02-08T18:08:32.1228703Z    |
2020-02-08T18:08:32.1228703Z    |
2020-02-08T18:08:32.1228911Z LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2020-02-08T18:08:32.1229064Z    |
2020-02-08T18:08:32.1229305Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2020-02-08T18:08:32.1229360Z 
2020-02-08T18:08:32.1229427Z error: lifetime may not live long enough
2020-02-08T18:08:32.1229427Z error: lifetime may not live long enough
2020-02-08T18:08:32.1229649Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:29:9
2020-02-08T18:08:32.1229727Z    |
2020-02-08T18:08:32.1229932Z LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2020-02-08T18:08:32.1230606Z    |                             |
2020-02-08T18:08:32.1230903Z    |                             let's call the lifetime of this reference `'2`
2020-02-08T18:08:32.1230976Z LL |         f //~ ERROR lifetime mismatch
2020-02-08T18:08:32.1230976Z LL |         f //~ ERROR lifetime mismatch
2020-02-08T18:08:32.1231428Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2020-02-08T18:08:32.1231576Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-02-08T18:08:32.1232002Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:32:57
2020-02-08T18:08:32.1232075Z    |
2020-02-08T18:08:32.1232075Z    |
2020-02-08T18:08:32.1232475Z LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2020-02-08T18:08:32.1232628Z    |
2020-02-08T18:08:32.1232849Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2020-02-08T18:08:32.1233094Z 
2020-02-08T18:08:32.1233146Z error: lifetime may not live long enough
2020-02-08T18:08:32.1233146Z error: lifetime may not live long enough
2020-02-08T18:08:32.1233387Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:33:9
2020-02-08T18:08:32.1233451Z    |
2020-02-08T18:08:32.1233854Z LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2020-02-08T18:08:32.1234214Z    |                                     |
2020-02-08T18:08:32.1234456Z    |                                     let's call the lifetime of this reference `'2`
2020-02-08T18:08:32.1234553Z LL |         f //~ ERROR lifetime mismatch
2020-02-08T18:08:32.1234553Z LL |         f //~ ERROR lifetime mismatch
2020-02-08T18:08:32.1234835Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2020-02-08T18:08:32.1234956Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-02-08T18:08:32.1235205Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:36:57
2020-02-08T18:08:32.1235283Z    |
2020-02-08T18:08:32.1235283Z    |
2020-02-08T18:08:32.1235497Z LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2020-02-08T18:08:32.1235647Z    |
2020-02-08T18:08:32.1235885Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2020-02-08T18:08:32.1235933Z 
2020-02-08T18:08:32.1236256Z error: lifetime may not live long enough
2020-02-08T18:08:32.1236256Z error: lifetime may not live long enough
2020-02-08T18:08:32.1236671Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:37:9
2020-02-08T18:08:32.1236816Z    |
2020-02-08T18:08:32.1237041Z LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2020-02-08T18:08:32.1237378Z    |                                     |
2020-02-08T18:08:32.1237621Z    |                                     let's call the lifetime of this reference `'2`
2020-02-08T18:08:32.1237689Z LL |         f //~ ERROR lifetime mismatch
2020-02-08T18:08:32.1237689Z LL |         f //~ ERROR lifetime mismatch
2020-02-08T18:08:32.1238137Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2020-02-08T18:08:32.1238269Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-02-08T18:08:32.1238504Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:40:66
2020-02-08T18:08:32.1238583Z    |
2020-02-08T18:08:32.1238583Z    |
2020-02-08T18:08:32.1238820Z LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2020-02-08T18:08:32.1238980Z    |
2020-02-08T18:08:32.1239195Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2020-02-08T18:08:32.1239256Z 
2020-02-08T18:08:32.1239306Z error: lifetime may not live long enough
2020-02-08T18:08:32.1239306Z error: lifetime may not live long enough
2020-02-08T18:08:32.1239535Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:41:9
2020-02-08T18:08:32.1239596Z    |
2020-02-08T18:08:32.1239833Z LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2020-02-08T18:08:32.1240191Z    |                                             |
2020-02-08T18:08:32.1240439Z    |                                             let's call the lifetime of this reference `'2`
2020-02-08T18:08:32.1240528Z LL |         f //~ ERROR lifetime mismatch
2020-02-08T18:08:32.1240528Z LL |         f //~ ERROR lifetime mismatch
2020-02-08T18:08:32.1240812Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2020-02-08T18:08:32.1240928Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-02-08T18:08:32.1241172Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:44:66
2020-02-08T18:08:32.1241248Z    |
2020-02-08T18:08:32.1241248Z    |
2020-02-08T18:08:32.1241464Z LL |     async fn box_pin_ref_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2020-02-08T18:08:32.1241618Z    |
2020-02-08T18:08:32.1242191Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2020-02-08T18:08:32.1242235Z 
2020-02-08T18:08:32.1242301Z error: lifetime may not live long enough
2020-02-08T18:08:32.1242301Z error: lifetime may not live long enough
2020-02-08T18:08:32.1242519Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:45:9
2020-02-08T18:08:32.1242602Z    |
2020-02-08T18:08:32.1242822Z LL |     async fn box_pin_ref_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2020-02-08T18:08:32.1243367Z    |                                             |
2020-02-08T18:08:32.1243634Z    |                                             let's call the lifetime of this reference `'2`
2020-02-08T18:08:32.1243726Z LL |         f //~ ERROR lifetime mismatch
2020-02-08T18:08:32.1243726Z LL |         f //~ ERROR lifetime mismatch
2020-02-08T18:08:32.1243993Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2020-02-08T18:08:32.1244129Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-02-08T18:08:32.1244375Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:48:69
2020-02-08T18:08:32.1244505Z    |
2020-02-08T18:08:32.1244505Z    |
2020-02-08T18:08:32.1244774Z LL |     async fn wrap_ref_Self_Self(self: Wrap<&Self, Self>, f: &u8) -> &u8 {
2020-02-08T18:08:32.1244998Z    |
2020-02-08T18:08:32.1245239Z    = note: hidden type `impl std::future::Future` captures lifetime '_#15r
2020-02-08T18:08:32.1245307Z 
2020-02-08T18:08:32.1245357Z error: lifetime may not live long enough
2020-02-08T18:08:32.1245357Z error: lifetime may not live long enough
2020-02-08T18:08:32.1245594Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:49:9
2020-02-08T18:08:32.1245656Z    |
2020-02-08T18:08:32.1245900Z LL |     async fn wrap_ref_Self_Self(self: Wrap<&Self, Self>, f: &u8) -> &u8 {
2020-02-08T18:08:32.1246273Z    |                                            |
2020-02-08T18:08:32.1246529Z    |                                            let's call the lifetime of this reference `'2`
2020-02-08T18:08:32.1246620Z LL |         f //~ ERROR lifetime mismatch
2020-02-08T18:08:32.1246620Z LL |         f //~ ERROR lifetime mismatch
2020-02-08T18:08:32.1247068Z    |         ^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
2020-02-08T18:08:32.1247173Z error: aborting due to 14 previous errors
2020-02-08T18:08:32.1247230Z 
2020-02-08T18:08:32.1247602Z For more information about this error, try `rustc --explain E0700`.
2020-02-08T18:08:32.1247661Z 
---
2020-02-08T18:08:32.1248976Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-08T18:08:32.1249057Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-08T18:08:32.1249124Z 
2020-02-08T18:08:32.1249152Z 
2020-02-08T18:08:32.1250687Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2020-02-08T18:08:32.1251211Z 
2020-02-08T18:08:32.1251243Z 
2020-02-08T18:08:32.1252709Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-08T18:08:32.1252780Z Build completed unsuccessfully in 1:44:06
2020-02-08T18:08:32.1252780Z Build completed unsuccessfully in 1:44:06
2020-02-08T18:08:32.1259981Z == clock drift check ==
2020-02-08T18:08:32.1273564Z   local time: Sat Feb  8 18:08:32 UTC 2020
2020-02-08T18:08:32.2860277Z   network time: Sat, 08 Feb 2020 18:08:32 GMT
2020-02-08T18:08:32.2860609Z == end clock drift check ==
2020-02-08T18:08:32.9284835Z 
2020-02-08T18:08:32.9395427Z ##[error]Bash exited with code '1'.
2020-02-08T18:08:32.9466091Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-08T18:08:32.9468104Z ==============================================================================
2020-02-08T18:08:32.9468197Z Task         : Get sources
2020-02-08T18:08:32.9468302Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
