plain
2019-07-20T17:22:03.3891110Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-20T17:22:03.4093530Z ##[command]git config gc.auto 0
2019-07-20T17:22:03.4187894Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-20T17:22:03.4237870Z ##[command]git config --get-all http.proxy
2019-07-20T17:22:03.4371937Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62829/merge:refs/remotes/pull/62829/merge
---
2019-07-20T17:22:37.2026699Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-20T17:22:37.2026944Z 
2019-07-20T17:22:37.2027329Z   git checkout -b <new-branch-name>
2019-07-20T17:22:37.2027365Z 
2019-07-20T17:22:37.2149594Z HEAD is now at 5490949f5 Merge 74cd3bd318de9838f5c13a6332eca519e7238ad8 into 95b1fe560d2bd8472f250fb8cfd2168520a58405
2019-07-20T17:22:37.2220309Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-20T17:22:37.2223261Z ==============================================================================
2019-07-20T17:22:37.2223324Z Task         : Bash
2019-07-20T17:22:37.2223378Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-20T17:26:06.1772840Z ###################                                                       26.7%
2019-07-20T17:26:06.1773299Z ######################################################################## 100.0%
2019-07-20T17:26:06.8087811Z extracting /checkout/obj/build/cache/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-07-20T17:26:08.7789109Z     Updating crates.io index
2019-07-20T17:26:29.6244234Z     Updating git repository `https://github.com/crlf0710/libtest`
2019-07-20T17:26:30.4824345Z     Updating git repository `https://github.com/crlf0710/termcolor`
---
2019-07-20T17:28:32.0720449Z     Checking unicode-width v0.1.5
2019-07-20T17:28:32.1770656Z     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
2019-07-20T17:28:35.3460780Z     Checking getopts v0.2.19
2019-07-20T17:28:51.8213713Z     Checking rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
2019-07-20T17:28:51.8722432Z     Checking termcolor v1.0.5 (https://github.com/crlf0710/termcolor#00591241)
2019-07-20T17:28:52.8446745Z     Checking libtest v0.0.2 (https://github.com/crlf0710/libtest#a47e7189)
2019-07-20T17:28:52.9654160Z error: duplicate lang item in crate `core`: `char`.
2019-07-20T17:28:52.9654639Z   |
2019-07-20T17:28:52.9654985Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9655057Z 
2019-07-20T17:28:52.9655383Z error: duplicate lang item in crate `core`: `str`.
2019-07-20T17:28:52.9655663Z   |
2019-07-20T17:28:52.9655991Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9656092Z 
2019-07-20T17:28:52.9656636Z error: duplicate lang item in crate `core`: `slice`.
2019-07-20T17:28:52.9656995Z   |
2019-07-20T17:28:52.9657348Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9657396Z 
2019-07-20T17:28:52.9657717Z error: duplicate lang item in crate `core`: `slice_u8`.
2019-07-20T17:28:52.9657991Z   |
2019-07-20T17:28:52.9658346Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9658390Z 
2019-07-20T17:28:52.9658706Z error: duplicate lang item in crate `core`: `const_ptr`.
2019-07-20T17:28:52.9659002Z   |
2019-07-20T17:28:52.9659331Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9659375Z 
2019-07-20T17:28:52.9659690Z error: duplicate lang item in crate `core`: `mut_ptr`.
2019-07-20T17:28:52.9660181Z   |
2019-07-20T17:28:52.9660516Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9660560Z 
2019-07-20T17:28:52.9661741Z error: duplicate lang item in crate `core`: `i8`.
2019-07-20T17:28:52.9662139Z   |
2019-07-20T17:28:52.9662470Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9662540Z 
2019-07-20T17:28:52.9662863Z error: duplicate lang item in crate `core`: `i16`.
2019-07-20T17:28:52.9663135Z   |
2019-07-20T17:28:52.9663463Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9663526Z 
2019-07-20T17:28:52.9663842Z error: duplicate lang item in crate `core`: `i32`.
2019-07-20T17:28:52.9664136Z   |
2019-07-20T17:28:52.9664519Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9664565Z 
2019-07-20T17:28:52.9664883Z error: duplicate lang item in crate `core`: `i64`.
2019-07-20T17:28:52.9665155Z   |
2019-07-20T17:28:52.9665506Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9665551Z 
2019-07-20T17:28:52.9665863Z error: duplicate lang item in crate `core`: `i128`.
2019-07-20T17:28:52.9666155Z   |
2019-07-20T17:28:52.9666486Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9666529Z 
2019-07-20T17:28:52.9666839Z error: duplicate lang item in crate `core`: `isize`.
2019-07-20T17:28:52.9667157Z   |
2019-07-20T17:28:52.9667497Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9667540Z 
2019-07-20T17:28:52.9667869Z error: duplicate lang item in crate `core`: `u8`.
2019-07-20T17:28:52.9668148Z   |
2019-07-20T17:28:52.9668476Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9668520Z 
2019-07-20T17:28:52.9668904Z error: duplicate lang item in crate `core`: `u16`.
2019-07-20T17:28:52.9669179Z   |
2019-07-20T17:28:52.9669504Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9669568Z 
2019-07-20T17:28:52.9669883Z error: duplicate lang item in crate `core`: `u32`.
2019-07-20T17:28:52.9670158Z   |
2019-07-20T17:28:52.9670935Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9671029Z 
2019-07-20T17:28:52.9671443Z error: duplicate lang item in crate `core`: `u64`.
2019-07-20T17:28:52.9671723Z   |
2019-07-20T17:28:52.9672080Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9672125Z 
2019-07-20T17:28:52.9672440Z error: duplicate lang item in crate `core`: `u128`.
2019-07-20T17:28:52.9672733Z   |
2019-07-20T17:28:52.9673065Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9673109Z 
2019-07-20T17:28:52.9673421Z error: duplicate lang item in crate `core`: `usize`.
2019-07-20T17:28:52.9673724Z   |
2019-07-20T17:28:52.9674052Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9674228Z 
2019-07-20T17:28:52.9674648Z error: duplicate lang item in crate `core`: `f32`.
2019-07-20T17:28:52.9674932Z   |
2019-07-20T17:28:52.9675261Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9675305Z 
2019-07-20T17:28:52.9675640Z error: duplicate lang item in crate `core`: `f64`.
2019-07-20T17:28:52.9675914Z   |
2019-07-20T17:28:52.9676244Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9676309Z 
2019-07-20T17:28:52.9676626Z error: duplicate lang item in crate `core`: `sized`.
2019-07-20T17:28:52.9676898Z   |
2019-07-20T17:28:52.9677242Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9677299Z 
2019-07-20T17:28:52.9677631Z error: duplicate lang item in crate `core`: `unsize`.
2019-07-20T17:28:52.9677909Z   |
2019-07-20T17:28:52.9678261Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9678304Z 
2019-07-20T17:28:52.9678613Z error: duplicate lang item in crate `core`: `copy`.
2019-07-20T17:28:52.9678905Z   |
2019-07-20T17:28:52.9679241Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9679284Z 
2019-07-20T17:28:52.9679595Z error: duplicate lang item in crate `core`: `clone`.
2019-07-20T17:28:52.9679892Z   |
2019-07-20T17:28:52.9680220Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9680276Z 
2019-07-20T17:28:52.9681061Z error: duplicate lang item in crate `core`: `sync`.
2019-07-20T17:28:52.9681425Z   |
2019-07-20T17:28:52.9681753Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9681797Z 
2019-07-20T17:28:52.9682135Z error: duplicate lang item in crate `core`: `freeze`.
2019-07-20T17:28:52.9682406Z   |
2019-07-20T17:28:52.9682732Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9682794Z 
2019-07-20T17:28:52.9683113Z error: duplicate lang item in crate `core`: `drop`.
2019-07-20T17:28:52.9683384Z   |
2019-07-20T17:28:52.9683728Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9683774Z 
2019-07-20T17:28:52.9684240Z error: duplicate lang item in crate `core`: `coerce_unsized`.
2019-07-20T17:28:52.9684565Z   |
2019-07-20T17:28:52.9684922Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9684966Z 
2019-07-20T17:28:52.9685289Z error: duplicate lang item in crate `core`: `dispatch_from_dyn`.
2019-07-20T17:28:52.9685583Z   |
2019-07-20T17:28:52.9685919Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9685962Z 
2019-07-20T17:28:52.9686269Z error: duplicate lang item in crate `core`: `add`.
2019-07-20T17:28:52.9686564Z   |
2019-07-20T17:28:52.9686889Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9686932Z 
2019-07-20T17:28:52.9687423Z error: duplicate lang item in crate `core`: `sub`.
2019-07-20T17:28:52.9687721Z   |
2019-07-20T17:28:52.9688050Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9688093Z 
2019-07-20T17:28:52.9688430Z error: duplicate lang item in crate `core`: `mul`.
2019-07-20T17:28:52.9688706Z   |
2019-07-20T17:28:52.9689032Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9689095Z 
2019-07-20T17:28:52.9689410Z error: duplicate lang item in crate `core`: `div`.
2019-07-20T17:28:52.9689684Z   |
2019-07-20T17:28:52.9690028Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9690074Z 
2019-07-20T17:28:52.9690390Z error: duplicate lang item in crate `core`: `rem`.
2019-07-20T17:28:52.9691039Z   |
2019-07-20T17:28:52.9691468Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9691516Z 
2019-07-20T17:28:52.9691831Z error: duplicate lang item in crate `core`: `neg`.
2019-07-20T17:28:52.9692126Z   |
2019-07-20T17:28:52.9692460Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9692504Z 
2019-07-20T17:28:52.9692812Z error: duplicate lang item in crate `core`: `not`.
2019-07-20T17:28:52.9693108Z   |
2019-07-20T17:28:52.9693439Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9693483Z 
2019-07-20T17:28:52.9693965Z error: duplicate lang item in crate `core`: `bitxor`.
2019-07-20T17:28:52.9694382Z   |
2019-07-20T17:28:52.9694722Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9694766Z 
2019-07-20T17:28:52.9695106Z error: duplicate lang item in crate `core`: `bitand`.
2019-07-20T17:28:52.9695379Z   |
2019-07-20T17:28:52.9695706Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9695768Z 
2019-07-20T17:28:52.9696085Z error: duplicate lang item in crate `core`: `bitor`.
2019-07-20T17:28:52.9696360Z   |
2019-07-20T17:28:52.9696703Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9696746Z 
2019-07-20T17:28:52.9697063Z error: duplicate lang item in crate `core`: `shl`.
2019-07-20T17:28:52.9697338Z   |
2019-07-20T17:28:52.9697827Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9697886Z 
2019-07-20T17:28:52.9698237Z error: duplicate lang item in crate `core`: `shr`.
2019-07-20T17:28:52.9698531Z   |
2019-07-20T17:28:52.9698865Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9698909Z 
2019-07-20T17:28:52.9699278Z error: duplicate lang item in crate `core`: `add_assign`.
2019-07-20T17:28:52.9699594Z   |
2019-07-20T17:28:52.9699923Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9699967Z 
2019-07-20T17:28:52.9700378Z error: duplicate lang item in crate `core`: `sub_assign`.
2019-07-20T17:28:52.9701059Z   |
2019-07-20T17:28:52.9701664Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9701712Z 
2019-07-20T17:28:52.9702067Z error: duplicate lang item in crate `core`: `mul_assign`.
2019-07-20T17:28:52.9702338Z   |
2019-07-20T17:28:52.9702665Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9702730Z 
2019-07-20T17:28:52.9703052Z error: duplicate lang item in crate `core`: `div_assign`.
2019-07-20T17:28:52.9703327Z   |
2019-07-20T17:28:52.9703672Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9703717Z 
2019-07-20T17:28:52.9704041Z error: duplicate lang item in crate `core`: `rem_assign`.
2019-07-20T17:28:52.9704315Z   |
2019-07-20T17:28:52.9704664Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9704728Z 
2019-07-20T17:28:52.9705054Z error: duplicate lang item in crate `core`: `bitxor_assign`.
2019-07-20T17:28:52.9705345Z   |
2019-07-20T17:28:52.9705679Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9705722Z 
2019-07-20T17:28:52.9706041Z error: duplicate lang item in crate `core`: `bitand_assign`.
2019-07-20T17:28:52.9706336Z   |
2019-07-20T17:28:52.9706665Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9706708Z 
2019-07-20T17:28:52.9707045Z error: duplicate lang item in crate `core`: `bitor_assign`.
2019-07-20T17:28:52.9707325Z   |
2019-07-20T17:28:52.9707650Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9707707Z 
2019-07-20T17:28:52.9708062Z error: duplicate lang item in crate `core`: `shl_assign`.
2019-07-20T17:28:52.9708338Z   |
2019-07-20T17:28:52.9708666Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9708730Z 
2019-07-20T17:28:52.9709050Z error: duplicate lang item in crate `core`: `shr_assign`.
2019-07-20T17:28:52.9709321Z   |
2019-07-20T17:28:52.9709665Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9709710Z 
2019-07-20T17:28:52.9710026Z error: duplicate lang item in crate `core`: `index`.
2019-07-20T17:28:52.9710301Z   |
2019-07-20T17:28:52.9711260Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9711336Z 
2019-07-20T17:28:52.9711853Z error: duplicate lang item in crate `core`: `index_mut`.
2019-07-20T17:28:52.9712209Z   |
2019-07-20T17:28:52.9712547Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9712591Z 
2019-07-20T17:28:52.9712907Z error: duplicate lang item in crate `core`: `unsafe_cell`.
2019-07-20T17:28:52.9713206Z   |
2019-07-20T17:28:52.9713535Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9713579Z 
2019-07-20T17:28:52.9713909Z error: duplicate lang item in crate `core`: `va_list`.
2019-07-20T17:28:52.9714190Z   |
2019-07-20T17:28:52.9714515Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9714662Z 
2019-07-20T17:28:52.9715112Z error: duplicate lang item in crate `core`: `deref`.
2019-07-20T17:28:52.9715395Z   |
2019-07-20T17:28:52.9715722Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9715786Z 
2019-07-20T17:28:52.9716107Z error: duplicate lang item in crate `core`: `deref_mut`.
2019-07-20T17:28:52.9716381Z   |
2019-07-20T17:28:52.9716725Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9716770Z 
2019-07-20T17:28:52.9717090Z error: duplicate lang item in crate `core`: `receiver`.
2019-07-20T17:28:52.9717364Z   |
2019-07-20T17:28:52.9717716Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9717772Z 
2019-07-20T17:28:52.9718092Z error: duplicate lang item in crate `core`: `fn`.
2019-07-20T17:28:52.9718390Z   |
2019-07-20T17:28:52.9718724Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9718767Z 
2019-07-20T17:28:52.9719077Z error: duplicate lang item in crate `core`: `fn_mut`.
2019-07-20T17:28:52.9719374Z   |
2019-07-20T17:28:52.9719700Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9719743Z 
2019-07-20T17:28:52.9720076Z error: duplicate lang item in crate `core`: `fn_once`.
2019-07-20T17:28:52.9720356Z   |
2019-07-20T17:28:52.9720954Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9721011Z 
2019-07-20T17:28:52.9721413Z error: duplicate lang item in crate `core`: `future_trait`.
2019-07-20T17:28:52.9721720Z   |
2019-07-20T17:28:52.9722052Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9722117Z 
2019-07-20T17:28:52.9722444Z error: duplicate lang item in crate `core`: `generator_state`.
2019-07-20T17:28:52.9722718Z   |
2019-07-20T17:28:52.9723061Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9723106Z 
2019-07-20T17:28:52.9723428Z error: duplicate lang item in crate `core`: `generator`.
2019-07-20T17:28:52.9723703Z   |
2019-07-20T17:28:52.9724051Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9724095Z 
2019-07-20T17:28:52.9724425Z error: duplicate lang item in crate `core`: `unpin`.
2019-07-20T17:28:52.9724913Z   |
2019-07-20T17:28:52.9725301Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9725346Z 
2019-07-20T17:28:52.9725658Z error: duplicate lang item in crate `core`: `pin`.
2019-07-20T17:28:52.9725959Z   |
2019-07-20T17:28:52.9726288Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9726333Z 
2019-07-20T17:28:52.9726658Z error: duplicate lang item in crate `core`: `eq`.
2019-07-20T17:28:52.9726939Z   |
2019-07-20T17:28:52.9727266Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9727309Z 
2019-07-20T17:28:52.9727648Z error: duplicate lang item in crate `core`: `partial_ord`.
2019-07-20T17:28:52.9728051Z   |
2019-07-20T17:28:52.9728392Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9728459Z 
2019-07-20T17:28:52.9728774Z error: duplicate lang item in crate `core`: `ord`.
2019-07-20T17:28:52.9729048Z   |
2019-07-20T17:28:52.9729393Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9729438Z 
2019-07-20T17:28:52.9729753Z error: duplicate lang item in crate `core`: `panic`.
2019-07-20T17:28:52.9730027Z   |
2019-07-20T17:28:52.9730374Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9730418Z 
2019-07-20T17:28:52.9731309Z error: duplicate lang item in crate `core`: `panic_bounds_check`.
2019-07-20T17:28:52.9731714Z   |
2019-07-20T17:28:52.9732063Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9732108Z 
2019-07-20T17:28:52.9732428Z error: duplicate lang item in crate `core`: `panic_info`.
2019-07-20T17:28:52.9732725Z   |
2019-07-20T17:28:52.9733054Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9733098Z 
2019-07-20T17:28:52.9733434Z error: duplicate lang item in crate `core`: `drop_in_place`.
2019-07-20T17:28:52.9733718Z   |
2019-07-20T17:28:52.9734809Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9734869Z 
2019-07-20T17:28:52.9735226Z error: duplicate lang item in crate `core`: `alloc_layout`.
2019-07-20T17:28:52.9735522Z   |
2019-07-20T17:28:52.9735861Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9735928Z 
2019-07-20T17:28:52.9736255Z error: duplicate lang item in crate `core`: `phantom_data`.
2019-07-20T17:28:52.9736527Z   |
2019-07-20T17:28:52.9736870Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9736916Z 
2019-07-20T17:28:52.9737240Z error: duplicate lang item in crate `core`: `manually_drop`.
2019-07-20T17:28:52.9737513Z   |
2019-07-20T17:28:52.9737867Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9737912Z 
2019-07-20T17:28:52.9738230Z error: duplicate lang item in crate `core`: `debug_trait`.
2019-07-20T17:28:52.9738520Z   |
2019-07-20T17:28:52.9739022Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9739083Z 
2019-07-20T17:28:52.9739454Z error: duplicate lang item in crate `core`: `align_offset`.
2019-07-20T17:28:52.9739760Z   |
2019-07-20T17:28:52.9740090Z   = note: first defined in crate `core`.
2019-07-20T17:28:52.9740133Z 
2019-07-20T17:28:53.0529222Z error[E0277]: the size for values of type `std::boxed::Box<(dyn std::io::Write + std::marker::Send + 'static)>` cannot be known at compilation time
2019-07-20T17:28:53.0529680Z   --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/a47e718/src/lib.rs:55:1
2019-07-20T17:28:53.0530304Z 55 | / fn set_print(
2019-07-20T17:28:53.0530304Z 55 | / fn set_print(
2019-07-20T17:28:53.0531352Z 56 | |     sink: Option<Box<dyn Write + Send>>,
2019-07-20T17:28:53.0531745Z 57 | | ) -> Option<Box<dyn Write + Send>> {
2019-07-20T17:28:53.0532099Z 58 | |     #[cfg(feature = "unstable")]
2019-07-20T17:28:53.0532705Z 65 | |     }
2019-07-20T17:28:53.0533016Z 66 | | }
2019-07-20T17:28:53.0533016Z 66 | | }
2019-07-20T17:28:53.0533362Z    | |_^ doesn't have a size known at compile-time
2019-07-20T17:28:53.0533649Z    |
2019-07-20T17:28:53.0534341Z    = help: the trait `std::marker::Sized` is not implemented for `std::boxed::Box<(dyn std::io::Write + std::marker::Send + 'static)>`
2019-07-20T17:28:53.0545745Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T17:28:53.0546172Z 
2019-07-20T17:28:53.0546501Z error[E0038]: the trait `std::io::Write` cannot be made into an object
2019-07-20T17:28:53.0546501Z error[E0038]: the trait `std::io::Write` cannot be made into an object
2019-07-20T17:28:53.0547001Z   --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/a47e718/src/lib.rs:55:1
2019-07-20T17:28:53.0547601Z 55 | / fn set_print(
2019-07-20T17:28:53.0547601Z 55 | / fn set_print(
2019-07-20T17:28:53.0548006Z 56 | |     sink: Option<Box<dyn Write + Send>>,
2019-07-20T17:28:53.0548362Z 57 | | ) -> Option<Box<dyn Write + Send>> {
2019-07-20T17:28:53.0548738Z 58 | |     #[cfg(feature = "unstable")]
2019-07-20T17:28:53.0549324Z 65 | |     }
2019-07-20T17:28:53.0549657Z 66 | | }
2019-07-20T17:28:53.0549657Z 66 | | }
2019-07-20T17:28:53.0550019Z    | |_^ the trait `std::io::Write` cannot be made into an object
2019-07-20T17:28:53.0550515Z    |
2019-07-20T17:28:53.0551234Z    = note: method `by_ref` references the `Self` type in its arguments or return type
2019-07-20T17:28:53.0551293Z 
2019-07-20T17:28:53.0551686Z error[E0277]: the size for values of type `std::boxed::Box<(dyn std::io::Write + std::marker::Send + 'static)>` cannot be known at compilation time
2019-07-20T17:28:53.0552060Z   --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/a47e718/src/lib.rs:68:1
2019-07-20T17:28:53.0552627Z 68 | / fn set_panic(
2019-07-20T17:28:53.0552627Z 68 | / fn set_panic(
2019-07-20T17:28:53.0574662Z 69 | |     sink: Option<Box<dyn Write + Send>>,
2019-07-20T17:28:53.0575516Z 70 | | ) -> Option<Box<dyn Write + Send>> {
2019-07-20T17:28:53.0575948Z 71 | |     #[cfg(feature = "unstable")]
2019-07-20T17:28:53.0576452Z 78 | |     }
2019-07-20T17:28:53.0576733Z 79 | | }
2019-07-20T17:28:53.0576733Z 79 | | }
2019-07-20T17:28:53.0577023Z    | |_^ doesn't have a size known at compile-time
2019-07-20T17:28:53.0577234Z    |
2019-07-20T17:28:53.0577592Z    = help: the trait `std::marker::Sized` is not implemented for `std::boxed::Box<(dyn std::io::Write + std::marker::Send + 'static)>`
2019-07-20T17:28:53.0577949Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T17:28:53.0578274Z 
2019-07-20T17:28:53.0578536Z error[E0038]: the trait `std::io::Write` cannot be made into an object
2019-07-20T17:28:53.0578536Z error[E0038]: the trait `std::io::Write` cannot be made into an object
2019-07-20T17:28:53.0578821Z   --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/a47e718/src/lib.rs:68:1
2019-07-20T17:28:53.0579401Z 68 | / fn set_panic(
2019-07-20T17:28:53.0579401Z 68 | / fn set_panic(
2019-07-20T17:28:53.0579687Z 69 | |     sink: Option<Box<dyn Write + Send>>,
2019-07-20T17:28:53.0579995Z 70 | | ) -> Option<Box<dyn Write + Send>> {
2019-07-20T17:28:53.0580269Z 71 | |     #[cfg(feature = "unstable")]
2019-07-20T17:28:53.0581784Z 78 | |     }
2019-07-20T17:28:53.0582084Z 79 | | }
2019-07-20T17:28:53.0582084Z 79 | | }
2019-07-20T17:28:53.0582431Z    | |_^ the trait `std::io::Write` cannot be made into an object
2019-07-20T17:28:53.0582661Z    |
2019-07-20T17:28:53.0583129Z    = note: method `by_ref` references the `Self` type in its arguments or return type
2019-07-20T17:28:53.0583206Z 
2019-07-20T17:28:53.0583552Z error[E0277]: the size for values of type `TestDescAndFn` cannot be known at compilation time
2019-07-20T17:28:53.0583862Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/a47e718/src/lib.rs:268:1
2019-07-20T17:28:53.0584398Z 268 | / pub fn test_main(
2019-07-20T17:28:53.0584398Z 268 | / pub fn test_main(
2019-07-20T17:28:53.0584698Z 269 | |     args: &[String],
2019-07-20T17:28:53.0585022Z 270 | |     tests: Vec<TestDescAndFn>,
2019-07-20T17:28:53.0585709Z ...   |
2019-07-20T17:28:53.0586007Z 297 | |     }
2019-07-20T17:28:53.0586289Z 298 | | }
2019-07-20T17:28:53.0586289Z 298 | | }
2019-07-20T17:28:53.0586601Z     | |_^ doesn't have a size known at compile-time
2019-07-20T17:28:53.0586843Z     |
2019-07-20T17:28:53.0587149Z     = help: the trait `std::marker::Sized` is not implemented for `TestDescAndFn`
2019-07-20T17:28:53.0587517Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T17:28:53.0587838Z     = note: required by `std::vec::Vec`
2019-07-20T17:28:53.0587880Z 
2019-07-20T17:28:53.0588220Z error[E0277]: the size for values of type `std::result::Result<TestOpts, std::string::String>` cannot be known at compilation time
2019-07-20T17:28:53.0588558Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/a47e718/src/lib.rs:520:1
2019-07-20T17:28:53.0588787Z     |
2019-07-20T17:28:53.0589128Z 520 | / pub fn parse_opts(args: &[String]) -> Option<OptRes> {
2019-07-20T17:28:53.0589448Z 521 | |     let mut allow_unstable = false;
2019-07-20T17:28:53.0589758Z 522 | |     let opts = optgroups();
2019-07-20T17:28:53.0590118Z 523 | |     let args = args.get(1..).unwrap_or(args);
2019-07-20T17:28:53.0590369Z ...   |
2019-07-20T17:28:53.0590897Z 669 | |     Some(Ok(test_opts))
2019-07-20T17:28:53.0591312Z 670 | | }
2019-07-20T17:28:53.0591628Z     | |_^ doesn't have a size known at compile-time
2019-07-20T17:28:53.0591869Z     |
2019-07-20T17:28:53.0592210Z     = help: the trait `std::marker::Sized` is not implemented for `std::result::Result<TestOpts, std::string::String>`
2019-07-20T17:28:53.0592709Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T17:28:53.0593098Z 
2019-07-20T17:28:53.0593098Z 
2019-07-20T17:28:53.0593391Z error[E0277]: the size for values of type `TestOpts` cannot be known at compilation time
2019-07-20T17:28:53.0593711Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/a47e718/src/lib.rs:520:1
2019-07-20T17:28:53.0593936Z     |
2019-07-20T17:28:53.0594260Z 520 | / pub fn parse_opts(args: &[String]) -> Option<OptRes> {
2019-07-20T17:28:53.0594595Z 521 | |     let mut allow_unstable = false;
2019-07-20T17:28:53.0595021Z 522 | |     let opts = optgroups();
2019-07-20T17:28:53.0595351Z 523 | |     let args = args.get(1..).unwrap_or(args);
2019-07-20T17:28:53.0595620Z ...   |
2019-07-20T17:28:53.0595919Z 669 | |     Some(Ok(test_opts))
2019-07-20T17:28:53.0596200Z 670 | | }
2019-07-20T17:28:53.0596531Z     | |_^ doesn't have a size known at compile-time
2019-07-20T17:28:53.0596755Z     |
2019-07-20T17:28:53.0597080Z     = help: the trait `std::marker::Sized` is not implemented for `TestOpts`
2019-07-20T17:28:53.0597503Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T17:28:53.0597871Z 
2019-07-20T17:28:53.0598200Z error[E0277]: the size for values of type `std::string::String` cannot be known at compilation time
2019-07-20T17:28:53.0598200Z error[E0277]: the size for values of type `std::string::String` cannot be known at compilation time
2019-07-20T17:28:53.0598530Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/a47e718/src/lib.rs:520:1
2019-07-20T17:28:53.0598790Z     |
2019-07-20T17:28:53.0599146Z 520 | / pub fn parse_opts(args: &[String]) -> Option<OptRes> {
2019-07-20T17:28:53.0599514Z 521 | |     let mut allow_unstable = false;
2019-07-20T17:28:53.0599863Z 522 | |     let opts = optgroups();
2019-07-20T17:28:53.0600243Z 523 | |     let args = args.get(1..).unwrap_or(args);
2019-07-20T17:28:53.0600514Z ...   |
2019-07-20T17:28:53.0601530Z 669 | |     Some(Ok(test_opts))
2019-07-20T17:28:53.0602206Z 670 | | }
2019-07-20T17:28:53.0602531Z     | |_^ doesn't have a size known at compile-time
2019-07-20T17:28:53.0602799Z     |
2019-07-20T17:28:53.0603229Z     = help: the trait `std::marker::Sized` is not implemented for `std::string::String`
2019-07-20T17:28:53.0603663Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T17:28:53.0603995Z 
2019-07-20T17:28:53.0603995Z 
2019-07-20T17:28:53.0604403Z error[E0277]: the size for values of type `std::fs::File` cannot be known at compilation time
2019-07-20T17:28:53.0604797Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/a47e718/src/lib.rs:713:5
2019-07-20T17:28:53.0604991Z     |
2019-07-20T17:28:53.0605227Z 713 |     log_out: Option<File>,
2019-07-20T17:28:53.0605655Z     |     ^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-07-20T17:28:53.0605856Z     |
2019-07-20T17:28:53.0606140Z     = help: the trait `std::marker::Sized` is not implemented for `std::fs::File`
2019-07-20T17:28:53.0606464Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T17:28:53.0606761Z 
2019-07-20T17:28:53.0606761Z 
2019-07-20T17:28:53.0607035Z error[E0277]: the size for values of type `(TestDesc, std::vec::Vec<u8>)` cannot be known at compilation time
2019-07-20T17:28:53.0607317Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/a47e718/src/lib.rs:722:5
2019-07-20T17:28:53.0607529Z     |
2019-07-20T17:28:53.0607781Z 722 |     failures: Vec<(TestDesc, Vec<u8>)>,
2019-07-20T17:28:53.0608088Z     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-07-20T17:28:53.0608306Z     |
2019-07-20T17:28:53.0608584Z     = help: the trait `std::marker::Sized` is not implemented for `(TestDesc, std::vec::Vec<u8>)`
2019-07-20T17:28:53.0608904Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T17:28:53.0609182Z     = note: required by `std::vec::Vec`
2019-07-20T17:28:53.0609215Z 
2019-07-20T17:28:53.0609466Z error[E0277]: the size for values of type `u8` cannot be known at compilation time
2019-07-20T17:28:53.0609746Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/a47e718/src/lib.rs:722:5
2019-07-20T17:28:53.0609938Z     |
2019-07-20T17:28:53.0610185Z 722 |     failures: Vec<(TestDesc, Vec<u8>)>,
2019-07-20T17:28:53.0610639Z     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-07-20T17:28:53.0611042Z     |
2019-07-20T17:28:53.0611337Z     = help: the trait `std::marker::Sized` is not implemented for `u8`
2019-07-20T17:28:53.0611865Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T17:28:53.0612188Z     = note: required by `std::vec::Vec`
2019-07-20T17:28:53.0612225Z 
2019-07-20T17:28:53.0612540Z error[E0277]: the size for values of type `TestDescAndFn` cannot be known at compilation time
2019-07-20T17:28:53.0612843Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/a47e718/src/lib.rs:833:1
2019-07-20T17:28:53.0613065Z     |
2019-07-20T17:28:53.0613382Z 833 | / pub fn list_tests_console(
2019-07-20T17:28:53.0613684Z 834 | |     opts: &TestOpts,
2019-07-20T17:28:53.0614621Z 835 | |     tests: Vec<TestDescAndFn>,
2019-07-20T17:28:53.0614921Z 836 | | ) -> io::Result<()> {
2019-07-20T17:28:53.0615407Z 887 | |     Ok(())
2019-07-20T17:28:53.0615658Z 888 | | }
2019-07-20T17:28:53.0615658Z 888 | | }
2019-07-20T17:28:53.0615931Z     | |_^ doesn't have a size known at compile-time
2019-07-20T17:28:53.0616141Z     |
2019-07-20T17:28:53.0616410Z     = help: the trait `std::marker::Sized` is not implemented for `TestDescAndFn`
2019-07-20T17:28:53.0616745Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-07-20T17:28:53.0617006Z     = note: required by `std::vec::Vec`
2019-07-20T17:28:53.0617038Z 
2019-07-20T17:28:53.0617288Z error[E0277]: the size for values of type `()` cannot be known at compilation time
2019-07-20T17:28:53.0617569Z    --> /cargo/git/checkouts/libtest-b51d23c1cc3e7dfa/a47e718/src/lib.rs:833:1
2019-07-20T17:28:53.0617764Z     |
2019-07-20T17:28:53.0618026Z 833 | / pub fn list_tests_console(
2019-07-20T17:28:53.0618304Z 834 | |     opts: &TestOpts,
2019-07-20T17:28:53.0618618Z 835 | |     tests: Vec<TestDescAndFn>,
2019-07-20T17:28:53.0618907Z 836 | | ) -> io::Result<()> {
2019-07-20T17:28:53.0619429Z 887 | |     Ok(())
2019-07-20T17:28:53.0619701Z 888 | | }
2019-07-20T17:28:53.0619701Z 888 | | }
2019-07-20T17:28:53.0620014Z     | |_^ doesn't have a size known at compile-time
2019-07-20T17:28:53.0620618Z     = help: the trait `std::marker::Sized` is not implemented for `()`
---
2019-07-20T17:28:53.1649930Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json"
2019-07-20T17:28:53.1650036Z expected success, got: exit code: 101
2019-07-20T17:28:53.1650095Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-20T17:28:53.1650157Z Build completed unsuccessfully in 0:03:05
2019-07-20T17:28:54.6577084Z ##[error]Bash exited with code '1'.
2019-07-20T17:28:54.6611900Z ##[section]Starting: Checkout
2019-07-20T17:28:54.6614321Z ==============================================================================
2019-07-20T17:28:54.6614408Z Task         : Get sources
2019-07-20T17:28:54.6614458Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
