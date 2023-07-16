plain
2019-12-08T23:20:28.7642126Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-08T23:20:28.7858663Z ##[command]git config gc.auto 0
2019-12-08T23:20:28.7951339Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-08T23:20:28.8014452Z ##[command]git config --get-all http.proxy
2019-12-08T23:20:28.8164513Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66277/merge:refs/remotes/pull/66277/merge
---
2019-12-08T23:25:01.9864428Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-08T23:25:01.9887687Z Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-08T23:25:02.3711135Z    Compiling cc v1.0.47
2019-12-08T23:25:02.3711579Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-12-08T23:25:04.9152264Z error[E0412]: cannot find type `NonZeroU8` in this scope
2019-12-08T23:25:04.9154404Z     |
2019-12-08T23:25:04.9154404Z     |
2019-12-08T23:25:04.9154848Z 398 | nzint_impl_from! { NonZeroU8, NonZeroU16, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:04.9155529Z     |
2019-12-08T23:25:04.9155852Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:04.9156327Z     |
2019-12-08T23:25:04.9156613Z 1   | use crate::num::NonZeroU8;
2019-12-08T23:25:04.9156613Z 1   | use crate::num::NonZeroU8;
2019-12-08T23:25:04.9157190Z     |
2019-12-08T23:25:04.9157710Z 
2019-12-08T23:25:04.9274056Z error[E0412]: cannot find type `NonZeroU16` in this scope
2019-12-08T23:25:04.9275024Z     |
2019-12-08T23:25:04.9275024Z     |
2019-12-08T23:25:04.9275456Z 398 | nzint_impl_from! { NonZeroU8, NonZeroU16, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:04.9276507Z     |
2019-12-08T23:25:04.9277176Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:04.9277660Z     |
2019-12-08T23:25:04.9277969Z 1   | use crate::num::NonZeroU16;
2019-12-08T23:25:04.9277969Z 1   | use crate::num::NonZeroU16;
2019-12-08T23:25:04.9278205Z     |
2019-12-08T23:25:04.9278286Z 
2019-12-08T23:25:04.9588819Z error[E0412]: cannot find type `NonZeroU8` in this scope
2019-12-08T23:25:04.9589651Z     |
2019-12-08T23:25:04.9589651Z     |
2019-12-08T23:25:04.9590196Z 399 | nzint_impl_from! { NonZeroU8, NonZeroU32, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:04.9591000Z     |
2019-12-08T23:25:04.9591304Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:04.9591570Z     |
2019-12-08T23:25:04.9591902Z 1   | use crate::num::NonZeroU8;
2019-12-08T23:25:04.9591902Z 1   | use crate::num::NonZeroU8;
2019-12-08T23:25:04.9592325Z     |
2019-12-08T23:25:04.9594509Z 
2019-12-08T23:25:04.9714960Z error[E0412]: cannot find type `NonZeroU32` in this scope
2019-12-08T23:25:04.9715525Z     |
2019-12-08T23:25:04.9715525Z     |
2019-12-08T23:25:04.9715889Z 399 | nzint_impl_from! { NonZeroU8, NonZeroU32, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:04.9716484Z     |
2019-12-08T23:25:04.9716800Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:04.9717020Z     |
2019-12-08T23:25:04.9717300Z 1   | use crate::num::NonZeroU32;
2019-12-08T23:25:04.9717300Z 1   | use crate::num::NonZeroU32;
2019-12-08T23:25:04.9717545Z     |
2019-12-08T23:25:04.9721378Z 
2019-12-08T23:25:04.9993870Z error[E0412]: cannot find type `NonZeroU8` in this scope
2019-12-08T23:25:04.9994547Z     |
2019-12-08T23:25:04.9994547Z     |
2019-12-08T23:25:04.9994891Z 400 | nzint_impl_from! { NonZeroU8, NonZeroU64, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:04.9995497Z     |
2019-12-08T23:25:04.9995792Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:04.9996220Z     |
2019-12-08T23:25:04.9996544Z 1   | use crate::num::NonZeroU8;
2019-12-08T23:25:04.9996544Z 1   | use crate::num::NonZeroU8;
2019-12-08T23:25:04.9996773Z     |
2019-12-08T23:25:05.0000888Z 
2019-12-08T23:25:05.0115521Z error[E0412]: cannot find type `NonZeroU64` in this scope
2019-12-08T23:25:05.0116413Z     |
2019-12-08T23:25:05.0116413Z     |
2019-12-08T23:25:05.0116791Z 400 | nzint_impl_from! { NonZeroU8, NonZeroU64, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.0117522Z     |
2019-12-08T23:25:05.0117861Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.0118170Z     |
2019-12-08T23:25:05.0118503Z 1   | use crate::num::NonZeroU64;
2019-12-08T23:25:05.0118503Z 1   | use crate::num::NonZeroU64;
2019-12-08T23:25:05.0118771Z     |
2019-12-08T23:25:05.0123047Z 
2019-12-08T23:25:05.0412315Z error[E0412]: cannot find type `NonZeroU8` in this scope
2019-12-08T23:25:05.0412911Z     |
2019-12-08T23:25:05.0412911Z     |
2019-12-08T23:25:05.0413285Z 401 | nzint_impl_from! { NonZeroU8, NonZeroU128, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.0413906Z     |
2019-12-08T23:25:05.0414204Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.0414431Z     |
2019-12-08T23:25:05.0414881Z 1   | use crate::num::NonZeroU8;
2019-12-08T23:25:05.0414881Z 1   | use crate::num::NonZeroU8;
2019-12-08T23:25:05.0415104Z     |
2019-12-08T23:25:05.0418820Z 
2019-12-08T23:25:05.0524868Z error[E0412]: cannot find type `NonZeroU128` in this scope
2019-12-08T23:25:05.0525584Z     |
2019-12-08T23:25:05.0525584Z     |
2019-12-08T23:25:05.0526328Z 401 | nzint_impl_from! { NonZeroU8, NonZeroU128, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.0526907Z     |
2019-12-08T23:25:05.0527391Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.0527625Z     |
2019-12-08T23:25:05.0527912Z 1   | use crate::num::NonZeroU128;
2019-12-08T23:25:05.0527912Z 1   | use crate::num::NonZeroU128;
2019-12-08T23:25:05.0528136Z     |
2019-12-08T23:25:05.0532307Z 
2019-12-08T23:25:05.0814081Z error[E0412]: cannot find type `NonZeroU8` in this scope
2019-12-08T23:25:05.0814694Z     |
2019-12-08T23:25:05.0814694Z     |
2019-12-08T23:25:05.0815282Z 402 | nzint_impl_from! { NonZeroU8, NonZeroUsize, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.0816156Z     |
2019-12-08T23:25:05.0816605Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.0816872Z     |
2019-12-08T23:25:05.0817171Z 1   | use crate::num::NonZeroU8;
2019-12-08T23:25:05.0817171Z 1   | use crate::num::NonZeroU8;
2019-12-08T23:25:05.0817430Z     |
2019-12-08T23:25:05.0821392Z 
2019-12-08T23:25:05.0931613Z error[E0412]: cannot find type `NonZeroUsize` in this scope
2019-12-08T23:25:05.0931968Z    --> src/libcore/convert/num.rs:402:31
2019-12-08T23:25:05.0932385Z     |
2019-12-08T23:25:05.0932730Z 402 | nzint_impl_from! { NonZeroU8, NonZeroUsize, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.0933335Z     |
2019-12-08T23:25:05.0933625Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.0933871Z     |
2019-12-08T23:25:05.0934146Z 1   | use crate::num::NonZeroUsize;
2019-12-08T23:25:05.0934146Z 1   | use crate::num::NonZeroUsize;
2019-12-08T23:25:05.0934368Z     |
2019-12-08T23:25:05.0938756Z 
2019-12-08T23:25:05.1188871Z error[E0412]: cannot find type `NonZeroU16` in this scope
2019-12-08T23:25:05.1189541Z     |
2019-12-08T23:25:05.1189541Z     |
2019-12-08T23:25:05.1189928Z 403 | nzint_impl_from! { NonZeroU16, NonZeroU32, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.1190586Z     |
2019-12-08T23:25:05.1190909Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.1191197Z     |
2019-12-08T23:25:05.1191552Z 1   | use crate::num::NonZeroU16;
2019-12-08T23:25:05.1191552Z 1   | use crate::num::NonZeroU16;
2019-12-08T23:25:05.1191828Z     |
2019-12-08T23:25:05.1196469Z 
2019-12-08T23:25:05.1313531Z error[E0412]: cannot find type `NonZeroU32` in this scope
2019-12-08T23:25:05.1314145Z     |
2019-12-08T23:25:05.1314145Z     |
2019-12-08T23:25:05.1314576Z 403 | nzint_impl_from! { NonZeroU16, NonZeroU32, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.1315207Z     |
2019-12-08T23:25:05.1315645Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.1315932Z     |
2019-12-08T23:25:05.1316535Z 1   | use crate::num::NonZeroU32;
2019-12-08T23:25:05.1316535Z 1   | use crate::num::NonZeroU32;
2019-12-08T23:25:05.1316889Z     |
2019-12-08T23:25:05.1321756Z 
2019-12-08T23:25:05.1652931Z error[E0412]: cannot find type `NonZeroU16` in this scope
2019-12-08T23:25:05.1653999Z     |
2019-12-08T23:25:05.1653999Z     |
2019-12-08T23:25:05.1654336Z 404 | nzint_impl_from! { NonZeroU16, NonZeroU64, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.1655323Z     |
2019-12-08T23:25:05.1655637Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.1655934Z     |
2019-12-08T23:25:05.1656247Z 1   | use crate::num::NonZeroU16;
2019-12-08T23:25:05.1656247Z 1   | use crate::num::NonZeroU16;
2019-12-08T23:25:05.1656501Z     |
2019-12-08T23:25:05.1660542Z 
2019-12-08T23:25:05.1777866Z error[E0412]: cannot find type `NonZeroU64` in this scope
2019-12-08T23:25:05.1778499Z     |
2019-12-08T23:25:05.1778499Z     |
2019-12-08T23:25:05.1778858Z 404 | nzint_impl_from! { NonZeroU16, NonZeroU64, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.1779488Z     |
2019-12-08T23:25:05.1779793Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.1780028Z     |
2019-12-08T23:25:05.1780335Z 1   | use crate::num::NonZeroU64;
2019-12-08T23:25:05.1780335Z 1   | use crate::num::NonZeroU64;
2019-12-08T23:25:05.1780577Z     |
2019-12-08T23:25:05.1785408Z 
2019-12-08T23:25:05.2068549Z error[E0412]: cannot find type `NonZeroU16` in this scope
2019-12-08T23:25:05.2069183Z     |
2019-12-08T23:25:05.2069183Z     |
2019-12-08T23:25:05.2069600Z 405 | nzint_impl_from! { NonZeroU16, NonZeroU128, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.2070426Z     |
2019-12-08T23:25:05.2070745Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.2071016Z     |
2019-12-08T23:25:05.2071369Z 1   | use crate::num::NonZeroU16;
2019-12-08T23:25:05.2071369Z 1   | use crate::num::NonZeroU16;
2019-12-08T23:25:05.2071635Z     |
2019-12-08T23:25:05.2076331Z 
2019-12-08T23:25:05.2188783Z error[E0412]: cannot find type `NonZeroU128` in this scope
2019-12-08T23:25:05.2189395Z     |
2019-12-08T23:25:05.2189395Z     |
2019-12-08T23:25:05.2189926Z 405 | nzint_impl_from! { NonZeroU16, NonZeroU128, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.2190790Z     |
2019-12-08T23:25:05.2191325Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.2191731Z     |
2019-12-08T23:25:05.2192174Z 1   | use crate::num::NonZeroU128;
2019-12-08T23:25:05.2192174Z 1   | use crate::num::NonZeroU128;
2019-12-08T23:25:05.2192456Z     |
2019-12-08T23:25:05.2196496Z 
2019-12-08T23:25:05.2475851Z error[E0412]: cannot find type `NonZeroU16` in this scope
2019-12-08T23:25:05.2476477Z     |
2019-12-08T23:25:05.2476477Z     |
2019-12-08T23:25:05.2476811Z 406 | nzint_impl_from! { NonZeroU16, NonZeroUsize, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.2477750Z     |
2019-12-08T23:25:05.2478043Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.2478282Z     |
2019-12-08T23:25:05.2478743Z 1   | use crate::num::NonZeroU16;
2019-12-08T23:25:05.2478743Z 1   | use crate::num::NonZeroU16;
2019-12-08T23:25:05.2478976Z     |
2019-12-08T23:25:05.2483418Z 
2019-12-08T23:25:05.2602906Z error[E0412]: cannot find type `NonZeroUsize` in this scope
2019-12-08T23:25:05.2603261Z    --> src/libcore/convert/num.rs:406:32
2019-12-08T23:25:05.2603515Z     |
2019-12-08T23:25:05.2603852Z 406 | nzint_impl_from! { NonZeroU16, NonZeroUsize, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.2604435Z     |
2019-12-08T23:25:05.2604726Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.2604977Z     |
2019-12-08T23:25:05.2605285Z 1   | use crate::num::NonZeroUsize;
2019-12-08T23:25:05.2605285Z 1   | use crate::num::NonZeroUsize;
2019-12-08T23:25:05.2605525Z     |
2019-12-08T23:25:05.2610514Z 
2019-12-08T23:25:05.2880555Z error[E0412]: cannot find type `NonZeroU32` in this scope
2019-12-08T23:25:05.2881407Z     |
2019-12-08T23:25:05.2881407Z     |
2019-12-08T23:25:05.2881767Z 407 | nzint_impl_from! { NonZeroU32, NonZeroU64, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.2882350Z     |
2019-12-08T23:25:05.2882663Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.2883046Z     |
2019-12-08T23:25:05.2883456Z 1   | use crate::num::NonZeroU32;
2019-12-08T23:25:05.2883456Z 1   | use crate::num::NonZeroU32;
2019-12-08T23:25:05.2883927Z     |
2019-12-08T23:25:05.2897259Z 
2019-12-08T23:25:05.2994210Z error[E0412]: cannot find type `NonZeroU64` in this scope
2019-12-08T23:25:05.2994991Z     |
2019-12-08T23:25:05.2994991Z     |
2019-12-08T23:25:05.2995337Z 407 | nzint_impl_from! { NonZeroU32, NonZeroU64, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.2995930Z     |
2019-12-08T23:25:05.2996223Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.2996479Z     |
2019-12-08T23:25:05.2996756Z 1   | use crate::num::NonZeroU64;
2019-12-08T23:25:05.2996756Z 1   | use crate::num::NonZeroU64;
2019-12-08T23:25:05.2996976Z     |
2019-12-08T23:25:05.3001032Z 
2019-12-08T23:25:05.3275207Z error[E0412]: cannot find type `NonZeroU32` in this scope
2019-12-08T23:25:05.3276270Z     |
2019-12-08T23:25:05.3276270Z     |
2019-12-08T23:25:05.3276608Z 408 | nzint_impl_from! { NonZeroU32, NonZeroU128, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.3277175Z     |
2019-12-08T23:25:05.3277471Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.3277691Z     |
2019-12-08T23:25:05.3277974Z 1   | use crate::num::NonZeroU32;
2019-12-08T23:25:05.3277974Z 1   | use crate::num::NonZeroU32;
2019-12-08T23:25:05.3278189Z     |
2019-12-08T23:25:05.3282385Z 
2019-12-08T23:25:05.3392701Z error[E0412]: cannot find type `NonZeroU128` in this scope
2019-12-08T23:25:05.3393318Z     |
2019-12-08T23:25:05.3393318Z     |
2019-12-08T23:25:05.3393735Z 408 | nzint_impl_from! { NonZeroU32, NonZeroU128, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.3394418Z     |
2019-12-08T23:25:05.3394745Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.3394995Z     |
2019-12-08T23:25:05.3395449Z 1   | use crate::num::NonZeroU128;
2019-12-08T23:25:05.3395449Z 1   | use crate::num::NonZeroU128;
2019-12-08T23:25:05.3395701Z     |
2019-12-08T23:25:05.3400140Z 
2019-12-08T23:25:05.3691711Z error[E0412]: cannot find type `NonZeroU64` in this scope
2019-12-08T23:25:05.3692374Z     |
2019-12-08T23:25:05.3692374Z     |
2019-12-08T23:25:05.3692756Z 409 | nzint_impl_from! { NonZeroU64, NonZeroU128, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.3693619Z     |
2019-12-08T23:25:05.3693961Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.3694403Z     |
2019-12-08T23:25:05.3694734Z 1   | use crate::num::NonZeroU64;
2019-12-08T23:25:05.3694734Z 1   | use crate::num::NonZeroU64;
2019-12-08T23:25:05.3695183Z     |
2019-12-08T23:25:05.3700023Z 
2019-12-08T23:25:05.3819197Z error[E0412]: cannot find type `NonZeroU128` in this scope
2019-12-08T23:25:05.3819777Z     |
2019-12-08T23:25:05.3819777Z     |
2019-12-08T23:25:05.3820134Z 409 | nzint_impl_from! { NonZeroU64, NonZeroU128, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.3820710Z     |
2019-12-08T23:25:05.3821005Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.3821428Z     |
2019-12-08T23:25:05.3821704Z 1   | use crate::num::NonZeroU128;
2019-12-08T23:25:05.3821704Z 1   | use crate::num::NonZeroU128;
2019-12-08T23:25:05.3821925Z     |
2019-12-08T23:25:05.3826328Z 
2019-12-08T23:25:05.4160598Z error[E0412]: cannot find type `NonZeroI8` in this scope
2019-12-08T23:25:05.4161464Z     |
2019-12-08T23:25:05.4161464Z     |
2019-12-08T23:25:05.4161873Z 412 | nzint_impl_from! { NonZeroI8, NonZeroI16, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.4162535Z     |
2019-12-08T23:25:05.4162867Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.4163116Z     |
2019-12-08T23:25:05.4163434Z 1   | use crate::num::NonZeroI8;
2019-12-08T23:25:05.4163434Z 1   | use crate::num::NonZeroI8;
2019-12-08T23:25:05.4163688Z     |
2019-12-08T23:25:05.4168022Z 
2019-12-08T23:25:05.5611015Z error[E0412]: cannot find type `NonZeroI16` in this scope
2019-12-08T23:25:05.5611671Z     |
2019-12-08T23:25:05.5611671Z     |
2019-12-08T23:25:05.5612065Z 412 | nzint_impl_from! { NonZeroI8, NonZeroI16, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.5612753Z     |
2019-12-08T23:25:05.5613084Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.5613334Z     |
2019-12-08T23:25:05.5613654Z 1   | use crate::num::NonZeroI16;
2019-12-08T23:25:05.5613654Z 1   | use crate::num::NonZeroI16;
2019-12-08T23:25:05.5614085Z     |
2019-12-08T23:25:05.5619292Z 
2019-12-08T23:25:05.5625249Z error[E0412]: cannot find type `NonZeroI8` in this scope
2019-12-08T23:25:05.5625866Z     |
2019-12-08T23:25:05.5625866Z     |
2019-12-08T23:25:05.5626449Z 413 | nzint_impl_from! { NonZeroI8, NonZeroI32, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.5627097Z     |
2019-12-08T23:25:05.5627434Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.5627733Z     |
2019-12-08T23:25:05.5628073Z 1   | use crate::num::NonZeroI8;
2019-12-08T23:25:05.5628073Z 1   | use crate::num::NonZeroI8;
2019-12-08T23:25:05.5628361Z     |
2019-12-08T23:25:05.5632789Z 
2019-12-08T23:25:05.5638754Z error[E0412]: cannot find type `NonZeroI32` in this scope
2019-12-08T23:25:05.5639379Z     |
2019-12-08T23:25:05.5639379Z     |
2019-12-08T23:25:05.5639755Z 413 | nzint_impl_from! { NonZeroI8, NonZeroI32, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.5640406Z     |
2019-12-08T23:25:05.5640735Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.5641038Z     |
2019-12-08T23:25:05.5641374Z 1   | use crate::num::NonZeroI32;
2019-12-08T23:25:05.5641374Z 1   | use crate::num::NonZeroI32;
2019-12-08T23:25:05.5641639Z     |
2019-12-08T23:25:05.5681176Z 
2019-12-08T23:25:05.5681803Z error[E0412]: cannot find type `NonZeroI8` in this scope
2019-12-08T23:25:05.5682415Z     |
2019-12-08T23:25:05.5682415Z     |
2019-12-08T23:25:05.5682795Z 414 | nzint_impl_from! { NonZeroI8, NonZeroI64, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.5683454Z     |
2019-12-08T23:25:05.5683780Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.5684032Z     |
2019-12-08T23:25:05.5684357Z 1   | use crate::num::NonZeroI8;
2019-12-08T23:25:05.5684357Z 1   | use crate::num::NonZeroI8;
2019-12-08T23:25:05.5684618Z     |
2019-12-08T23:25:05.5684653Z 
2019-12-08T23:25:05.5684969Z error[E0412]: cannot find type `NonZeroI64` in this scope
2019-12-08T23:25:05.5685903Z     |
2019-12-08T23:25:05.5685903Z     |
2019-12-08T23:25:05.5687392Z 414 | nzint_impl_from! { NonZeroI8, NonZeroI64, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.5688200Z     |
2019-12-08T23:25:05.5688611Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.5688879Z     |
2019-12-08T23:25:05.5690380Z 1   | use crate::num::NonZeroI64;
2019-12-08T23:25:05.5690380Z 1   | use crate::num::NonZeroI64;
2019-12-08T23:25:05.5690680Z     |
2019-12-08T23:25:05.5690716Z 
2019-12-08T23:25:05.5691010Z error[E0412]: cannot find type `NonZeroI8` in this scope
2019-12-08T23:25:05.5691653Z     |
2019-12-08T23:25:05.5691653Z     |
2019-12-08T23:25:05.5692040Z 415 | nzint_impl_from! { NonZeroI8, NonZeroI128, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.5692682Z     |
2019-12-08T23:25:05.5692997Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.5693656Z     |
2019-12-08T23:25:05.5693977Z 1   | use crate::num::NonZeroI8;
2019-12-08T23:25:05.5693977Z 1   | use crate::num::NonZeroI8;
2019-12-08T23:25:05.5694225Z     |
2019-12-08T23:25:05.5694258Z 
2019-12-08T23:25:05.5694578Z error[E0412]: cannot find type `NonZeroI128` in this scope
2019-12-08T23:25:05.5695421Z     |
2019-12-08T23:25:05.5695421Z     |
2019-12-08T23:25:05.5695803Z 415 | nzint_impl_from! { NonZeroI8, NonZeroI128, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.5696423Z     |
2019-12-08T23:25:05.5696958Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.5697380Z     |
2019-12-08T23:25:05.5697699Z 1   | use crate::num::NonZeroI128;
2019-12-08T23:25:05.5697699Z 1   | use crate::num::NonZeroI128;
2019-12-08T23:25:05.5697951Z     |
2019-12-08T23:25:05.5697985Z 
2019-12-08T23:25:05.5698671Z error[E0412]: cannot find type `NonZeroI8` in this scope
2019-12-08T23:25:05.5699254Z     |
2019-12-08T23:25:05.5699254Z     |
2019-12-08T23:25:05.5699629Z 416 | nzint_impl_from! { NonZeroI8, NonZeroIsize, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.5700279Z     |
2019-12-08T23:25:05.5700600Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.5700895Z     |
2019-12-08T23:25:05.5701222Z 1   | use crate::num::NonZeroI8;
2019-12-08T23:25:05.5701222Z 1   | use crate::num::NonZeroI8;
2019-12-08T23:25:05.5701490Z     |
2019-12-08T23:25:05.5701542Z 
2019-12-08T23:25:05.5701996Z error[E0412]: cannot find type `NonZeroIsize` in this scope
2019-12-08T23:25:05.5702666Z    --> src/libcore/convert/num.rs:416:31
2019-12-08T23:25:05.5702935Z     |
2019-12-08T23:25:05.5703315Z 416 | nzint_impl_from! { NonZeroI8, NonZeroIsize, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.5704545Z     |
2019-12-08T23:25:05.5704866Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.5705135Z     |
2019-12-08T23:25:05.5705486Z 1   | use crate::num::NonZeroIsize;
2019-12-08T23:25:05.5705486Z 1   | use crate::num::NonZeroIsize;
2019-12-08T23:25:05.5705764Z     |
2019-12-08T23:25:05.5705799Z 
2019-12-08T23:25:05.5843153Z error[E0412]: cannot find type `NonZeroI16` in this scope
2019-12-08T23:25:05.5843790Z     |
2019-12-08T23:25:05.5843790Z     |
2019-12-08T23:25:05.5844215Z 417 | nzint_impl_from! { NonZeroI16, NonZeroI32, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.5844842Z     |
2019-12-08T23:25:05.5845571Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.5845840Z     |
2019-12-08T23:25:05.5846161Z 1   | use crate::num::NonZeroI16;
2019-12-08T23:25:05.5846161Z 1   | use crate::num::NonZeroI16;
2019-12-08T23:25:05.5846437Z     |
2019-12-08T23:25:05.5850671Z 
2019-12-08T23:25:05.5974857Z error[E0412]: cannot find type `NonZeroI32` in this scope
2019-12-08T23:25:05.5975429Z     |
2019-12-08T23:25:05.5975429Z     |
2019-12-08T23:25:05.5975761Z 417 | nzint_impl_from! { NonZeroI16, NonZeroI32, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.5976338Z     |
2019-12-08T23:25:05.5976630Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.5976905Z     |
2019-12-08T23:25:05.5977195Z 1   | use crate::num::NonZeroI32;
2019-12-08T23:25:05.5977195Z 1   | use crate::num::NonZeroI32;
2019-12-08T23:25:05.5977428Z     |
2019-12-08T23:25:05.5981666Z 
2019-12-08T23:25:05.6270501Z error[E0412]: cannot find type `NonZeroI16` in this scope
2019-12-08T23:25:05.6271221Z     |
2019-12-08T23:25:05.6271221Z     |
2019-12-08T23:25:05.6271589Z 418 | nzint_impl_from! { NonZeroI16, NonZeroI64, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.6272556Z     |
2019-12-08T23:25:05.6272918Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.6273194Z     |
2019-12-08T23:25:05.6273535Z 1   | use crate::num::NonZeroI16;
2019-12-08T23:25:05.6273535Z 1   | use crate::num::NonZeroI16;
2019-12-08T23:25:05.6273932Z     |
2019-12-08T23:25:05.6278278Z 
2019-12-08T23:25:05.6387725Z error[E0412]: cannot find type `NonZeroI64` in this scope
2019-12-08T23:25:05.6388327Z     |
2019-12-08T23:25:05.6388327Z     |
2019-12-08T23:25:05.6388744Z 418 | nzint_impl_from! { NonZeroI16, NonZeroI64, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.6389557Z     |
2019-12-08T23:25:05.6389896Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.6390188Z     |
2019-12-08T23:25:05.6390509Z 1   | use crate::num::NonZeroI64;
2019-12-08T23:25:05.6390509Z 1   | use crate::num::NonZeroI64;
2019-12-08T23:25:05.6390791Z     |
2019-12-08T23:25:05.6394996Z 
2019-12-08T23:25:05.6667564Z error[E0412]: cannot find type `NonZeroI16` in this scope
2019-12-08T23:25:05.6668168Z     |
2019-12-08T23:25:05.6668168Z     |
2019-12-08T23:25:05.6668546Z 419 | nzint_impl_from! { NonZeroI16, NonZeroI128, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.6669139Z     |
2019-12-08T23:25:05.6669445Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.6669726Z     |
2019-12-08T23:25:05.6670029Z 1   | use crate::num::NonZeroI16;
2019-12-08T23:25:05.6670029Z 1   | use crate::num::NonZeroI16;
2019-12-08T23:25:05.6670287Z     |
2019-12-08T23:25:05.6675386Z 
2019-12-08T23:25:05.6783417Z error[E0412]: cannot find type `NonZeroI128` in this scope
2019-12-08T23:25:05.6784088Z     |
2019-12-08T23:25:05.6784088Z     |
2019-12-08T23:25:05.6784476Z 419 | nzint_impl_from! { NonZeroI16, NonZeroI128, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.6785151Z     |
2019-12-08T23:25:05.6785476Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.6785743Z     |
2019-12-08T23:25:05.6786051Z 1   | use crate::num::NonZeroI128;
2019-12-08T23:25:05.6786051Z 1   | use crate::num::NonZeroI128;
2019-12-08T23:25:05.6786663Z     |
2019-12-08T23:25:05.6790831Z 
2019-12-08T23:25:05.7071373Z error[E0412]: cannot find type `NonZeroI16` in this scope
2019-12-08T23:25:05.7072430Z     |
2019-12-08T23:25:05.7072430Z     |
2019-12-08T23:25:05.7072800Z 420 | nzint_impl_from! { NonZeroI16, NonZeroIsize, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.7073755Z     |
2019-12-08T23:25:05.7074077Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.7074317Z     |
2019-12-08T23:25:05.7074873Z 1   | use crate::num::NonZeroI16;
2019-12-08T23:25:05.7074873Z 1   | use crate::num::NonZeroI16;
2019-12-08T23:25:05.7075289Z     |
2019-12-08T23:25:05.7079980Z 
2019-12-08T23:25:05.7197669Z error[E0412]: cannot find type `NonZeroIsize` in this scope
2019-12-08T23:25:05.7198024Z    --> src/libcore/convert/num.rs:420:32
2019-12-08T23:25:05.7198281Z     |
2019-12-08T23:25:05.7198684Z 420 | nzint_impl_from! { NonZeroI16, NonZeroIsize, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.7199414Z     |
2019-12-08T23:25:05.7199766Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.7200054Z     |
2019-12-08T23:25:05.7200394Z 1   | use crate::num::NonZeroIsize;
2019-12-08T23:25:05.7200394Z 1   | use crate::num::NonZeroIsize;
2019-12-08T23:25:05.7200685Z     |
2019-12-08T23:25:05.7204935Z 
2019-12-08T23:25:05.7486288Z error[E0412]: cannot find type `NonZeroI32` in this scope
2019-12-08T23:25:05.7487013Z     |
2019-12-08T23:25:05.7487013Z     |
2019-12-08T23:25:05.7487394Z 421 | nzint_impl_from! { NonZeroI32, NonZeroI64, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.7488035Z     |
2019-12-08T23:25:05.7488361Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.7488711Z     |
2019-12-08T23:25:05.7489018Z 1   | use crate::num::NonZeroI32;
2019-12-08T23:25:05.7489018Z 1   | use crate::num::NonZeroI32;
2019-12-08T23:25:05.7489264Z     |
2019-12-08T23:25:05.7493961Z 
2019-12-08T23:25:05.7606171Z error[E0412]: cannot find type `NonZeroI64` in this scope
2019-12-08T23:25:05.7606768Z     |
2019-12-08T23:25:05.7606768Z     |
2019-12-08T23:25:05.7607108Z 421 | nzint_impl_from! { NonZeroI32, NonZeroI64, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.7607696Z     |
2019-12-08T23:25:05.7608317Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.7608598Z     |
2019-12-08T23:25:05.7608904Z 1   | use crate::num::NonZeroI64;
2019-12-08T23:25:05.7608904Z 1   | use crate::num::NonZeroI64;
2019-12-08T23:25:05.7609789Z     |
2019-12-08T23:25:05.7614825Z 
2019-12-08T23:25:05.7915822Z error[E0412]: cannot find type `NonZeroI32` in this scope
2019-12-08T23:25:05.7916474Z     |
2019-12-08T23:25:05.7916474Z     |
2019-12-08T23:25:05.7916915Z 422 | nzint_impl_from! { NonZeroI32, NonZeroI128, #[stable(feature = "nz_int_conv", since = "1.41.0")] }
2019-12-08T23:25:05.7917592Z     |
2019-12-08T23:25:05.7917926Z help: possible candidate is found in another module, you can import it into scope
2019-12-08T23:25:05.7918214Z     |
2019-12-08T23:25:05.7918567Z 1   | use crate::num::NonZeroI32;
2019-12-08T23:25:05.7918567Z 1   | use crate::num::NonZeroI32;
2019-12-08T23:25:05.7918858Z     |
2019-12-08T23:25:05.7923787Z 
2019-12-08T23:25:05.8038060Z error[E0412]: cannot find type `NonZeroI128` in this scope
---
2019-12-08T23:25:14.1748621Z   local time: Sun Dec  8 23:25:14 UTC 2019
2019-12-08T23:25:14.4377401Z   network time: Sun, 08 Dec 2019 23:25:14 GMT
2019-12-08T23:25:14.4379012Z == end clock drift check ==
2019-12-08T23:25:27.6808682Z 
2019-12-08T23:25:27.6901773Z ##[error]Bash exited with code '1'.
2019-12-08T23:25:27.6926689Z ##[section]Starting: Checkout
2019-12-08T23:25:27.6928188Z ==============================================================================
2019-12-08T23:25:27.6928239Z Task         : Get sources
2019-12-08T23:25:27.6928281Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
