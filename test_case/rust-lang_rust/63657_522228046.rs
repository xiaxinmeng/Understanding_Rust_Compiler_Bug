plain
2019-08-17T09:57:57.6330397Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-17T09:57:57.6549485Z ##[command]git config gc.auto 0
2019-08-17T09:57:57.6616317Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-17T09:57:57.6681840Z ##[command]git config --get-all http.proxy
2019-08-17T09:57:57.6822205Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63657/merge:refs/remotes/pull/63657/merge
---
2019-08-17T09:58:32.9071320Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-17T09:58:32.9071608Z 
2019-08-17T09:58:32.9072112Z   git checkout -b <new-branch-name>
2019-08-17T09:58:32.9072414Z 
2019-08-17T09:58:32.9072665Z HEAD is now at 5254ae491 Merge a9efa738ab4e9517a86314f5c0787efef7d8e2a9 into e910be8d7c7c3ae00a2839b310cc4062d5de8163
2019-08-17T09:58:32.9246784Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-17T09:58:32.9249827Z ==============================================================================
2019-08-17T09:58:32.9249897Z Task         : Bash
2019-08-17T09:58:32.9249947Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-17T11:00:29.7381263Z ........FFF......................................................................................... 1500/8924
2019-08-17T11:00:35.2839181Z .................................................................................................... 1600/8924
2019-08-17T11:00:48.2103762Z ................................i...............i................................................... 1700/8924
2019-08-17T11:00:55.7316171Z .................................................................................................... 1800/8924
2019-08-17T11:01:10.0064080Z .......................iiiii........................................................................ 1900/8924
2019-08-17T11:01:20.6989316Z .................................................................................................... 2100/8924
2019-08-17T11:01:23.3960464Z .................................................................................................... 2200/8924
2019-08-17T11:01:28.2977224Z .................................................................................................... 2300/8924
2019-08-17T11:01:35.2263982Z .................................................................................................... 2400/8924
---
2019-08-17T11:04:27.9441748Z .................................................................................................... 4600/8924
2019-08-17T11:04:34.9817799Z .....i...............i.............................................................................. 4700/8924
2019-08-17T11:04:46.3712826Z .................................................................................................... 4800/8924
2019-08-17T11:04:52.3168528Z .................................................................................................... 4900/8924
2019-08-17T11:05:04.6160922Z ......................................................................................ii.ii......... 5000/8924
2019-08-17T11:05:14.0931908Z .................................................................................................... 5200/8924
2019-08-17T11:05:24.4733731Z .................................................................................................... 5300/8924
2019-08-17T11:05:30.8498966Z ..........................................i......................................................... 5400/8924
2019-08-17T11:05:37.3946146Z .................................................................................................... 5500/8924
2019-08-17T11:05:37.3946146Z .................................................................................................... 5500/8924
2019-08-17T11:05:48.3933183Z .................................................................................................... 5600/8924
2019-08-17T11:06:01.5106403Z ...................................ii...i..ii...........i........................................... 5700/8924
2019-08-17T11:06:18.7066296Z .................................................................................................... 5900/8924
2019-08-17T11:06:23.6776398Z .................................................................................................... 6000/8924
2019-08-17T11:06:23.6776398Z .................................................................................................... 6000/8924
2019-08-17T11:06:37.4164786Z ....................................i..ii........................................................... 6100/8924
2019-08-17T11:06:58.0971804Z ..............................................................................i..................... 6300/8924
2019-08-17T11:07:00.3803067Z .................................................................................................... 6400/8924
2019-08-17T11:07:02.6511136Z ..................................................i................................................. 6500/8924
2019-08-17T11:07:05.9809835Z .................................................................................................... 6600/8924
---
2019-08-17T11:11:01.3511081Z ---- [ui] ui/consts/const-eval/ub-ref.rs stdout ----
2019-08-17T11:11:01.3511327Z diff of stderr:
2019-08-17T11:11:01.3511466Z 
2019-08-17T11:11:01.3511646Z 38    |
2019-08-17T11:11:01.3512285Z 39    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-17T11:11:01.3512849Z 40 
2019-08-17T11:11:01.3513378Z + warning: the type `&'static u16` does not permit zero-initialization
2019-08-17T11:11:01.3513811Z +   --> $DIR/ub-ref.rs:11:29
2019-08-17T11:11:01.3514015Z +    |
2019-08-17T11:11:01.3514173Z + LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
2019-08-17T11:11:01.3514505Z +    |                             |
2019-08-17T11:11:01.3514686Z +    |                             this code causes undefined behavior when executed
2019-08-17T11:11:01.3514867Z +    |                             help: use `MaybeUninit<T>` instead
2019-08-17T11:11:01.3515042Z +    |
---
2019-08-17T11:11:01.3516722Z 43 For more information about this error, try `rustc --explain E0080`.
2019-08-17T11:11:01.3516904Z 
2019-08-17T11:11:01.3517043Z 
2019-08-17T11:11:01.3517224Z The actual stderr differed from the expected stderr.
2019-08-17T11:11:01.3517728Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref/ub-ref.stderr
2019-08-17T11:11:01.3518227Z To update references, rerun the tests and pass the `--bless` flag
2019-08-17T11:11:01.3518743Z To only update this specific test, also pass `--test-args consts/const-eval/ub-ref.rs`
2019-08-17T11:11:01.3519101Z error: 1 errors occurred comparing output.
2019-08-17T11:11:01.3519258Z status: exit code: 1
2019-08-17T11:11:01.3519258Z status: exit code: 1
2019-08-17T11:11:01.3520426Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref/auxiliary" "-A" "unused"
2019-08-17T11:11:01.3521201Z ------------------------------------------
2019-08-17T11:11:01.3521427Z 
2019-08-17T11:11:01.3521852Z ------------------------------------------
2019-08-17T11:11:01.3522066Z stderr:
2019-08-17T11:11:01.3522066Z stderr:
2019-08-17T11:11:01.3523058Z ------------------------------------------
2019-08-17T11:11:01.3523340Z error[E0080]: it is undefined behavior to use this value
2019-08-17T11:11:01.3523850Z   --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:7:1
2019-08-17T11:11:01.3524063Z    |
2019-08-17T11:11:01.3524234Z LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
2019-08-17T11:11:01.3524424Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered unaligned reference (required 2 byte alignment but found 1)
2019-08-17T11:11:01.3524586Z    |
2019-08-17T11:11:01.3525194Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-17T11:11:01.3525772Z error[E0080]: it is undefined behavior to use this value
2019-08-17T11:11:01.3526258Z   --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:11:1
2019-08-17T11:11:01.3526461Z    |
2019-08-17T11:11:01.3526461Z    |
2019-08-17T11:11:01.3526619Z LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
2019-08-17T11:11:01.3526820Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
2019-08-17T11:11:01.3526980Z    |
2019-08-17T11:11:01.3527580Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-17T11:11:01.3527924Z error[E0080]: it is undefined behavior to use this value
2019-08-17T11:11:01.3528374Z   --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:14:1
2019-08-17T11:11:01.3528566Z    |
2019-08-17T11:11:01.3528566Z    |
2019-08-17T11:11:01.3528740Z LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
2019-08-17T11:11:01.3529274Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
2019-08-17T11:11:01.3529490Z    |
2019-08-17T11:11:01.3530078Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-17T11:11:01.3530445Z error[E0080]: it is undefined behavior to use this value
2019-08-17T11:11:01.3530854Z   --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:17:1
2019-08-17T11:11:01.3531063Z    |
2019-08-17T11:11:01.3531063Z    |
2019-08-17T11:11:01.3531223Z LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
2019-08-17T11:11:01.3531866Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .<deref>, but expected plain (non-pointer) bytes
2019-08-17T11:11:01.3532306Z    |
2019-08-17T11:11:01.3535518Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-17T11:11:01.3536181Z error[E0080]: it is undefined behavior to use this value
2019-08-17T11:11:01.3538423Z   --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:20:1
2019-08-17T11:11:01.3538518Z    |
2019-08-17T11:11:01.3538518Z    |
2019-08-17T11:11:01.3538812Z LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
2019-08-17T11:11:01.3538883Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling reference (created from integer)
2019-08-17T11:11:01.3538959Z    |
2019-08-17T11:11:01.3539385Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-17T11:11:01.3539471Z 
2019-08-17T11:11:01.3539742Z warning: the type `&'static u16` does not permit zero-initialization
2019-08-17T11:11:01.3540079Z    |
2019-08-17T11:11:01.3540079Z    |
2019-08-17T11:11:01.3540139Z LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
2019-08-17T11:11:01.3540261Z    |                             |
2019-08-17T11:11:01.3540319Z    |                             this code causes undefined behavior when executed
2019-08-17T11:11:01.3540382Z    |                             help: use `MaybeUninit<T>` instead
2019-08-17T11:11:01.3540454Z    |
---
2019-08-17T11:11:01.3542237Z ---- [ui] ui/consts/const-eval/ub-nonnull.rs stdout ----
2019-08-17T11:11:01.3542308Z diff of stderr:
2019-08-17T11:11:01.3542341Z 
2019-08-17T11:11:01.3542387Z 64    |
2019-08-17T11:11:01.3543183Z 65    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-17T11:11:01.3543288Z 66 
2019-08-17T11:11:01.3543578Z + warning: the type `std::ptr::NonNull<u8>` does not permit zero-initialization
2019-08-17T11:11:01.3543831Z +   --> $DIR/ub-nonnull.rs:11:40
2019-08-17T11:11:01.3543894Z +    |
2019-08-17T11:11:01.3543946Z + LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
2019-08-17T11:11:01.3544076Z +    |                                        |
2019-08-17T11:11:01.3544145Z +    |                                        this code causes undefined behavior when executed
2019-08-17T11:11:01.3544207Z +    |                                        help: use `MaybeUninit<T>` instead
2019-08-17T11:11:01.3544275Z +    |
2019-08-17T11:11:01.3544275Z +    |
2019-08-17T11:11:01.3544329Z +    = note: `#[warn(invalid_value)]` on by default
2019-08-17T11:11:01.3544608Z +    = note: std::ptr::NonNull<u8> must be non-null
2019-08-17T11:11:01.3544678Z + 
2019-08-17T11:11:01.3544974Z + warning: the type `std::num::NonZeroU8` does not permit zero-initialization
2019-08-17T11:11:01.3545228Z +   --> $DIR/ub-nonnull.rs:22:37
2019-08-17T11:11:01.3545295Z +    |
2019-08-17T11:11:01.3545349Z + LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
2019-08-17T11:11:01.3545490Z +    |                                     |
2019-08-17T11:11:01.3545552Z +    |                                     this code causes undefined behavior when executed
2019-08-17T11:11:01.3545742Z +    |                                     help: use `MaybeUninit<T>` instead
2019-08-17T11:11:01.3545821Z +    |
2019-08-17T11:11:01.3545821Z +    |
2019-08-17T11:11:01.3546132Z +    = note: std::num::NonZeroU8 must be non-null
2019-08-17T11:11:01.3546184Z + 
2019-08-17T11:11:01.3546482Z + warning: the type `std::num::NonZeroUsize` does not permit zero-initialization
2019-08-17T11:11:01.3546760Z +   --> $DIR/ub-nonnull.rs:24:43
2019-08-17T11:11:01.3546812Z +    |
2019-08-17T11:11:01.3546866Z + LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
2019-08-17T11:11:01.3547010Z +    |                                           |
2019-08-17T11:11:01.3547072Z +    |                                           this code causes undefined behavior when executed
2019-08-17T11:11:01.3547153Z +    |                                           help: use `MaybeUninit<T>` instead
2019-08-17T11:11:01.3550489Z +    |
---
2019-08-17T11:11:01.3551428Z 69 For more information about this error, try `rustc --explain E0080`.
2019-08-17T11:11:01.3551470Z 
2019-08-17T11:11:01.3551498Z 
2019-08-17T11:11:01.3551546Z The actual stderr differed from the expected stderr.
2019-08-17T11:11:01.3551898Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/ub-nonnull.stderr
2019-08-17T11:11:01.3552171Z To update references, rerun the tests and pass the `--bless` flag
2019-08-17T11:11:01.3553017Z To only update this specific test, also pass `--test-args consts/const-eval/ub-nonnull.rs`
2019-08-17T11:11:01.3553136Z error: 1 errors occurred comparing output.
2019-08-17T11:11:01.3553186Z status: exit code: 1
2019-08-17T11:11:01.3553186Z status: exit code: 1
2019-08-17T11:11:01.3554012Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-nonnull.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/auxiliary" "-A" "unused"
2019-08-17T11:11:01.3554378Z ------------------------------------------
2019-08-17T11:11:01.3554440Z 
2019-08-17T11:11:01.3554685Z ------------------------------------------
2019-08-17T11:11:01.3554733Z stderr:
2019-08-17T11:11:01.3554733Z stderr:
2019-08-17T11:11:01.3554981Z ------------------------------------------
2019-08-17T11:11:01.3555038Z error[E0080]: it is undefined behavior to use this value
2019-08-17T11:11:01.3555308Z   --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:11:1
2019-08-17T11:11:01.3555389Z    |
2019-08-17T11:11:01.3555442Z LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
2019-08-17T11:11:01.3555510Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
2019-08-17T11:11:01.3555584Z    |
2019-08-17T11:11:01.3556013Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-17T11:11:01.3556144Z error: any use of this value will cause an error
2019-08-17T11:11:01.3556416Z   --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:18:29
2019-08-17T11:11:01.3556472Z    |
2019-08-17T11:11:01.3556472Z    |
2019-08-17T11:11:01.3556541Z LL | / const OUT_OF_BOUNDS_PTR: NonNull<u8> = { unsafe {
2019-08-17T11:11:01.3556727Z LL | |     let ptr: &[u8; 256] = mem::transmute(&0u8); // &0 gets promoted so it does not dangle
2019-08-17T11:11:01.3557076Z LL | |     // Use address-of-element for pointer arithmetic. This could wrap around to NULL!
2019-08-17T11:11:01.3557164Z LL | |     let out_of_bounds_ptr = &ptr[255]; //~ ERROR any use of this value will cause an error
2019-08-17T11:11:01.3557537Z    | |                             ^^^^^^^^^ Memory access failed: pointer must be in-bounds at offset 256, but is outside bounds of allocation 6 which has size 1
2019-08-17T11:11:01.3557620Z LL | |     mem::transmute(out_of_bounds_ptr)
2019-08-17T11:11:01.3557673Z LL | | } };
2019-08-17T11:11:01.3560044Z    |
2019-08-17T11:11:01.3560091Z note: lint level defined here
2019-08-17T11:11:01.3560378Z   --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:14:8
2019-08-17T11:11:01.3560430Z    |
2019-08-17T11:11:01.3560430Z    |
2019-08-17T11:11:01.3560500Z LL | #[deny(const_err)] // this triggers a `const_err` so validation does not even happen
2019-08-17T11:11:01.3560587Z 
2019-08-17T11:11:01.3560661Z error[E0080]: it is undefined behavior to use this value
2019-08-17T11:11:01.3560931Z   --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:22:1
2019-08-17T11:11:01.3560982Z    |
2019-08-17T11:11:01.3560982Z    |
2019-08-17T11:11:01.3561033Z LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
2019-08-17T11:11:01.3561118Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
2019-08-17T11:11:01.3561174Z    |
2019-08-17T11:11:01.3561619Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-17T11:11:01.3562934Z error[E0080]: it is undefined behavior to use this value
2019-08-17T11:11:01.3563405Z   --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:24:1
2019-08-17T11:11:01.3563466Z    |
2019-08-17T11:11:01.3563466Z    |
2019-08-17T11:11:01.3563536Z LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
2019-08-17T11:11:01.3563621Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
2019-08-17T11:11:01.3563679Z    |
2019-08-17T11:11:01.3564112Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-17T11:11:01.3564245Z error[E0080]: it is undefined behavior to use this value
2019-08-17T11:11:01.3564517Z   --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:31:1
2019-08-17T11:11:01.3564589Z    |
2019-08-17T11:11:01.3564589Z    |
2019-08-17T11:11:01.3564640Z LL | const UNINIT: NonZeroU8 = unsafe { Transmute { uninit: () }.out };
2019-08-17T11:11:01.3564719Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected something greater or equal to 1
2019-08-17T11:11:01.3564799Z    |
2019-08-17T11:11:01.3565229Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-17T11:11:01.3565350Z error[E0080]: it is undefined behavior to use this value
2019-08-17T11:11:01.3565616Z   --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:39:1
2019-08-17T11:11:01.3565696Z    |
2019-08-17T11:11:01.3565696Z    |
2019-08-17T11:11:01.3565749Z LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
2019-08-17T11:11:01.3565818Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 42, but expected something in the range 10..=30
2019-08-17T11:11:01.3565892Z    |
2019-08-17T11:11:01.3566458Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-17T11:11:01.3566596Z error[E0080]: it is undefined behavior to use this value
2019-08-17T11:11:01.3566907Z   --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:45:1
2019-08-17T11:11:01.3566962Z    |
2019-08-17T11:11:01.3566962Z    |
2019-08-17T11:11:01.3567030Z LL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };
2019-08-17T11:11:01.3567102Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 20, but expected something less or equal to 10, or greater or equal to 30
2019-08-17T11:11:01.3567191Z    |
2019-08-17T11:11:01.3567623Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-17T11:11:01.3567686Z 
2019-08-17T11:11:01.3567981Z warning: the type `std::ptr::NonNull<u8>` does not permit zero-initialization
2019-08-17T11:11:01.3568311Z    |
2019-08-17T11:11:01.3568311Z    |
2019-08-17T11:11:01.3568379Z LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
2019-08-17T11:11:01.3568488Z    |                                        |
2019-08-17T11:11:01.3568565Z    |                                        this code causes undefined behavior when executed
2019-08-17T11:11:01.3568732Z    |                                        help: use `MaybeUninit<T>` instead
2019-08-17T11:11:01.3568785Z    |
2019-08-17T11:11:01.3568785Z    |
2019-08-17T11:11:01.3568859Z    = note: `#[warn(invalid_value)]` on by default
2019-08-17T11:11:01.3569172Z    = note: std::ptr::NonNull<u8> must be non-null
2019-08-17T11:11:01.3569211Z 
2019-08-17T11:11:01.3569518Z warning: the type `std::num::NonZeroU8` does not permit zero-initialization
2019-08-17T11:11:01.3569885Z    |
2019-08-17T11:11:01.3569885Z    |
2019-08-17T11:11:01.3569938Z LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
2019-08-17T11:11:01.3570066Z    |                                     |
2019-08-17T11:11:01.3570127Z    |                                     this code causes undefined behavior when executed
2019-08-17T11:11:01.3570206Z    |                                     help: use `MaybeUninit<T>` instead
2019-08-17T11:11:01.3570267Z    |
2019-08-17T11:11:01.3570267Z    |
2019-08-17T11:11:01.3570533Z    = note: std::num::NonZeroU8 must be non-null
2019-08-17T11:11:01.3570571Z 
2019-08-17T11:11:01.3570882Z warning: the type `std::num::NonZeroUsize` does not permit zero-initialization
2019-08-17T11:11:01.3571237Z    |
2019-08-17T11:11:01.3571237Z    |
2019-08-17T11:11:01.3571307Z LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
2019-08-17T11:11:01.3571423Z    |                                           |
2019-08-17T11:11:01.3571501Z    |                                           this code causes undefined behavior when executed
2019-08-17T11:11:01.3571566Z    |                                           help: use `MaybeUninit<T>` instead
2019-08-17T11:11:01.3571618Z    |
---
2019-08-17T11:11:01.3573520Z ---- [ui] ui/consts/const-eval/ub-upvars.rs stdout ----
2019-08-17T11:11:01.3573574Z diff of stderr:
2019-08-17T11:11:01.3573605Z 
2019-08-17T11:11:01.3573667Z 10    |
2019-08-17T11:11:01.3574101Z 11    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-17T11:11:01.3574172Z 12 
2019-08-17T11:11:01.3574465Z + warning: the type `&'static u16` does not permit zero-initialization
2019-08-17T11:11:01.3574709Z +   --> $DIR/ub-upvars.rs:7:42
2019-08-17T11:11:01.3574759Z +    |
2019-08-17T11:11:01.3575045Z + LL |     let bad_ref: &'static u16 = unsafe { mem::transmute(0usize) };
2019-08-17T11:11:01.3575161Z +    |                                          |
2019-08-17T11:11:01.3575247Z +    |                                          this code causes undefined behavior when executed
2019-08-17T11:11:01.3575309Z +    |                                          help: use `MaybeUninit<T>` instead
2019-08-17T11:11:01.3575359Z +    |
---
2019-08-17T11:11:01.3576093Z 15 For more information about this error, try `rustc --explain E0080`.
2019-08-17T11:11:01.3576231Z 
2019-08-17T11:11:01.3576280Z 
2019-08-17T11:11:01.3576332Z The actual stderr differed from the expected stderr.
2019-08-17T11:11:01.3576692Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars/ub-upvars.stderr
2019-08-17T11:11:01.3576962Z To update references, rerun the tests and pass the `--bless` flag
2019-08-17T11:11:01.3577304Z To only update this specific test, also pass `--test-args consts/const-eval/ub-upvars.rs`
2019-08-17T11:11:01.3577395Z error: 1 errors occurred comparing output.
2019-08-17T11:11:01.3577463Z status: exit code: 1
2019-08-17T11:11:01.3577463Z status: exit code: 1
2019-08-17T11:11:01.3578286Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-upvars.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars/auxiliary" "-A" "unused"
2019-08-17T11:11:01.3578677Z ------------------------------------------
2019-08-17T11:11:01.3578716Z 
2019-08-17T11:11:01.3579000Z ------------------------------------------
2019-08-17T11:11:01.3579053Z stderr:
2019-08-17T11:11:01.3579053Z stderr:
2019-08-17T11:11:01.3579297Z ------------------------------------------
2019-08-17T11:11:01.3579354Z error[E0080]: it is undefined behavior to use this value
2019-08-17T11:11:01.3579657Z   --> /checkout/src/test/ui/consts/const-eval/ub-upvars.rs:6:1
2019-08-17T11:11:01.3579714Z    |
2019-08-17T11:11:01.3579772Z LL | / const BAD_UPVAR: &dyn FnOnce() = &{ //~ ERROR it is undefined behavior to use this value
2019-08-17T11:11:01.3580094Z LL | |     let bad_ref: &'static u16 = unsafe { mem::transmute(0usize) };
2019-08-17T11:11:01.3580165Z LL | |     let another_var = 13;
2019-08-17T11:11:01.3580236Z LL | |     move || { let _ = bad_ref; let _ = another_var; }
2019-08-17T11:11:01.3580288Z LL | | };
2019-08-17T11:11:01.3580656Z    | |__^ type validation failed: encountered 0 at .<deref>.<dyn-downcast>.<closure-var(bad_ref)>, but expected something greater or equal to 1
2019-08-17T11:11:01.3580736Z    |
2019-08-17T11:11:01.3581276Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-17T11:11:01.3581343Z 
2019-08-17T11:11:01.3581682Z warning: the type `&'static u16` does not permit zero-initialization
2019-08-17T11:11:01.3582033Z    |
2019-08-17T11:11:01.3582033Z    |
2019-08-17T11:11:01.3582338Z LL |     let bad_ref: &'static u16 = unsafe { mem::transmute(0usize) };
2019-08-17T11:11:01.3582715Z    |                                          |
2019-08-17T11:11:01.3582798Z    |                                          this code causes undefined behavior when executed
2019-08-17T11:11:01.3582863Z    |                                          help: use `MaybeUninit<T>` instead
2019-08-17T11:11:01.3582928Z    |
---
2019-08-17T11:11:01.3585945Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-17T11:11:01.3586029Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-17T11:11:02.0555645Z 
2019-08-17T11:11:02.0556406Z 
2019-08-17T11:11:02.0558937Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-17T11:11:02.0559609Z 
2019-08-17T11:11:02.0559784Z 
2019-08-17T11:11:02.0560091Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-17T11:11:02.0560275Z Build completed unsuccessfully in 1:05:52
2019-08-17T11:11:02.0560275Z Build completed unsuccessfully in 1:05:52
2019-08-17T11:11:02.0560465Z == clock drift check ==
2019-08-17T11:11:02.0560627Z   local time: Sat Aug 17 11:11:01 UTC 2019
2019-08-17T11:11:02.0561144Z   network time: Sat, 17 Aug 2019 11:11:01 GMT
2019-08-17T11:11:02.0561397Z == end clock drift check ==
2019-08-17T11:11:02.9109804Z ##[error]Bash exited with code '1'.
2019-08-17T11:11:02.9161618Z ##[section]Starting: Checkout
2019-08-17T11:11:02.9165360Z ==============================================================================
2019-08-17T11:11:02.9165430Z Task         : Get sources
2019-08-17T11:11:02.9165484Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
