plain
2019-08-26T13:41:33.2100595Z 306 |         services: &BlobImageResources,
2019-08-26T13:41:33.2100661Z     |                    ^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn BlobImageResources`
2019-08-26T13:41:33.2100725Z 
2019-08-26T13:41:51.2543655Z    Compiling winit v0.16.2
2019-08-26T13:42:19.5339016Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:19.5346290Z    --> webrender/src/frame_builder.rs:111:37
2019-08-26T13:42:19.5350522Z     |
2019-08-26T13:42:19.5354787Z 111 |     #[cfg_attr(feature = "capture", serde(skip))] //TODO
2019-08-26T13:42:19.5362572Z     |
2019-08-26T13:42:19.5367128Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5367128Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5372073Z     = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:19.5374810Z 
2019-08-26T13:42:19.5379329Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:19.5404440Z     |
2019-08-26T13:42:19.5404440Z     |
2019-08-26T13:42:19.5404625Z 216 |     #[cfg_attr(any(feature = "capture", feature = "replay"), serde(skip))]
2019-08-26T13:42:19.5404925Z     |
2019-08-26T13:42:19.5405470Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5405470Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5405676Z     = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:19.5405782Z 
2019-08-26T13:42:19.5411643Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:19.5420254Z     |
2019-08-26T13:42:19.5420254Z     |
2019-08-26T13:42:19.5424669Z 336 |     #[cfg_attr(feature = "serde", serde(skip))]
2019-08-26T13:42:19.5433096Z     |
2019-08-26T13:42:19.5436823Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5436823Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5441515Z     = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:19.5444441Z 
2019-08-26T13:42:19.5500642Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:19.5502075Z     |
2019-08-26T13:42:19.5502075Z     |
2019-08-26T13:42:19.5502144Z 443 |     #[cfg_attr(feature = "serde", serde(skip))]
2019-08-26T13:42:19.5502510Z     |
2019-08-26T13:42:19.5502909Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5502909Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5503008Z     = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:19.5503248Z 
2019-08-26T13:42:19.5503315Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:19.5504121Z     |
2019-08-26T13:42:19.5504121Z     |
2019-08-26T13:42:19.5504372Z 447 |     #[cfg_attr(feature = "serde", serde(skip))]
2019-08-26T13:42:19.5504499Z     |
2019-08-26T13:42:19.5504735Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5504735Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5504824Z     = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:19.5504868Z 
2019-08-26T13:42:19.5544197Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:19.5553795Z    |
2019-08-26T13:42:19.5553795Z    |
2019-08-26T13:42:19.5557976Z 65 |     #[cfg_attr(feature = "capture", serde(skip))] //TODO
2019-08-26T13:42:19.5567921Z    |
2019-08-26T13:42:19.5573134Z    = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5573134Z    = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5576886Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:19.5580261Z 
2019-08-26T13:42:19.5613452Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:19.5614007Z      |
2019-08-26T13:42:19.5614007Z      |
2019-08-26T13:42:19.5614067Z 2157 |     #[cfg_attr(feature = "capture", serde(skip))]
2019-08-26T13:42:19.5614227Z      |
2019-08-26T13:42:19.5614523Z      = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5614523Z      = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5614609Z      = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:19.5614676Z 
2019-08-26T13:42:19.5614749Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:19.5615084Z      |
2019-08-26T13:42:19.5615084Z      |
2019-08-26T13:42:19.5615161Z 2219 |     #[cfg_attr(feature = "capture", serde(skip))] //TODO
2019-08-26T13:42:19.5615394Z      |
2019-08-26T13:42:19.5615657Z      = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5615657Z      = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5615765Z      = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:19.5615813Z 
2019-08-26T13:42:19.5770351Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:19.5781473Z    |
2019-08-26T13:42:19.5781473Z    |
2019-08-26T13:42:19.5786338Z 60 |     #[cfg_attr(feature = "replay", serde(default = "FrameId::first"))]
2019-08-26T13:42:19.5795941Z    |
2019-08-26T13:42:19.5800873Z    = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5800873Z    = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.5805759Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:19.5808404Z 
2019-08-26T13:42:19.5976158Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:19.5986186Z     |
2019-08-26T13:42:19.5986186Z     |
2019-08-26T13:42:19.5990648Z 496 |     #[cfg_attr(all(feature = "serde", any(feature = "capture", feature = "replay")), serde(skip))]
2019-08-26T13:42:19.5999240Z     |
2019-08-26T13:42:19.6004994Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.6004994Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.6009575Z     = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:19.6012625Z 
2019-08-26T13:42:19.6043205Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:19.6048077Z     --> webrender/src/tiling.rs:1202:62
2019-08-26T13:42:19.6053529Z      |
2019-08-26T13:42:19.6057661Z 1202 |     #[cfg_attr(any(feature = "capture", feature = "replay"), serde(default = "FrameProfileCounters::new", skip))]
2019-08-26T13:42:19.6066923Z      |
2019-08-26T13:42:19.6072289Z      = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.6072289Z      = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.6076838Z      = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:19.6079254Z 
2019-08-26T13:42:19.6138622Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:19.6139092Z     --> webrender/src/tiling.rs:1228:35
2019-08-26T13:42:19.6139783Z      |
2019-08-26T13:42:19.6139868Z 1228 |     #[cfg_attr(feature = "serde", serde(skip))]
2019-08-26T13:42:19.6140035Z      |
2019-08-26T13:42:19.6140425Z      = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.6140425Z      = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:19.6140542Z      = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:20.2244432Z warning: use of deprecated item 'try': use the `?` operator instead
2019-08-26T13:42:20.2249976Z    --> webrender/src/clip.rs:176:1
2019-08-26T13:42:20.2256365Z     |
2019-08-26T13:42:20.2261876Z 176 | / bitflags! {
---
2019-08-26T13:42:21.9654528Z     |
2019-08-26T13:42:21.9654672Z 713 |             let mut cbox: FT_BBox = mem::uninitialized();
2019-08-26T13:42:21.9654829Z     |                                     ^^^^^^^^^^^^^^^^^^
2019-08-26T13:42:21.9654963Z 
2019-08-26T13:42:35.8837508Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:35.8839300Z    --> webrender/src/frame_builder.rs:111:37
2019-08-26T13:42:35.8839688Z     |
2019-08-26T13:42:35.8839868Z 111 |     #[cfg_attr(feature = "capture", serde(skip))] //TODO
2019-08-26T13:42:35.8840244Z     |
2019-08-26T13:42:35.8840775Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.8840775Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.8840996Z     = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:35.8841156Z 
2019-08-26T13:42:35.8877149Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:35.8878551Z     |
2019-08-26T13:42:35.8878551Z     |
2019-08-26T13:42:35.8878795Z 216 |     #[cfg_attr(any(feature = "capture", feature = "replay"), serde(skip))]
2019-08-26T13:42:35.8879175Z     |
2019-08-26T13:42:35.8879677Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.8879677Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.8879925Z     = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:35.8880105Z 
2019-08-26T13:42:35.8923698Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:35.8924539Z     |
2019-08-26T13:42:35.8924539Z     |
2019-08-26T13:42:35.8924679Z 336 |     #[cfg_attr(feature = "serde", serde(skip))]
2019-08-26T13:42:35.8924963Z     |
2019-08-26T13:42:35.8925532Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.8925532Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.8925717Z     = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:35.8925830Z 
2019-08-26T13:42:35.8940428Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:35.8941703Z     |
2019-08-26T13:42:35.8941703Z     |
2019-08-26T13:42:35.8941841Z 443 |     #[cfg_attr(feature = "serde", serde(skip))]
2019-08-26T13:42:35.8942117Z     |
2019-08-26T13:42:35.8942495Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.8942495Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.8942698Z     = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:35.8942837Z 
2019-08-26T13:42:35.8947592Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:35.8948896Z     |
2019-08-26T13:42:35.8948896Z     |
2019-08-26T13:42:35.8949095Z 447 |     #[cfg_attr(feature = "serde", serde(skip))]
2019-08-26T13:42:35.8949432Z     |
2019-08-26T13:42:35.8949859Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.8949859Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.8950273Z     = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:35.8950417Z 
2019-08-26T13:42:35.9063528Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:35.9064480Z    |
2019-08-26T13:42:35.9064480Z    |
2019-08-26T13:42:35.9064620Z 65 |     #[cfg_attr(feature = "capture", serde(skip))] //TODO
2019-08-26T13:42:35.9064871Z    |
2019-08-26T13:42:35.9065266Z    = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.9065266Z    = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.9065438Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:35.9065543Z 
2019-08-26T13:42:35.9091460Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:35.9092935Z      |
2019-08-26T13:42:35.9092935Z      |
2019-08-26T13:42:35.9093091Z 2157 |     #[cfg_attr(feature = "capture", serde(skip))]
2019-08-26T13:42:35.9093656Z      |
2019-08-26T13:42:35.9094236Z      = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.9094236Z      = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.9094438Z      = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:35.9094586Z 
2019-08-26T13:42:35.9101502Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:35.9102588Z      |
2019-08-26T13:42:35.9102588Z      |
2019-08-26T13:42:35.9102891Z 2219 |     #[cfg_attr(feature = "capture", serde(skip))] //TODO
2019-08-26T13:42:35.9103420Z      |
2019-08-26T13:42:35.9104076Z      = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.9104076Z      = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.9104642Z      = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:35.9104780Z 
2019-08-26T13:42:35.9316447Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:35.9318271Z    |
2019-08-26T13:42:35.9318271Z    |
2019-08-26T13:42:35.9318506Z 60 |     #[cfg_attr(feature = "replay", serde(default = "FrameId::first"))]
2019-08-26T13:42:35.9318882Z    |
2019-08-26T13:42:35.9319430Z    = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.9319430Z    = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.9319697Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:35.9319858Z 
2019-08-26T13:42:35.9530161Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:35.9531273Z     |
2019-08-26T13:42:35.9531273Z     |
2019-08-26T13:42:35.9531694Z 496 |     #[cfg_attr(all(feature = "serde", any(feature = "capture", feature = "replay")), serde(skip))]
2019-08-26T13:42:35.9532448Z     |
2019-08-26T13:42:35.9533316Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.9533316Z     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.9533813Z     = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:35.9534111Z 
2019-08-26T13:42:35.9601915Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:35.9602448Z     --> webrender/src/tiling.rs:1202:62
2019-08-26T13:42:35.9602816Z      |
2019-08-26T13:42:35.9602983Z 1202 |     #[cfg_attr(any(feature = "capture", feature = "replay"), serde(default = "FrameProfileCounters::new", skip))]
2019-08-26T13:42:35.9603579Z      |
2019-08-26T13:42:35.9603993Z      = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.9603993Z      = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.9604243Z      = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:35.9604683Z 
2019-08-26T13:42:35.9611061Z error[E0658]: the attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-26T13:42:35.9611684Z     --> webrender/src/tiling.rs:1228:35
2019-08-26T13:42:35.9611747Z      |
2019-08-26T13:42:35.9611826Z 1228 |     #[cfg_attr(feature = "serde", serde(skip))]
2019-08-26T13:42:35.9611993Z      |
2019-08-26T13:42:35.9612271Z      = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.9612271Z      = note: for more information, see https://github.com/rust-lang/rust/issues/29642
2019-08-26T13:42:35.9612372Z      = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-26T13:42:36.5937293Z warning: use of deprecated item 'try': use the `?` operator instead
2019-08-26T13:42:36.5937670Z    --> webrender/src/clip.rs:176:1
2019-08-26T13:42:36.5938791Z     |
2019-08-26T13:42:36.5938880Z 176 | / bitflags! {
---
2019-08-26T13:42:38.4490856Z 
2019-08-26T13:42:44.9755895Z error: aborting due to 12 previous errors
2019-08-26T13:42:44.9756058Z 
2019-08-26T13:42:44.9757311Z For more information about this error, try `rustc --explain E0658`.
2019-08-26T13:42:45.1965990Z error: Could not compile `webrender`.
2019-08-26T13:43:02.2096952Z error: aborting due to 12 previous errors
2019-08-26T13:43:02.2097902Z 
2019-08-26T13:43:02.2099024Z For more information about this error, try `rustc --explain E0658`.
2019-08-26T13:43:02.2099024Z For more information about this error, try `rustc --explain E0658`.
2019-08-26T13:43:02.4469629Z error: Could not compile `webrender`.
2019-08-26T13:43:02.4470638Z To learn more, run the command again with --verbose.
2019-08-26T13:43:02.4496799Z thread 'main' panicked at 'tests failed for https://github.com/servo/webrender', src/tools/cargotest/main.rs:86:9
2019-08-26T13:43:02.4497392Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-26T13:43:02.4497518Z 
2019-08-26T13:43:02.4497518Z 
2019-08-26T13:43:02.4497557Z 
2019-08-26T13:43:02.4498118Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
2019-08-26T13:43:02.4498275Z expected success, got: exit code: 101
2019-08-26T13:43:02.4498323Z 
2019-08-26T13:43:02.4498374Z 
2019-08-26T13:43:02.4507400Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2019-08-26T13:43:02.4507568Z Build completed unsuccessfully in 2:06:56
2019-08-26T13:43:02.4556894Z Makefile:50: recipe for target 'check-aux' failed
2019-08-26T13:43:02.4557669Z make: *** [check-aux] Error 1
2019-08-26T13:43:02.4567592Z   local time: Mon Aug 26 13:43:02 UTC 2019
2019-08-26T13:43:02.5023206Z   network time: Mon, 26 Aug 2019 13:43:02 GMT
2019-08-26T13:43:02.5023410Z == end clock drift check ==
2019-08-26T13:43:02.5023410Z == end clock drift check ==
2019-08-26T13:43:04.0877747Z ##[error]Bash exited with code '2'.
2019-08-26T13:43:04.0923843Z ##[section]Starting: Upload CPU usage statistics
2019-08-26T13:43:04.0937893Z ==============================================================================
2019-08-26T13:43:04.0938012Z Task         : Bash
2019-08-26T13:43:04.0938100Z Description  : Run a Bash script on macOS, Linux, or Windows
