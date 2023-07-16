plain
2019-10-18T23:14:13.9832529Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-18T23:14:14.0022172Z ##[command]git config gc.auto 0
2019-10-18T23:14:14.0119365Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-18T23:14:14.0199897Z ##[command]git config --get-all http.proxy
2019-10-18T23:14:14.0398767Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65579/merge:refs/remotes/pull/65579/merge
---
2019-10-19T00:18:34.3537044Z .................................................................................................... 1600/9196
2019-10-19T00:18:39.7291125Z .................................................................................................... 1700/9196
2019-10-19T00:18:53.2528502Z .............................i...............i...................................................... 1800/9196
2019-10-19T00:19:00.9812399Z .................................................................................................... 1900/9196
2019-10-19T00:19:16.1152088Z ...................iiiii............................................................................ 2000/9196
2019-10-19T00:19:27.1154039Z .................................................................................................... 2200/9196
2019-10-19T00:19:29.8905974Z .................................................................................................... 2300/9196
2019-10-19T00:19:35.5752553Z .................................................................................................... 2400/9196
2019-10-19T00:19:58.6841943Z .................................................................................................... 2500/9196
---
2019-10-19T00:23:03.0810985Z ......................i...............i............................................................. 4800/9196
2019-10-19T00:23:15.4014832Z .................................................................................................... 4900/9196
2019-10-19T00:23:22.1357690Z .................................................................................................... 5000/9196
2019-10-19T00:23:33.2937147Z .................................................................................................... 5100/9196
2019-10-19T00:23:40.5945134Z ......................ii.ii......................................................................... 5200/9196
2019-10-19T00:23:50.9175026Z .................................................................................................... 5400/9196
2019-10-19T00:24:02.1005909Z ........................................................................................i........... 5500/9196
2019-10-19T00:24:10.1777628Z .................................................................................................... 5600/9196
2019-10-19T00:24:15.5284868Z .................................................................................................... 5700/9196
2019-10-19T00:24:15.5284868Z .................................................................................................... 5700/9196
2019-10-19T00:24:26.7627050Z .................................................F..........F........................ii...i..ii..... 5800/9196
2019-10-19T00:24:54.5636638Z .................................................................................................... 6000/9196
2019-10-19T00:25:04.6865109Z .................................................................................................... 6100/9196
2019-10-19T00:25:13.4675836Z .................................................................................................... 6200/9196
2019-10-19T00:25:13.4675836Z .................................................................................................... 6200/9196
2019-10-19T00:25:28.3271781Z .......i..ii........................................................................................ 6300/9196
2019-10-19T00:25:49.2584449Z ...................................................................i................................ 6500/9196
2019-10-19T00:25:51.5730052Z .................................................................................................... 6600/9196
2019-10-19T00:25:54.2356039Z .........................................i.......................................................... 6700/9196
2019-10-19T00:25:58.3952261Z .................................................................................................... 6800/9196
---
2019-10-19T00:30:16.3050199Z 
2019-10-19T00:30:16.3051482Z ---- [ui] ui/mismatched_types/cast-rfc0401.rs stdout ----
2019-10-19T00:30:16.3051590Z diff of stderr:
2019-10-19T00:30:16.3051631Z 
2019-10-19T00:30:16.3051890Z 158 LL |     let _ = v as *const [u8];
2019-10-19T00:30:16.3052038Z 160 
2019-10-19T00:30:16.3052095Z + error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
2019-10-19T00:30:16.3052417Z +   --> $DIR/cast-rfc0401.rs:53:13
2019-10-19T00:30:16.3052497Z +    |
2019-10-19T00:30:16.3052497Z +    |
2019-10-19T00:30:16.3052547Z + LL |     let _ = fat_v as *const dyn Foo;
2019-10-19T00:30:16.3052901Z +    |
2019-10-19T00:30:16.3052954Z +    = help: the trait `std::marker::Sized` is not implemented for `[u8]`
2019-10-19T00:30:16.3053311Z +    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-10-19T00:30:16.3053402Z +    = note: required for the cast to the object type `dyn Foo`
2019-10-19T00:30:16.3053402Z +    = note: required for the cast to the object type `dyn Foo`
2019-10-19T00:30:16.3053454Z + 
2019-10-19T00:30:16.3053505Z 161 error[E0606]: casting `&dyn Foo` as `*const str` is invalid
2019-10-19T00:30:16.3053911Z 163    |
2019-10-19T00:30:16.3053942Z 
2019-10-19T00:30:16.3053987Z 196    |
2019-10-19T00:30:16.3054341Z 197    = help: cast through a thin pointer first
2019-10-19T00:30:16.3054341Z 197    = help: cast through a thin pointer first
2019-10-19T00:30:16.3054388Z 198 
2019-10-19T00:30:16.3054443Z + error[E0277]: the size for values of type `str` cannot be known at compilation time
2019-10-19T00:30:16.3054759Z +   --> $DIR/cast-rfc0401.rs:62:13
2019-10-19T00:30:16.3054811Z +    |
2019-10-19T00:30:16.3054858Z + LL |     let _ = a as *const dyn Foo;
2019-10-19T00:30:16.3055142Z +    |             ^ doesn't have a size known at compile-time
2019-10-19T00:30:16.3055248Z +    = help: the trait `std::marker::Sized` is not implemented for `str`
2019-10-19T00:30:16.3055979Z +    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-10-19T00:30:16.3056061Z +    = note: required for the cast to the object type `dyn Foo`
2019-10-19T00:30:16.3056112Z + 
2019-10-19T00:30:16.3056112Z + 
2019-10-19T00:30:16.3056203Z 199 error[E0606]: casting `*const dyn Foo` as `*const [u16]` is invalid
2019-10-19T00:30:16.3056560Z 201    |
2019-10-19T00:30:16.3056593Z 
2019-10-19T00:30:16.3056665Z 211    |             ^^^^^^^^^^^^^^^^^^^^
2019-10-19T00:30:16.3056715Z 212    |
2019-10-19T00:30:16.3056715Z 212    |
2019-10-19T00:30:16.3056765Z 213    = note: vtable kinds may not match
2019-10-19T00:30:16.3057320Z - error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
2019-10-19T00:30:16.3057581Z -   --> $DIR/cast-rfc0401.rs:53:13
2019-10-19T00:30:16.3057796Z -    |
2019-10-19T00:30:16.3057796Z -    |
2019-10-19T00:30:16.3058078Z - LL |     let _ = fat_v as *const dyn Foo;
2019-10-19T00:30:16.3058586Z -    |
2019-10-19T00:30:16.3058968Z -    = help: the trait `std::marker::Sized` is not implemented for `[u8]`
2019-10-19T00:30:16.3059578Z -    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-10-19T00:30:16.3059943Z -    = note: required for the cast to the object type `dyn Foo`
2019-10-19T00:30:16.3059943Z -    = note: required for the cast to the object type `dyn Foo`
2019-10-19T00:30:16.3060192Z - 
2019-10-19T00:30:16.3060498Z - error[E0277]: the size for values of type `str` cannot be known at compilation time
2019-10-19T00:30:16.3060757Z -   --> $DIR/cast-rfc0401.rs:62:13
2019-10-19T00:30:16.3061000Z -    |
2019-10-19T00:30:16.3061254Z - LL |     let _ = a as *const dyn Foo;
2019-10-19T00:30:16.3061535Z -    |             ^ doesn't have a size known at compile-time
2019-10-19T00:30:16.3062064Z -    = help: the trait `std::marker::Sized` is not implemented for `str`
2019-10-19T00:30:16.3062433Z -    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-10-19T00:30:16.3062744Z -    = note: required for the cast to the object type `dyn Foo`
2019-10-19T00:30:16.3062810Z 234 
2019-10-19T00:30:16.3062810Z 234 
2019-10-19T00:30:16.3062862Z 235 error[E0606]: casting `&{float}` as `f32` is invalid
2019-10-19T00:30:16.3063193Z 
2019-10-19T00:30:16.3063224Z 
2019-10-19T00:30:16.3063276Z The actual stderr differed from the expected stderr.
2019-10-19T00:30:16.3063657Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401/cast-rfc0401.stderr
2019-10-19T00:30:16.3063657Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401/cast-rfc0401.stderr
2019-10-19T00:30:16.3063951Z To update references, rerun the tests and pass the `--bless` flag
2019-10-19T00:30:16.3064265Z To only update this specific test, also pass `--test-args mismatched_types/cast-rfc0401.rs`
2019-10-19T00:30:16.3064457Z error: 1 errors occurred comparing output.
2019-10-19T00:30:16.3064509Z status: exit code: 1
2019-10-19T00:30:16.3064509Z status: exit code: 1
2019-10-19T00:30:16.3065615Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/cast-rfc0401.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401/auxiliary" "-A" "unused"
2019-10-19T00:30:16.3066271Z ------------------------------------------
2019-10-19T00:30:16.3066333Z 
2019-10-19T00:30:16.3066577Z ------------------------------------------
2019-10-19T00:30:16.3066627Z stderr:
2019-10-19T00:30:16.3066627Z stderr:
2019-10-19T00:30:16.3066878Z ------------------------------------------
2019-10-19T00:30:16.3066936Z error[E0606]: casting `*const U` as `*const V` is invalid
2019-10-19T00:30:16.3067284Z    |
2019-10-19T00:30:16.3067284Z    |
2019-10-19T00:30:16.3067334Z LL |     u as *const V //~ ERROR is invalid
2019-10-19T00:30:16.3067440Z    |
2019-10-19T00:30:16.3067440Z    |
2019-10-19T00:30:16.3067515Z    = note: vtable kinds may not match
2019-10-19T00:30:16.3067548Z 
2019-10-19T00:30:16.3067598Z error[E0606]: casting `*const U` as `*const str` is invalid
2019-10-19T00:30:16.3067946Z    |
2019-10-19T00:30:16.3067946Z    |
2019-10-19T00:30:16.3067996Z LL |     u as *const str //~ ERROR is invalid
2019-10-19T00:30:16.3068116Z    |
2019-10-19T00:30:16.3068116Z    |
2019-10-19T00:30:16.3068166Z    = note: vtable kinds may not match
2019-10-19T00:30:16.3068200Z 
2019-10-19T00:30:16.3068270Z error[E0609]: no field `f` on type `fn() {main}`
2019-10-19T00:30:16.3068622Z    |
2019-10-19T00:30:16.3068622Z    |
2019-10-19T00:30:16.3068675Z LL |     let _ = main.f as *const u32; //~ ERROR no field
2019-10-19T00:30:16.3068916Z 
2019-10-19T00:30:16.3068916Z 
2019-10-19T00:30:16.3069239Z error[E0605]: non-primitive cast: `*const u8` as `&u8`
2019-10-19T00:30:16.3069621Z    |
2019-10-19T00:30:16.3069621Z    |
2019-10-19T00:30:16.3069914Z LL |     let _ = v as &u8; //~ ERROR non-primitive cast
2019-10-19T00:30:16.3070039Z    |
2019-10-19T00:30:16.3070039Z    |
2019-10-19T00:30:16.3070098Z    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
2019-10-19T00:30:16.3070141Z 
2019-10-19T00:30:16.3070440Z error[E0605]: non-primitive cast: `*const u8` as `E`
2019-10-19T00:30:16.3070839Z    |
2019-10-19T00:30:16.3070839Z    |
2019-10-19T00:30:16.3071128Z LL |     let _ = v as E; //~ ERROR non-primitive cast
2019-10-19T00:30:16.3071284Z    |
2019-10-19T00:30:16.3071284Z    |
2019-10-19T00:30:16.3071374Z    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
2019-10-19T00:30:16.3071415Z 
2019-10-19T00:30:16.3071726Z error[E0605]: non-primitive cast: `*const u8` as `fn()`
2019-10-19T00:30:16.3072087Z    |
2019-10-19T00:30:16.3072087Z    |
2019-10-19T00:30:16.3072356Z LL |     let _ = v as fn(); //~ ERROR non-primitive cast
2019-10-19T00:30:16.3072481Z    |
2019-10-19T00:30:16.3072481Z    |
2019-10-19T00:30:16.3072541Z    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
2019-10-19T00:30:16.3072581Z 
2019-10-19T00:30:16.3072859Z error[E0605]: non-primitive cast: `*const u8` as `(u32,)`
2019-10-19T00:30:16.3073225Z    |
2019-10-19T00:30:16.3073225Z    |
2019-10-19T00:30:16.3073495Z LL |     let _ = v as (u32,); //~ ERROR non-primitive cast
2019-10-19T00:30:16.3073630Z    |
2019-10-19T00:30:16.3073630Z    |
2019-10-19T00:30:16.3073689Z    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
2019-10-19T00:30:16.3073866Z 
2019-10-19T00:30:16.3074207Z error[E0605]: non-primitive cast: `std::option::Option<&*const u8>` as `*const u8`
2019-10-19T00:30:16.3074558Z    |
2019-10-19T00:30:16.3074558Z    |
2019-10-19T00:30:16.3074862Z LL |     let _ = Some(&v) as *const u8; //~ ERROR non-primitive cast
2019-10-19T00:30:16.3074972Z    |
2019-10-19T00:30:16.3074972Z    |
2019-10-19T00:30:16.3075087Z    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
2019-10-19T00:30:16.3075127Z 
2019-10-19T00:30:16.3075180Z error[E0606]: casting `*const u8` as `f32` is invalid
2019-10-19T00:30:16.3075914Z    |
2019-10-19T00:30:16.3075914Z    |
2019-10-19T00:30:16.3075966Z LL |     let _ = v as f32; //~ ERROR is invalid
2019-10-19T00:30:16.3076079Z 
2019-10-19T00:30:16.3076079Z 
2019-10-19T00:30:16.3076129Z error[E0606]: casting `fn() {main}` as `f64` is invalid
2019-10-19T00:30:16.3076485Z    |
2019-10-19T00:30:16.3076485Z    |
2019-10-19T00:30:16.3076535Z LL |     let _ = main as f64; //~ ERROR is invalid
2019-10-19T00:30:16.3076617Z 
2019-10-19T00:30:16.3076617Z 
2019-10-19T00:30:16.3076685Z error[E0606]: casting `&*const u8` as `usize` is invalid
2019-10-19T00:30:16.3077004Z    |
2019-10-19T00:30:16.3077073Z LL |     let _ = &v as usize; //~ ERROR is invalid
2019-10-19T00:30:16.3077124Z    |             ^^^^^^^^^^^
2019-10-19T00:30:16.3077169Z    |
2019-10-19T00:30:16.3077169Z    |
2019-10-19T00:30:16.3077359Z    = help: cast through a raw pointer first
2019-10-19T00:30:16.3077423Z 
2019-10-19T00:30:16.3077474Z error[E0606]: casting `f32` as `*const u8` is invalid
2019-10-19T00:30:16.3077865Z    |
2019-10-19T00:30:16.3077865Z    |
2019-10-19T00:30:16.3077915Z LL |     let _ = f as *const u8; //~ ERROR is invalid
2019-10-19T00:30:16.3077997Z 
2019-10-19T00:30:16.3078064Z error[E0054]: cannot cast as `bool`
2019-10-19T00:30:16.3078331Z   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:39:13
2019-10-19T00:30:16.3078383Z    |
2019-10-19T00:30:16.3078383Z    |
2019-10-19T00:30:16.3078453Z LL |     let _ = 3_i32 as bool; //~ ERROR cannot cast
2019-10-19T00:30:16.3078511Z    |             ^^^^^^^^^^^^^ help: compare with zero instead: `3_i32 != 0`
2019-10-19T00:30:16.3078593Z error[E0054]: cannot cast as `bool`
2019-10-19T00:30:16.3078880Z   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:40:13
2019-10-19T00:30:16.3078940Z    |
2019-10-19T00:30:16.3078940Z    |
2019-10-19T00:30:16.3078989Z LL |     let _ = E::A as bool; //~ ERROR cannot cast
2019-10-19T00:30:16.3079072Z    |             ^^^^^^^^^^^^ unsupported cast
2019-10-19T00:30:16.3079104Z 
2019-10-19T00:30:16.3079153Z error[E0604]: only `u8` can be cast as `char`, not `u32`
2019-10-19T00:30:16.3079492Z    |
2019-10-19T00:30:16.3079492Z    |
2019-10-19T00:30:16.3079543Z LL |     let _ = 0x61u32 as char; //~ ERROR can be cast as
2019-10-19T00:30:16.3079645Z 
2019-10-19T00:30:16.3079645Z 
2019-10-19T00:30:16.3079694Z error[E0606]: casting `bool` as `f32` is invalid
2019-10-19T00:30:16.3080032Z    |
2019-10-19T00:30:16.3080032Z    |
2019-10-19T00:30:16.3080083Z LL |     let _ = false as f32; //~ ERROR is invalid
2019-10-19T00:30:16.3080180Z    |
2019-10-19T00:30:16.3080256Z    = help: cast through an integer first
2019-10-19T00:30:16.3080289Z 
2019-10-19T00:30:16.3080289Z 
2019-10-19T00:30:16.3080338Z error[E0606]: casting `E` as `f32` is invalid
2019-10-19T00:30:16.3080818Z    |
2019-10-19T00:30:16.3080818Z    |
2019-10-19T00:30:16.3080867Z LL |     let _ = E::A as f32; //~ ERROR is invalid
2019-10-19T00:30:16.3080982Z    |
2019-10-19T00:30:16.3081030Z    = help: cast through an integer first
2019-10-19T00:30:16.3081062Z 
2019-10-19T00:30:16.3081062Z 
2019-10-19T00:30:16.3081111Z error[E0606]: casting `char` as `f32` is invalid
2019-10-19T00:30:16.3081451Z    |
2019-10-19T00:30:16.3081451Z    |
2019-10-19T00:30:16.3082400Z LL |     let _ = 'a' as f32; //~ ERROR is invalid
2019-10-19T00:30:16.3082536Z    |
2019-10-19T00:30:16.3082582Z    = help: cast through an integer first
2019-10-19T00:30:16.3082633Z 
2019-10-19T00:30:16.3082633Z 
2019-10-19T00:30:16.3082696Z error[E0606]: casting `bool` as `*const u8` is invalid
2019-10-19T00:30:16.3083082Z    |
2019-10-19T00:30:16.3083082Z    |
2019-10-19T00:30:16.3083156Z LL |     let _ = false as *const u8; //~ ERROR is invalid
2019-10-19T00:30:16.3083239Z 
2019-10-19T00:30:16.3083239Z 
2019-10-19T00:30:16.3083308Z error[E0606]: casting `E` as `*const u8` is invalid
2019-10-19T00:30:16.3083633Z    |
2019-10-19T00:30:16.3083633Z    |
2019-10-19T00:30:16.3083682Z LL |     let _ = E::A as *const u8; //~ ERROR is invalid
2019-10-19T00:30:16.3083786Z 
2019-10-19T00:30:16.3083786Z 
2019-10-19T00:30:16.3083836Z error[E0606]: casting `char` as `*const u8` is invalid
2019-10-19T00:30:16.3084177Z    |
2019-10-19T00:30:16.3084177Z    |
2019-10-19T00:30:16.3084558Z LL |     let _ = 'a' as *const u8; //~ ERROR is invalid
2019-10-19T00:30:16.3084695Z 
2019-10-19T00:30:16.3084695Z 
2019-10-19T00:30:16.3084743Z error[E0606]: casting `usize` as `*const [u8]` is invalid
2019-10-19T00:30:16.3085142Z    |
2019-10-19T00:30:16.3085142Z    |
2019-10-19T00:30:16.3085268Z LL |     let _ = 42usize as *const [u8]; //~ ERROR is invalid
2019-10-19T00:30:16.3085592Z 
2019-10-19T00:30:16.3085592Z 
2019-10-19T00:30:16.3085684Z error[E0607]: cannot cast thin pointer `*const u8` to fat pointer `*const [u8]`
2019-10-19T00:30:16.3086086Z    |
2019-10-19T00:30:16.3086086Z    |
2019-10-19T00:30:16.3086156Z LL |     let _ = v as *const [u8]; //~ ERROR cannot cast
2019-10-19T00:30:16.3086238Z 
2019-10-19T00:30:16.3086305Z error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
2019-10-19T00:30:16.3086607Z   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:53:13
2019-10-19T00:30:16.3086669Z    |
2019-10-19T00:30:16.3086669Z    |
2019-10-19T00:30:16.3086721Z LL |     let _ = fat_v as *const dyn Foo; //~ ERROR the size for values of type
2019-10-19T00:30:16.3087062Z    |
2019-10-19T00:30:16.3087114Z    = help: the trait `std::marker::Sized` is not implemented for `[u8]`
2019-10-19T00:30:16.3087483Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-10-19T00:30:16.3087551Z    = note: required for the cast to the object type `dyn Foo`
2019-10-19T00:30:16.3087551Z    = note: required for the cast to the object type `dyn Foo`
2019-10-19T00:30:16.3087586Z 
2019-10-19T00:30:16.3087661Z error[E0606]: casting `&dyn Foo` as `*const str` is invalid
2019-10-19T00:30:16.3088012Z    |
2019-10-19T00:30:16.3088012Z    |
2019-10-19T00:30:16.3088094Z LL |     let _ = foo as *const str; //~ ERROR is invalid
2019-10-19T00:30:16.3088406Z 
2019-10-19T00:30:16.3088406Z 
2019-10-19T00:30:16.3088459Z error[E0606]: casting `&dyn Foo` as `*mut str` is invalid
2019-10-19T00:30:16.3088869Z    |
2019-10-19T00:30:16.3088869Z    |
2019-10-19T00:30:16.3088922Z LL |     let _ = foo as *mut str; //~ ERROR is invalid
2019-10-19T00:30:16.3089031Z 
2019-10-19T00:30:16.3089031Z 
2019-10-19T00:30:16.3089085Z error[E0606]: casting `fn() {main}` as `*mut str` is invalid
2019-10-19T00:30:16.3089452Z    |
2019-10-19T00:30:16.3089452Z    |
2019-10-19T00:30:16.3089504Z LL |     let _ = main as *mut str; //~ ERROR is invalid
2019-10-19T00:30:16.3089610Z 
2019-10-19T00:30:16.3089610Z 
2019-10-19T00:30:16.3089672Z error[E0606]: casting `&f32` as `*mut f32` is invalid
2019-10-19T00:30:16.3090028Z    |
2019-10-19T00:30:16.3090028Z    |
2019-10-19T00:30:16.3090099Z LL |     let _ = &f as *mut f32; //~ ERROR is invalid
2019-10-19T00:30:16.3090185Z 
2019-10-19T00:30:16.3090185Z 
2019-10-19T00:30:16.3090256Z error[E0606]: casting `&f32` as `*const f64` is invalid
2019-10-19T00:30:16.3090602Z    |
2019-10-19T00:30:16.3090602Z    |
2019-10-19T00:30:16.3090656Z LL |     let _ = &f as *const f64; //~ ERROR is invalid
2019-10-19T00:30:16.3090764Z 
2019-10-19T00:30:16.3090764Z 
2019-10-19T00:30:16.3090816Z error[E0606]: casting `*const [i8]` as `usize` is invalid
2019-10-19T00:30:16.3091185Z    |
2019-10-19T00:30:16.3091185Z    |
2019-10-19T00:30:16.3091342Z LL |     let _ = fat_sv as usize; //~ ERROR is invalid
2019-10-19T00:30:16.3091476Z    |
2019-10-19T00:30:16.3091790Z    = help: cast through a thin pointer first
2019-10-19T00:30:16.3091825Z 
2019-10-19T00:30:16.3091904Z error[E0277]: the size for values of type `str` cannot be known at compilation time
2019-10-19T00:30:16.3091904Z error[E0277]: the size for values of type `str` cannot be known at compilation time
2019-10-19T00:30:16.3092252Z   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:62:13
2019-10-19T00:30:16.3092308Z    |
2019-10-19T00:30:16.3092382Z LL |     let _ = a as *const dyn Foo; //~ ERROR the size for values of type
2019-10-19T00:30:16.3092668Z    |             ^ doesn't have a size known at compile-time
2019-10-19T00:30:16.3092795Z    = help: the trait `std::marker::Sized` is not implemented for `str`
2019-10-19T00:30:16.3093167Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-10-19T00:30:16.3093247Z    = note: required for the cast to the object type `dyn Foo`
2019-10-19T00:30:16.3093283Z 
2019-10-19T00:30:16.3093283Z 
2019-10-19T00:30:16.3093358Z error[E0606]: casting `*const dyn Foo` as `*const [u16]` is invalid
2019-10-19T00:30:16.3093713Z    |
2019-10-19T00:30:16.3093713Z    |
2019-10-19T00:30:16.3093787Z LL |     let _ = cf as *const [u16]; //~ ERROR is invalid
2019-10-19T00:30:16.3093887Z    |
2019-10-19T00:30:16.3093887Z    |
2019-10-19T00:30:16.3093958Z    = note: vtable kinds may not match
2019-10-19T00:30:16.3093992Z 
2019-10-19T00:30:16.3094046Z error[E0606]: casting `*const dyn Foo` as `*const dyn Bar` is invalid
2019-10-19T00:30:16.3094431Z    |
2019-10-19T00:30:16.3094431Z    |
2019-10-19T00:30:16.3094481Z LL |     let _ = cf as *const dyn Bar; //~ ERROR is invalid
2019-10-19T00:30:16.3094604Z    |
2019-10-19T00:30:16.3094604Z    |
2019-10-19T00:30:16.3094661Z    = note: vtable kinds may not match
2019-10-19T00:30:16.3094694Z 
2019-10-19T00:30:16.3094745Z error[E0606]: casting `&{float}` as `f32` is invalid
2019-10-19T00:30:16.3095264Z    |
2019-10-19T00:30:16.3095264Z    |
2019-10-19T00:30:16.3095605Z LL |     vec![0.0].iter().map(|s| s as f32).collect::<Vec<f32>>(); //~ ERROR is invalid
2019-10-19T00:30:16.3096382Z    |                              -^^^^^^^
2019-10-19T00:30:16.3096449Z    |                              |
2019-10-19T00:30:16.3096503Z    |                              cannot cast `&{float}` as `f32`
2019-10-19T00:30:16.3096586Z    |                              help: dereference the expression: `*s`
2019-10-19T00:30:16.3096669Z error: aborting due to 34 previous errors
2019-10-19T00:30:16.3096701Z 
2019-10-19T00:30:16.3096775Z Some errors have detailed explanations: E0054, E0277, E0604, E0605, E0606, E0607, E0609.
2019-10-19T00:30:16.3097072Z For more information about an error, try `rustc --explain E0054`.
---
2019-10-19T00:30:16.3097806Z 
2019-10-19T00:30:16.3097855Z + error[E0631]: type mismatch in closure arguments
2019-10-19T00:30:16.3098097Z +   --> $DIR/issue-36053-2.rs:7:32
2019-10-19T00:30:16.3098166Z +    |
2019-10-19T00:30:16.3098220Z + LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
2019-10-19T00:30:16.3098530Z +    |                                ^^^^^^ -------------- found signature of `for<'r> fn(&'r str) -> _`
2019-10-19T00:30:16.3098608Z +    |                                |
2019-10-19T00:30:16.3098894Z +    |                                expected signature of `for<'r> fn(&'r &str) -> _`
2019-10-19T00:30:16.3098947Z + 
2019-10-19T00:30:16.3099569Z 1 error[E0599]: no method named `count` found for type `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:7:39: 7:53]>` in the current scope
2019-10-19T00:30:16.3099918Z 2   --> $DIR/issue-36053-2.rs:7:55
2019-10-19T00:30:16.3100004Z 
2019-10-19T00:30:16.3100080Z 7    = note: the method `count` exists but the following trait bounds were not satisfied:
2019-10-19T00:30:16.3100080Z 7    = note: the method `count` exists but the following trait bounds were not satisfied:
2019-10-19T00:30:16.3100443Z 8            `&mut std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:7:39: 7:53]> : std::iter::Iterator`
2019-10-19T00:30:16.3100813Z 9            `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:7:39: 7:53]> : std::iter::Iterator`
2019-10-19T00:30:16.3101288Z - error[E0631]: type mismatch in closure arguments
2019-10-19T00:30:16.3101526Z -   --> $DIR/issue-36053-2.rs:7:32
2019-10-19T00:30:16.3101749Z -    |
2019-10-19T00:30:16.3101749Z -    |
2019-10-19T00:30:16.3102023Z - LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
2019-10-19T00:30:16.3102335Z -    |                                ^^^^^^ -------------- found signature of `for<'r> fn(&'r str) -> _`
2019-10-19T00:30:16.3102610Z -    |                                |
2019-10-19T00:30:16.3102894Z -    |                                expected signature of `for<'r> fn(&'r &str) -> _`
2019-10-19T00:30:16.3103018Z 19 error: aborting due to 2 previous errors
2019-10-19T00:30:16.3103064Z 20 
2019-10-19T00:30:16.3103093Z 
2019-10-19T00:30:16.3103121Z 
2019-10-19T00:30:16.3103121Z 
2019-10-19T00:30:16.3103187Z The actual stderr differed from the expected stderr.
2019-10-19T00:30:16.3103534Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/issue-36053-2.stderr
2019-10-19T00:30:16.3103828Z To update references, rerun the tests and pass the `--bless` flag
2019-10-19T00:30:16.3104165Z To only update this specific test, also pass `--test-args mismatched_types/issue-36053-2.rs`
2019-10-19T00:30:16.3104266Z error: 1 errors occurred comparing output.
2019-10-19T00:30:16.3104436Z status: exit code: 1
2019-10-19T00:30:16.3104436Z status: exit code: 1
2019-10-19T00:30:16.3105619Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/issue-36053-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/auxiliary" "-A" "unused"
2019-10-19T00:30:16.3106088Z ------------------------------------------
2019-10-19T00:30:16.3106127Z 
2019-10-19T00:30:16.3106367Z ------------------------------------------
2019-10-19T00:30:16.3106438Z stderr:
2019-10-19T00:30:16.3106438Z stderr:
2019-10-19T00:30:16.3106691Z ------------------------------------------
2019-10-19T00:30:16.3106747Z error[E0631]: type mismatch in closure arguments
2019-10-19T00:30:16.3107049Z   --> /checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:32
2019-10-19T00:30:16.3107104Z    |
2019-10-19T00:30:16.3107160Z LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
2019-10-19T00:30:16.3107485Z    |                                ^^^^^^ -------------- found signature of `for<'r> fn(&'r str) -> _`
2019-10-19T00:30:16.3107547Z    |                                |
2019-10-19T00:30:16.3107833Z    |                                expected signature of `for<'r> fn(&'r &str) -> _`
2019-10-19T00:30:16.3107893Z 
2019-10-19T00:30:16.3108320Z error[E0599]: no method named `count` found for type `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53]>` in the current scope
2019-10-19T00:30:16.3108756Z   --> /checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:55
2019-10-19T00:30:16.3108845Z    |
2019-10-19T00:30:16.3108897Z LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
2019-10-19T00:30:16.3109449Z    |                                                       ^^^^^ method not found in `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53]>`
2019-10-19T00:30:16.3109590Z    = note: the method `count` exists but the following trait bounds were not satisfied:
2019-10-19T00:30:16.3109590Z    = note: the method `count` exists but the following trait bounds were not satisfied:
2019-10-19T00:30:16.3109985Z            `&mut std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53]> : std::iter::Iterator`
2019-10-19T00:30:16.3110401Z            `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53]> : std::iter::Iterator`
2019-10-19T00:30:16.3110505Z error: aborting due to 2 previous errors
2019-10-19T00:30:16.3110559Z 
2019-10-19T00:30:16.3110828Z For more information about this error, try `rustc --explain E0599`.
2019-10-19T00:30:16.3110873Z 
2019-10-19T00:30:16.3110873Z 
2019-10-19T00:30:16.3111109Z ------------------------------------------
2019-10-19T00:30:16.3111164Z 
2019-10-19T00:30:16.3111192Z 
2019-10-19T00:30:16.3111464Z ---- [ui] ui/traits/reservation-impls/reservation-impl-no-use.rs stdout ----
2019-10-19T00:30:16.3111518Z diff of stderr:
2019-10-19T00:30:16.3111570Z 
2019-10-19T00:30:16.3111620Z 1 error[E0277]: the trait bound `(): MyTrait` is not satisfied
2019-10-19T00:30:16.3112117Z +   --> $DIR/reservation-impl-no-use.rs:12:5
2019-10-19T00:30:16.3112188Z 3    |
2019-10-19T00:30:16.3112188Z 3    |
2019-10-19T00:30:16.3112235Z 4 LL | trait MyTrait { fn foo(&self); }
2019-10-19T00:30:16.3112502Z 5    |                 -------------- required by `MyTrait::foo`
2019-10-19T00:30:16.3112601Z 6 ...
2019-10-19T00:30:16.3112601Z 6 ...
2019-10-19T00:30:16.3112656Z 7 LL |     <() as MyTrait>::foo(&());
2019-10-19T00:30:16.3112941Z -    |                          ^^^ the trait `MyTrait` is not implemented for `()`
2019-10-19T00:30:16.3113134Z +    |     ^^^^^^^^^^^^^^^^^^^^ the trait `MyTrait` is not implemented for `()`
2019-10-19T00:30:16.3113234Z 10    = help: the following implementations were found:
2019-10-19T00:30:16.3113234Z 10    = help: the following implementations were found:
2019-10-19T00:30:16.3113307Z 11              <() as MyTrait>
2019-10-19T00:30:16.3113367Z 
2019-10-19T00:30:16.3113416Z The actual stderr differed from the expected stderr.
2019-10-19T00:30:16.3113837Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/reservation-impls/reservation-impl-no-use/reservation-impl-no-use.stderr
2019-10-19T00:30:16.3113837Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/reservation-impls/reservation-impl-no-use/reservation-impl-no-use.stderr
2019-10-19T00:30:16.3114112Z To update references, rerun the tests and pass the `--bless` flag
2019-10-19T00:30:16.3114423Z To only update this specific test, also pass `--test-args traits/reservation-impls/reservation-impl-no-use.rs`
2019-10-19T00:30:16.3114585Z error: 1 errors occurred comparing output.
2019-10-19T00:30:16.3114641Z status: exit code: 1
2019-10-19T00:30:16.3114641Z status: exit code: 1
2019-10-19T00:30:16.3115911Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/reservation-impls/reservation-impl-no-use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/reservation-impls/reservation-impl-no-use" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/reservation-impls/reservation-impl-no-use/auxiliary" "-A" "unused"
2019-10-19T00:30:16.3116348Z ------------------------------------------
2019-10-19T00:30:16.3116386Z 
2019-10-19T00:30:16.3116773Z ------------------------------------------
2019-10-19T00:30:16.3116861Z stderr:
2019-10-19T00:30:16.3116861Z stderr:
2019-10-19T00:30:16.3117132Z ------------------------------------------
2019-10-19T00:30:16.3117199Z error[E0277]: the trait bound `(): MyTrait` is not satisfied
2019-10-19T00:30:16.3117573Z    |
2019-10-19T00:30:16.3117573Z    |
2019-10-19T00:30:16.3117622Z LL | trait MyTrait { fn foo(&self); }
2019-10-19T00:30:16.3117902Z    |                 -------------- required by `MyTrait::foo`
2019-10-19T00:30:16.3117954Z ...
2019-10-19T00:30:16.3118001Z LL |     <() as MyTrait>::foo(&());
2019-10-19T00:30:16.3118056Z    |     ^^^^^^^^^^^^^^^^^^^^ the trait `MyTrait` is not implemented for `()`
2019-10-19T00:30:16.3118182Z    = help: the following implementations were found:
2019-10-19T00:30:16.3118182Z    = help: the following implementations were found:
2019-10-19T00:30:16.3118236Z              <() as MyTrait>
2019-10-19T00:30:16.3118339Z error: aborting due to previous error
2019-10-19T00:30:16.3118372Z 
2019-10-19T00:30:16.3118734Z For more information about this error, try `rustc --explain E0277`.
2019-10-19T00:30:16.3118801Z 
---
2019-10-19T00:30:16.3119885Z -   --> $DIR/trait-object-safety.rs:15:22
2019-10-19T00:30:16.3120148Z +   --> $DIR/trait-object-safety.rs:15:12
2019-10-19T00:30:16.3120200Z 3    |
2019-10-19T00:30:16.3120267Z 4 LL |     fn foo();
2019-10-19T00:30:16.3120558Z 5    |        --- associated function `foo` has no `self` parameter
2019-10-19T00:30:16.3120643Z 6 ...
2019-10-19T00:30:16.3120643Z 6 ...
2019-10-19T00:30:16.3120714Z 7 LL |     let _: &dyn Tr = &St;
2019-10-19T00:30:16.3121020Z -    |                      ^^^ the trait `Tr` cannot be made into an object
2019-10-19T00:30:16.3121244Z -    |
2019-10-19T00:30:16.3121591Z -    = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<&dyn Tr>` for `&St`
2019-10-19T00:30:16.3121964Z +    |            ^^^^^^^ the trait `Tr` cannot be made into an object
2019-10-19T00:30:16.3122098Z 12 error[E0038]: the trait `Tr` cannot be made into an object
2019-10-19T00:30:16.3122440Z -   --> $DIR/trait-object-safety.rs:15:12
2019-10-19T00:30:16.3122709Z +   --> $DIR/trait-object-safety.rs:15:22
2019-10-19T00:30:16.3122775Z 14    |
2019-10-19T00:30:16.3122775Z 14    |
2019-10-19T00:30:16.3122825Z 15 LL |     fn foo();
2019-10-19T00:30:16.3152282Z 16    |        --- associated function `foo` has no `self` parameter
2019-10-19T00:30:16.3152415Z 17 ...
2019-10-19T00:30:16.3152415Z 17 ...
2019-10-19T00:30:16.3152457Z 18 LL |     let _: &dyn Tr = &St;
2019-10-19T00:30:16.3152903Z -    |            ^^^^^^^ the trait `Tr` cannot be made into an object
2019-10-19T00:30:16.3152975Z +    |                      ^^^ the trait `Tr` cannot be made into an object
2019-10-19T00:30:16.3153017Z +    |
2019-10-19T00:30:16.3153086Z +    = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<&dyn Tr>` for `&St`
2019-10-19T00:30:16.3153172Z 21 error: aborting due to 2 previous errors
2019-10-19T00:30:16.3153222Z 22 
2019-10-19T00:30:16.3153247Z 
2019-10-19T00:30:16.3153271Z 
2019-10-19T00:30:16.3153271Z 
2019-10-19T00:30:16.3153312Z The actual stderr differed from the expected stderr.
2019-10-19T00:30:16.3153632Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-safety/trait-object-safety.stderr
2019-10-19T00:30:16.3153863Z To update references, rerun the tests and pass the `--bless` flag
2019-10-19T00:30:16.3154102Z To only update this specific test, also pass `--test-args traits/trait-object-safety.rs`
2019-10-19T00:30:16.3154190Z error: 1 errors occurred comparing output.
2019-10-19T00:30:16.3154397Z status: exit code: 1
2019-10-19T00:30:16.3154397Z status: exit code: 1
2019-10-19T00:30:16.3155255Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-object-safety.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-safety" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-safety/auxiliary" "-A" "unused"
2019-10-19T00:30:16.3156070Z ------------------------------------------
2019-10-19T00:30:16.3156126Z 
2019-10-19T00:30:16.3156345Z ------------------------------------------
2019-10-19T00:30:16.3156389Z stderr:
2019-10-19T00:30:16.3156389Z stderr:
2019-10-19T00:30:16.3156605Z ------------------------------------------
2019-10-19T00:30:16.3156664Z error[E0038]: the trait `Tr` cannot be made into an object
2019-10-19T00:30:16.3156908Z   --> /checkout/src/test/ui/traits/trait-object-safety.rs:15:12
2019-10-19T00:30:16.3156981Z    |
2019-10-19T00:30:16.3157022Z LL |     fn foo();
2019-10-19T00:30:16.3157257Z    |        --- associated function `foo` has no `self` parameter
2019-10-19T00:30:16.3157311Z ...
2019-10-19T00:30:16.3157357Z LL |     let _: &dyn Tr = &St; //~ ERROR E0038
2019-10-19T00:30:16.3157407Z    |            ^^^^^^^ the trait `Tr` cannot be made into an object
2019-10-19T00:30:16.3157498Z error[E0038]: the trait `Tr` cannot be made into an object
2019-10-19T00:30:16.3157735Z   --> /checkout/src/test/ui/traits/trait-object-safety.rs:15:22
2019-10-19T00:30:16.3157781Z    |
2019-10-19T00:30:16.3157831Z LL |     fn foo();
2019-10-19T00:30:16.3157831Z LL |     fn foo();
2019-10-19T00:30:16.3158061Z    |        --- associated function `foo` has no `self` parameter
2019-10-19T00:30:16.3158106Z ...
2019-10-19T00:30:16.3158149Z LL |     let _: &dyn Tr = &St; //~ ERROR E0038
2019-10-19T00:30:16.3158214Z    |                      ^^^ the trait `Tr` cannot be made into an object
2019-10-19T00:30:16.3158259Z    |
2019-10-19T00:30:16.3158475Z    = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<&dyn Tr>` for `&St`
2019-10-19T00:30:16.3158570Z error: aborting due to 2 previous errors
2019-10-19T00:30:16.3158597Z 
2019-10-19T00:30:16.3158870Z For more information about this error, try `rustc --explain E0038`.
2019-10-19T00:30:16.3159020Z 
---
2019-10-19T00:30:16.3159789Z 23 
2019-10-19T00:30:16.3159834Z 24 error[E0038]: the trait `bar` cannot be made into an object
2019-10-19T00:30:16.3160073Z +   --> $DIR/trait-test-2.rs:11:5
2019-10-19T00:30:16.3160133Z +    |
2019-10-19T00:30:16.3160376Z + LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }
2019-10-19T00:30:16.3160632Z +    |                ---                    ---- method `blah` has generic type parameters
2019-10-19T00:30:16.3160695Z +    |                |
2019-10-19T00:30:16.3160744Z +    |                method `dup` references the `Self` type in its parameters or return type
2019-10-19T00:30:16.3160791Z + ...
2019-10-19T00:30:16.3160846Z + LL |     (box 10 as Box<dyn bar>).dup();
2019-10-19T00:30:16.3160897Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `bar` cannot be made into an object
2019-10-19T00:30:16.3160985Z + error[E0038]: the trait `bar` cannot be made into an object
2019-10-19T00:30:16.3161218Z 25   --> $DIR/trait-test-2.rs:11:6
2019-10-19T00:30:16.3161261Z 26    |
2019-10-19T00:30:16.3161261Z 26    |
2019-10-19T00:30:16.3161584Z 27 LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }
2019-10-19T00:30:16.3162066Z 34    |
2019-10-19T00:30:16.3162066Z 34    |
2019-10-19T00:30:16.3162123Z 35    = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<std::boxed::Box<dyn bar>>` for `std::boxed::Box<{integer}>`
2019-10-19T00:30:16.3162520Z - error: aborting due to 4 previous errors
2019-10-19T00:30:16.3162569Z + error: aborting due to 5 previous errors
2019-10-19T00:30:16.3162621Z 38 
2019-10-19T00:30:16.3162665Z 39 Some errors have detailed explanations: E0038, E0107.
2019-10-19T00:30:16.3162665Z 39 Some errors have detailed explanations: E0038, E0107.
2019-10-19T00:30:16.3162911Z 40 For more information about an error, try `rustc --explain E0038`.
2019-10-19T00:30:16.3162945Z 
2019-10-19T00:30:16.3162981Z 
2019-10-19T00:30:16.3163025Z The actual stderr differed from the expected stderr.
2019-10-19T00:30:16.3163305Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-test-2/trait-test-2.stderr
2019-10-19T00:30:16.3163562Z To update references, rerun the tests and pass the `--bless` flag
2019-10-19T00:30:16.3163817Z To only update this specific test, also pass `--test-args traits/trait-test-2.rs`
2019-10-19T00:30:16.3163901Z error: 1 errors occurred comparing output.
2019-10-19T00:30:16.3163955Z status: exit code: 1
2019-10-19T00:30:16.3163955Z status: exit code: 1
2019-10-19T00:30:16.3164628Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-test-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-test-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-test-2/auxiliary" "-A" "unused"
2019-10-19T00:30:16.3164946Z ------------------------------------------
2019-10-19T00:30:16.3165093Z 
2019-10-19T00:30:16.3165726Z ------------------------------------------
2019-10-19T00:30:16.3165784Z stderr:
2019-10-19T00:30:16.3165784Z stderr:
2019-10-19T00:30:16.3166053Z ------------------------------------------
2019-10-19T00:30:16.3166280Z error[E0107]: wrong number of type arguments: expected 0, found 1
2019-10-19T00:30:16.3166551Z   --> /checkout/src/test/ui/traits/trait-test-2.rs:9:14
2019-10-19T00:30:16.3166602Z    |
2019-10-19T00:30:16.3166661Z LL |     10.dup::<i32>(); //~ ERROR wrong number of type arguments: expected 0, found 1
2019-10-19T00:30:16.3166713Z    |              ^^^ unexpected type argument
2019-10-19T00:30:16.3166789Z error[E0107]: wrong number of type arguments: expected 1, found 2
2019-10-19T00:30:16.3167035Z   --> /checkout/src/test/ui/traits/trait-test-2.rs:10:20
2019-10-19T00:30:16.3167080Z    |
2019-10-19T00:30:16.3167080Z    |
2019-10-19T00:30:16.3167129Z LL |     10.blah::<i32, i32>(); //~ ERROR wrong number of type arguments: expected 1, found 2
2019-10-19T00:30:16.3167193Z    |                    ^^^ unexpected type argument
2019-10-19T00:30:16.3167276Z error[E0038]: the trait `bar` cannot be made into an object
2019-10-19T00:30:16.3167530Z   --> /checkout/src/test/ui/traits/trait-test-2.rs:11:16
2019-10-19T00:30:16.3167575Z    |
2019-10-19T00:30:16.3167575Z    |
2019-10-19T00:30:16.3167801Z LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }
2019-10-19T00:30:16.3168056Z    |                ---                    ---- method `blah` has generic type parameters
2019-10-19T00:30:16.3168122Z    |                |
2019-10-19T00:30:16.3168172Z    |                method `dup` references the `Self` type in its parameters or return type
2019-10-19T00:30:16.3168218Z ...
2019-10-19T00:30:16.3168277Z LL |     (box 10 as Box<dyn bar>).dup();
2019-10-19T00:30:16.3168328Z    |                ^^^^^^^^^^^^ the trait `bar` cannot be made into an object
2019-10-19T00:30:16.3168418Z error[E0038]: the trait `bar` cannot be made into an object
2019-10-19T00:30:16.3168762Z   --> /checkout/src/test/ui/traits/trait-test-2.rs:11:5
2019-10-19T00:30:16.3168899Z    |
2019-10-19T00:30:16.3168899Z    |
2019-10-19T00:30:16.3169154Z LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }
2019-10-19T00:30:16.3169426Z    |                ---                    ---- method `blah` has generic type parameters
2019-10-19T00:30:16.3169473Z    |                |
2019-10-19T00:30:16.3169522Z    |                method `dup` references the `Self` type in its parameters or return type
2019-10-19T00:30:16.3169692Z ...
2019-10-19T00:30:16.3169731Z LL |     (box 10 as Box<dyn bar>).dup();
2019-10-19T00:30:16.3169777Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `bar` cannot be made into an object
2019-10-19T00:30:16.3169862Z error[E0038]: the trait `bar` cannot be made into an object
2019-10-19T00:30:16.3170079Z   --> /checkout/src/test/ui/traits/trait-test-2.rs:11:6
2019-10-19T00:30:16.3170121Z    |
2019-10-19T00:30:16.3170121Z    |
2019-10-19T00:30:16.3170344Z LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }
2019-10-19T00:30:16.3170587Z    |                ---                    ---- method `blah` has generic type parameters
2019-10-19T00:30:16.3170634Z    |                |
2019-10-19T00:30:16.3170703Z    |                method `dup` references the `Self` type in its parameters or return type
2019-10-19T00:30:16.3170745Z ...
2019-10-19T00:30:16.3170785Z LL |     (box 10 as Box<dyn bar>).dup();
2019-10-19T00:30:16.3170842Z    |      ^^^^^^ the trait `bar` cannot be made into an object
2019-10-19T00:30:16.3170882Z    |
2019-10-19T00:30:16.3170934Z    = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<std::boxed::Box<dyn bar>>` for `std::boxed::Box<{integer}>`
2019-10-19T00:30:16.3171024Z error: aborting due to 5 previous errors
2019-10-19T00:30:16.3171050Z 
2019-10-19T00:30:16.3171090Z Some errors have detailed explanations: E0038, E0107.
2019-10-19T00:30:16.3171331Z For more information about an error, try `rustc --explain E0038`.
---
2019-10-19T00:30:16.3172048Z 1 error[E0277]: the size for values of type `X` cannot be known at compilation time
2019-10-19T00:30:16.3172274Z -   --> $DIR/unsized3.rs:7:13
2019-10-19T00:30:16.3172464Z +   --> $DIR/unsized3.rs:7:5
2019-10-19T00:30:16.3172520Z 3    |
2019-10-19T00:30:16.3172558Z 4 LL |     f2::<X>(x);
2019-10-19T00:30:16.3172772Z -    |             ^ doesn't have a size known at compile-time
2019-10-19T00:30:16.3173039Z 6 ...
2019-10-19T00:30:16.3173039Z 6 ...
2019-10-19T00:30:16.3173077Z 7 LL | fn f2<X>(x: &X) {
2019-10-19T00:30:16.3173278Z 8    |    -- - required by this bound in `f2`
2019-10-19T00:30:16.3173320Z 
2019-10-19T00:30:16.3173362Z 12    = help: consider adding a `where X: std::marker::Sized` bound
2019-10-19T00:30:16.3173465Z 14 error[E0277]: the size for values of type `X` cannot be known at compilation time
2019-10-19T00:30:16.3173670Z -   --> $DIR/unsized3.rs:18:13
2019-10-19T00:30:16.3173858Z +   --> $DIR/unsized3.rs:18:5
2019-10-19T00:30:16.3173898Z 16    |
2019-10-19T00:30:16.3173898Z 16    |
2019-10-19T00:30:16.3173953Z 17 LL |     f4::<X>(x);
2019-10-19T00:30:16.3174168Z -    |             ^ doesn't have a size known at compile-time
2019-10-19T00:30:16.3174440Z 19 ...
2019-10-19T00:30:16.3174440Z 19 ...
2019-10-19T00:30:16.3174478Z 20 LL | fn f4<X: T>(x: &X) {
2019-10-19T00:30:16.3174680Z 21    |    -- - required by this bound in `f4`
2019-10-19T00:30:16.3174747Z 
2019-10-19T00:30:16.3174788Z The actual stderr differed from the expected stderr.
2019-10-19T00:30:16.3175157Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized3/unsized3.stderr
2019-10-19T00:30:16.3175157Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized3/unsized3.stderr
2019-10-19T00:30:16.3180970Z To update references, rerun the tests and pass the `--bless` flag
2019-10-19T00:30:16.3181315Z To only update this specific test, also pass `--test-args unsized3.rs`
2019-10-19T00:30:16.3181410Z error: 1 errors occurred comparing output.
2019-10-19T00:30:16.3181469Z status: exit code: 1
2019-10-19T00:30:16.3181469Z status: exit code: 1
2019-10-19T00:30:16.3182166Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized3/auxiliary" "-A" "unused"
2019-10-19T00:30:16.3182498Z ------------------------------------------
2019-10-19T00:30:16.3182553Z 
2019-10-19T00:30:16.3182887Z ------------------------------------------
2019-10-19T00:30:16.3182930Z stderr:
2019-10-19T00:30:16.3182930Z stderr:
2019-10-19T00:30:16.3183151Z ------------------------------------------
2019-10-19T00:30:16.3183210Z error[E0277]: the size for values of type `X` cannot be known at compilation time
2019-10-19T00:30:16.3183426Z   --> /checkout/src/test/ui/unsized3.rs:7:5
2019-10-19T00:30:16.3183494Z    |
2019-10-19T00:30:16.3183533Z LL |     f2::<X>(x);
2019-10-19T00:30:16.3183817Z ...
2019-10-19T00:30:16.3183817Z ...
2019-10-19T00:30:16.3183857Z LL | fn f2<X>(x: &X) {
2019-10-19T00:30:16.3184062Z    |    -- - required by this bound in `f2`
2019-10-19T00:30:16.3184169Z    = help: the trait `std::marker::Sized` is not implemented for `X`
2019-10-19T00:30:16.3184472Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-10-19T00:30:16.3184472Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-10-19T00:30:16.3184538Z    = help: consider adding a `where X: std::marker::Sized` bound
2019-10-19T00:30:16.3184638Z error[E0277]: the size for values of type `X` cannot be known at compilation time
2019-10-19T00:30:16.3185105Z   --> /checkout/src/test/ui/unsized3.rs:18:5
2019-10-19T00:30:16.3185172Z    |
2019-10-19T00:30:16.3185172Z    |
2019-10-19T00:30:16.3185212Z LL |     f4::<X>(x);
2019-10-19T00:30:16.3185836Z ...
2019-10-19T00:30:16.3185836Z ...
2019-10-19T00:30:16.3185877Z LL | fn f4<X: T>(x: &X) {
2019-10-19T00:30:16.3186096Z    |    -- - required by this bound in `f4`
2019-10-19T00:30:16.3186205Z    = help: the trait `std::marker::Sized` is not implemented for `X`
2019-10-19T00:30:16.3186515Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-10-19T00:30:16.3186515Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-10-19T00:30:16.3186575Z    = help: consider adding a `where X: std::marker::Sized` bound
2019-10-19T00:30:16.3186686Z error[E0277]: the size for values of type `X` cannot be known at compilation time
2019-10-19T00:30:16.3186908Z   --> /checkout/src/test/ui/unsized3.rs:33:8
2019-10-19T00:30:16.3186981Z    |
2019-10-19T00:30:16.3186981Z    |
2019-10-19T00:30:16.3187021Z LL | fn f5<Y>(x: &Y) {}
2019-10-19T00:30:16.3187237Z    |    -- - required by this bound in `f5`
2019-10-19T00:30:16.3187341Z LL |     f5(x1);
2019-10-19T00:30:16.3187566Z    |        ^^ doesn't have a size known at compile-time
2019-10-19T00:30:16.3187612Z    |
2019-10-19T00:30:16.3187612Z    |
2019-10-19T00:30:16.3187680Z    = help: within `S<X>`, the trait `std::marker::Sized` is not implemented for `X`
2019-10-19T00:30:16.3187988Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-10-19T00:30:16.3188046Z    = help: consider adding a `where X: std::marker::Sized` bound
2019-10-19T00:30:16.3188115Z    = note: required because it appears within the type `S<X>`
2019-10-19T00:30:16.3188327Z error[E0277]: the size for values of type `X` cannot be known at compilation time
2019-10-19T00:30:16.3188728Z   --> /checkout/src/test/ui/unsized3.rs:40:8
2019-10-19T00:30:16.3188773Z    |
2019-10-19T00:30:16.3188773Z    |
2019-10-19T00:30:16.3188812Z LL |     f5(&(*x1, 34));
2019-10-19T00:30:16.3189099Z    |
2019-10-19T00:30:16.3189099Z    |
2019-10-19T00:30:16.3189145Z    = help: within `S<X>`, the trait `std::marker::Sized` is not implemented for `X`
2019-10-19T00:30:16.3189442Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-10-19T00:30:16.3189519Z    = help: consider adding a `where X: std::marker::Sized` bound
2019-10-19T00:30:16.3189567Z    = note: required because it appears within the type `S<X>`
2019-10-19T00:30:16.3189663Z 
2019-10-19T00:30:16.3189717Z error[E0277]: the size for values of type `X` cannot be known at compilation time
2019-10-19T00:30:16.3190055Z   --> /checkout/src/test/ui/unsized3.rs:45:9
2019-10-19T00:30:16.3190126Z    |
2019-10-19T00:30:16.3190126Z    |
2019-10-19T00:30:16.3190167Z LL |     f5(&(32, *x1));
2019-10-19T00:30:16.3190462Z    |
2019-10-19T00:30:16.3190462Z    |
2019-10-19T00:30:16.3190512Z    = help: within `({integer}, S<X>)`, the trait `std::marker::Sized` is not implemented for `X`
2019-10-19T00:30:16.3190819Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-10-19T00:30:16.3190895Z    = help: consider adding a `where X: std::marker::Sized` bound
2019-10-19T00:30:16.3190946Z    = note: required because it appears within the type `S<X>`
2019-10-19T00:30:16.3190995Z    = note: required because it appears within the type `({integer}, S<X>)`
2019-10-19T00:30:16.3191092Z 
2019-10-19T00:30:16.3191142Z error[E0277]: the size for values of type `X` cannot be known at compilation time
2019-10-19T00:30:16.3191516Z   --> /checkout/src/test/ui/unsized3.rs:45:8
2019-10-19T00:30:16.3191584Z    |
2019-10-19T00:30:16.3191584Z    |
2019-10-19T00:30:16.3191628Z LL | fn f5<Y>(x: &Y) {}
2019-10-19T00:30:16.3191864Z    |    -- - required by this bound in `f5`
2019-10-19T00:30:16.3191930Z ...
2019-10-19T00:30:16.3191974Z LL |     f5(&(32, *x1));
2019-10-19T00:30:16.3192274Z    |
2019-10-19T00:30:16.3192274Z    |
2019-10-19T00:30:16.3192347Z    = help: within `({integer}, S<X>)`, the trait `std::marker::Sized` is not implemented for `X`
2019-10-19T00:30:16.3192678Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-10-19T00:30:16.3192740Z    = help: consider adding a `where X: std::marker::Sized` bound
2019-10-19T00:30:16.3192820Z    = note: required because it appears within the type `S<X>`
2019-10-19T00:30:16.3192873Z    = note: required because it appears within the type `({integer}, S<X>)`
2019-10-19T00:30:16.3192978Z error: aborting due to 6 previous errors
2019-10-19T00:30:16.3193008Z 
2019-10-19T00:30:16.3193262Z For more information about this error, try `rustc --explain E0277`.
2019-10-19T00:30:16.3193297Z 
2019-10-19T00:30:16.3193297Z 
2019-10-19T00:30:16.3193540Z ------------------------------------------
2019-10-19T00:30:16.3193572Z 
2019-10-19T00:30:16.3193598Z 
2019-10-19T00:30:16.3193852Z ---- [ui] ui/where-clauses/where-clauses-method-unsatisfied.rs stdout ----
2019-10-19T00:30:16.3193925Z diff of stderr:
2019-10-19T00:30:16.3193954Z 
2019-10-19T00:30:16.3194002Z 1 error[E0277]: the trait bound `Bar: std::cmp::Eq` is not satisfied
2019-10-19T00:30:16.3194520Z +   --> $DIR/where-clauses-method-unsatisfied.rs:18:7
2019-10-19T00:30:16.3194567Z 3    |
2019-10-19T00:30:16.3194567Z 3    |
2019-10-19T00:30:16.3194696Z 4 LL |     x.equals(&x);
2019-10-19T00:30:16.3195013Z -    |              ^^ the trait `std::cmp::Eq` is not implemented for `Bar`
2019-10-19T00:30:16.3195142Z +    |       ^^^^^^ the trait `std::cmp::Eq` is not implemented for `Bar`
2019-10-19T00:30:16.3195252Z 7 error: aborting due to previous error
2019-10-19T00:30:16.3195546Z 8 
2019-10-19T00:30:16.3195578Z 
2019-10-19T00:30:16.3195604Z 
2019-10-19T00:30:16.3195604Z 
2019-10-19T00:30:16.3195671Z The actual stderr differed from the expected stderr.
2019-10-19T00:30:16.3196083Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/where-clauses/where-clauses-method-unsatisfied/where-clauses-method-unsatisfied.stderr
2019-10-19T00:30:16.3196334Z To update references, rerun the tests and pass the `--bless` flag
2019-10-19T00:30:16.3196629Z To only update this specific test, also pass `--test-args where-clauses/where-clauses-method-unsatisfied.rs`
2019-10-19T00:30:16.3196721Z error: 1 errors occurred comparing output.
2019-10-19T00:30:16.3196766Z status: exit code: 1
2019-10-19T00:30:16.3196766Z status: exit code: 1
2019-10-19T00:30:16.3197574Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/where-clauses/where-clauses-method-unsatisfied.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/where-clauses/where-clauses-method-unsatisfied" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/where-clauses/where-clauses-method-unsatisfied/auxiliary" "-A" "unused"
2019-10-19T00:30:16.3197903Z ------------------------------------------
2019-10-19T00:30:16.3197937Z 
2019-10-19T00:30:16.3198149Z ------------------------------------------
2019-10-19T00:30:16.3198211Z stderr:
2019-10-19T00:30:16.3198211Z stderr:
2019-10-19T00:30:16.3198429Z ------------------------------------------
2019-10-19T00:30:16.3198480Z error[E0277]: the trait bound `Bar: std::cmp::Eq` is not satisfied
2019-10-19T00:30:16.3199042Z   --> /checkout/src/test/ui/where-clauses/where-clauses-method-unsatisfied.rs:18:7
2019-10-19T00:30:16.3199091Z    |
2019-10-19T00:30:16.3199129Z LL |     x.equals(&x);
2019-10-19T00:30:16.3199194Z    |       ^^^^^^ the trait `std::cmp::Eq` is not implemented for `Bar`
2019-10-19T00:30:16.3199263Z error: aborting due to previous error
2019-10-19T00:30:16.3199289Z 
2019-10-19T00:30:16.3199529Z For more information about this error, try `rustc --explain E0277`.
2019-10-19T00:30:16.3199560Z 
---
2019-10-19T00:30:16.3203072Z 
2019-10-19T00:30:16.3203396Z 
2019-10-19T00:30:16.3204110Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-19T00:30:16.3204461Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-19T00:30:16.3206769Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-19T00:30:16.3207616Z 
2019-10-19T00:30:16.3207829Z 
2019-10-19T00:30:16.3220198Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-19T00:30:16.3220294Z Build completed unsuccessfully in 1:09:14
2019-10-19T00:30:16.3220294Z Build completed unsuccessfully in 1:09:14
2019-10-19T00:30:16.3226191Z == clock drift check ==
2019-10-19T00:30:16.3252431Z   local time: Sat Oct 19 00:30:16 UTC 2019
2019-10-19T00:30:16.6043613Z   network time: Sat, 19 Oct 2019 00:30:16 GMT
2019-10-19T00:30:16.6043785Z == end clock drift check ==
2019-10-19T00:30:17.5786686Z 
2019-10-19T00:30:17.5927963Z ##[error]Bash exited with code '1'.
2019-10-19T00:30:17.5985982Z ##[section]Starting: Checkout
2019-10-19T00:30:17.5988173Z ==============================================================================
2019-10-19T00:30:17.5988255Z Task         : Get sources
2019-10-19T00:30:17.5988307Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
