plain
2019-10-28T10:05:51.9715302Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-28T10:05:51.9892407Z ##[command]git config gc.auto 0
2019-10-28T10:05:51.9961309Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-28T10:05:52.0033001Z ##[command]git config --get-all http.proxy
2019-10-28T10:05:52.0172131Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65891/merge:refs/remotes/pull/65891/merge
---
2019-10-28T10:33:30.7130733Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-10-28T10:33:31.1994248Z    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
2019-10-28T10:33:31.6774160Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-10-28T10:33:32.1676278Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-10-28T10:33:33.7140419Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:37.8099152Z    Compiling rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2019-10-28T10:33:37.8392195Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:39.7942594Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:41.5033810Z    Compiling alloc v0.0.0 (/checkout/src/liballoc)
2019-10-28T10:33:41.5864674Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:41.8110862Z    Compiling cfg-if v0.1.8
2019-10-28T10:33:41.8419828Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:41.8492892Z    Compiling rustc-demangle v0.1.16
2019-10-28T10:33:42.1698143Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:43.4508762Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:43.4602104Z    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-10-28T10:33:43.5001560Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:43.5774226Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:43.5982854Z    Compiling backtrace v0.3.37
2019-10-28T10:33:43.8088071Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:44.2324532Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:44.4914769Z    Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-10-28T10:33:44.5195221Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:44.5569534Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:44.5935556Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:44.6482137Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:44.6841960Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:44.6955281Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-10-28T10:33:44.8271348Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:45.6734093Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:33:50.6917591Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:03.2331594Z    Compiling term v0.0.0 (/checkout/src/libterm)
2019-10-28T10:34:03.2331594Z    Compiling term v0.0.0 (/checkout/src/libterm)
2019-10-28T10:34:03.2738262Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:03.2933947Z    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
2019-10-28T10:34:03.5725403Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:06.1420560Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:08.1122837Z    Compiling unicode-width v0.1.6
2019-10-28T10:34:08.1921947Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:08.2128756Z    Compiling getopts v0.2.21
2019-10-28T10:34:08.4977824Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:16.3909012Z    Compiling test v0.0.0 (/checkout/src/libtest)
2019-10-28T10:34:17.2020670Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:27.7349649Z Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-28T10:34:27.7366057Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-28T10:34:28.2434382Z    Compiling libc v0.2.64
2019-10-28T10:34:28.2434659Z    Compiling cfg-if v0.1.8
2019-10-28T10:34:28.2434659Z    Compiling cfg-if v0.1.8
2019-10-28T10:34:29.1481559Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:29.1548930Z    Compiling semver-parser v0.7.0
2019-10-28T10:34:29.1549010Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:29.2694120Z    Compiling lazy_static v1.3.0
2019-10-28T10:34:29.3402222Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:29.3655309Z    Compiling scopeguard v1.0.0
2019-10-28T10:34:29.4231117Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:30.4744365Z    Compiling log v0.4.8
2019-10-28T10:34:30.7965905Z    Compiling smallvec v0.6.10
2019-10-28T10:34:31.0656004Z    Compiling nodrop v0.1.12
2019-10-28T10:34:31.0656004Z    Compiling nodrop v0.1.12
2019-10-28T10:34:31.6494304Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:31.6495540Z    Compiling proc-macro2 v1.0.3
2019-10-28T10:34:31.6495765Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:31.6496511Z    Compiling unicode-xid v0.2.0
2019-10-28T10:34:31.6496511Z    Compiling unicode-xid v0.2.0
2019-10-28T10:34:31.6496688Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:31.6496992Z    Compiling indexmap v1.0.2
2019-10-28T10:34:32.1895107Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:32.3790581Z    Compiling either v1.5.0
2019-10-28T10:34:32.3790581Z    Compiling either v1.5.0
2019-10-28T10:34:32.5478188Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:32.5829659Z    Compiling stable_deref_trait v1.1.0
2019-10-28T10:34:32.6363740Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:32.8164637Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:32.8505721Z    Compiling graphviz v0.0.0 (/checkout/src/libgraphviz)
2019-10-28T10:34:33.0007227Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:33.7512241Z    Compiling scoped-tls v1.0.0
2019-10-28T10:34:33.7512241Z    Compiling scoped-tls v1.0.0
2019-10-28T10:34:33.8203350Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:33.8900633Z    Compiling unicode-width v0.1.6
2019-10-28T10:34:33.9413360Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:33.9682936Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:33.9861497Z    Compiling bitflags v1.1.0
2019-10-28T10:34:34.9096627Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:35.1368558Z    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
2019-10-28T10:34:35.3084322Z    Compiling termcolor v1.0.4
2019-10-28T10:34:35.3084322Z    Compiling termcolor v1.0.4
2019-10-28T10:34:35.5909343Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:36.9095960Z    Compiling annotate-snippets v0.6.1
2019-10-28T10:34:37.2248181Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:40.7325317Z    Compiling lazy_static v0.2.11
2019-10-28T10:34:40.8011227Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:41.0710867Z    Compiling datafrog v2.0.1
2019-10-28T10:34:41.0710867Z    Compiling datafrog v2.0.1
2019-10-28T10:34:41.5774777Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:41.6964152Z    Compiling rustc-demangle v0.1.16
2019-10-28T10:34:42.0190493Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:42.4751520Z    Compiling rustc_fs_util v0.0.0 (/checkout/src/librustc_fs_util)
2019-10-28T10:34:42.5522417Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:43.5825597Z    Compiling rustc_metadata v0.0.0 (/checkout/src/librustc_metadata)
2019-10-28T10:34:43.7653799Z    Compiling getrandom v0.1.12
2019-10-28T10:34:43.8501932Z    Compiling ppv-lite86 v0.2.5
2019-10-28T10:34:44.2376803Z    Compiling serde v1.0.99
2019-10-28T10:34:44.2376803Z    Compiling serde v1.0.99
2019-10-28T10:34:44.9342147Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:45.2108465Z    Compiling rustc_incremental v0.0.0 (/checkout/src/librustc_incremental)
2019-10-28T10:34:45.3924845Z    Compiling punycode v0.4.0
2019-10-28T10:34:45.3924845Z    Compiling punycode v0.4.0
2019-10-28T10:34:45.5001653Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:45.9531885Z    Compiling remove_dir_all v0.5.2
2019-10-28T10:34:45.9964975Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:46.3440071Z    Compiling itoa v0.4.4
2019-10-28T10:34:46.5185202Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:46.5264132Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:46.7285609Z    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
2019-10-28T10:34:46.9090555Z    Compiling once_cell v1.1.0
2019-10-28T10:34:46.9090555Z    Compiling once_cell v1.1.0
2019-10-28T10:34:47.0686050Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:47.2750229Z    Compiling semver v0.9.0
2019-10-28T10:34:47.5844511Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:49.1424802Z    Compiling crossbeam-utils v0.6.5
2019-10-28T10:34:49.5655077Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:50.9595587Z    Compiling log_settings v0.1.2
2019-10-28T10:34:51.0252690Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:51.2075237Z    Compiling lock_api v0.3.1
2019-10-28T10:34:51.5233592Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:51.6002661Z    Compiling arrayvec v0.4.7
2019-10-28T10:34:52.0672766Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:52.1382341Z    Compiling serialize v0.0.0 (/checkout/src/libserialize)
2019-10-28T10:34:53.9369750Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:54.7307383Z    Compiling itertools v0.8.0
2019-10-28T10:34:56.2003676Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:56.4535368Z    Compiling rustc_lexer v0.1.0 (/checkout/src/librustc_lexer)
2019-10-28T10:34:56.7664814Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:58.2202265Z    Compiling chalk-macros v0.1.0
2019-10-28T10:34:58.3276607Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:34:59.1232875Z    Compiling miniz-sys v0.1.11
2019-10-28T10:35:00.1554768Z    Compiling c2-chacha v0.2.2
2019-10-28T10:35:00.2332637Z    Compiling rustc_version v0.2.3
2019-10-28T10:35:00.2332637Z    Compiling rustc_version v0.2.3
2019-10-28T10:35:00.4209502Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:00.7365413Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:02.7771647Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:03.0751818Z    Compiling crossbeam-queue v0.1.2
2019-10-28T10:35:03.2433411Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:03.5087407Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:03.8707831Z    Compiling rustc_index v0.0.0 (/checkout/src/librustc_index)
2019-10-28T10:35:04.0510337Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:04.3871386Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:04.4795120Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:04.7769308Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:05.6387370Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:10.0092904Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:10.7879120Z    Compiling parking_lot_core v0.6.2
2019-10-28T10:35:11.1631077Z    Compiling parking_lot v0.9.0
2019-10-28T10:35:11.5383121Z    Compiling num_cpus v1.8.0
2019-10-28T10:35:11.5383121Z    Compiling num_cpus v1.8.0
2019-10-28T10:35:11.6269444Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:12.9676381Z    Compiling atty v0.2.11
2019-10-28T10:35:13.0183822Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:13.0508845Z    Compiling term_size v0.3.1
2019-10-28T10:35:13.1206225Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:13.1680596Z    Compiling memmap v0.6.2
2019-10-28T10:35:13.3078821Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:13.8139298Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:13.9555950Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:14.2059235Z    Compiling ena v0.13.1
2019-10-28T10:35:14.5176176Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:14.5827150Z    Compiling jobserver v0.1.16
2019-10-28T10:35:14.7604885Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:16.2109921Z    Compiling env_logger v0.7.0
2019-10-28T10:35:16.4984968Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:17.5105813Z    Compiling rustc-hash v1.0.1
2019-10-28T10:35:18.1786216Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:18.1787227Z    Compiling rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
2019-10-28T10:35:18.4921902Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:18.8966712Z    Compiling quote v1.0.2
2019-10-28T10:35:19.2334405Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:19.3247438Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:19.4041254Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:19.4920723Z    Compiling rand_core v0.5.0
2019-10-28T10:35:19.7187962Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:20.0112849Z    Compiling rls-span v0.5.1
2019-10-28T10:35:20.6157221Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:21.6727617Z    Compiling measureme v0.4.0
2019-10-28T10:35:21.6727617Z    Compiling measureme v0.4.0
2019-10-28T10:35:22.0121868Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:22.7078340Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:24.2285144Z    Compiling chalk-engine v0.9.0
2019-10-28T10:35:26.8859048Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:27.4129092Z    Compiling polonius-engine v0.10.0
2019-10-28T10:35:27.9930814Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:28.3478773Z    Compiling backtrace v0.3.37
2019-10-28T10:35:28.6640697Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:30.2621264Z    Compiling flate2 v1.0.6
2019-10-28T10:35:30.8968845Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:31.8059820Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:32.1004294Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:32.7309630Z    Compiling rand_chacha v0.2.1
2019-10-28T10:35:32.8699328Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:32.9650941Z    Compiling rls-data v0.19.0
2019-10-28T10:35:34.1754361Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:34.8532372Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:35.4668588Z    Compiling crossbeam-epoch v0.7.2
2019-10-28T10:35:35.8603527Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:37.3421244Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:39.5644987Z    Compiling rand v0.7.0
2019-10-28T10:35:41.1002663Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:35:42.9091860Z    Compiling synstructure v0.12.1
2019-10-28T10:35:43.3671932Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:36:17.7659656Z    Compiling crossbeam-deque v0.7.1
2019-10-28T10:36:18.0269671Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:36:18.0745641Z    Compiling tempfile v3.1.0
2019-10-28T10:36:18.3298339Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:36:20.5836227Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:36:24.3006565Z    Compiling rustc-rayon v0.3.0
2019-10-28T10:36:27.3921062Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:36:28.7909221Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-10-28T10:36:30.3790669Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:36:31.2806679Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-10-28T10:36:31.4533122Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:36:33.8422526Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-28T10:36:34.4224438Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:36:44.4732072Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-28T10:36:45.8131890Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:36:48.8935536Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:36:51.7356957Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-28T10:36:52.9873096Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:37:12.0956895Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-28T10:37:12.2807387Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:37:22.8098486Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:38:05.8195023Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:38:45.6549215Z    Compiling syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-10-28T10:38:47.3743924Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:40:04.7369650Z    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-10-28T10:40:12.5711204Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:43:25.4544323Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:44:18.0135273Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-10-28T10:44:19.6320134Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:45:00.9541599Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-10-28T10:45:18.8736270Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:49:40.7219703Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-10-28T10:49:41.7919020Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:50:30.0952374Z    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-10-28T10:50:31.2342265Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:51:51.0125969Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:52:19.4213941Z    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-28T10:52:21.1744402Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:53:11.7221035Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2019-10-28T10:53:12.2163943Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:53:52.8100429Z    Compiling rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2019-10-28T10:53:53.3555366Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:54:17.5318535Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin)
2019-10-28T10:54:17.7454767Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:54:32.8385628Z    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-10-28T10:54:32.8385628Z    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-10-28T10:54:33.2959049Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:54:37.1694123Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:55:29.4765144Z    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2019-10-28T10:55:30.4498228Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:55:43.0040583Z    Compiling rustc_plugin v0.0.0 (/checkout/src/librustc_plugin/deprecated)
2019-10-28T10:55:43.0725990Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:55:44.0005931Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:56:43.3443289Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:57:21.0919543Z     Finished release [optimized] target(s) in 22m 53s
2019-10-28T10:57:21.1345536Z Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-28T10:57:21.1383099Z Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
2019-10-28T10:57:21.4266002Z    Compiling cc v1.0.35
2019-10-28T10:57:21.4266002Z    Compiling cc v1.0.35
2019-10-28T10:57:21.4266365Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-10-28T10:57:21.5756484Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:57:21.9166872Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:57:27.1579344Z    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
2019-10-28T10:57:27.1579344Z    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
2019-10-28T10:57:30.3122822Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:57:34.5853186Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T10:58:37.8407281Z Assembling stage2 compiler (x86_64-unknown-linux-gnu)
2019-10-28T10:58:37.8425990Z Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-28T10:58:37.8426648Z Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-28T10:58:37.8436944Z Building test helpers
---
2019-10-28T11:02:11.4964392Z ..F................................................................................................. 1600/9259
2019-10-28T11:02:17.1166612Z ........................................F........................................................... 1700/9259
2019-10-28T11:02:29.0913010Z ..........................................................i...............i......................... 1800/9259
2019-10-28T11:02:36.5217077Z .................................................................................................... 1900/9259
2019-10-28T11:02:50.4728928Z ................................................iiiii............................................... 2000/9259
2019-10-28T11:02:57.1321784Z ...............FFFFFF............................................................................... 2100/9259
2019-10-28T11:03:03.5048092Z .................................................................................................... 2300/9259
2019-10-28T11:03:07.2146086Z .................................................................................................... 2400/9259
2019-10-28T11:03:29.4688261Z .................................................................................................... 2500/9259
2019-10-28T11:03:31.9661693Z .................................................................................................... 2600/9259
---
2019-10-28T11:06:16.6273950Z ..................F..............................i...............i.................................. 4800/9259
2019-10-28T11:06:25.2604008Z ................................................F................................................... 4900/9259
2019-10-28T11:06:33.3836864Z ..........................................................................F......................... 5000/9259
2019-10-28T11:06:39.4526721Z .................................................................................................... 5100/9259
2019-10-28T11:06:49.4065346Z ..................................................ii.ii...........i...FF............................ 5200/9259
2019-10-28T11:06:58.8725377Z .................................................................................................... 5400/9259
2019-10-28T11:07:07.7413957Z .................................................................................................... 5500/9259
2019-10-28T11:07:15.3027045Z ....................i............................................................................... 5600/9259
2019-10-28T11:07:21.1325014Z .................................................................................................... 5700/9259
2019-10-28T11:07:21.1325014Z .................................................................................................... 5700/9259
2019-10-28T11:07:32.6163122Z .................................................................................................... 5800/9259
2019-10-28T11:07:44.3526671Z .....ii...i..ii...........i......................................................................... 5900/9259
2019-10-28T11:08:05.5187099Z .................................................................................................... 6100/9259
2019-10-28T11:08:12.5459790Z ....................F............................................................................... 6200/9259
2019-10-28T11:08:12.5459790Z ....................F............................................................................... 6200/9259
2019-10-28T11:08:26.3617701Z .......................Fi..ii....................................................................... 6300/9259
2019-10-28T11:08:45.8044473Z .......................................F..................................................i......... 6500/9259
2019-10-28T11:08:47.9811907Z .................................................................................................... 6600/9259
2019-10-28T11:08:50.2576579Z .................................................................i.................................. 6700/9259
2019-10-28T11:08:53.1166974Z .................................................................................................... 6800/9259
---
2019-10-28T11:12:53.8746873Z failures:
2019-10-28T11:12:53.8747062Z 
2019-10-28T11:12:53.8747386Z ---- [ui] ui/associated-types/issue-48010.rs stdout ----
2019-10-28T11:12:53.8747488Z normalized stderr:
2019-10-28T11:12:53.8747558Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8747613Z 
2019-10-28T11:12:53.8747635Z 
2019-10-28T11:12:53.8747689Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8747980Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-48010/issue-48010.stderr
2019-10-28T11:12:53.8747980Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-48010/issue-48010.stderr
2019-10-28T11:12:53.8748385Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8748949Z To only update this specific test, also pass `--test-args associated-types/issue-48010.rs`
2019-10-28T11:12:53.8749379Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8749458Z status: exit code: 0
2019-10-28T11:12:53.8749458Z status: exit code: 0
2019-10-28T11:12:53.8750213Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-48010.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-48010" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-48010/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8750529Z ------------------------------------------
2019-10-28T11:12:53.8750684Z 
2019-10-28T11:12:53.8750943Z ------------------------------------------
2019-10-28T11:12:53.8750987Z stderr:
2019-10-28T11:12:53.8750987Z stderr:
2019-10-28T11:12:53.8751189Z ------------------------------------------
2019-10-28T11:12:53.8751261Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8751499Z ------------------------------------------
2019-10-28T11:12:53.8751529Z 
2019-10-28T11:12:53.8751573Z 
2019-10-28T11:12:53.8751796Z ---- [ui] ui/async-await/async-fn-send-uses-nonsend.rs stdout ----
2019-10-28T11:12:53.8751796Z ---- [ui] ui/async-await/async-fn-send-uses-nonsend.rs stdout ----
2019-10-28T11:12:53.8751843Z normalized stderr:
2019-10-28T11:12:53.8751904Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8752123Z 
2019-10-28T11:12:53.8752145Z 
2019-10-28T11:12:53.8752186Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8752505Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-send-uses-nonsend/async-fn-send-uses-nonsend.stderr
2019-10-28T11:12:53.8752505Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-send-uses-nonsend/async-fn-send-uses-nonsend.stderr
2019-10-28T11:12:53.8752909Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8753171Z To only update this specific test, also pass `--test-args async-await/async-fn-send-uses-nonsend.rs`
2019-10-28T11:12:53.8753242Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8753281Z status: exit code: 0
2019-10-28T11:12:53.8753281Z status: exit code: 0
2019-10-28T11:12:53.8753996Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-send-uses-nonsend.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-send-uses-nonsend" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-send-uses-nonsend/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8754302Z ------------------------------------------
2019-10-28T11:12:53.8754333Z 
2019-10-28T11:12:53.8754521Z ------------------------------------------
2019-10-28T11:12:53.8754577Z stderr:
2019-10-28T11:12:53.8754577Z stderr:
2019-10-28T11:12:53.8754763Z ------------------------------------------
2019-10-28T11:12:53.8754810Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8755043Z ------------------------------------------
2019-10-28T11:12:53.8755070Z 
2019-10-28T11:12:53.8755093Z 
2019-10-28T11:12:53.8755331Z ---- [ui] ui/async-await/conditional-and-guaranteed-initialization.rs stdout ----
2019-10-28T11:12:53.8755331Z ---- [ui] ui/async-await/conditional-and-guaranteed-initialization.rs stdout ----
2019-10-28T11:12:53.8755375Z normalized stderr:
2019-10-28T11:12:53.8755590Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8755700Z 
2019-10-28T11:12:53.8755722Z 
2019-10-28T11:12:53.8755762Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8756170Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/conditional-and-guaranteed-initialization/conditional-and-guaranteed-initialization.stderr
2019-10-28T11:12:53.8756170Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/conditional-and-guaranteed-initialization/conditional-and-guaranteed-initialization.stderr
2019-10-28T11:12:53.8756456Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8756725Z To only update this specific test, also pass `--test-args async-await/conditional-and-guaranteed-initialization.rs`
2019-10-28T11:12:53.8756814Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8756853Z status: exit code: 0
2019-10-28T11:12:53.8756853Z status: exit code: 0
2019-10-28T11:12:53.8758104Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/conditional-and-guaranteed-initialization.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/conditional-and-guaranteed-initialization" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/conditional-and-guaranteed-initialization/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8758811Z ------------------------------------------
2019-10-28T11:12:53.8758841Z 
2019-10-28T11:12:53.8759032Z ------------------------------------------
2019-10-28T11:12:53.8759072Z stderr:
2019-10-28T11:12:53.8759072Z stderr:
2019-10-28T11:12:53.8759272Z ------------------------------------------
2019-10-28T11:12:53.8759318Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8759548Z ------------------------------------------
2019-10-28T11:12:53.8759584Z 
2019-10-28T11:12:53.8759606Z 
2019-10-28T11:12:53.8759808Z ---- [ui] ui/async-await/generics-and-bounds.rs stdout ----
2019-10-28T11:12:53.8759808Z ---- [ui] ui/async-await/generics-and-bounds.rs stdout ----
2019-10-28T11:12:53.8759865Z normalized stderr:
2019-10-28T11:12:53.8759917Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8759967Z 
2019-10-28T11:12:53.8759988Z 
2019-10-28T11:12:53.8760043Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8760318Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generics-and-bounds/generics-and-bounds.stderr
2019-10-28T11:12:53.8760318Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generics-and-bounds/generics-and-bounds.stderr
2019-10-28T11:12:53.8760553Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8760789Z To only update this specific test, also pass `--test-args async-await/generics-and-bounds.rs`
2019-10-28T11:12:53.8760857Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8760910Z status: exit code: 0
2019-10-28T11:12:53.8760910Z status: exit code: 0
2019-10-28T11:12:53.8761606Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/generics-and-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generics-and-bounds" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generics-and-bounds/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8761895Z ------------------------------------------
2019-10-28T11:12:53.8761923Z 
2019-10-28T11:12:53.8762127Z ------------------------------------------
2019-10-28T11:12:53.8762167Z stderr:
2019-10-28T11:12:53.8762167Z stderr:
2019-10-28T11:12:53.8762515Z ------------------------------------------
2019-10-28T11:12:53.8762576Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8762798Z ------------------------------------------
2019-10-28T11:12:53.8762824Z 
2019-10-28T11:12:53.8762932Z 
2019-10-28T11:12:53.8763172Z ---- [ui] ui/async-await/move-part-await-return-rest-struct.rs stdout ----
2019-10-28T11:12:53.8763172Z ---- [ui] ui/async-await/move-part-await-return-rest-struct.rs stdout ----
2019-10-28T11:12:53.8763213Z normalized stderr:
2019-10-28T11:12:53.8763255Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8763320Z 
2019-10-28T11:12:53.8763341Z 
2019-10-28T11:12:53.8763378Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8763684Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/move-part-await-return-rest-struct/move-part-await-return-rest-struct.stderr
2019-10-28T11:12:53.8763684Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/move-part-await-return-rest-struct/move-part-await-return-rest-struct.stderr
2019-10-28T11:12:53.8763894Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8764184Z To only update this specific test, also pass `--test-args async-await/move-part-await-return-rest-struct.rs`
2019-10-28T11:12:53.8764334Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8764379Z status: exit code: 0
2019-10-28T11:12:53.8764379Z status: exit code: 0
2019-10-28T11:12:53.8765312Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/move-part-await-return-rest-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/move-part-await-return-rest-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/move-part-await-return-rest-struct/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8765599Z ------------------------------------------
2019-10-28T11:12:53.8765634Z 
2019-10-28T11:12:53.8765823Z ------------------------------------------
2019-10-28T11:12:53.8765877Z stderr:
2019-10-28T11:12:53.8765877Z stderr:
2019-10-28T11:12:53.8766059Z ------------------------------------------
2019-10-28T11:12:53.8766113Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8766349Z ------------------------------------------
2019-10-28T11:12:53.8766376Z 
2019-10-28T11:12:53.8766398Z 
2019-10-28T11:12:53.8766731Z ---- [ui] ui/async-await/move-part-await-return-rest-tuple.rs stdout ----
2019-10-28T11:12:53.8766731Z ---- [ui] ui/async-await/move-part-await-return-rest-tuple.rs stdout ----
2019-10-28T11:12:53.8766778Z normalized stderr:
2019-10-28T11:12:53.8766998Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8767074Z 
2019-10-28T11:12:53.8767099Z 
2019-10-28T11:12:53.8767143Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8767490Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/move-part-await-return-rest-tuple/move-part-await-return-rest-tuple.stderr
2019-10-28T11:12:53.8767490Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/move-part-await-return-rest-tuple/move-part-await-return-rest-tuple.stderr
2019-10-28T11:12:53.8767768Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8768078Z To only update this specific test, also pass `--test-args async-await/move-part-await-return-rest-tuple.rs`
2019-10-28T11:12:53.8768181Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8768227Z status: exit code: 0
2019-10-28T11:12:53.8768227Z status: exit code: 0
2019-10-28T11:12:53.8769073Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/move-part-await-return-rest-tuple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/move-part-await-return-rest-tuple" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/move-part-await-return-rest-tuple/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8769519Z ------------------------------------------
2019-10-28T11:12:53.8769584Z 
2019-10-28T11:12:53.8769846Z ------------------------------------------
2019-10-28T11:12:53.8769894Z stderr:
2019-10-28T11:12:53.8769894Z stderr:
2019-10-28T11:12:53.8770135Z ------------------------------------------
2019-10-28T11:12:53.8770191Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8770473Z ------------------------------------------
2019-10-28T11:12:53.8770506Z 
2019-10-28T11:12:53.8770533Z 
2019-10-28T11:12:53.8770929Z ---- [ui] ui/bad/bad-intrinsic-monomorphization.rs stdout ----
2019-10-28T11:12:53.8770929Z ---- [ui] ui/bad/bad-intrinsic-monomorphization.rs stdout ----
2019-10-28T11:12:53.8770977Z diff of stderr:
2019-10-28T11:12:53.8771020Z 
2019-10-28T11:12:53.8771229Z + [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8771364Z 1 error[E0511]: invalid monomorphization of `cttz` intrinsic: expected basic integer type, found `Foo`
2019-10-28T11:12:53.8771674Z 3    |
2019-10-28T11:12:53.8771700Z 
2019-10-28T11:12:53.8771723Z 
2019-10-28T11:12:53.8771781Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8771781Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8772242Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bad/bad-intrinsic-monomorphization/bad-intrinsic-monomorphization.stderr
2019-10-28T11:12:53.8772466Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8772914Z To only update this specific test, also pass `--test-args bad/bad-intrinsic-monomorphization.rs`
2019-10-28T11:12:53.8773151Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8773207Z status: exit code: 1
2019-10-28T11:12:53.8773207Z status: exit code: 1
2019-10-28T11:12:53.8773890Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/bad/bad-intrinsic-monomorphization.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bad/bad-intrinsic-monomorphization" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bad/bad-intrinsic-monomorphization/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8774374Z ------------------------------------------
2019-10-28T11:12:53.8774405Z 
2019-10-28T11:12:53.8774617Z ------------------------------------------
2019-10-28T11:12:53.8774659Z stderr:
2019-10-28T11:12:53.8774659Z stderr:
2019-10-28T11:12:53.8775030Z ------------------------------------------
2019-10-28T11:12:53.8775083Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8775166Z error[E0511]: invalid monomorphization of `cttz` intrinsic: expected basic integer type, found `Foo`
2019-10-28T11:12:53.8775664Z    |
2019-10-28T11:12:53.8775709Z LL |     intrinsics::cttz(v)
2019-10-28T11:12:53.8775753Z    |     ^^^^^^^^^^^^^^^^^^^
2019-10-28T11:12:53.8775780Z 
2019-10-28T11:12:53.8775780Z 
2019-10-28T11:12:53.8775849Z error[E0511]: invalid monomorphization of `fadd_fast` intrinsic: expected basic float type, found `Foo`
2019-10-28T11:12:53.8776148Z    |
2019-10-28T11:12:53.8776148Z    |
2019-10-28T11:12:53.8776207Z LL |     intrinsics::fadd_fast(a, b)
2019-10-28T11:12:53.8776283Z 
2019-10-28T11:12:53.8776283Z 
2019-10-28T11:12:53.8776599Z error[E0511]: invalid monomorphization of `simd_add` intrinsic: expected SIMD input type, found non-SIMD `Foo`
2019-10-28T11:12:53.8776918Z    |
2019-10-28T11:12:53.8776960Z LL |     simd_add(a, b)
2019-10-28T11:12:53.8777022Z    |     ^^^^^^^^^^^^^^
2019-10-28T11:12:53.8777050Z 
---
2019-10-28T11:12:53.8778179Z 
2019-10-28T11:12:53.8778221Z 12 LL | #![warn(const_err)]
2019-10-28T11:12:53.8778426Z 13    |         ^^^^^^^^^
2019-10-28T11:12:53.8778484Z 14 
2019-10-28T11:12:53.8778531Z + [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8778796Z 
2019-10-28T11:12:53.8778820Z 
2019-10-28T11:12:53.8778945Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8779267Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/pub_const_err/pub_const_err.stderr
2019-10-28T11:12:53.8779267Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/pub_const_err/pub_const_err.stderr
2019-10-28T11:12:53.8779530Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8779793Z To only update this specific test, also pass `--test-args consts/const-eval/pub_const_err.rs`
2019-10-28T11:12:53.8779890Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8779932Z status: exit code: 0
2019-10-28T11:12:53.8779932Z status: exit code: 0
2019-10-28T11:12:53.8781570Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/pub_const_err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/pub_const_err" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/pub_const_err/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8782119Z ------------------------------------------
2019-10-28T11:12:53.8782170Z 
2019-10-28T11:12:53.8782389Z ------------------------------------------
2019-10-28T11:12:53.8782600Z stderr:
2019-10-28T11:12:53.8782600Z stderr:
2019-10-28T11:12:53.8782815Z ------------------------------------------
2019-10-28T11:12:53.8782865Z warning: any use of this value will cause an error
2019-10-28T11:12:53.8783093Z   --> /checkout/src/test/ui/consts/const-eval/pub_const_err.rs:6:20
2019-10-28T11:12:53.8783141Z    |
2019-10-28T11:12:53.8783511Z LL | pub const Z: u32 = 0 - 1;
2019-10-28T11:12:53.8784081Z    |                    |
2019-10-28T11:12:53.8784142Z    |                    attempt to subtract with overflow
2019-10-28T11:12:53.8784183Z    |
2019-10-28T11:12:53.8784231Z note: lint level defined here
2019-10-28T11:12:53.8784231Z note: lint level defined here
2019-10-28T11:12:53.8784474Z   --> /checkout/src/test/ui/consts/const-eval/pub_const_err.rs:2:9
2019-10-28T11:12:53.8784520Z    |
2019-10-28T11:12:53.8784567Z LL | #![warn(const_err)]
2019-10-28T11:12:53.8784857Z    |         ^^^^^^^^^
2019-10-28T11:12:53.8784885Z 
2019-10-28T11:12:53.8784933Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8785543Z ------------------------------------------
2019-10-28T11:12:53.8785575Z 
2019-10-28T11:12:53.8785600Z 
2019-10-28T11:12:53.8785993Z ---- [ui] ui/consts/invalid_promotion.rs stdout ----
2019-10-28T11:12:53.8785993Z ---- [ui] ui/consts/invalid_promotion.rs stdout ----
2019-10-28T11:12:53.8786058Z normalized stderr:
2019-10-28T11:12:53.8786110Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8786169Z 
2019-10-28T11:12:53.8786193Z 
2019-10-28T11:12:53.8786256Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8786566Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid_promotion/invalid_promotion.stderr
2019-10-28T11:12:53.8786566Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid_promotion/invalid_promotion.stderr
2019-10-28T11:12:53.8786918Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8787240Z To only update this specific test, also pass `--test-args consts/invalid_promotion.rs`
2019-10-28T11:12:53.8787321Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8787382Z status: exit code: 0
2019-10-28T11:12:53.8787382Z status: exit code: 0
2019-10-28T11:12:53.8788336Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/invalid_promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid_promotion" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid_promotion/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8788998Z ------------------------------------------
2019-10-28T11:12:53.8789202Z 
2019-10-28T11:12:53.8789412Z ------------------------------------------
2019-10-28T11:12:53.8789452Z stderr:
2019-10-28T11:12:53.8789452Z stderr:
2019-10-28T11:12:53.8789633Z ------------------------------------------
2019-10-28T11:12:53.8789696Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8789910Z ------------------------------------------
2019-10-28T11:12:53.8789938Z 
2019-10-28T11:12:53.8789975Z 
2019-10-28T11:12:53.8790168Z ---- [ui] ui/duplicate/dupe-symbols-1.rs stdout ----
2019-10-28T11:12:53.8790168Z ---- [ui] ui/duplicate/dupe-symbols-1.rs stdout ----
2019-10-28T11:12:53.8790209Z diff of stderr:
2019-10-28T11:12:53.8790233Z 
2019-10-28T11:12:53.8790345Z + [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8790389Z 1 error: symbol `fail` is already defined
2019-10-28T11:12:53.8790586Z 2   --> $DIR/dupe-symbols-1.rs:10:1
2019-10-28T11:12:53.8790720Z 
2019-10-28T11:12:53.8790742Z 
2019-10-28T11:12:53.8790788Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8790788Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8791244Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-1/dupe-symbols-1.stderr
2019-10-28T11:12:53.8791466Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8791700Z To only update this specific test, also pass `--test-args duplicate/dupe-symbols-1.rs`
2019-10-28T11:12:53.8791786Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8791826Z status: exit code: 1
2019-10-28T11:12:53.8791826Z status: exit code: 1
2019-10-28T11:12:53.8792496Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/duplicate/dupe-symbols-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-1/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8792800Z ------------------------------------------
2019-10-28T11:12:53.8792831Z 
2019-10-28T11:12:53.8793023Z ------------------------------------------
2019-10-28T11:12:53.8793063Z stderr:
2019-10-28T11:12:53.8793063Z stderr:
2019-10-28T11:12:53.8793267Z ------------------------------------------
2019-10-28T11:12:53.8793316Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8793363Z error: symbol `fail` is already defined
2019-10-28T11:12:53.8793593Z   --> /checkout/src/test/ui/duplicate/dupe-symbols-1.rs:10:1
2019-10-28T11:12:53.8793637Z    |
2019-10-28T11:12:53.8793675Z LL | / pub fn b() {
2019-10-28T11:12:53.8793741Z LL | | //~^ symbol `fail` is already defined
2019-10-28T11:12:53.8793817Z    | |_^
2019-10-28T11:12:53.8793841Z 
2019-10-28T11:12:53.8793975Z error: aborting due to previous error
2019-10-28T11:12:53.8794008Z 
2019-10-28T11:12:53.8794008Z 
2019-10-28T11:12:53.8794030Z 
2019-10-28T11:12:53.8794247Z ------------------------------------------
2019-10-28T11:12:53.8794276Z 
2019-10-28T11:12:53.8794316Z 
2019-10-28T11:12:53.8794513Z ---- [ui] ui/duplicate/dupe-symbols-2.rs stdout ----
2019-10-28T11:12:53.8794557Z diff of stderr:
2019-10-28T11:12:53.8794582Z 
2019-10-28T11:12:53.8794642Z + [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8794687Z 1 error: symbol `fail` is already defined
2019-10-28T11:12:53.8794875Z 2   --> $DIR/dupe-symbols-2.rs:13:5
2019-10-28T11:12:53.8794956Z 
2019-10-28T11:12:53.8794979Z 
2019-10-28T11:12:53.8795019Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8795019Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8795397Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-2/dupe-symbols-2.stderr
2019-10-28T11:12:53.8795628Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8795870Z To only update this specific test, also pass `--test-args duplicate/dupe-symbols-2.rs`
2019-10-28T11:12:53.8795957Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8795996Z status: exit code: 1
2019-10-28T11:12:53.8795996Z status: exit code: 1
2019-10-28T11:12:53.8796660Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/duplicate/dupe-symbols-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-2/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8796957Z ------------------------------------------
2019-10-28T11:12:53.8796994Z 
2019-10-28T11:12:53.8797190Z ------------------------------------------
2019-10-28T11:12:53.8797231Z stderr:
2019-10-28T11:12:53.8797231Z stderr:
2019-10-28T11:12:53.8797433Z ------------------------------------------
2019-10-28T11:12:53.8797482Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8797528Z error: symbol `fail` is already defined
2019-10-28T11:12:53.8797760Z   --> /checkout/src/test/ui/duplicate/dupe-symbols-2.rs:13:5
2019-10-28T11:12:53.8797802Z    |
2019-10-28T11:12:53.8797842Z LL | /     pub extern fn fail() {
2019-10-28T11:12:53.8797902Z LL | |     //~^ symbol `fail` is already defined
2019-10-28T11:12:53.8797980Z    | |_____^
2019-10-28T11:12:53.8798022Z 
2019-10-28T11:12:53.8798062Z error: aborting due to previous error
2019-10-28T11:12:53.8798095Z 
2019-10-28T11:12:53.8798095Z 
2019-10-28T11:12:53.8798118Z 
2019-10-28T11:12:53.8798315Z ------------------------------------------
2019-10-28T11:12:53.8798360Z 
2019-10-28T11:12:53.8798383Z 
2019-10-28T11:12:53.8798587Z ---- [ui] ui/duplicate/dupe-symbols-3.rs stdout ----
2019-10-28T11:12:53.8798630Z diff of stderr:
2019-10-28T11:12:53.8798673Z 
2019-10-28T11:12:53.8798717Z + [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8798762Z 1 error: symbol `fail` is already defined
2019-10-28T11:12:53.8798974Z 2   --> $DIR/dupe-symbols-3.rs:10:1
2019-10-28T11:12:53.8799040Z 
2019-10-28T11:12:53.8799063Z 
2019-10-28T11:12:53.8799103Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8799103Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8799385Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-3/dupe-symbols-3.stderr
2019-10-28T11:12:53.8799603Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8799863Z To only update this specific test, also pass `--test-args duplicate/dupe-symbols-3.rs`
2019-10-28T11:12:53.8800008Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8800053Z status: exit code: 1
2019-10-28T11:12:53.8800053Z status: exit code: 1
2019-10-28T11:12:53.8800742Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/duplicate/dupe-symbols-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-3/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8801033Z ------------------------------------------
2019-10-28T11:12:53.8801137Z 
2019-10-28T11:12:53.8801463Z ------------------------------------------
2019-10-28T11:12:53.8801520Z stderr:
2019-10-28T11:12:53.8801520Z stderr:
2019-10-28T11:12:53.8801703Z ------------------------------------------
2019-10-28T11:12:53.8801759Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8801820Z error: symbol `fail` is already defined
2019-10-28T11:12:53.8802031Z   --> /checkout/src/test/ui/duplicate/dupe-symbols-3.rs:10:1
2019-10-28T11:12:53.8802074Z    |
2019-10-28T11:12:53.8802125Z LL | / pub fn fail() {
2019-10-28T11:12:53.8802166Z LL | | //~^ symbol `fail` is already defined
2019-10-28T11:12:53.8802239Z    | |_^
2019-10-28T11:12:53.8802280Z 
2019-10-28T11:12:53.8802317Z error: aborting due to previous error
2019-10-28T11:12:53.8802342Z 
2019-10-28T11:12:53.8802342Z 
2019-10-28T11:12:53.8802364Z 
2019-10-28T11:12:53.8802569Z ------------------------------------------
2019-10-28T11:12:53.8802596Z 
2019-10-28T11:12:53.8802618Z 
2019-10-28T11:12:53.8802820Z ---- [ui] ui/duplicate/dupe-symbols-4.rs stdout ----
2019-10-28T11:12:53.8802878Z diff of stderr:
2019-10-28T11:12:53.8802902Z 
2019-10-28T11:12:53.8802951Z + [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8802994Z 1 error: symbol `fail` is already defined
2019-10-28T11:12:53.8803199Z 2   --> $DIR/dupe-symbols-4.rs:21:5
2019-10-28T11:12:53.8803262Z 
2019-10-28T11:12:53.8803284Z 
2019-10-28T11:12:53.8803337Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8803337Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8803595Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-4/dupe-symbols-4.stderr
2019-10-28T11:12:53.8804065Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8804569Z To only update this specific test, also pass `--test-args duplicate/dupe-symbols-4.rs`
2019-10-28T11:12:53.8804649Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8804719Z status: exit code: 1
2019-10-28T11:12:53.8804719Z status: exit code: 1
2019-10-28T11:12:53.8805451Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/duplicate/dupe-symbols-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-4" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-4/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8805774Z ------------------------------------------
2019-10-28T11:12:53.8805808Z 
2019-10-28T11:12:53.8806018Z ------------------------------------------
2019-10-28T11:12:53.8806082Z stderr:
2019-10-28T11:12:53.8806082Z stderr:
2019-10-28T11:12:53.8806290Z ------------------------------------------
2019-10-28T11:12:53.8806343Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8806419Z error: symbol `fail` is already defined
2019-10-28T11:12:53.8806735Z   --> /checkout/src/test/ui/duplicate/dupe-symbols-4.rs:21:5
2019-10-28T11:12:53.8806852Z LL |     fn fail(self) {}
2019-10-28T11:12:53.8806896Z    |     ^^^^^^^^^^^^^^^^
2019-10-28T11:12:53.8806924Z 
2019-10-28T11:12:53.8806965Z error: aborting due to previous error
---
2019-10-28T11:12:53.8807346Z 
2019-10-28T11:12:53.8807890Z ---- [ui] ui/duplicate/dupe-symbols-5.rs stdout ----
2019-10-28T11:12:53.8807933Z diff of stderr:
2019-10-28T11:12:53.8807958Z 
2019-10-28T11:12:53.8808017Z + [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8808062Z 1 error: symbol `fail` is already defined
2019-10-28T11:12:53.8808352Z 2   --> $DIR/dupe-symbols-5.rs:9:1
2019-10-28T11:12:53.8808436Z 
2019-10-28T11:12:53.8808460Z 
2019-10-28T11:12:53.8808499Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8808499Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8808788Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-5/dupe-symbols-5.stderr
2019-10-28T11:12:53.8809017Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8809253Z To only update this specific test, also pass `--test-args duplicate/dupe-symbols-5.rs`
2019-10-28T11:12:53.8809340Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8809379Z status: exit code: 1
2019-10-28T11:12:53.8809379Z status: exit code: 1
2019-10-28T11:12:53.8810046Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/duplicate/dupe-symbols-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-5" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-5/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8810345Z ------------------------------------------
2019-10-28T11:12:53.8810375Z 
2019-10-28T11:12:53.8810564Z ------------------------------------------
2019-10-28T11:12:53.8810604Z stderr:
2019-10-28T11:12:53.8810604Z stderr:
2019-10-28T11:12:53.8810809Z ------------------------------------------
2019-10-28T11:12:53.8810858Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8810904Z error: symbol `fail` is already defined
2019-10-28T11:12:53.8811134Z   --> /checkout/src/test/ui/duplicate/dupe-symbols-5.rs:9:1
2019-10-28T11:12:53.8811176Z    |
2019-10-28T11:12:53.8811214Z LL | / pub fn b() {
2019-10-28T11:12:53.8811280Z LL | | //~^ symbol `fail` is already defined
2019-10-28T11:12:53.8811357Z    | |_^
2019-10-28T11:12:53.8811381Z 
2019-10-28T11:12:53.8811437Z error: aborting due to previous error
2019-10-28T11:12:53.8811470Z 
2019-10-28T11:12:53.8811470Z 
2019-10-28T11:12:53.8811493Z 
2019-10-28T11:12:53.8811691Z ------------------------------------------
2019-10-28T11:12:53.8811719Z 
2019-10-28T11:12:53.8811758Z 
2019-10-28T11:12:53.8811957Z ---- [ui] ui/duplicate/dupe-symbols-6.rs stdout ----
2019-10-28T11:12:53.8812000Z diff of stderr:
2019-10-28T11:12:53.8812025Z 
2019-10-28T11:12:53.8812085Z + [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8812130Z 1 error: symbol `fail` is already defined
2019-10-28T11:12:53.8812319Z 2   --> $DIR/dupe-symbols-6.rs:8:1
2019-10-28T11:12:53.8812399Z 
2019-10-28T11:12:53.8812422Z 
2019-10-28T11:12:53.8812462Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8812462Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8812750Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-6/dupe-symbols-6.stderr
2019-10-28T11:12:53.8813708Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8814221Z To only update this specific test, also pass `--test-args duplicate/dupe-symbols-6.rs`
2019-10-28T11:12:53.8814351Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8814393Z status: exit code: 1
2019-10-28T11:12:53.8814393Z status: exit code: 1
2019-10-28T11:12:53.8815118Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/duplicate/dupe-symbols-6.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-6" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-6/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8815535Z ------------------------------------------
2019-10-28T11:12:53.8815566Z 
2019-10-28T11:12:53.8815771Z ------------------------------------------
2019-10-28T11:12:53.8815812Z stderr:
2019-10-28T11:12:53.8815812Z stderr:
2019-10-28T11:12:53.8816019Z ------------------------------------------
2019-10-28T11:12:53.8816068Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8816114Z error: symbol `fail` is already defined
2019-10-28T11:12:53.8816352Z   --> /checkout/src/test/ui/duplicate/dupe-symbols-6.rs:8:1
2019-10-28T11:12:53.8816395Z    |
2019-10-28T11:12:53.8816435Z LL | static HELLO_TWICE: u16 = 0;
2019-10-28T11:12:53.8816518Z 
2019-10-28T11:12:53.8816556Z error: aborting due to previous error
2019-10-28T11:12:53.8816582Z 
2019-10-28T11:12:53.8816621Z 
2019-10-28T11:12:53.8816621Z 
2019-10-28T11:12:53.8816814Z ------------------------------------------
2019-10-28T11:12:53.8816851Z 
2019-10-28T11:12:53.8816873Z 
2019-10-28T11:12:53.8817069Z ---- [ui] ui/issues/issue-22638.rs stdout ----
2019-10-28T11:12:53.8817131Z diff of stderr:
2019-10-28T11:12:53.8817163Z 
2019-10-28T11:12:53.8817206Z + [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8817461Z 1 error: reached the type-length limit while instantiating `D::matches::$CLOSURE`
2019-10-28T11:12:53.8817654Z 2   --> $DIR/issue-22638.rs:52:5
2019-10-28T11:12:53.8817719Z 
2019-10-28T11:12:53.8817757Z 
2019-10-28T11:12:53.8817798Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8818060Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22638/issue-22638.stderr
2019-10-28T11:12:53.8818060Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22638/issue-22638.stderr
2019-10-28T11:12:53.8818473Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8818743Z To only update this specific test, also pass `--test-args issues/issue-22638.rs`
2019-10-28T11:12:53.8818991Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8819221Z status: exit code: 1
2019-10-28T11:12:53.8819221Z status: exit code: 1
2019-10-28T11:12:53.8820042Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-22638.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22638" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22638/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8820333Z ------------------------------------------
2019-10-28T11:12:53.8820362Z 
2019-10-28T11:12:53.8820568Z ------------------------------------------
2019-10-28T11:12:53.8820608Z stderr:
2019-10-28T11:12:53.8820608Z stderr:
2019-10-28T11:12:53.8820806Z ------------------------------------------
2019-10-28T11:12:53.8822058Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8822618Z error: reached the type-length limit while instantiating `D::matches::<[closure@/checkout/...s/issue-22638.rs:29:33: 29:38]]>`
2019-10-28T11:12:53.8822934Z   --> /checkout/src/test/ui/issues/issue-22638.rs:52:5
2019-10-28T11:12:53.8823010Z    |
2019-10-28T11:12:53.8823057Z LL | /     pub fn matches<F: Fn()>(&self, f: &F) {
2019-10-28T11:12:53.8823341Z LL | |         //~^ ERROR reached the type-length limit while instantiating `D::matches::<[closure
2019-10-28T11:12:53.8823414Z LL | |         let &D(ref a) = self;
2019-10-28T11:12:53.8823460Z LL | |         a.matches(f)
2019-10-28T11:12:53.8823565Z    | |_____^
2019-10-28T11:12:53.8823605Z    |
2019-10-28T11:12:53.8823605Z    |
2019-10-28T11:12:53.8823841Z    = note: consider adding a `#![type_length_limit="26214380"]` attribute to your crate
2019-10-28T11:12:53.8824199Z error: aborting due to previous error
2019-10-28T11:12:53.8824229Z 
2019-10-28T11:12:53.8824254Z 
2019-10-28T11:12:53.8824506Z ------------------------------------------
2019-10-28T11:12:53.8824506Z ------------------------------------------
2019-10-28T11:12:53.8824564Z 
2019-10-28T11:12:53.8824591Z 
2019-10-28T11:12:53.8825209Z ---- [ui] ui/issues/issue-47309.rs stdout ----
2019-10-28T11:12:53.8825265Z normalized stderr:
2019-10-28T11:12:53.8825335Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8825392Z 
2019-10-28T11:12:53.8825416Z 
2019-10-28T11:12:53.8825477Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8825768Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47309/issue-47309.stderr
2019-10-28T11:12:53.8825768Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47309/issue-47309.stderr
2019-10-28T11:12:53.8826753Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8827053Z To only update this specific test, also pass `--test-args issues/issue-47309.rs`
2019-10-28T11:12:53.8827145Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8827188Z status: exit code: 0
2019-10-28T11:12:53.8827188Z status: exit code: 0
2019-10-28T11:12:53.8828055Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-47309.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47309" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Clink-dead-code" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47309/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8828353Z ------------------------------------------
2019-10-28T11:12:53.8828383Z 
2019-10-28T11:12:53.8828752Z ------------------------------------------
2019-10-28T11:12:53.8828811Z stderr:
2019-10-28T11:12:53.8828811Z stderr:
2019-10-28T11:12:53.8829020Z ------------------------------------------
2019-10-28T11:12:53.8829070Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8829328Z ------------------------------------------
2019-10-28T11:12:53.8829357Z 
2019-10-28T11:12:53.8829381Z 
2019-10-28T11:12:53.8829581Z ---- [ui] ui/issues/issue-51947.rs stdout ----
2019-10-28T11:12:53.8829581Z ---- [ui] ui/issues/issue-51947.rs stdout ----
2019-10-28T11:12:53.8829643Z normalized stderr:
2019-10-28T11:12:53.8829689Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8829741Z 
2019-10-28T11:12:53.8829945Z 
2019-10-28T11:12:53.8829985Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8830247Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51947/issue-51947.stderr
2019-10-28T11:12:53.8830247Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51947/issue-51947.stderr
2019-10-28T11:12:53.8830488Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8830732Z To only update this specific test, also pass `--test-args issues/issue-51947.rs`
2019-10-28T11:12:53.8830817Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8830973Z status: exit code: 0
2019-10-28T11:12:53.8830973Z status: exit code: 0
2019-10-28T11:12:53.8831649Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-51947.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51947" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51947/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8831939Z ------------------------------------------
2019-10-28T11:12:53.8831968Z 
2019-10-28T11:12:53.8832177Z ------------------------------------------
2019-10-28T11:12:53.8832303Z stderr:
2019-10-28T11:12:53.8832303Z stderr:
2019-10-28T11:12:53.8832514Z ------------------------------------------
2019-10-28T11:12:53.8832589Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8832820Z ------------------------------------------
2019-10-28T11:12:53.8832848Z 
2019-10-28T11:12:53.8832886Z 
2019-10-28T11:12:53.8833103Z ---- [ui] ui/issues/issue-58375-monomorphize-default-impls.rs stdout ----
2019-10-28T11:12:53.8833103Z ---- [ui] ui/issues/issue-58375-monomorphize-default-impls.rs stdout ----
2019-10-28T11:12:53.8833147Z normalized stderr:
2019-10-28T11:12:53.8833207Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8833259Z 
2019-10-28T11:12:53.8833281Z 
2019-10-28T11:12:53.8833321Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8833648Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-58375-monomorphize-default-impls/issue-58375-monomorphize-default-impls.stderr
2019-10-28T11:12:53.8833648Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-58375-monomorphize-default-impls/issue-58375-monomorphize-default-impls.stderr
2019-10-28T11:12:53.8833885Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8834164Z To only update this specific test, also pass `--test-args issues/issue-58375-monomorphize-default-impls.rs`
2019-10-28T11:12:53.8834237Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8834276Z status: exit code: 0
2019-10-28T11:12:53.8834276Z status: exit code: 0
2019-10-28T11:12:53.8835024Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-58375-monomorphize-default-impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-58375-monomorphize-default-impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "link-dead-code" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-58375-monomorphize-default-impls/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8835328Z ------------------------------------------
2019-10-28T11:12:53.8835357Z 
2019-10-28T11:12:53.8835552Z ------------------------------------------
2019-10-28T11:12:53.8835608Z stderr:
2019-10-28T11:12:53.8835608Z stderr:
2019-10-28T11:12:53.8835985Z ------------------------------------------
2019-10-28T11:12:53.8836038Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8836289Z ------------------------------------------
2019-10-28T11:12:53.8836319Z 
2019-10-28T11:12:53.8836343Z 
2019-10-28T11:12:53.8836560Z ---- [ui] ui/json-multiple.rs stdout ----
2019-10-28T11:12:53.8836560Z ---- [ui] ui/json-multiple.rs stdout ----
2019-10-28T11:12:53.8836604Z diff of stderr:
2019-10-28T11:12:53.8836631Z 
2019-10-28T11:12:53.8836676Z + [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8837092Z 1 {"artifact":"$TEST_BUILD_DIR/json-multiple/libjson_multiple.rlib","emit":"link"}
2019-10-28T11:12:53.8837169Z 
2019-10-28T11:12:53.8837192Z 
2019-10-28T11:12:53.8837248Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8837598Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-multiple/json-multiple.stderr
2019-10-28T11:12:53.8837598Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-multiple/json-multiple.stderr
2019-10-28T11:12:53.8837856Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8838120Z To only update this specific test, also pass `--test-args json-multiple.rs`
2019-10-28T11:12:53.8838197Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8838240Z status: exit code: 0
2019-10-28T11:12:53.8838240Z status: exit code: 0
2019-10-28T11:12:53.8838961Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/json-multiple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-multiple" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--json=diagnostic-short" "--json" "artifacts" "--error-format=json" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-multiple/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8839536Z ------------------------------------------
2019-10-28T11:12:53.8839567Z 
2019-10-28T11:12:53.8839758Z ------------------------------------------
2019-10-28T11:12:53.8839818Z stderr:
2019-10-28T11:12:53.8839818Z stderr:
2019-10-28T11:12:53.8840004Z ------------------------------------------
2019-10-28T11:12:53.8840053Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8840290Z ------------------------------------------
2019-10-28T11:12:53.8840317Z 
2019-10-28T11:12:53.8840339Z 
2019-10-28T11:12:53.8840536Z ---- [ui] ui/json-options.rs stdout ----
2019-10-28T11:12:53.8840536Z ---- [ui] ui/json-options.rs stdout ----
2019-10-28T11:12:53.8840578Z diff of stderr:
2019-10-28T11:12:53.8840603Z 
2019-10-28T11:12:53.8840656Z + [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8840905Z 1 {"artifact":"$TEST_BUILD_DIR/json-options/libjson_options.rlib","emit":"link"}
2019-10-28T11:12:53.8840980Z 
2019-10-28T11:12:53.8841003Z 
2019-10-28T11:12:53.8841059Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8841315Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-options/json-options.stderr
2019-10-28T11:12:53.8841315Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-options/json-options.stderr
2019-10-28T11:12:53.8841532Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8841785Z To only update this specific test, also pass `--test-args json-options.rs`
2019-10-28T11:12:53.8841857Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8841898Z status: exit code: 0
2019-10-28T11:12:53.8841898Z status: exit code: 0
2019-10-28T11:12:53.8842582Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/json-options.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-options" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--json=diagnostic-short,artifacts" "--error-format=json" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-options/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8842903Z ------------------------------------------
2019-10-28T11:12:53.8842934Z 
2019-10-28T11:12:53.8843138Z ------------------------------------------
2019-10-28T11:12:53.8843196Z stderr:
2019-10-28T11:12:53.8843196Z stderr:
2019-10-28T11:12:53.8843400Z ------------------------------------------
2019-10-28T11:12:53.8843450Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8843701Z ------------------------------------------
2019-10-28T11:12:53.8843729Z 
2019-10-28T11:12:53.8843761Z 
2019-10-28T11:12:53.8844003Z ---- [ui] ui/never_type/never_transmute_never.rs stdout ----
2019-10-28T11:12:53.8844003Z ---- [ui] ui/never_type/never_transmute_never.rs stdout ----
2019-10-28T11:12:53.8844049Z normalized stderr:
2019-10-28T11:12:53.8844173Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8844249Z 
2019-10-28T11:12:53.8844273Z 
2019-10-28T11:12:53.8844315Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8844636Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/never_transmute_never/never_transmute_never.stderr
2019-10-28T11:12:53.8844636Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/never_transmute_never/never_transmute_never.stderr
2019-10-28T11:12:53.8844891Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8845147Z To only update this specific test, also pass `--test-args never_type/never_transmute_never.rs`
2019-10-28T11:12:53.8845238Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8845280Z status: exit code: 0
2019-10-28T11:12:53.8845280Z status: exit code: 0
2019-10-28T11:12:53.8845985Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/never_transmute_never.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/never_transmute_never" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/never_transmute_never/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8846392Z ------------------------------------------
2019-10-28T11:12:53.8846441Z 
2019-10-28T11:12:53.8846648Z ------------------------------------------
2019-10-28T11:12:53.8846690Z stderr:
2019-10-28T11:12:53.8846690Z stderr:
2019-10-28T11:12:53.8846909Z ------------------------------------------
2019-10-28T11:12:53.8846959Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8847207Z ------------------------------------------
2019-10-28T11:12:53.8847251Z 
2019-10-28T11:12:53.8847276Z 
2019-10-28T11:12:53.8847498Z ---- [ui] ui/nll/ty-outlives/issue-55756.rs stdout ----
2019-10-28T11:12:53.8847498Z ---- [ui] ui/nll/ty-outlives/issue-55756.rs stdout ----
2019-10-28T11:12:53.8847545Z normalized stderr:
2019-10-28T11:12:53.8847607Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8847661Z 
2019-10-28T11:12:53.8847685Z 
2019-10-28T11:12:53.8847744Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8848035Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/issue-55756/issue-55756.stderr
2019-10-28T11:12:53.8848035Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/issue-55756/issue-55756.stderr
2019-10-28T11:12:53.8848270Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8848540Z To only update this specific test, also pass `--test-args nll/ty-outlives/issue-55756.rs`
2019-10-28T11:12:53.8848613Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8848680Z status: exit code: 0
2019-10-28T11:12:53.8848680Z status: exit code: 0
2019-10-28T11:12:53.8849367Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/issue-55756.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/issue-55756" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/issue-55756/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8849678Z ------------------------------------------
2019-10-28T11:12:53.8849710Z 
2019-10-28T11:12:53.8849908Z ------------------------------------------
2019-10-28T11:12:53.8849967Z stderr:
2019-10-28T11:12:53.8849967Z stderr:
2019-10-28T11:12:53.8850167Z ------------------------------------------
2019-10-28T11:12:53.8850226Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8850485Z ------------------------------------------
2019-10-28T11:12:53.8850611Z 
2019-10-28T11:12:53.8850643Z 
2019-10-28T11:12:53.8850892Z ---- [ui] ui/non-integer-atomic.rs stdout ----
2019-10-28T11:12:53.8850892Z ---- [ui] ui/non-integer-atomic.rs stdout ----
2019-10-28T11:12:53.8850938Z diff of stderr:
2019-10-28T11:12:53.8850965Z 
2019-10-28T11:12:53.8851010Z + [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8851084Z 1 error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `bool`
2019-10-28T11:12:53.8851989Z 3    |
2019-10-28T11:12:53.8852033Z 
2019-10-28T11:12:53.8852056Z 
2019-10-28T11:12:53.8852096Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8852096Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8852372Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-integer-atomic/non-integer-atomic.stderr
2019-10-28T11:12:53.8852769Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8853013Z To only update this specific test, also pass `--test-args non-integer-atomic.rs`
2019-10-28T11:12:53.8853103Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8853142Z status: exit code: 1
2019-10-28T11:12:53.8853142Z status: exit code: 1
2019-10-28T11:12:53.8853780Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/non-integer-atomic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-integer-atomic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-integer-atomic/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8854078Z ------------------------------------------
2019-10-28T11:12:53.8854123Z 
2019-10-28T11:12:53.8854313Z ------------------------------------------
2019-10-28T11:12:53.8854354Z stderr:
2019-10-28T11:12:53.8854354Z stderr:
2019-10-28T11:12:53.8854545Z ------------------------------------------
2019-10-28T11:12:53.8854611Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8854663Z error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `bool`
2019-10-28T11:12:53.8855332Z    |
2019-10-28T11:12:53.8855371Z LL |     intrinsics::atomic_load(p);
2019-10-28T11:12:53.8855412Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-10-28T11:12:53.8855454Z 
2019-10-28T11:12:53.8855454Z 
2019-10-28T11:12:53.8855500Z error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `bool`
2019-10-28T11:12:53.8856316Z    |
2019-10-28T11:12:53.8856316Z    |
2019-10-28T11:12:53.8856355Z LL |     intrinsics::atomic_store(p, v);
2019-10-28T11:12:53.8856422Z 
2019-10-28T11:12:53.8856422Z 
2019-10-28T11:12:53.8856482Z error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `bool`
2019-10-28T11:12:53.8873502Z    |
2019-10-28T11:12:53.8873502Z    |
2019-10-28T11:12:53.8873545Z LL |     intrinsics::atomic_xchg(p, v);
2019-10-28T11:12:53.8873625Z 
2019-10-28T11:12:53.8873625Z 
2019-10-28T11:12:53.8873673Z error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `bool`
2019-10-28T11:12:53.8873993Z    |
2019-10-28T11:12:53.8873993Z    |
2019-10-28T11:12:53.8874033Z LL |     intrinsics::atomic_cxchg(p, v, v);
2019-10-28T11:12:53.8874134Z 
2019-10-28T11:12:53.8874134Z 
2019-10-28T11:12:53.8874181Z error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `Foo`
2019-10-28T11:12:53.8874628Z    |
2019-10-28T11:12:53.8874667Z LL |     intrinsics::atomic_load(p);
2019-10-28T11:12:53.8874708Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-10-28T11:12:53.8874735Z 
2019-10-28T11:12:53.8874735Z 
2019-10-28T11:12:53.8874788Z error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `Foo`
2019-10-28T11:12:53.8875416Z    |
2019-10-28T11:12:53.8875416Z    |
2019-10-28T11:12:53.8875462Z LL |     intrinsics::atomic_store(p, v);
2019-10-28T11:12:53.8875529Z 
2019-10-28T11:12:53.8875529Z 
2019-10-28T11:12:53.8875574Z error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `Foo`
2019-10-28T11:12:53.8875991Z    |
2019-10-28T11:12:53.8875991Z    |
2019-10-28T11:12:53.8876030Z LL |     intrinsics::atomic_xchg(p, v);
2019-10-28T11:12:53.8876115Z 
2019-10-28T11:12:53.8876115Z 
2019-10-28T11:12:53.8876160Z error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `Foo`
2019-10-28T11:12:53.8876432Z    |
2019-10-28T11:12:53.8876432Z    |
2019-10-28T11:12:53.8876470Z LL |     intrinsics::atomic_cxchg(p, v, v);
2019-10-28T11:12:53.8876554Z 
2019-10-28T11:12:53.8876554Z 
2019-10-28T11:12:53.8876601Z error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-10-28T11:12:53.8876878Z    |
2019-10-28T11:12:53.8876924Z LL |     intrinsics::atomic_load(p);
2019-10-28T11:12:53.8876965Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-10-28T11:12:53.8876990Z 
2019-10-28T11:12:53.8876990Z 
2019-10-28T11:12:53.8877054Z error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-10-28T11:12:53.8877317Z    |
2019-10-28T11:12:53.8877317Z    |
2019-10-28T11:12:53.8877364Z LL |     intrinsics::atomic_store(p, v);
2019-10-28T11:12:53.8877430Z 
2019-10-28T11:12:53.8877430Z 
2019-10-28T11:12:53.8877483Z error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-10-28T11:12:53.8877737Z    |
2019-10-28T11:12:53.8877737Z    |
2019-10-28T11:12:53.8877780Z LL |     intrinsics::atomic_xchg(p, v);
2019-10-28T11:12:53.8877846Z 
2019-10-28T11:12:53.8877846Z 
2019-10-28T11:12:53.8877902Z error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-10-28T11:12:53.8878177Z    |
2019-10-28T11:12:53.8878177Z    |
2019-10-28T11:12:53.8878217Z LL |     intrinsics::atomic_cxchg(p, v, v);
2019-10-28T11:12:53.8878299Z 
2019-10-28T11:12:53.8878299Z 
2019-10-28T11:12:53.8878345Z error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `[u8; 100]`
2019-10-28T11:12:53.8878773Z    |
2019-10-28T11:12:53.8878810Z LL |     intrinsics::atomic_load(p);
2019-10-28T11:12:53.8878848Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-10-28T11:12:53.8878880Z 
2019-10-28T11:12:53.8878880Z 
2019-10-28T11:12:53.8878924Z error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `[u8; 100]`
2019-10-28T11:12:53.8879188Z    |
2019-10-28T11:12:53.8879188Z    |
2019-10-28T11:12:53.8879226Z LL |     intrinsics::atomic_store(p, v);
2019-10-28T11:12:53.8879379Z 
2019-10-28T11:12:53.8879379Z 
2019-10-28T11:12:53.8879439Z error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `[u8; 100]`
2019-10-28T11:12:53.8879711Z    |
2019-10-28T11:12:53.8879711Z    |
2019-10-28T11:12:53.8879753Z LL |     intrinsics::atomic_xchg(p, v);
2019-10-28T11:12:53.8879817Z 
2019-10-28T11:12:53.8879817Z 
2019-10-28T11:12:53.8880506Z error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `[u8; 100]`
2019-10-28T11:12:53.8880877Z    |
2019-10-28T11:12:53.8880877Z    |
2019-10-28T11:12:53.8880918Z LL |     intrinsics::atomic_cxchg(p, v, v);
2019-10-28T11:12:53.8881131Z 
2019-10-28T11:12:53.8881170Z error: aborting due to 16 previous errors
2019-10-28T11:12:53.8881197Z 
2019-10-28T11:12:53.8881228Z 
2019-10-28T11:12:53.8881228Z 
2019-10-28T11:12:53.8881490Z ------------------------------------------
2019-10-28T11:12:53.8881519Z 
2019-10-28T11:12:53.8881542Z 
2019-10-28T11:12:53.8881940Z ---- [ui] ui/panic_implementation-closures.rs stdout ----
2019-10-28T11:12:53.8881999Z normalized stderr:
2019-10-28T11:12:53.8882044Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8882095Z 
2019-10-28T11:12:53.8882131Z 
2019-10-28T11:12:53.8882171Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8882459Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic_implementation-closures/panic_implementation-closures.stderr
2019-10-28T11:12:53.8882459Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic_implementation-closures/panic_implementation-closures.stderr
2019-10-28T11:12:53.8882918Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8883195Z To only update this specific test, also pass `--test-args panic_implementation-closures.rs`
2019-10-28T11:12:53.8883279Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8883814Z status: exit code: 0
2019-10-28T11:12:53.8883814Z status: exit code: 0
2019-10-28T11:12:53.8885173Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic_implementation-closures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic_implementation-closures" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic_implementation-closures/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8885660Z ------------------------------------------
2019-10-28T11:12:53.8885704Z 
2019-10-28T11:12:53.8885920Z ------------------------------------------
2019-10-28T11:12:53.8885963Z stderr:
2019-10-28T11:12:53.8885963Z stderr:
2019-10-28T11:12:53.8886153Z ------------------------------------------
2019-10-28T11:12:53.8886222Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8886618Z ------------------------------------------
2019-10-28T11:12:53.8886647Z 
2019-10-28T11:12:53.8886684Z 
2019-10-28T11:12:53.8886884Z ---- [ui] ui/proc-macro/no-missing-docs.rs stdout ----
2019-10-28T11:12:53.8886884Z ---- [ui] ui/proc-macro/no-missing-docs.rs stdout ----
2019-10-28T11:12:53.8886928Z normalized stderr:
2019-10-28T11:12:53.8886973Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8887038Z 
2019-10-28T11:12:53.8887060Z 
2019-10-28T11:12:53.8887101Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8887379Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/no-missing-docs/no-missing-docs.stderr
2019-10-28T11:12:53.8887379Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/no-missing-docs/no-missing-docs.stderr
2019-10-28T11:12:53.8887609Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8887984Z To only update this specific test, also pass `--test-args proc-macro/no-missing-docs.rs`
2019-10-28T11:12:53.8888083Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8888289Z status: exit code: 0
2019-10-28T11:12:53.8888289Z status: exit code: 0
2019-10-28T11:12:53.8888951Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/no-missing-docs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/no-missing-docs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/no-missing-docs/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8889326Z ------------------------------------------
2019-10-28T11:12:53.8889357Z 
2019-10-28T11:12:53.8889543Z ------------------------------------------
2019-10-28T11:12:53.8889580Z stderr:
2019-10-28T11:12:53.8889580Z stderr:
2019-10-28T11:12:53.8889776Z ------------------------------------------
2019-10-28T11:12:53.8889823Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8890051Z ------------------------------------------
2019-10-28T11:12:53.8890078Z 
2019-10-28T11:12:53.8890100Z 
2019-10-28T11:12:53.8890300Z ---- [ui] ui/proc-macro/proc-macro-deprecated-attr.rs stdout ----
2019-10-28T11:12:53.8890300Z ---- [ui] ui/proc-macro/proc-macro-deprecated-attr.rs stdout ----
2019-10-28T11:12:53.8890351Z normalized stderr:
2019-10-28T11:12:53.8890394Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8890443Z 
2019-10-28T11:12:53.8890477Z 
2019-10-28T11:12:53.8890517Z The actual stderr differed from the expected stderr.
2019-10-28T11:12:53.8890808Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-deprecated-attr/proc-macro-deprecated-attr.stderr
2019-10-28T11:12:53.8890808Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-deprecated-attr/proc-macro-deprecated-attr.stderr
2019-10-28T11:12:53.8891041Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T11:12:53.8891302Z To only update this specific test, also pass `--test-args proc-macro/proc-macro-deprecated-attr.rs`
2019-10-28T11:12:53.8891375Z error: 1 errors occurred comparing output.
2019-10-28T11:12:53.8891425Z status: exit code: 0
2019-10-28T11:12:53.8891425Z status: exit code: 0
2019-10-28T11:12:53.8892095Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/proc-macro-deprecated-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-deprecated-attr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-deprecated-attr/auxiliary" "-A" "unused"
2019-10-28T11:12:53.8892404Z ------------------------------------------
2019-10-28T11:12:53.8892435Z 
2019-10-28T11:12:53.8892647Z ------------------------------------------
2019-10-28T11:12:53.8892688Z stderr:
2019-10-28T11:12:53.8892688Z stderr:
2019-10-28T11:12:53.8892883Z ------------------------------------------
2019-10-28T11:12:53.8892937Z [src/librustc/ty/context.rs:1411] "generate_crate_metadata" = "generate_crate_metadata"
2019-10-28T11:12:53.8893169Z ------------------------------------------
2019-10-28T11:12:53.8893196Z 
2019-10-28T11:12:53.8893225Z 
2019-10-28T11:12:53.8893432Z ---- [ui] ui/rfc1717/missing-link-attr.rs stdout ----
2019-10-28T11:12:53.8893432Z ---- [ui] ui/rfc1717/missing-link-attr.rs stdout ----
2019-10-28T11:12:53.8893475Z diff of stderr:
2019-10-28T11:12:53.8893500Z 
---
2019-10-28T11:12:53.8927792Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-28T11:12:53.8927855Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-28T11:12:53.8927885Z 
2019-10-28T11:12:53.8927981Z 
2019-10-28T11:12:53.8929336Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-28T11:12:53.8929551Z 
2019-10-28T11:12:53.8929578Z 
2019-10-28T11:12:53.8929628Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-28T11:12:53.8929674Z Build completed unsuccessfully in 1:00:45
2019-10-28T11:12:53.8929674Z Build completed unsuccessfully in 1:00:45
2019-10-28T11:12:53.8929715Z == clock drift check ==
2019-10-28T11:12:53.8929764Z   local time: Mon Oct 28 11:12:53 UTC 2019
2019-10-28T11:12:54.0795796Z   network time: Mon, 28 Oct 2019 11:12:54 GMT
2019-10-28T11:12:54.0799224Z == end clock drift check ==
2019-10-28T11:12:55.1401999Z 
2019-10-28T11:12:55.1471234Z ##[error]Bash exited with code '1'.
2019-10-28T11:12:55.1507782Z ##[section]Starting: Checkout
2019-10-28T11:12:55.1509382Z ==============================================================================
2019-10-28T11:12:55.1509433Z Task         : Get sources
2019-10-28T11:12:55.1509493Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
