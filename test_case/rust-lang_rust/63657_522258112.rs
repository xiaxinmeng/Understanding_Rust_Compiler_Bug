plain
2019-08-17T17:51:34.1109377Z diff of stderr:
2019-08-17T17:51:34.1109421Z 
2019-08-17T17:51:34.1109507Z 295    |                              help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1109582Z 296    |
2019-08-17T17:51:34.1109877Z 297 note: std::ptr::Unique<i32> must be non-null (in this struct field)
2019-08-17T17:51:34.1110123Z -   --> $SRC_DIR/liballoc/raw_vec.rs:LL:COL
2019-08-17T17:51:34.1110778Z -    |
2019-08-17T17:51:34.1111055Z - LL |     ptr: Unique<T>,
2019-08-17T17:51:34.1111385Z 302 
2019-08-17T17:51:34.1111385Z 302 
2019-08-17T17:51:34.1111471Z 303 error: the type `std::vec::Vec<i32>` does not permit being left uninitialized
2019-08-17T17:51:34.1111843Z 
2019-08-17T17:51:34.1111922Z 310    |                              help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1112023Z 311    |
2019-08-17T17:51:34.1112023Z 311    |
2019-08-17T17:51:34.1112312Z 312 note: std::ptr::Unique<i32> must be non-null (in this struct field)
2019-08-17T17:51:34.1112597Z -   --> $SRC_DIR/liballoc/raw_vec.rs:LL:COL
2019-08-17T17:51:34.1112815Z -    |
2019-08-17T17:51:34.1113063Z - LL |     ptr: Unique<T>,
2019-08-17T17:51:34.1113386Z 317 
2019-08-17T17:51:34.1113386Z 317 
2019-08-17T17:51:34.1113465Z 318 error: the type `bool` does not permit being left uninitialized
2019-08-17T17:51:34.1114024Z 
2019-08-17T17:51:34.1114056Z 
2019-08-17T17:51:34.1114135Z The actual stderr differed from the expected stderr.
2019-08-17T17:51:34.1114471Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed/uninitialized-zeroed.stderr
2019-08-17T17:51:34.1114471Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed/uninitialized-zeroed.stderr
2019-08-17T17:51:34.1114750Z To update references, rerun the tests and pass the `--bless` flag
2019-08-17T17:51:34.1115035Z To only update this specific test, also pass `--test-args lint/uninitialized-zeroed.rs`
2019-08-17T17:51:34.1115162Z error: 1 errors occurred comparing output.
2019-08-17T17:51:34.1115227Z status: exit code: 1
2019-08-17T17:51:34.1115227Z status: exit code: 1
2019-08-17T17:51:34.1116186Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/uninitialized-zeroed.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed/auxiliary" "-A" "unused"
2019-08-17T17:51:34.1116674Z ------------------------------------------
2019-08-17T17:51:34.1116719Z 
2019-08-17T17:51:34.1116950Z ------------------------------------------
2019-08-17T17:51:34.1117014Z stderr:
2019-08-17T17:51:34.1117014Z stderr:
2019-08-17T17:51:34.1117240Z ------------------------------------------
2019-08-17T17:51:34.1117475Z error: the type `&'static T` does not permit zero-initialization
2019-08-17T17:51:34.1117829Z    |
2019-08-17T17:51:34.1117829Z    |
2019-08-17T17:51:34.1118097Z LL |         let _val: &'static T = mem::zeroed(); //~ ERROR: does not permit zero-initialization
2019-08-17T17:51:34.1118279Z    |                                |
2019-08-17T17:51:34.1118374Z    |                                this code causes undefined behavior when executed
2019-08-17T17:51:34.1118566Z    |                                help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1118651Z    |
2019-08-17T17:51:34.1118651Z    |
2019-08-17T17:51:34.1118708Z note: lint level defined here
2019-08-17T17:51:34.1118990Z   --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:7:9
2019-08-17T17:51:34.1119060Z    |
2019-08-17T17:51:34.1119139Z LL | #![deny(invalid_value)]
2019-08-17T17:51:34.1119203Z    |         ^^^^^^^^^^^^^
2019-08-17T17:51:34.1119433Z    = note: References must be non-null
2019-08-17T17:51:34.1119476Z 
2019-08-17T17:51:34.1119723Z error: the type `&'static T` does not permit being left uninitialized
2019-08-17T17:51:34.1120055Z    |
2019-08-17T17:51:34.1120055Z    |
2019-08-17T17:51:34.1120513Z LL |         let _val: &'static T = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
2019-08-17T17:51:34.1121073Z    |                                |
2019-08-17T17:51:34.1121188Z    |                                this code causes undefined behavior when executed
2019-08-17T17:51:34.1121286Z    |                                help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1121387Z    |
2019-08-17T17:51:34.1121387Z    |
2019-08-17T17:51:34.1121659Z    = note: References must be non-null
2019-08-17T17:51:34.1121729Z 
2019-08-17T17:51:34.1122008Z error: the type `Wrap<&'static T>` does not permit zero-initialization
2019-08-17T17:51:34.1122391Z    |
2019-08-17T17:51:34.1122391Z    |
2019-08-17T17:51:34.1122726Z LL |         let _val: Wrap<&'static T> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
2019-08-17T17:51:34.1122939Z    |                                      |
2019-08-17T17:51:34.1123034Z    |                                      this code causes undefined behavior when executed
2019-08-17T17:51:34.1123153Z    |                                      help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1123245Z    |
2019-08-17T17:51:34.1123245Z    |
2019-08-17T17:51:34.1123534Z note: References must be non-null (in this struct field)
2019-08-17T17:51:34.1123917Z    |
2019-08-17T17:51:34.1123917Z    |
2019-08-17T17:51:34.1123987Z LL | struct Wrap<T> { wrapped: T }
2019-08-17T17:51:34.1124283Z 
2019-08-17T17:51:34.1124283Z 
2019-08-17T17:51:34.1124535Z error: the type `Wrap<&'static T>` does not permit being left uninitialized
2019-08-17T17:51:34.1124855Z    |
2019-08-17T17:51:34.1124855Z    |
2019-08-17T17:51:34.1125213Z LL |         let _val: Wrap<&'static T> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
2019-08-17T17:51:34.1125394Z    |                                      |
2019-08-17T17:51:34.1125495Z    |                                      this code causes undefined behavior when executed
2019-08-17T17:51:34.1125578Z    |                                      help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1125664Z    |
2019-08-17T17:51:34.1125664Z    |
2019-08-17T17:51:34.1125911Z note: References must be non-null (in this struct field)
2019-08-17T17:51:34.1126229Z    |
2019-08-17T17:51:34.1126229Z    |
2019-08-17T17:51:34.1126301Z LL | struct Wrap<T> { wrapped: T }
2019-08-17T17:51:34.1126418Z 
2019-08-17T17:51:34.1126418Z 
2019-08-17T17:51:34.1126635Z error: the type `!` does not permit zero-initialization
2019-08-17T17:51:34.1126952Z    |
2019-08-17T17:51:34.1126952Z    |
2019-08-17T17:51:34.1127220Z LL |         let _val: ! = mem::zeroed(); //~ ERROR: does not permit zero-initialization
2019-08-17T17:51:34.1127451Z    |                       |
2019-08-17T17:51:34.1127521Z    |                       this code causes undefined behavior when executed
2019-08-17T17:51:34.1127613Z    |                       help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1127676Z    |
2019-08-17T17:51:34.1127676Z    |
2019-08-17T17:51:34.1127753Z    = note: The never type (`!`) has no valid value
2019-08-17T17:51:34.1127794Z 
2019-08-17T17:51:34.1127871Z error: the type `!` does not permit being left uninitialized
2019-08-17T17:51:34.1128395Z    |
2019-08-17T17:51:34.1128395Z    |
2019-08-17T17:51:34.1128471Z LL |         let _val: ! = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
2019-08-17T17:51:34.1128806Z    |                       |
2019-08-17T17:51:34.1128893Z    |                       this code causes undefined behavior when executed
2019-08-17T17:51:34.1128969Z    |                       help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1129056Z    |
2019-08-17T17:51:34.1129056Z    |
2019-08-17T17:51:34.1129115Z    = note: The never type (`!`) has no valid value
2019-08-17T17:51:34.1129171Z 
2019-08-17T17:51:34.1129404Z error: the type `(i32, !)` does not permit zero-initialization
2019-08-17T17:51:34.1129721Z    |
2019-08-17T17:51:34.1129721Z    |
2019-08-17T17:51:34.1130173Z LL |         let _val: (i32, !) = mem::zeroed(); //~ ERROR: does not permit zero-initialization
2019-08-17T17:51:34.1130684Z    |                              |
2019-08-17T17:51:34.1130782Z    |                              this code causes undefined behavior when executed
2019-08-17T17:51:34.1130907Z    |                              help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1130987Z    |
2019-08-17T17:51:34.1130987Z    |
2019-08-17T17:51:34.1131077Z    = note: The never type (`!`) has no valid value
2019-08-17T17:51:34.1131135Z 
2019-08-17T17:51:34.1131229Z error: the type `(i32, !)` does not permit being left uninitialized
2019-08-17T17:51:34.1131652Z    |
2019-08-17T17:51:34.1131652Z    |
2019-08-17T17:51:34.1131744Z LL |         let _val: (i32, !) = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
2019-08-17T17:51:34.1131943Z    |                              |
2019-08-17T17:51:34.1132051Z    |                              this code causes undefined behavior when executed
2019-08-17T17:51:34.1132148Z    |                              help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1132540Z    |
2019-08-17T17:51:34.1132540Z    |
2019-08-17T17:51:34.1132729Z    = note: The never type (`!`) has no valid value
2019-08-17T17:51:34.1132803Z 
2019-08-17T17:51:34.1134736Z error: the type `Void` does not permit zero-initialization
2019-08-17T17:51:34.1135444Z    |
2019-08-17T17:51:34.1135444Z    |
2019-08-17T17:51:34.1136149Z LL |         let _val: Void = mem::zeroed(); //~ ERROR: does not permit zero-initialization
2019-08-17T17:51:34.1136326Z    |                          |
2019-08-17T17:51:34.1136401Z    |                          this code causes undefined behavior when executed
2019-08-17T17:51:34.1136499Z    |                          help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1136566Z    |
2019-08-17T17:51:34.1136566Z    |
2019-08-17T17:51:34.1136854Z    = note: 0-variant enums have no valid value
2019-08-17T17:51:34.1136899Z 
2019-08-17T17:51:34.1136982Z error: the type `Void` does not permit being left uninitialized
2019-08-17T17:51:34.1137327Z    |
2019-08-17T17:51:34.1137327Z    |
2019-08-17T17:51:34.1137567Z LL |         let _val: Void = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
2019-08-17T17:51:34.1138036Z    |                          |
2019-08-17T17:51:34.1138126Z    |                          this code causes undefined behavior when executed
2019-08-17T17:51:34.1138206Z    |                          help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1138289Z    |
2019-08-17T17:51:34.1138289Z    |
2019-08-17T17:51:34.1138702Z    = note: 0-variant enums have no valid value
2019-08-17T17:51:34.1138745Z 
2019-08-17T17:51:34.1138990Z error: the type `&'static i32` does not permit zero-initialization
2019-08-17T17:51:34.1139308Z    |
2019-08-17T17:51:34.1139308Z    |
2019-08-17T17:51:34.1139582Z LL |         let _val: &'static i32 = mem::zeroed(); //~ ERROR: does not permit zero-initialization
2019-08-17T17:51:34.1139758Z    |                                  |
2019-08-17T17:51:34.1139832Z    |                                  this code causes undefined behavior when executed
2019-08-17T17:51:34.1139937Z    |                                  help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1140184Z    |
2019-08-17T17:51:34.1140184Z    |
2019-08-17T17:51:34.1140961Z    = note: References must be non-null
2019-08-17T17:51:34.1141018Z 
2019-08-17T17:51:34.1141322Z error: the type `&'static i32` does not permit being left uninitialized
2019-08-17T17:51:34.1141728Z    |
2019-08-17T17:51:34.1141728Z    |
2019-08-17T17:51:34.1142057Z LL |         let _val: &'static i32 = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
2019-08-17T17:51:34.1142267Z    |                                  |
2019-08-17T17:51:34.1142387Z    |                                  this code causes undefined behavior when executed
2019-08-17T17:51:34.1142484Z    |                                  help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1142593Z    |
2019-08-17T17:51:34.1142593Z    |
2019-08-17T17:51:34.1142845Z    = note: References must be non-null
2019-08-17T17:51:34.1142896Z 
2019-08-17T17:51:34.1143177Z error: the type `Ref` does not permit zero-initialization
2019-08-17T17:51:34.1143560Z    |
2019-08-17T17:51:34.1143560Z    |
2019-08-17T17:51:34.1143879Z LL |         let _val: Ref = mem::zeroed(); //~ ERROR: does not permit zero-initialization
2019-08-17T17:51:34.1144233Z    |                         |
2019-08-17T17:51:34.1144317Z    |                         this code causes undefined behavior when executed
2019-08-17T17:51:34.1144842Z    |                         help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1145021Z    |
2019-08-17T17:51:34.1145021Z    |
2019-08-17T17:51:34.1145478Z note: References must be non-null (in this struct field)
2019-08-17T17:51:34.1145838Z    |
2019-08-17T17:51:34.1145838Z    |
2019-08-17T17:51:34.1146061Z LL | struct Ref(&'static i32);
2019-08-17T17:51:34.1146566Z 
2019-08-17T17:51:34.1146566Z 
2019-08-17T17:51:34.1146630Z error: the type `Ref` does not permit being left uninitialized
2019-08-17T17:51:34.1146964Z    |
2019-08-17T17:51:34.1146964Z    |
2019-08-17T17:51:34.1147054Z LL |         let _val: Ref = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
2019-08-17T17:51:34.1147221Z    |                         |
2019-08-17T17:51:34.1147310Z    |                         this code causes undefined behavior when executed
2019-08-17T17:51:34.1147398Z    |                         help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1147482Z    |
2019-08-17T17:51:34.1147482Z    |
2019-08-17T17:51:34.1148055Z note: References must be non-null (in this struct field)
2019-08-17T17:51:34.1148829Z    |
2019-08-17T17:51:34.1148829Z    |
2019-08-17T17:51:34.1149237Z LL | struct Ref(&'static i32);
2019-08-17T17:51:34.1149343Z 
2019-08-17T17:51:34.1149343Z 
2019-08-17T17:51:34.1149593Z error: the type `fn()` does not permit zero-initialization
2019-08-17T17:51:34.1149927Z    |
2019-08-17T17:51:34.1149927Z    |
2019-08-17T17:51:34.1150917Z LL |         let _val: fn() = mem::zeroed(); //~ ERROR: does not permit zero-initialization
2019-08-17T17:51:34.1151141Z    |                          |
2019-08-17T17:51:34.1151228Z    |                          this code causes undefined behavior when executed
2019-08-17T17:51:34.1151352Z    |                          help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1151433Z    |
2019-08-17T17:51:34.1151433Z    |
2019-08-17T17:51:34.1151749Z    = note: Function pointers must be non-null
2019-08-17T17:51:34.1151810Z 
2019-08-17T17:51:34.1151888Z error: the type `fn()` does not permit being left uninitialized
2019-08-17T17:51:34.1152281Z    |
2019-08-17T17:51:34.1152281Z    |
2019-08-17T17:51:34.1152385Z LL |         let _val: fn() = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
2019-08-17T17:51:34.1152583Z    |                          |
2019-08-17T17:51:34.1152686Z    |                          this code causes undefined behavior when executed
2019-08-17T17:51:34.1152782Z    |                          help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1152877Z    |
2019-08-17T17:51:34.1152877Z    |
2019-08-17T17:51:34.1153146Z    = note: Function pointers must be non-null
2019-08-17T17:51:34.1153200Z 
2019-08-17T17:51:34.1153492Z error: the type `Wrap<fn()>` does not permit zero-initialization
2019-08-17T17:51:34.1153887Z    |
2019-08-17T17:51:34.1153887Z    |
2019-08-17T17:51:34.1154354Z LL |         let _val: Wrap<fn()> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
2019-08-17T17:51:34.1161535Z    |                                |
2019-08-17T17:51:34.1161877Z    |                                this code causes undefined behavior when executed
2019-08-17T17:51:34.1162028Z    |                                help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1162110Z    |
2019-08-17T17:51:34.1162110Z    |
2019-08-17T17:51:34.1162560Z note: Function pointers must be non-null (in this struct field)
2019-08-17T17:51:34.1164134Z    |
2019-08-17T17:51:34.1164134Z    |
2019-08-17T17:51:34.1164681Z LL | struct Wrap<T> { wrapped: T }
2019-08-17T17:51:34.1164827Z 
2019-08-17T17:51:34.1164827Z 
2019-08-17T17:51:34.1164895Z error: the type `Wrap<fn()>` does not permit being left uninitialized
2019-08-17T17:51:34.1165667Z    |
2019-08-17T17:51:34.1165667Z    |
2019-08-17T17:51:34.1165755Z LL |         let _val: Wrap<fn()> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
2019-08-17T17:51:34.1165932Z    |                                |
2019-08-17T17:51:34.1166027Z    |                                this code causes undefined behavior when executed
2019-08-17T17:51:34.1166116Z    |                                help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1166201Z    |
2019-08-17T17:51:34.1166201Z    |
2019-08-17T17:51:34.1166488Z note: Function pointers must be non-null (in this struct field)
2019-08-17T17:51:34.1173098Z    |
2019-08-17T17:51:34.1173098Z    |
2019-08-17T17:51:34.1173186Z LL | struct Wrap<T> { wrapped: T }
2019-08-17T17:51:34.1173533Z 
2019-08-17T17:51:34.1173533Z 
2019-08-17T17:51:34.1173903Z error: the type `WrapEnum<fn()>` does not permit zero-initialization
2019-08-17T17:51:34.1174448Z    |
2019-08-17T17:51:34.1174448Z    |
2019-08-17T17:51:34.1174769Z LL |         let _val: WrapEnum<fn()> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
2019-08-17T17:51:34.1174963Z    |                                    |
2019-08-17T17:51:34.1175048Z    |                                    this code causes undefined behavior when executed
2019-08-17T17:51:34.1175158Z    |                                    help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1175235Z    |
2019-08-17T17:51:34.1175235Z    |
2019-08-17T17:51:34.1176242Z note: Function pointers must be non-null (in this enum field)
2019-08-17T17:51:34.1176797Z    |
2019-08-17T17:51:34.1176797Z    |
2019-08-17T17:51:34.1176856Z LL | enum WrapEnum<T> { Wrapped(T) }
2019-08-17T17:51:34.1176978Z 
2019-08-17T17:51:34.1176978Z 
2019-08-17T17:51:34.1177063Z error: the type `WrapEnum<fn()>` does not permit being left uninitialized
2019-08-17T17:51:34.1177407Z    |
2019-08-17T17:51:34.1177407Z    |
2019-08-17T17:51:34.1177487Z LL |         let _val: WrapEnum<fn()> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
2019-08-17T17:51:34.1177663Z    |                                    |
2019-08-17T17:51:34.1177756Z    |                                    this code causes undefined behavior when executed
2019-08-17T17:51:34.1177847Z    |                                    help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1177934Z    |
2019-08-17T17:51:34.1177934Z    |
2019-08-17T17:51:34.1178168Z note: Function pointers must be non-null (in this enum field)
2019-08-17T17:51:34.1178692Z    |
2019-08-17T17:51:34.1178692Z    |
2019-08-17T17:51:34.1178951Z LL | enum WrapEnum<T> { Wrapped(T) }
2019-08-17T17:51:34.1179288Z 
2019-08-17T17:51:34.1179288Z 
2019-08-17T17:51:34.1179540Z error: the type `Wrap<(RefPair, i32)>` does not permit zero-initialization
2019-08-17T17:51:34.1179879Z    |
2019-08-17T17:51:34.1179879Z    |
2019-08-17T17:51:34.1180565Z LL |         let _val: Wrap<(RefPair, i32)> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
2019-08-17T17:51:34.1181301Z    |                                          |
2019-08-17T17:51:34.1181406Z    |                                          this code causes undefined behavior when executed
2019-08-17T17:51:34.1181527Z    |                                          help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1181620Z    |
2019-08-17T17:51:34.1181620Z    |
2019-08-17T17:51:34.1181968Z note: References must be non-null (in this struct field)
2019-08-17T17:51:34.1182357Z    |
2019-08-17T17:51:34.1182357Z    |
2019-08-17T17:51:34.1182610Z LL | struct RefPair((&'static i32, i32));
2019-08-17T17:51:34.1182761Z 
2019-08-17T17:51:34.1182761Z 
2019-08-17T17:51:34.1182858Z error: the type `Wrap<(RefPair, i32)>` does not permit being left uninitialized
2019-08-17T17:51:34.1183258Z    |
2019-08-17T17:51:34.1183258Z    |
2019-08-17T17:51:34.1183361Z LL |         let _val: Wrap<(RefPair, i32)> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
2019-08-17T17:51:34.1183576Z    |                                          |
2019-08-17T17:51:34.1183782Z    |                                          this code causes undefined behavior when executed
2019-08-17T17:51:34.1183886Z    |                                          help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1183988Z    |
2019-08-17T17:51:34.1183988Z    |
2019-08-17T17:51:34.1184442Z note: References must be non-null (in this struct field)
2019-08-17T17:51:34.1184954Z    |
2019-08-17T17:51:34.1184954Z    |
2019-08-17T17:51:34.1185190Z LL | struct RefPair((&'static i32, i32));
2019-08-17T17:51:34.1185318Z 
2019-08-17T17:51:34.1185318Z 
2019-08-17T17:51:34.1185565Z error: the type `std::vec::Vec<i32>` does not permit zero-initialization
2019-08-17T17:51:34.1185916Z    |
2019-08-17T17:51:34.1185916Z    |
2019-08-17T17:51:34.1186210Z LL |         let _val: Vec<i32> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
2019-08-17T17:51:34.1186390Z    |                              |
2019-08-17T17:51:34.1186468Z    |                              this code causes undefined behavior when executed
2019-08-17T17:51:34.1186571Z    |                              help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1186641Z    |
2019-08-17T17:51:34.1186641Z    |
2019-08-17T17:51:34.1186907Z note: std::ptr::Unique<i32> must be non-null (in this struct field)
2019-08-17T17:51:34.1186957Z 
2019-08-17T17:51:34.1187045Z error: the type `std::vec::Vec<i32>` does not permit being left uninitialized
2019-08-17T17:51:34.1187390Z    |
2019-08-17T17:51:34.1187390Z    |
2019-08-17T17:51:34.1187474Z LL |         let _val: Vec<i32> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
2019-08-17T17:51:34.1187653Z    |                              |
2019-08-17T17:51:34.1187755Z    |                              this code causes undefined behavior when executed
2019-08-17T17:51:34.1188015Z    |                              help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1188107Z    |
2019-08-17T17:51:34.1188107Z    |
2019-08-17T17:51:34.1188387Z note: std::ptr::Unique<i32> must be non-null (in this struct field)
2019-08-17T17:51:34.1188439Z 
2019-08-17T17:51:34.1188508Z error: the type `bool` does not permit being left uninitialized
2019-08-17T17:51:34.1188861Z    |
2019-08-17T17:51:34.1188861Z    |
2019-08-17T17:51:34.1188959Z LL |         let _val: bool = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
2019-08-17T17:51:34.1189213Z    |                          |
2019-08-17T17:51:34.1189300Z    |                          this code causes undefined behavior when executed
2019-08-17T17:51:34.1189402Z    |                          help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1189482Z    |
2019-08-17T17:51:34.1189482Z    |
2019-08-17T17:51:34.1189736Z    = note: Booleans must be `true` or `false`
2019-08-17T17:51:34.1189783Z 
2019-08-17T17:51:34.1189873Z error: the type `Wrap<char>` does not permit being left uninitialized
2019-08-17T17:51:34.1190823Z    |
2019-08-17T17:51:34.1190823Z    |
2019-08-17T17:51:34.1190915Z LL |         let _val: Wrap<char> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
2019-08-17T17:51:34.1191116Z    |                                |
2019-08-17T17:51:34.1191223Z    |                                this code causes undefined behavior when executed
2019-08-17T17:51:34.1191331Z    |                                help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1191431Z    |
2019-08-17T17:51:34.1191431Z    |
2019-08-17T17:51:34.1191510Z note: Characters must be a valid unicode codepoint (in this struct field)
2019-08-17T17:51:34.1192127Z    |
2019-08-17T17:51:34.1192127Z    |
2019-08-17T17:51:34.1192200Z LL | struct Wrap<T> { wrapped: T }
2019-08-17T17:51:34.1192347Z 
2019-08-17T17:51:34.1192347Z 
2019-08-17T17:51:34.1192428Z error: the type `NonBig` does not permit being left uninitialized
2019-08-17T17:51:34.1192857Z    |
2019-08-17T17:51:34.1192857Z    |
2019-08-17T17:51:34.1192970Z LL |         let _val: NonBig = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
2019-08-17T17:51:34.1193178Z    |                            |
2019-08-17T17:51:34.1193279Z    |                            this code causes undefined behavior when executed
2019-08-17T17:51:34.1193403Z    |                            help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1193515Z    |
2019-08-17T17:51:34.1193515Z    |
2019-08-17T17:51:34.1193597Z    = note: NonBig must be initialized inside its custom valid range
2019-08-17T17:51:34.1193654Z 
2019-08-17T17:51:34.1194138Z error: the type `&'static i32` does not permit zero-initialization
2019-08-17T17:51:34.1194486Z    |
2019-08-17T17:51:34.1194486Z    |
2019-08-17T17:51:34.1194773Z LL |         let _val: &'static i32 = mem::transmute(0usize); //~ ERROR: does not permit zero-initialization
2019-08-17T17:51:34.1194957Z    |                                  |
2019-08-17T17:51:34.1195054Z    |                                  this code causes undefined behavior when executed
2019-08-17T17:51:34.1195165Z    |                                  help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1195237Z    |
2019-08-17T17:51:34.1195237Z    |
2019-08-17T17:51:34.1195506Z    = note: References must be non-null
2019-08-17T17:51:34.1195556Z 
2019-08-17T17:51:34.1195820Z error: the type `&'static [i32]` does not permit zero-initialization
2019-08-17T17:51:34.1196161Z    |
2019-08-17T17:51:34.1196161Z    |
2019-08-17T17:51:34.1196475Z LL |         let _val: &'static [i32] = mem::transmute((0usize, 0usize)); //~ ERROR: does not permit zero-initialization
2019-08-17T17:51:34.1196663Z    |                                    |
2019-08-17T17:51:34.1196760Z    |                                    this code causes undefined behavior when executed
2019-08-17T17:51:34.1196847Z    |                                    help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1196935Z    |
2019-08-17T17:51:34.1196935Z    |
2019-08-17T17:51:34.1197234Z    = note: References must be non-null
2019-08-17T17:51:34.1197289Z 
2019-08-17T17:51:34.1197581Z error: the type `std::num::NonZeroU32` does not permit zero-initialization
2019-08-17T17:51:34.1198226Z    |
2019-08-17T17:51:34.1198226Z    |
2019-08-17T17:51:34.1199126Z LL |         let _val: NonZeroU32 = mem::transmute(0); //~ ERROR: does not permit zero-initialization
2019-08-17T17:51:34.1199346Z    |                                |
2019-08-17T17:51:34.1199455Z    |                                this code causes undefined behavior when executed
2019-08-17T17:51:34.1199575Z    |                                help: use `MaybeUninit<T>` instead
2019-08-17T17:51:34.1199656Z    |
---
2019-08-17T17:51:34.1202227Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-17T17:51:34.1202333Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-17T17:51:34.1202411Z 
2019-08-17T17:51:34.1202449Z 
2019-08-17T17:51:34.1204438Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-17T17:51:34.1205030Z 
2019-08-17T17:51:34.1205072Z 
2019-08-17T17:51:34.1210996Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-08-17T17:51:34.1211128Z Build completed unsuccessfully in 1:17:49
2019-08-17T17:51:34.1211128Z Build completed unsuccessfully in 1:17:49
2019-08-17T17:51:34.1227843Z == clock drift check ==
2019-08-17T17:51:34.1241992Z   local time: Sat Aug 17 17:51:34 UTC 2019
2019-08-17T17:51:34.2877742Z   network time: Sat, 17 Aug 2019 17:51:34 GMT
2019-08-17T17:51:34.2877864Z == end clock drift check ==
2019-08-17T17:51:35.0554673Z ##[error]Bash exited with code '1'.
2019-08-17T17:51:35.0602882Z ##[section]Starting: Upload CPU usage statistics
2019-08-17T17:51:35.0610892Z ==============================================================================
2019-08-17T17:51:35.0610975Z Task         : Bash
2019-08-17T17:51:35.0611053Z Description  : Run a Bash script on macOS, Linux, or Windows
