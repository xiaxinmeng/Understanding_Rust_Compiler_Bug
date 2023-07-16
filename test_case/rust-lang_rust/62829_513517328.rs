plain
2019-07-21T03:46:01.2285084Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-21T03:46:01.2453878Z ##[command]git config gc.auto 0
2019-07-21T03:46:01.2526615Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-21T03:46:01.2589333Z ##[command]git config --get-all http.proxy
2019-07-21T03:46:01.2722136Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62829/merge:refs/remotes/pull/62829/merge
---
2019-07-21T03:46:35.5808763Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-21T03:46:35.5810305Z 
2019-07-21T03:46:35.5812111Z   git checkout -b <new-branch-name>
2019-07-21T03:46:35.5813567Z 
2019-07-21T03:46:35.5814178Z HEAD is now at b9774e15e Merge cde1544fc1127d16550f6210b4848d2cc8ab41f5 into 95b1fe560d2bd8472f250fb8cfd2168520a58405
2019-07-21T03:46:35.5944694Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-21T03:46:35.5948615Z ==============================================================================
2019-07-21T03:46:35.5948676Z Task         : Bash
2019-07-21T03:46:35.5948725Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-21T03:49:43.5658361Z #######################################                                   54.6%
2019-07-21T03:49:43.5658480Z ######################################################################## 100.0%
2019-07-21T03:49:43.9885115Z extracting /checkout/obj/build/cache/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-07-21T03:49:45.0497205Z     Updating crates.io index
2019-07-21T03:50:06.3118679Z     Updating git repository `https://github.com/crlf0710/libtest`
2019-07-21T03:50:07.1395666Z     Updating git repository `https://github.com/crlf0710/getopts`
2019-07-21T03:50:07.7445859Z     Updating git repository `https://github.com/crlf0710/termcolor`
2019-07-21T03:50:08.5569692Z     Updating git repository `https://github.com/crlf0710/unicode-width`
---
2019-07-21T03:52:36.6217281Z     Checking cfg-if v0.1.8
2019-07-21T03:52:36.6787973Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-07-21T03:52:40.3240004Z     Checking rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-07-21T03:52:40.4277253Z     Checking hashbrown v0.4.0
2019-07-21T03:52:42.5122710Z error: duplicate lang item in crate `core`: `char`.
2019-07-21T03:52:42.5138566Z   |
2019-07-21T03:52:42.5150257Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5161544Z 
2019-07-21T03:52:42.5163215Z error: duplicate lang item in crate `core`: `str`.
2019-07-21T03:52:42.5164054Z   |
2019-07-21T03:52:42.5164663Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5164917Z 
2019-07-21T03:52:42.5165563Z error: duplicate lang item in crate `core`: `slice`.
2019-07-21T03:52:42.5166484Z   |
2019-07-21T03:52:42.5167181Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5167571Z 
2019-07-21T03:52:42.5168038Z error: duplicate lang item in crate `core`: `slice_u8`.
2019-07-21T03:52:42.5168493Z   |
2019-07-21T03:52:42.5168989Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5169251Z 
2019-07-21T03:52:42.5169708Z error: duplicate lang item in crate `core`: `const_ptr`.
2019-07-21T03:52:42.5170157Z   |
2019-07-21T03:52:42.5170668Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5170891Z 
2019-07-21T03:52:42.5171332Z error: duplicate lang item in crate `core`: `mut_ptr`.
2019-07-21T03:52:42.5172428Z   |
2019-07-21T03:52:42.5172996Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5173252Z 
2019-07-21T03:52:42.5173760Z error: duplicate lang item in crate `core`: `i8`.
2019-07-21T03:52:42.5174250Z   |
2019-07-21T03:52:42.5174790Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5175555Z 
2019-07-21T03:52:42.5176282Z error: duplicate lang item in crate `core`: `i16`.
2019-07-21T03:52:42.5176690Z   |
2019-07-21T03:52:42.5177144Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5177451Z 
2019-07-21T03:52:42.5177842Z error: duplicate lang item in crate `core`: `i32`.
2019-07-21T03:52:42.5178248Z   |
2019-07-21T03:52:42.5178649Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5178793Z 
2019-07-21T03:52:42.5179185Z error: duplicate lang item in crate `core`: `i64`.
2019-07-21T03:52:42.5179547Z   |
2019-07-21T03:52:42.5179953Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5180116Z 
2019-07-21T03:52:42.5180481Z error: duplicate lang item in crate `core`: `i128`.
2019-07-21T03:52:42.5180836Z   |
2019-07-21T03:52:42.5181260Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5181925Z 
2019-07-21T03:52:42.5182408Z error: duplicate lang item in crate `core`: `isize`.
2019-07-21T03:52:42.5182826Z   |
2019-07-21T03:52:42.5183277Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5183433Z 
2019-07-21T03:52:42.5184006Z error: duplicate lang item in crate `core`: `u8`.
2019-07-21T03:52:42.5184458Z   |
2019-07-21T03:52:42.5184895Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5185071Z 
2019-07-21T03:52:42.5185762Z error: duplicate lang item in crate `core`: `u16`.
2019-07-21T03:52:42.5186107Z   |
2019-07-21T03:52:42.5186522Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5186659Z 
2019-07-21T03:52:42.5187011Z error: duplicate lang item in crate `core`: `u32`.
2019-07-21T03:52:42.5187374Z   |
2019-07-21T03:52:42.5187912Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5188057Z 
2019-07-21T03:52:42.5188436Z error: duplicate lang item in crate `core`: `u64`.
2019-07-21T03:52:42.5188790Z   |
2019-07-21T03:52:42.5189208Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5189348Z 
2019-07-21T03:52:42.5190533Z error: duplicate lang item in crate `core`: `u128`.
2019-07-21T03:52:42.5191059Z   |
2019-07-21T03:52:42.5192013Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5192769Z 
2019-07-21T03:52:42.5193288Z error: duplicate lang item in crate `core`: `usize`.
2019-07-21T03:52:42.5193688Z   |
2019-07-21T03:52:42.5194669Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5194937Z 
2019-07-21T03:52:42.5195695Z error: duplicate lang item in crate `core`: `f32`.
2019-07-21T03:52:42.5196074Z   |
2019-07-21T03:52:42.5196706Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5196851Z 
2019-07-21T03:52:42.5197207Z error: duplicate lang item in crate `core`: `f64`.
2019-07-21T03:52:42.5197576Z   |
2019-07-21T03:52:42.5197969Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5198107Z 
2019-07-21T03:52:42.5198477Z error: duplicate lang item in crate `core`: `sized`.
2019-07-21T03:52:42.5198828Z   |
2019-07-21T03:52:42.5199248Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5199415Z 
2019-07-21T03:52:42.5199800Z error: duplicate lang item in crate `core`: `unsize`.
2019-07-21T03:52:42.5200163Z   |
2019-07-21T03:52:42.5200598Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5200747Z 
2019-07-21T03:52:42.5201110Z error: duplicate lang item in crate `core`: `copy`.
2019-07-21T03:52:42.5201856Z   |
2019-07-21T03:52:42.5202375Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5202542Z 
2019-07-21T03:52:42.5202939Z error: duplicate lang item in crate `core`: `clone`.
2019-07-21T03:52:42.5203329Z   |
2019-07-21T03:52:42.5203981Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5204478Z 
2019-07-21T03:52:42.5204949Z error: duplicate lang item in crate `core`: `sync`.
2019-07-21T03:52:42.5205477Z   |
2019-07-21T03:52:42.5206046Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5206231Z 
2019-07-21T03:52:42.5206795Z error: duplicate lang item in crate `core`: `freeze`.
2019-07-21T03:52:42.5207516Z   |
2019-07-21T03:52:42.5208313Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5208486Z 
2019-07-21T03:52:42.5208859Z error: duplicate lang item in crate `core`: `drop`.
2019-07-21T03:52:42.5209226Z   |
2019-07-21T03:52:42.5209634Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5209783Z 
2019-07-21T03:52:42.5210180Z error: duplicate lang item in crate `core`: `coerce_unsized`.
2019-07-21T03:52:42.5210915Z   |
2019-07-21T03:52:42.5212202Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5212415Z 
2019-07-21T03:52:42.5212857Z error: duplicate lang item in crate `core`: `dispatch_from_dyn`.
2019-07-21T03:52:42.5213273Z   |
2019-07-21T03:52:42.5216235Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5216276Z 
2019-07-21T03:52:42.5216536Z error: duplicate lang item in crate `core`: `add`.
2019-07-21T03:52:42.5216740Z   |
2019-07-21T03:52:42.5216985Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5217032Z 
2019-07-21T03:52:42.5217268Z error: duplicate lang item in crate `core`: `sub`.
2019-07-21T03:52:42.5217665Z   |
2019-07-21T03:52:42.5217920Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5217978Z 
2019-07-21T03:52:42.5218223Z error: duplicate lang item in crate `core`: `mul`.
2019-07-21T03:52:42.5218433Z   |
2019-07-21T03:52:42.5218706Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5218738Z 
2019-07-21T03:52:42.5218980Z error: duplicate lang item in crate `core`: `div`.
2019-07-21T03:52:42.5219204Z   |
2019-07-21T03:52:42.5219457Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5219491Z 
2019-07-21T03:52:42.5219732Z error: duplicate lang item in crate `core`: `rem`.
2019-07-21T03:52:42.5219972Z   |
2019-07-21T03:52:42.5220226Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5220258Z 
2019-07-21T03:52:42.5220525Z error: duplicate lang item in crate `core`: `neg`.
2019-07-21T03:52:42.5220740Z   |
2019-07-21T03:52:42.5220993Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5221025Z 
2019-07-21T03:52:42.5221284Z error: duplicate lang item in crate `core`: `not`.
2019-07-21T03:52:42.5221924Z   |
2019-07-21T03:52:42.5222221Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5222276Z 
2019-07-21T03:52:42.5222552Z error: duplicate lang item in crate `core`: `bitxor`.
2019-07-21T03:52:42.5222784Z   |
2019-07-21T03:52:42.5223080Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5223136Z 
2019-07-21T03:52:42.5223534Z error: duplicate lang item in crate `core`: `bitand`.
2019-07-21T03:52:42.5223810Z   |
2019-07-21T03:52:42.5224110Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5224146Z 
2019-07-21T03:52:42.5224417Z error: duplicate lang item in crate `core`: `bitor`.
2019-07-21T03:52:42.5224647Z   |
2019-07-21T03:52:42.5224946Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5224983Z 
2019-07-21T03:52:42.5225407Z error: duplicate lang item in crate `core`: `shl`.
2019-07-21T03:52:42.5225636Z   |
2019-07-21T03:52:42.5225992Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5226025Z 
2019-07-21T03:52:42.5226278Z error: duplicate lang item in crate `core`: `shr`.
2019-07-21T03:52:42.5226510Z   |
2019-07-21T03:52:42.5226765Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5226798Z 
2019-07-21T03:52:42.5227061Z error: duplicate lang item in crate `core`: `add_assign`.
2019-07-21T03:52:42.5227430Z   |
2019-07-21T03:52:42.5227675Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5227722Z 
2019-07-21T03:52:42.5227962Z error: duplicate lang item in crate `core`: `sub_assign`.
2019-07-21T03:52:42.5228163Z   |
2019-07-21T03:52:42.5228408Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5228465Z 
2019-07-21T03:52:42.5228708Z error: duplicate lang item in crate `core`: `mul_assign`.
2019-07-21T03:52:42.5228919Z   |
2019-07-21T03:52:42.5229183Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5229215Z 
2019-07-21T03:52:42.5229454Z error: duplicate lang item in crate `core`: `div_assign`.
2019-07-21T03:52:42.5229656Z   |
2019-07-21T03:52:42.5229917Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5229949Z 
2019-07-21T03:52:42.5230187Z error: duplicate lang item in crate `core`: `rem_assign`.
2019-07-21T03:52:42.5230409Z   |
2019-07-21T03:52:42.5230653Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5230694Z 
2019-07-21T03:52:42.5230939Z error: duplicate lang item in crate `core`: `bitxor_assign`.
2019-07-21T03:52:42.5231166Z   |
2019-07-21T03:52:42.5231975Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5232032Z 
2019-07-21T03:52:42.5232371Z error: duplicate lang item in crate `core`: `bitand_assign`.
2019-07-21T03:52:42.5232606Z   |
2019-07-21T03:52:42.5232887Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5232939Z 
2019-07-21T03:52:42.5233219Z error: duplicate lang item in crate `core`: `bitor_assign`.
2019-07-21T03:52:42.5233451Z   |
2019-07-21T03:52:42.5233732Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5233799Z 
2019-07-21T03:52:42.5234078Z error: duplicate lang item in crate `core`: `shl_assign`.
2019-07-21T03:52:42.5234406Z   |
2019-07-21T03:52:42.5234726Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5234784Z 
2019-07-21T03:52:42.5235062Z error: duplicate lang item in crate `core`: `shr_assign`.
2019-07-21T03:52:42.5235655Z   |
2019-07-21T03:52:42.5235933Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5235966Z 
2019-07-21T03:52:42.5236211Z error: duplicate lang item in crate `core`: `index`.
2019-07-21T03:52:42.5236419Z   |
2019-07-21T03:52:42.5237528Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5237696Z 
2019-07-21T03:52:42.5237972Z error: duplicate lang item in crate `core`: `index_mut`.
2019-07-21T03:52:42.5238198Z   |
2019-07-21T03:52:42.5238456Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5238659Z 
2019-07-21T03:52:42.5238912Z error: duplicate lang item in crate `core`: `unsafe_cell`.
2019-07-21T03:52:42.5239141Z   |
2019-07-21T03:52:42.5239394Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5239427Z 
2019-07-21T03:52:42.5239842Z error: duplicate lang item in crate `core`: `va_list`.
2019-07-21T03:52:42.5240045Z   |
2019-07-21T03:52:42.5240289Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5240320Z 
2019-07-21T03:52:42.5240585Z error: duplicate lang item in crate `core`: `deref`.
2019-07-21T03:52:42.5240787Z   |
2019-07-21T03:52:42.5241040Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5241088Z 
2019-07-21T03:52:42.5241331Z error: duplicate lang item in crate `core`: `deref_mut`.
2019-07-21T03:52:42.5241926Z   |
2019-07-21T03:52:42.5242236Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5242273Z 
2019-07-21T03:52:42.5242550Z error: duplicate lang item in crate `core`: `receiver`.
2019-07-21T03:52:42.5242782Z   |
2019-07-21T03:52:42.5243079Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5243115Z 
2019-07-21T03:52:42.5243383Z error: duplicate lang item in crate `core`: `fn`.
2019-07-21T03:52:42.5244188Z   |
2019-07-21T03:52:42.5244501Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5244539Z 
2019-07-21T03:52:42.5244814Z error: duplicate lang item in crate `core`: `fn_mut`.
2019-07-21T03:52:42.5245206Z   |
2019-07-21T03:52:42.5245450Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5245482Z 
2019-07-21T03:52:42.5245734Z error: duplicate lang item in crate `core`: `fn_once`.
2019-07-21T03:52:42.5245938Z   |
2019-07-21T03:52:42.5246358Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5246391Z 
2019-07-21T03:52:42.5246662Z error: duplicate lang item in crate `core`: `future_trait`.
2019-07-21T03:52:42.5246881Z   |
2019-07-21T03:52:42.5248244Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5248438Z 
2019-07-21T03:52:42.5248775Z error: duplicate lang item in crate `core`: `generator_state`.
2019-07-21T03:52:42.5248981Z   |
2019-07-21T03:52:42.5249245Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5249276Z 
2019-07-21T03:52:42.5249515Z error: duplicate lang item in crate `core`: `generator`.
2019-07-21T03:52:42.5249716Z   |
2019-07-21T03:52:42.5249978Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5250010Z 
2019-07-21T03:52:42.5250247Z error: duplicate lang item in crate `core`: `unpin`.
2019-07-21T03:52:42.5250573Z   |
2019-07-21T03:52:42.5250823Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5250854Z 
2019-07-21T03:52:42.5251098Z error: duplicate lang item in crate `core`: `pin`.
2019-07-21T03:52:42.5251320Z   |
2019-07-21T03:52:42.5251988Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5252030Z 
2019-07-21T03:52:42.5252325Z error: duplicate lang item in crate `core`: `eq`.
2019-07-21T03:52:42.5252560Z   |
2019-07-21T03:52:42.5252842Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5252880Z 
2019-07-21T03:52:42.5253173Z error: duplicate lang item in crate `core`: `partial_ord`.
2019-07-21T03:52:42.5253422Z   |
2019-07-21T03:52:42.5253705Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5253758Z 
2019-07-21T03:52:42.5254037Z error: duplicate lang item in crate `core`: `ord`.
2019-07-21T03:52:42.5254275Z   |
2019-07-21T03:52:42.5254574Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5254610Z 
2019-07-21T03:52:42.5254883Z error: duplicate lang item in crate `core`: `panic`.
2019-07-21T03:52:42.5255270Z   |
2019-07-21T03:52:42.5255533Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5255565Z 
2019-07-21T03:52:42.5255811Z error: duplicate lang item in crate `core`: `panic_bounds_check`.
2019-07-21T03:52:42.5256032Z   |
2019-07-21T03:52:42.5256289Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5256321Z 
2019-07-21T03:52:42.5256568Z error: duplicate lang item in crate `core`: `panic_info`.
2019-07-21T03:52:42.5256791Z   |
2019-07-21T03:52:42.5257035Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5257066Z 
2019-07-21T03:52:42.5257323Z error: duplicate lang item in crate `core`: `drop_in_place`.
2019-07-21T03:52:42.5257527Z   |
2019-07-21T03:52:42.5257774Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5257806Z 
2019-07-21T03:52:42.5258064Z error: duplicate lang item in crate `core`: `alloc_layout`.
2019-07-21T03:52:42.5258266Z   |
2019-07-21T03:52:42.5258522Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5258569Z 
2019-07-21T03:52:42.5258901Z error: duplicate lang item in crate `core`: `phantom_data`.
2019-07-21T03:52:42.5259134Z   |
2019-07-21T03:52:42.5259399Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5259431Z 
2019-07-21T03:52:42.5259674Z error: duplicate lang item in crate `core`: `manually_drop`.
2019-07-21T03:52:42.5259875Z   |
2019-07-21T03:52:42.5260136Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5260168Z 
2019-07-21T03:52:42.5260408Z error: duplicate lang item in crate `core`: `debug_trait`.
2019-07-21T03:52:42.5260628Z   |
2019-07-21T03:52:42.5260971Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5261004Z 
2019-07-21T03:52:42.5261253Z error: duplicate lang item in crate `core`: `align_offset`.
2019-07-21T03:52:42.5261836Z   |
2019-07-21T03:52:42.5262145Z   = note: first defined in crate `core`.
2019-07-21T03:52:42.5262183Z 
2019-07-21T03:52:42.5262491Z error: duplicate lang item in crate `compiler_builtins`: `i128_add`.
2019-07-21T03:52:42.5262729Z   |
2019-07-21T03:52:42.5263023Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5263087Z 
2019-07-21T03:52:42.5263388Z error: duplicate lang item in crate `compiler_builtins`: `u128_add`.
2019-07-21T03:52:42.5263623Z   |
2019-07-21T03:52:42.5263947Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5263986Z 
2019-07-21T03:52:42.5264283Z error: duplicate lang item in crate `compiler_builtins`: `i128_sub`.
2019-07-21T03:52:42.5264519Z   |
2019-07-21T03:52:42.5264831Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5264870Z 
2019-07-21T03:52:42.5265462Z error: duplicate lang item in crate `compiler_builtins`: `u128_sub`.
2019-07-21T03:52:42.5265687Z   |
2019-07-21T03:52:42.5265952Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5265985Z 
2019-07-21T03:52:42.5266398Z error: duplicate lang item in crate `compiler_builtins`: `i128_mul`.
2019-07-21T03:52:42.5266630Z   |
2019-07-21T03:52:42.5266886Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5266919Z 
2019-07-21T03:52:42.5267804Z error: duplicate lang item in crate `compiler_builtins`: `u128_mul`.
2019-07-21T03:52:42.5268021Z   |
2019-07-21T03:52:42.5268283Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5268318Z 
2019-07-21T03:52:42.5268593Z error: duplicate lang item in crate `compiler_builtins`: `i128_div`.
2019-07-21T03:52:42.5268803Z   |
2019-07-21T03:52:42.5269067Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5269123Z 
2019-07-21T03:52:42.5269383Z error: duplicate lang item in crate `compiler_builtins`: `u128_div`.
2019-07-21T03:52:42.5269605Z   |
2019-07-21T03:52:42.5270245Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5270373Z 
2019-07-21T03:52:42.5270843Z error: duplicate lang item in crate `compiler_builtins`: `i128_rem`.
2019-07-21T03:52:42.5271407Z   |
2019-07-21T03:52:42.5272169Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5272210Z 
2019-07-21T03:52:42.5272501Z error: duplicate lang item in crate `compiler_builtins`: `u128_rem`.
2019-07-21T03:52:42.5272754Z   |
2019-07-21T03:52:42.5273048Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5273086Z 
2019-07-21T03:52:42.5273389Z error: duplicate lang item in crate `compiler_builtins`: `i128_shl`.
2019-07-21T03:52:42.5273752Z   |
2019-07-21T03:52:42.5274057Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5274096Z 
2019-07-21T03:52:42.5274405Z error: duplicate lang item in crate `compiler_builtins`: `u128_shl`.
2019-07-21T03:52:42.5274638Z   |
2019-07-21T03:52:42.5274931Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5274986Z 
2019-07-21T03:52:42.5275574Z error: duplicate lang item in crate `compiler_builtins`: `i128_shr`.
2019-07-21T03:52:42.5275784Z   |
2019-07-21T03:52:42.5276062Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5276097Z 
2019-07-21T03:52:42.5276521Z error: duplicate lang item in crate `compiler_builtins`: `u128_shr`.
2019-07-21T03:52:42.5276724Z   |
2019-07-21T03:52:42.5277006Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5277041Z 
2019-07-21T03:52:42.5277496Z error: duplicate lang item in crate `compiler_builtins`: `i128_addo`.
2019-07-21T03:52:42.5277723Z   |
2019-07-21T03:52:42.5277990Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5278025Z 
2019-07-21T03:52:42.5278283Z error: duplicate lang item in crate `compiler_builtins`: `u128_addo`.
2019-07-21T03:52:42.5278509Z   |
2019-07-21T03:52:42.5278771Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5278813Z 
2019-07-21T03:52:42.5279092Z error: duplicate lang item in crate `compiler_builtins`: `i128_subo`.
2019-07-21T03:52:42.5279301Z   |
2019-07-21T03:52:42.5279573Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5279624Z 
2019-07-21T03:52:42.5279887Z error: duplicate lang item in crate `compiler_builtins`: `u128_subo`.
2019-07-21T03:52:42.5280097Z   |
2019-07-21T03:52:42.5280374Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5280408Z 
2019-07-21T03:52:42.5280668Z error: duplicate lang item in crate `compiler_builtins`: `i128_mulo`.
2019-07-21T03:52:42.5280878Z   |
2019-07-21T03:52:42.5281158Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5281201Z 
2019-07-21T03:52:42.5281461Z error: duplicate lang item in crate `compiler_builtins`: `u128_mulo`.
2019-07-21T03:52:42.5282272Z   |
2019-07-21T03:52:42.5282600Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5282640Z 
2019-07-21T03:52:42.5282930Z error: duplicate lang item in crate `compiler_builtins`: `i128_shlo`.
2019-07-21T03:52:42.5283184Z   |
2019-07-21T03:52:42.5283625Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5283668Z 
2019-07-21T03:52:42.5284001Z error: duplicate lang item in crate `compiler_builtins`: `u128_shlo`.
2019-07-21T03:52:42.5284236Z   |
2019-07-21T03:52:42.5284663Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5284701Z 
2019-07-21T03:52:42.5285018Z error: duplicate lang item in crate `compiler_builtins`: `i128_shro`.
2019-07-21T03:52:42.5285512Z   |
2019-07-21T03:52:42.5285776Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5285827Z 
2019-07-21T03:52:42.5286087Z error: duplicate lang item in crate `compiler_builtins`: `u128_shro`.
2019-07-21T03:52:42.5286515Z   |
2019-07-21T03:52:42.5286803Z   = note: first defined in crate `compiler_builtins`.
2019-07-21T03:52:42.5286839Z 
2019-07-21T03:52:42.5287097Z error: duplicate lang item in crate `alloc`: `str_alloc`.
2019-07-21T03:52:42.5287490Z   |
2019-07-21T03:52:42.5287773Z   = note: first defined in crate `alloc`.
2019-07-21T03:52:42.5287806Z 
2019-07-21T03:52:42.5288063Z error: duplicate lang item in crate `alloc`: `slice_alloc`.
2019-07-21T03:52:42.5288293Z   |
2019-07-21T03:52:42.5288545Z   = note: first defined in crate `alloc`.
2019-07-21T03:52:42.5288579Z 
2019-07-21T03:52:42.5288830Z error: duplicate lang item in crate `alloc`: `slice_u8_alloc`.
2019-07-21T03:52:42.5289057Z   |
2019-07-21T03:52:42.5289310Z   = note: first defined in crate `alloc`.
2019-07-21T03:52:42.5289344Z 
2019-07-21T03:52:42.5289613Z error: duplicate lang item in crate `alloc`: `exchange_malloc`.
2019-07-21T03:52:42.5289824Z   |
2019-07-21T03:52:42.5290088Z   = note: first defined in crate `alloc`.
2019-07-21T03:52:42.5290137Z 
2019-07-21T03:52:42.5290392Z error: duplicate lang item in crate `alloc`: `box_free`.
2019-07-21T03:52:42.5290604Z   |
2019-07-21T03:52:42.5290856Z   = note: first defined in crate `alloc`.
2019-07-21T03:52:42.5290905Z 
2019-07-21T03:52:42.5291153Z error: duplicate lang item in crate `alloc`: `owned_box`.
2019-07-21T03:52:42.5291361Z   |
2019-07-21T03:52:42.5292023Z   = note: first defined in crate `alloc`.
2019-07-21T03:52:42.5292064Z 
2019-07-21T03:52:42.5292340Z error: duplicate lang item in crate `alloc`: `arc`.
2019-07-21T03:52:42.5292590Z   |
2019-07-21T03:52:42.5292889Z   = note: first defined in crate `alloc`.
2019-07-21T03:52:42.5292926Z 
2019-07-21T03:52:42.5293195Z error: duplicate lang item in crate `alloc`: `rc`.
2019-07-21T03:52:42.5293546Z   |
2019-07-21T03:52:42.5293862Z   = note: first defined in crate `alloc`.
2019-07-21T03:52:42.5293900Z 
2019-07-21T03:52:42.5833875Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-07-21T03:52:42.5834532Z     |
2019-07-21T03:52:42.5834532Z     |
2019-07-21T03:52:42.5834824Z 111 | pub struct Mutex<T: ?Sized> {
2019-07-21T03:52:42.5835366Z 
2019-07-21T03:52:42.5835366Z 
2019-07-21T03:52:42.5853097Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-07-21T03:52:42.5853747Z     |
2019-07-21T03:52:42.5853747Z     |
2019-07-21T03:52:42.5854065Z 145 | pub struct MutexGuard<'a, T: ?Sized + 'a> {
2019-07-21T03:52:42.5854419Z 
2019-07-21T03:52:42.5854419Z 
2019-07-21T03:52:42.5864257Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-07-21T03:52:42.5873817Z    |
2019-07-21T03:52:42.5873817Z    |
2019-07-21T03:52:42.5878829Z 67 | pub struct RwLock<T: ?Sized> {
2019-07-21T03:52:42.5880250Z 
2019-07-21T03:52:42.5880250Z 
2019-07-21T03:52:42.5928340Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-07-21T03:52:42.5928683Z     --> src/libstd/collections/hash/map.rs:1026:9
2019-07-21T03:52:42.5928956Z      |
2019-07-21T03:52:42.5929275Z 1026 | impl<K, Q: ?Sized, V, S> Index<&Q> for HashMap<K, V, S>
2019-07-21T03:52:42.5929582Z 
2019-07-21T03:52:42.5929582Z 
2019-07-21T03:52:42.5954694Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-07-21T03:52:42.5955230Z    --> src/libstd/collections/hash/map.rs:697:16
2019-07-21T03:52:42.5955514Z     |
2019-07-21T03:52:42.5955826Z 697 |     pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
2019-07-21T03:52:42.5956156Z 
2019-07-21T03:52:42.5956156Z 
2019-07-21T03:52:42.5956503Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-07-21T03:52:42.5956789Z    --> src/libstd/collections/hash/map.rs:727:26
2019-07-21T03:52:42.5957038Z     |
2019-07-21T03:52:42.5957381Z 727 |     pub fn get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &V)>
2019-07-21T03:52:42.5957928Z 
2019-07-21T03:52:42.5957928Z 
2019-07-21T03:52:42.5958318Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-07-21T03:52:42.5958607Z    --> src/libstd/collections/hash/map.rs:756:25
2019-07-21T03:52:42.5958853Z     |
2019-07-21T03:52:42.5959163Z 756 |     pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
2019-07-21T03:52:42.5959481Z 
2019-07-21T03:52:42.5959481Z 
2019-07-21T03:52:42.5959970Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-07-21T03:52:42.5960269Z    --> src/libstd/collections/hash/map.rs:787:20
2019-07-21T03:52:42.5960673Z     |
2019-07-21T03:52:42.5960999Z 787 |     pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
2019-07-21T03:52:42.5961302Z 
2019-07-21T03:52:42.5961302Z 
2019-07-21T03:52:42.5962105Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-07-21T03:52:42.5962415Z    --> src/libstd/collections/hash/map.rs:848:19
2019-07-21T03:52:42.5962655Z     |
2019-07-21T03:52:42.5963014Z 848 |     pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
2019-07-21T03:52:42.5963350Z 
2019-07-21T03:52:42.5963350Z 
2019-07-21T03:52:42.5963933Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-07-21T03:52:42.5964242Z    --> src/libstd/collections/hash/map.rs:880:25
