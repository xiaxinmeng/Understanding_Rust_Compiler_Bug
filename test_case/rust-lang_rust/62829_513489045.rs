plain
2019-07-20T18:16:19.5520084Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-20T18:16:19.5707688Z ##[command]git config gc.auto 0
2019-07-20T18:16:19.5781527Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-20T18:16:19.5839372Z ##[command]git config --get-all http.proxy
2019-07-20T18:16:19.5973907Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62829/merge:refs/remotes/pull/62829/merge
---
2019-07-20T18:16:52.7253959Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-20T18:16:52.7255698Z 
2019-07-20T18:16:52.7257823Z   git checkout -b <new-branch-name>
2019-07-20T18:16:52.7259357Z 
2019-07-20T18:16:52.7260847Z HEAD is now at 70505f0de Merge b2d74fae7f7910c586eeed46f018023a587d30e9 into 95b1fe560d2bd8472f250fb8cfd2168520a58405
2019-07-20T18:16:52.7418594Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-20T18:16:52.7421551Z ==============================================================================
2019-07-20T18:16:52.7421617Z Task         : Bash
2019-07-20T18:16:52.7421660Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-20T18:19:56.2225415Z #######################################                                   54.3%
2019-07-20T18:19:56.2243350Z ######################################################################## 100.0%
2019-07-20T18:19:56.8634953Z extracting /checkout/obj/build/cache/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-07-20T18:19:57.2385609Z     Updating crates.io index
2019-07-20T18:20:17.2982581Z     Updating git repository `https://github.com/crlf0710/libtest`
2019-07-20T18:20:18.1127420Z     Updating git repository `https://github.com/crlf0710/termcolor`
---
2019-07-20T18:22:12.7624139Z Checking test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-20T18:22:13.0008454Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-07-20T18:22:13.0014242Z    Compiling libc v0.2.54
2019-07-20T18:22:13.7752420Z     Checking unicode-width v0.1.5
2019-07-20T18:22:13.8775786Z     Checking termcolor v1.0.5 (https://github.com/crlf0710/termcolor#b10766b0)
2019-07-20T18:22:17.2103558Z     Checking getopts v0.2.19
2019-07-20T18:22:32.7540637Z     Checking rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
2019-07-20T18:22:32.7540637Z     Checking rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
2019-07-20T18:22:33.6638883Z     Checking libtest v0.0.2 (https://github.com/crlf0710/libtest#1d7193d8)
2019-07-20T18:22:33.7763979Z error: duplicate lang item in crate `core`: `char`.
2019-07-20T18:22:33.7775926Z   |
2019-07-20T18:22:33.7782445Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7785203Z 
2019-07-20T18:22:33.7790882Z error: duplicate lang item in crate `core`: `str`.
2019-07-20T18:22:33.7796717Z   |
2019-07-20T18:22:33.7802063Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7804846Z 
2019-07-20T18:22:33.7811133Z error: duplicate lang item in crate `core`: `slice`.
2019-07-20T18:22:33.7815716Z   |
2019-07-20T18:22:33.7821988Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7822061Z 
2019-07-20T18:22:33.7822280Z error: duplicate lang item in crate `core`: `slice_u8`.
2019-07-20T18:22:33.7822444Z   |
2019-07-20T18:22:33.7822662Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7822704Z 
2019-07-20T18:22:33.7823084Z error: duplicate lang item in crate `core`: `const_ptr`.
2019-07-20T18:22:33.7823249Z   |
2019-07-20T18:22:33.7823471Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7823500Z 
2019-07-20T18:22:33.7824066Z error: duplicate lang item in crate `core`: `mut_ptr`.
2019-07-20T18:22:33.7824258Z   |
2019-07-20T18:22:33.7824475Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7824504Z 
2019-07-20T18:22:33.7824712Z error: duplicate lang item in crate `core`: `i8`.
2019-07-20T18:22:33.7824909Z   |
2019-07-20T18:22:33.7825124Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7825316Z 
2019-07-20T18:22:33.7825581Z error: duplicate lang item in crate `core`: `i16`.
2019-07-20T18:22:33.7825759Z   |
2019-07-20T18:22:33.7826578Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7826618Z 
2019-07-20T18:22:33.7844658Z error: duplicate lang item in crate `core`: `i32`.
2019-07-20T18:22:33.7844880Z   |
2019-07-20T18:22:33.7845122Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7845155Z 
2019-07-20T18:22:33.7845365Z error: duplicate lang item in crate `core`: `i64`.
2019-07-20T18:22:33.7845557Z   |
2019-07-20T18:22:33.7845931Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7845960Z 
2019-07-20T18:22:33.7846769Z error: duplicate lang item in crate `core`: `i128`.
2019-07-20T18:22:33.7847015Z   |
2019-07-20T18:22:33.7847273Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7847309Z 
2019-07-20T18:22:33.7847575Z error: duplicate lang item in crate `core`: `isize`.
2019-07-20T18:22:33.7847793Z   |
2019-07-20T18:22:33.7848052Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7848106Z 
2019-07-20T18:22:33.7848354Z error: duplicate lang item in crate `core`: `u8`.
2019-07-20T18:22:33.7848559Z   |
2019-07-20T18:22:33.7848815Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7848866Z 
2019-07-20T18:22:33.7849114Z error: duplicate lang item in crate `core`: `u16`.
2019-07-20T18:22:33.7849331Z   |
2019-07-20T18:22:33.7849603Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7849640Z 
2019-07-20T18:22:33.7849888Z error: duplicate lang item in crate `core`: `u32`.
2019-07-20T18:22:33.7850786Z   |
2019-07-20T18:22:33.7851238Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7851270Z 
2019-07-20T18:22:33.7851485Z error: duplicate lang item in crate `core`: `u64`.
2019-07-20T18:22:33.7851681Z   |
2019-07-20T18:22:33.7851903Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7851934Z 
2019-07-20T18:22:33.7852150Z error: duplicate lang item in crate `core`: `u128`.
2019-07-20T18:22:33.7852357Z   |
2019-07-20T18:22:33.7852581Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7852611Z 
2019-07-20T18:22:33.7852842Z error: duplicate lang item in crate `core`: `usize`.
2019-07-20T18:22:33.7853026Z   |
2019-07-20T18:22:33.7853254Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7853301Z 
2019-07-20T18:22:33.7853520Z error: duplicate lang item in crate `core`: `f32`.
2019-07-20T18:22:33.7853861Z   |
2019-07-20T18:22:33.7854077Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7854124Z 
2019-07-20T18:22:33.7854330Z error: duplicate lang item in crate `core`: `f64`.
2019-07-20T18:22:33.7854504Z   |
2019-07-20T18:22:33.7854832Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7854863Z 
2019-07-20T18:22:33.7855252Z error: duplicate lang item in crate `core`: `sized`.
2019-07-20T18:22:33.7855431Z   |
2019-07-20T18:22:33.7855679Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7855710Z 
2019-07-20T18:22:33.7856487Z error: duplicate lang item in crate `core`: `unsize`.
2019-07-20T18:22:33.7856721Z   |
2019-07-20T18:22:33.7856979Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7857013Z 
2019-07-20T18:22:33.7857259Z error: duplicate lang item in crate `core`: `copy`.
2019-07-20T18:22:33.7857486Z   |
2019-07-20T18:22:33.7857754Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7857789Z 
2019-07-20T18:22:33.7858053Z error: duplicate lang item in crate `core`: `clone`.
2019-07-20T18:22:33.7858447Z   |
2019-07-20T18:22:33.7858719Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7858753Z 
2019-07-20T18:22:33.7859023Z error: duplicate lang item in crate `core`: `sync`.
2019-07-20T18:22:33.7859228Z   |
2019-07-20T18:22:33.7859639Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7859846Z 
2019-07-20T18:22:33.7860209Z error: duplicate lang item in crate `core`: `freeze`.
2019-07-20T18:22:33.7860371Z   |
2019-07-20T18:22:33.7860588Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7860623Z 
2019-07-20T18:22:33.7860822Z error: duplicate lang item in crate `core`: `drop`.
2019-07-20T18:22:33.7860986Z   |
2019-07-20T18:22:33.7861206Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7861233Z 
2019-07-20T18:22:33.7861527Z error: duplicate lang item in crate `core`: `coerce_unsized`.
2019-07-20T18:22:33.7861742Z   |
2019-07-20T18:22:33.7861948Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7861976Z 
2019-07-20T18:22:33.7862180Z error: duplicate lang item in crate `core`: `dispatch_from_dyn`.
2019-07-20T18:22:33.7862362Z   |
2019-07-20T18:22:33.7862564Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7862599Z 
2019-07-20T18:22:33.7862986Z error: duplicate lang item in crate `core`: `add`.
2019-07-20T18:22:33.7863159Z   |
2019-07-20T18:22:33.7863366Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7863395Z 
2019-07-20T18:22:33.7863620Z error: duplicate lang item in crate `core`: `sub`.
2019-07-20T18:22:33.7863791Z   |
2019-07-20T18:22:33.7863999Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7864043Z 
2019-07-20T18:22:33.7864246Z error: duplicate lang item in crate `core`: `mul`.
2019-07-20T18:22:33.7864592Z   |
2019-07-20T18:22:33.7864823Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7864942Z 
2019-07-20T18:22:33.7865170Z error: duplicate lang item in crate `core`: `div`.
2019-07-20T18:22:33.7865345Z   |
2019-07-20T18:22:33.7865582Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7865611Z 
2019-07-20T18:22:33.7866468Z error: duplicate lang item in crate `core`: `rem`.
2019-07-20T18:22:33.7866706Z   |
2019-07-20T18:22:33.7866963Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7866998Z 
2019-07-20T18:22:33.7867243Z error: duplicate lang item in crate `core`: `neg`.
2019-07-20T18:22:33.7867467Z   |
2019-07-20T18:22:33.7867721Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7867756Z 
2019-07-20T18:22:33.7868029Z error: duplicate lang item in crate `core`: `not`.
2019-07-20T18:22:33.7868236Z   |
2019-07-20T18:22:33.7868491Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7868526Z 
2019-07-20T18:22:33.7868801Z error: duplicate lang item in crate `core`: `bitxor`.
2019-07-20T18:22:33.7869012Z   |
2019-07-20T18:22:33.7869266Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7869317Z 
2019-07-20T18:22:33.7869564Z error: duplicate lang item in crate `core`: `bitand`.
2019-07-20T18:22:33.7870234Z   |
2019-07-20T18:22:33.7870656Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7870687Z 
2019-07-20T18:22:33.7871049Z error: duplicate lang item in crate `core`: `bitor`.
2019-07-20T18:22:33.7871225Z   |
2019-07-20T18:22:33.7871629Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7871656Z 
2019-07-20T18:22:33.7872077Z error: duplicate lang item in crate `core`: `shl`.
2019-07-20T18:22:33.7872354Z   |
2019-07-20T18:22:33.7872596Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7872626Z 
2019-07-20T18:22:33.7872828Z error: duplicate lang item in crate `core`: `shr`.
2019-07-20T18:22:33.7873013Z   |
2019-07-20T18:22:33.7873220Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7873249Z 
2019-07-20T18:22:33.7873469Z error: duplicate lang item in crate `core`: `add_assign`.
2019-07-20T18:22:33.7873650Z   |
2019-07-20T18:22:33.7873859Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7873887Z 
2019-07-20T18:22:33.7874109Z error: duplicate lang item in crate `core`: `sub_assign`.
2019-07-20T18:22:33.7874277Z   |
2019-07-20T18:22:33.7874491Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7874539Z 
2019-07-20T18:22:33.7874901Z error: duplicate lang item in crate `core`: `mul_assign`.
2019-07-20T18:22:33.7875223Z   |
2019-07-20T18:22:33.7875439Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7875466Z 
2019-07-20T18:22:33.7876033Z error: duplicate lang item in crate `core`: `div_assign`.
2019-07-20T18:22:33.7876362Z   |
2019-07-20T18:22:33.7876642Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7876677Z 
2019-07-20T18:22:33.7876927Z error: duplicate lang item in crate `core`: `rem_assign`.
2019-07-20T18:22:33.7877150Z   |
2019-07-20T18:22:33.7877417Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7877452Z 
2019-07-20T18:22:33.7877708Z error: duplicate lang item in crate `core`: `bitxor_assign`.
2019-07-20T18:22:33.7877934Z   |
2019-07-20T18:22:33.7878190Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7878225Z 
2019-07-20T18:22:33.7878496Z error: duplicate lang item in crate `core`: `bitand_assign`.
2019-07-20T18:22:33.7878716Z   |
2019-07-20T18:22:33.7878972Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7879007Z 
2019-07-20T18:22:33.7879577Z error: duplicate lang item in crate `core`: `bitor_assign`.
2019-07-20T18:22:33.7879745Z   |
2019-07-20T18:22:33.7879961Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7880004Z 
2019-07-20T18:22:33.7880212Z error: duplicate lang item in crate `core`: `shl_assign`.
2019-07-20T18:22:33.7880378Z   |
2019-07-20T18:22:33.7880602Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7880631Z 
2019-07-20T18:22:33.7880992Z error: duplicate lang item in crate `core`: `shr_assign`.
2019-07-20T18:22:33.7881156Z   |
2019-07-20T18:22:33.7881386Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7881414Z 
2019-07-20T18:22:33.7881765Z error: duplicate lang item in crate `core`: `index`.
2019-07-20T18:22:33.7882184Z   |
2019-07-20T18:22:33.7882485Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7882521Z 
2019-07-20T18:22:33.7882739Z error: duplicate lang item in crate `core`: `index_mut`.
2019-07-20T18:22:33.7882922Z   |
2019-07-20T18:22:33.7883122Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7883151Z 
2019-07-20T18:22:33.7883368Z error: duplicate lang item in crate `core`: `unsafe_cell`.
2019-07-20T18:22:33.7883532Z   |
2019-07-20T18:22:33.7883921Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7883949Z 
2019-07-20T18:22:33.7884169Z error: duplicate lang item in crate `core`: `va_list`.
2019-07-20T18:22:33.7884338Z   |
2019-07-20T18:22:33.7884553Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7884597Z 
2019-07-20T18:22:33.7884804Z error: duplicate lang item in crate `core`: `deref`.
2019-07-20T18:22:33.7884971Z   |
2019-07-20T18:22:33.7885356Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7885384Z 
2019-07-20T18:22:33.7885586Z error: duplicate lang item in crate `core`: `deref_mut`.
2019-07-20T18:22:33.7886089Z   |
2019-07-20T18:22:33.7886378Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7886511Z 
2019-07-20T18:22:33.7886782Z error: duplicate lang item in crate `core`: `receiver`.
2019-07-20T18:22:33.7887009Z   |
2019-07-20T18:22:33.7887265Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7887308Z 
2019-07-20T18:22:33.7887552Z error: duplicate lang item in crate `core`: `fn`.
2019-07-20T18:22:33.7887778Z   |
2019-07-20T18:22:33.7888033Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7888069Z 
2019-07-20T18:22:33.7888333Z error: duplicate lang item in crate `core`: `fn_mut`.
2019-07-20T18:22:33.7888541Z   |
2019-07-20T18:22:33.7888796Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7888839Z 
2019-07-20T18:22:33.7889110Z error: duplicate lang item in crate `core`: `fn_once`.
2019-07-20T18:22:33.7889624Z   |
2019-07-20T18:22:33.7889998Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7890043Z 
2019-07-20T18:22:33.7890258Z error: duplicate lang item in crate `core`: `future_trait`.
2019-07-20T18:22:33.7890430Z   |
2019-07-20T18:22:33.7890654Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7890682Z 
2019-07-20T18:22:33.7890893Z error: duplicate lang item in crate `core`: `generator_state`.
2019-07-20T18:22:33.7891066Z   |
2019-07-20T18:22:33.7891292Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7891327Z 
2019-07-20T18:22:33.7891688Z error: duplicate lang item in crate `core`: `generator`.
2019-07-20T18:22:33.7891867Z   |
2019-07-20T18:22:33.7892070Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7892098Z 
2019-07-20T18:22:33.7892473Z error: duplicate lang item in crate `core`: `unpin`.
2019-07-20T18:22:33.7892691Z   |
2019-07-20T18:22:33.7892894Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7892924Z 
2019-07-20T18:22:33.7893290Z error: duplicate lang item in crate `core`: `pin`.
2019-07-20T18:22:33.7893452Z   |
2019-07-20T18:22:33.7893647Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7893682Z 
2019-07-20T18:22:33.7894067Z error: duplicate lang item in crate `core`: `eq`.
2019-07-20T18:22:33.7894231Z   |
2019-07-20T18:22:33.7894432Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7894475Z 
2019-07-20T18:22:33.7894680Z error: duplicate lang item in crate `core`: `partial_ord`.
2019-07-20T18:22:33.7894847Z   |
2019-07-20T18:22:33.7895065Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7895093Z 
2019-07-20T18:22:33.7895289Z error: duplicate lang item in crate `core`: `ord`.
2019-07-20T18:22:33.7895452Z   |
2019-07-20T18:22:33.7895672Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7895700Z 
2019-07-20T18:22:33.7896339Z error: duplicate lang item in crate `core`: `panic`.
2019-07-20T18:22:33.7896713Z   |
2019-07-20T18:22:33.7896974Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7897009Z 
2019-07-20T18:22:33.7897267Z error: duplicate lang item in crate `core`: `panic_bounds_check`.
2019-07-20T18:22:33.7897502Z   |
2019-07-20T18:22:33.7897758Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7897794Z 
2019-07-20T18:22:33.7898060Z error: duplicate lang item in crate `core`: `panic_info`.
2019-07-20T18:22:33.7908061Z   |
2019-07-20T18:22:33.7908367Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7908405Z 
2019-07-20T18:22:33.7908689Z error: duplicate lang item in crate `core`: `drop_in_place`.
2019-07-20T18:22:33.7908915Z   |
2019-07-20T18:22:33.7909172Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7909223Z 
2019-07-20T18:22:33.7909782Z error: duplicate lang item in crate `core`: `alloc_layout`.
2019-07-20T18:22:33.7909956Z   |
2019-07-20T18:22:33.7910176Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7910204Z 
2019-07-20T18:22:33.7910407Z error: duplicate lang item in crate `core`: `phantom_data`.
2019-07-20T18:22:33.7910572Z   |
2019-07-20T18:22:33.7910794Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7910822Z 
2019-07-20T18:22:33.7911024Z error: duplicate lang item in crate `core`: `manually_drop`.
2019-07-20T18:22:33.7911215Z   |
2019-07-20T18:22:33.7911418Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7911446Z 
2019-07-20T18:22:33.7911645Z error: duplicate lang item in crate `core`: `debug_trait`.
2019-07-20T18:22:33.7911930Z   |
2019-07-20T18:22:33.7912171Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7912201Z 
2019-07-20T18:22:33.7912574Z error: duplicate lang item in crate `core`: `align_offset`.
2019-07-20T18:22:33.7912736Z   |
2019-07-20T18:22:33.7912931Z   = note: first defined in crate `core`.
2019-07-20T18:22:33.7912973Z 
2019-07-20T18:22:33.8188472Z error[E0277]: the size for values of type `std::boxed::Box<(dyn std::io::Write + std::marker::Send + 'static)>` cannot be known at compilation time
2019-07-20T18:22:33.8189416Z   --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/1d7193d/src/lib.rs:56:1
2019-07-20T18:22:33.8193719Z 56 | / fn set_print(
2019-07-20T18:22:33.8193719Z 56 | / fn set_print(
2019-07-20T18:22:33.8195004Z 57 | |     sink: Option<Box<dyn Write + Send>>,
2019-07-20T18:22:33.8195721Z 58 | | ) -> Option<Box<dyn Write + Send>> {
2019-07-20T18:22:33.8196461Z 59 | |     #[cfg(feature = "unstable")]
2019-07-20T18:22:33.8237885Z 66 | |     }
2019-07-20T18:22:33.8241487Z 67 | | }
2019-07-20T18:22:33.8241487Z 67 | | }
2019-07-20T18:22:33.8254350Z    | |_^ doesn't have a size known at compile-time
2019-07-20T18:22:33.8264458Z    |
2019-07-20T18:22:33.8274812Z    = help: the trait `std::marker::Sized` is not implemented for `std::boxed::Box<(dyn std::io::Write + std::marker::Send + 'static)>`
2019-07-20T18:22:33.8280021Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T18:22:33.8296934Z 
2019-07-20T18:22:33.8303129Z error[E0038]: the trait `std::io::Write` cannot be made into an object
2019-07-20T18:22:33.8303129Z error[E0038]: the trait `std::io::Write` cannot be made into an object
2019-07-20T18:22:33.8318382Z   --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/1d7193d/src/lib.rs:56:1
2019-07-20T18:22:33.8327595Z 56 | / fn set_print(
2019-07-20T18:22:33.8327595Z 56 | / fn set_print(
2019-07-20T18:22:33.8333524Z 57 | |     sink: Option<Box<dyn Write + Send>>,
2019-07-20T18:22:33.8343901Z 58 | | ) -> Option<Box<dyn Write + Send>> {
2019-07-20T18:22:33.8347699Z 59 | |     #[cfg(feature = "unstable")]
2019-07-20T18:22:33.8365248Z 66 | |     }
2019-07-20T18:22:33.8371600Z 67 | | }
2019-07-20T18:22:33.8371600Z 67 | | }
2019-07-20T18:22:33.8377216Z    | |_^ the trait `std::io::Write` cannot be made into an object
2019-07-20T18:22:33.8480152Z    |
2019-07-20T18:22:33.8484208Z    = note: method `by_ref` references the `Self` type in its arguments or return type
2019-07-20T18:22:33.8486683Z 
2019-07-20T18:22:33.8488114Z error[E0277]: the size for values of type `std::boxed::Box<(dyn std::io::Write + std::marker::Send + 'static)>` cannot be known at compilation time
2019-07-20T18:22:33.8490530Z   --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/1d7193d/src/lib.rs:69:1
2019-07-20T18:22:33.8494576Z 69 | / fn set_panic(
2019-07-20T18:22:33.8494576Z 69 | / fn set_panic(
2019-07-20T18:22:33.8499563Z 70 | |     sink: Option<Box<dyn Write + Send>>,
2019-07-20T18:22:33.8503314Z 71 | | ) -> Option<Box<dyn Write + Send>> {
2019-07-20T18:22:33.8507244Z 72 | |     #[cfg(feature = "unstable")]
2019-07-20T18:22:33.8513443Z 79 | |     }
2019-07-20T18:22:33.8516998Z 80 | | }
2019-07-20T18:22:33.8516998Z 80 | | }
2019-07-20T18:22:33.8520826Z    | |_^ doesn't have a size known at compile-time
2019-07-20T18:22:33.8524166Z    |
2019-07-20T18:22:33.8527650Z    = help: the trait `std::marker::Sized` is not implemented for `std::boxed::Box<(dyn std::io::Write + std::marker::Send + 'static)>`
2019-07-20T18:22:33.8531872Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T18:22:33.8536458Z 
2019-07-20T18:22:33.8538001Z error[E0038]: the trait `std::io::Write` cannot be made into an object
2019-07-20T18:22:33.8538001Z error[E0038]: the trait `std::io::Write` cannot be made into an object
2019-07-20T18:22:33.8540155Z   --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/1d7193d/src/lib.rs:69:1
2019-07-20T18:22:33.8550287Z 69 | / fn set_panic(
2019-07-20T18:22:33.8550287Z 69 | / fn set_panic(
2019-07-20T18:22:33.8553426Z 70 | |     sink: Option<Box<dyn Write + Send>>,
2019-07-20T18:22:33.8556918Z 71 | | ) -> Option<Box<dyn Write + Send>> {
2019-07-20T18:22:33.8560644Z 72 | |     #[cfg(feature = "unstable")]
2019-07-20T18:22:33.8566676Z 79 | |     }
2019-07-20T18:22:33.8571284Z 80 | | }
2019-07-20T18:22:33.8571284Z 80 | | }
2019-07-20T18:22:33.8575412Z    | |_^ the trait `std::io::Write` cannot be made into an object
2019-07-20T18:22:33.8578750Z    |
2019-07-20T18:22:33.8581781Z    = note: method `by_ref` references the `Self` type in its arguments or return type
2019-07-20T18:22:33.8582834Z 
2019-07-20T18:22:33.8584526Z error[E0277]: the size for values of type `TestDescAndFn` cannot be known at compilation time
2019-07-20T18:22:33.8587689Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/1d7193d/src/lib.rs:269:1
2019-07-20T18:22:33.8592020Z 269 | / pub fn test_main(
2019-07-20T18:22:33.8592020Z 269 | / pub fn test_main(
2019-07-20T18:22:33.8593525Z 270 | |     args: &[String],
2019-07-20T18:22:33.8594899Z 271 | |     tests: Vec<TestDescAndFn>,
2019-07-20T18:22:33.8599479Z ...   |
2019-07-20T18:22:33.8600985Z 298 | |     }
2019-07-20T18:22:33.8603597Z 299 | | }
2019-07-20T18:22:33.8603597Z 299 | | }
2019-07-20T18:22:33.8606972Z     | |_^ doesn't have a size known at compile-time
2019-07-20T18:22:33.8611724Z     |
2019-07-20T18:22:33.8614321Z     = help: the trait `std::marker::Sized` is not implemented for `TestDescAndFn`
2019-07-20T18:22:33.8617090Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T18:22:33.8626975Z     = note: required by `std::vec::Vec`
2019-07-20T18:22:33.8627030Z 
2019-07-20T18:22:33.8627399Z error[E0277]: the size for values of type `std::result::Result<TestOpts, std::string::String>` cannot be known at compilation time
2019-07-20T18:22:33.8627704Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/1d7193d/src/lib.rs:521:1
2019-07-20T18:22:33.8627918Z     |
2019-07-20T18:22:33.8628279Z 521 | / pub fn parse_opts(args: &[String]) -> Option<OptRes> {
2019-07-20T18:22:33.8628589Z 522 | |     let mut allow_unstable = false;
2019-07-20T18:22:33.8628915Z 523 | |     let opts = optgroups();
2019-07-20T18:22:33.8629559Z 524 | |     let args = args.get(1..).unwrap_or(args);
2019-07-20T18:22:33.8629918Z ...   |
2019-07-20T18:22:33.8630169Z 670 | |     Some(Ok(test_opts))
2019-07-20T18:22:33.8630394Z 671 | | }
2019-07-20T18:22:33.8630648Z     | |_^ doesn't have a size known at compile-time
2019-07-20T18:22:33.8630998Z     |
2019-07-20T18:22:33.8631556Z     = help: the trait `std::marker::Sized` is not implemented for `std::result::Result<TestOpts, std::string::String>`
2019-07-20T18:22:33.8632257Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T18:22:33.8632555Z 
2019-07-20T18:22:33.8632555Z 
2019-07-20T18:22:33.8632803Z error[E0277]: the size for values of type `TestOpts` cannot be known at compilation time
2019-07-20T18:22:33.8633078Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/1d7193d/src/lib.rs:521:1
2019-07-20T18:22:33.8633275Z     |
2019-07-20T18:22:33.8633553Z 521 | / pub fn parse_opts(args: &[String]) -> Option<OptRes> {
2019-07-20T18:22:33.8633851Z 522 | |     let mut allow_unstable = false;
2019-07-20T18:22:33.8634111Z 523 | |     let opts = optgroups();
2019-07-20T18:22:33.8634403Z 524 | |     let args = args.get(1..).unwrap_or(args);
2019-07-20T18:22:33.8634615Z ...   |
2019-07-20T18:22:33.8635028Z 670 | |     Some(Ok(test_opts))
2019-07-20T18:22:33.8635679Z 671 | | }
2019-07-20T18:22:33.8636504Z     | |_^ doesn't have a size known at compile-time
2019-07-20T18:22:33.8636728Z     |
2019-07-20T18:22:33.8637054Z     = help: the trait `std::marker::Sized` is not implemented for `TestOpts`
2019-07-20T18:22:33.8637424Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T18:22:33.8637750Z 
2019-07-20T18:22:33.8638046Z error[E0277]: the size for values of type `std::string::String` cannot be known at compilation time
2019-07-20T18:22:33.8638046Z error[E0277]: the size for values of type `std::string::String` cannot be known at compilation time
2019-07-20T18:22:33.8638352Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/1d7193d/src/lib.rs:521:1
2019-07-20T18:22:33.8638582Z     |
2019-07-20T18:22:33.8638909Z 521 | / pub fn parse_opts(args: &[String]) -> Option<OptRes> {
2019-07-20T18:22:33.8639222Z 522 | |     let mut allow_unstable = false;
2019-07-20T18:22:33.8640015Z 523 | |     let opts = optgroups();
2019-07-20T18:22:33.8640266Z 524 | |     let args = args.get(1..).unwrap_or(args);
2019-07-20T18:22:33.8640454Z ...   |
2019-07-20T18:22:33.8640714Z 670 | |     Some(Ok(test_opts))
2019-07-20T18:22:33.8640933Z 671 | | }
2019-07-20T18:22:33.8641265Z     | |_^ doesn't have a size known at compile-time
2019-07-20T18:22:33.8641468Z     |
2019-07-20T18:22:33.8641711Z     = help: the trait `std::marker::Sized` is not implemented for `std::string::String`
2019-07-20T18:22:33.8642187Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T18:22:33.8642451Z 
2019-07-20T18:22:33.8642451Z 
2019-07-20T18:22:33.8642707Z error[E0277]: the size for values of type `std::fs::File` cannot be known at compilation time
2019-07-20T18:22:33.8642950Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/1d7193d/src/lib.rs:714:5
2019-07-20T18:22:33.8643122Z     |
2019-07-20T18:22:33.8643360Z 714 |     log_out: Option<File>,
2019-07-20T18:22:33.8643636Z     |     ^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-07-20T18:22:33.8643811Z     |
2019-07-20T18:22:33.8644246Z     = help: the trait `std::marker::Sized` is not implemented for `std::fs::File`
2019-07-20T18:22:33.8644556Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T18:22:33.8645175Z 
2019-07-20T18:22:33.8645175Z 
2019-07-20T18:22:33.8646017Z error[E0277]: the size for values of type `(TestDesc, std::vec::Vec<u8>)` cannot be known at compilation time
2019-07-20T18:22:33.8646350Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/1d7193d/src/lib.rs:723:5
2019-07-20T18:22:33.8646585Z     |
2019-07-20T18:22:33.8647222Z 723 |     failures: Vec<(TestDesc, Vec<u8>)>,
2019-07-20T18:22:33.8647574Z     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-07-20T18:22:33.8647810Z     |
2019-07-20T18:22:33.8648135Z     = help: the trait `std::marker::Sized` is not implemented for `(TestDesc, std::vec::Vec<u8>)`
2019-07-20T18:22:33.8648505Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T18:22:33.8648797Z     = note: required by `std::vec::Vec`
2019-07-20T18:22:33.8648832Z 
2019-07-20T18:22:33.8649109Z error[E0277]: the size for values of type `u8` cannot be known at compilation time
2019-07-20T18:22:33.8649572Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/1d7193d/src/lib.rs:723:5
2019-07-20T18:22:33.8649910Z     |
2019-07-20T18:22:33.8650130Z 723 |     failures: Vec<(TestDesc, Vec<u8>)>,
2019-07-20T18:22:33.8650429Z     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-07-20T18:22:33.8650602Z     |
2019-07-20T18:22:33.8650926Z     = help: the trait `std::marker::Sized` is not implemented for `u8`
2019-07-20T18:22:33.8651244Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T18:22:33.8651612Z     = note: required by `std::vec::Vec`
2019-07-20T18:22:33.8651655Z 
2019-07-20T18:22:33.8651881Z error[E0277]: the size for values of type `TestDescAndFn` cannot be known at compilation time
2019-07-20T18:22:33.8652108Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/1d7193d/src/lib.rs:834:1
2019-07-20T18:22:33.8652298Z     |
2019-07-20T18:22:33.8652530Z 834 | / pub fn list_tests_console(
2019-07-20T18:22:33.8652760Z 835 | |     opts: &TestOpts,
2019-07-20T18:22:33.8653013Z 836 | |     tests: Vec<TestDescAndFn>,
2019-07-20T18:22:33.8653243Z 837 | | ) -> io::Result<()> {
2019-07-20T18:22:33.8653652Z 888 | |     Ok(())
2019-07-20T18:22:33.8653947Z 889 | | }
2019-07-20T18:22:33.8653947Z 889 | | }
2019-07-20T18:22:33.8654184Z     | |_^ doesn't have a size known at compile-time
2019-07-20T18:22:33.8654368Z     |
2019-07-20T18:22:33.8654603Z     = help: the trait `std::marker::Sized` is not implemented for `TestDescAndFn`
2019-07-20T18:22:33.8654881Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T18:22:33.8655112Z     = note: required by `std::vec::Vec`
2019-07-20T18:22:33.8655139Z 
2019-07-20T18:22:33.8655526Z error[E0277]: the size for values of type `()` cannot be known at compilation time
2019-07-20T18:22:33.8656229Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/1d7193d/src/lib.rs:834:1
2019-07-20T18:22:33.8656451Z     |
2019-07-20T18:22:33.8656751Z 834 | / pub fn list_tests_console(
2019-07-20T18:22:33.8657392Z 835 | |     opts: &TestOpts,
2019-07-20T18:22:33.8657701Z 836 | |     tests: Vec<TestDescAndFn>,
2019-07-20T18:22:33.8657997Z 837 | | ) -> io::Result<()> {
2019-07-20T18:22:33.8695760Z 888 | |     Ok(())
2019-07-20T18:22:33.8696519Z 889 | | }
---
2019-07-20T18:22:33.9610595Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json"
2019-07-20T18:22:33.9610669Z expected success, got: exit code: 101
2019-07-20T18:22:33.9610728Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-20T18:22:33.9610783Z Build completed unsuccessfully in 0:02:52
2019-07-20T18:22:35.4722665Z ##[error]Bash exited with code '1'.
2019-07-20T18:22:35.4760738Z ##[section]Starting: Checkout
2019-07-20T18:22:35.4762326Z ==============================================================================
2019-07-20T18:22:35.4762373Z Task         : Get sources
2019-07-20T18:22:35.4762415Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
