plain
2020-01-18T11:25:44.3027411Z ========================== Starting Command Output ===========================
2020-01-18T11:25:44.3031391Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/12e2c069-e84d-4a4b-8020-1ea2e9d0115c.sh
2020-01-18T11:25:44.3031584Z 
2020-01-18T11:25:44.3035199Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T11:25:44.3041757Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68335/merge to s
2020-01-18T11:25:44.3043383Z Task         : Get sources
2020-01-18T11:25:44.3043472Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T11:25:44.3043506Z Version      : 1.0.0
2020-01-18T11:25:44.3043540Z Author       : Microsoft
---
2020-01-18T11:25:45.1473958Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T11:25:45.1576743Z ##[command]git config gc.auto 0
2020-01-18T11:25:45.1662508Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T11:25:45.1729817Z ##[command]git config --get-all http.proxy
2020-01-18T11:25:45.1882603Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68335/merge:refs/remotes/pull/68335/merge
---
2020-01-18T12:21:43.2234557Z .................................................................................................... 1700/9535
2020-01-18T12:21:51.3419636Z .................................................................................................... 1800/9535
2020-01-18T12:22:01.3779183Z .............i...................................................................................... 1900/9535
2020-01-18T12:22:08.5546501Z .................................................................................................... 2000/9535
2020-01-18T12:22:24.6673323Z ...iiiii............................................................................................ 2100/9535
2020-01-18T12:22:33.4573443Z .................................................................................................... 2300/9535
2020-01-18T12:22:35.8989246Z .................................................................................................... 2400/9535
2020-01-18T12:22:41.6785903Z .................................................................................................... 2500/9535
2020-01-18T12:23:02.5445050Z .................................................................................................... 2600/9535
2020-01-18T12:23:02.5445050Z .................................................................................................... 2600/9535
2020-01-18T12:23:05.0942835Z .................................................................................................... 2700/9535
2020-01-18T12:23:16.0191732Z ............................................................................i....................... 2800/9535
2020-01-18T12:23:22.7134169Z .................................................................................................... 2900/9535
2020-01-18T12:23:30.8782426Z .................................................................................................... 3000/9535
2020-01-18T12:23:35.9471832Z .............i...................................................................................... 3100/9535
2020-01-18T12:23:45.1081367Z .................................................................................................... 3200/9535
2020-01-18T12:23:49.9087695Z .................................................................................................... 3300/9535
2020-01-18T12:23:57.9755154Z .ii................................................................................................. 3400/9535
2020-01-18T12:24:14.1701988Z ............................................................................................i....... 3600/9535
2020-01-18T12:24:21.4285015Z .......................................i............................................................ 3700/9535
2020-01-18T12:24:27.1826645Z .................................................................................................... 3800/9535
2020-01-18T12:24:32.8784062Z .................................................................................................... 3900/9535
---
2020-01-18T12:25:45.9471531Z ................................................i...............i................................... 4900/9535
2020-01-18T12:25:54.3916697Z .................................................................................................... 5000/9535
2020-01-18T12:26:01.9971449Z ...........................................................................................i........ 5100/9535
2020-01-18T12:26:07.1846735Z .................................................................................................... 5200/9535
2020-01-18T12:26:18.2166372Z ...............................................................ii.ii...........i.................... 5300/9535
2020-01-18T12:26:27.4847141Z i................................................................................................... 5500/9535
2020-01-18T12:26:38.0014449Z .................................................................................................... 5600/9535
2020-01-18T12:26:44.4016395Z .................................................i.................................................. 5700/9535
2020-01-18T12:26:51.5664473Z .................................................................................................... 5800/9535
2020-01-18T12:26:51.5664473Z .................................................................................................... 5800/9535
2020-01-18T12:27:02.1244469Z .................................................................................................... 5900/9535
2020-01-18T12:27:08.9082121Z ........................................ii...i..ii...........i...................................... 6000/9535
2020-01-18T12:27:31.4861139Z .................................................................................................... 6200/9535
2020-01-18T12:27:39.9674869Z .................................................................................................... 6300/9535
2020-01-18T12:27:39.9674869Z .................................................................................................... 6300/9535
2020-01-18T12:27:49.5289292Z ....................................................................i..ii........................... 6400/9535
2020-01-18T12:28:19.1116580Z .................................................................................................... 6600/9535
2020-01-18T12:28:21.4756750Z ............................................i....................................................... 6700/9535
2020-01-18T12:28:23.7103136Z .................................................................................................... 6800/9535
2020-01-18T12:28:26.2392787Z ............................................i....................................................... 6900/9535
---
2020-01-18T12:30:06.1751718Z .................................................................................................... 7500/9535
2020-01-18T12:30:11.0099305Z .................................................................................................... 7600/9535
2020-01-18T12:30:17.1738054Z .................................................................................................... 7700/9535
2020-01-18T12:30:23.7358559Z .................................................................................................... 7800/9535
2020-01-18T12:30:34.6308851Z ...............................................................................................iiiii 7900/9535
2020-01-18T12:30:41.4036250Z ii.................................................................................................. 8000/9535
2020-01-18T12:30:56.6126988Z .................................................................................................... 8200/9535
2020-01-18T12:31:08.7739390Z .................................................................................................... 8300/9535
2020-01-18T12:31:21.5167766Z .................................................................................................... 8400/9535
2020-01-18T12:31:27.5230961Z .................................................................................................... 8500/9535
---
2020-01-18T12:33:53.9857109Z  finished in 7.603
2020-01-18T12:33:54.0076698Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-18T12:33:54.2264073Z 
2020-01-18T12:33:54.2264197Z running 166 tests
2020-01-18T12:33:57.3504946Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-18T12:33:59.6965609Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-18T12:33:59.6970756Z 
2020-01-18T12:33:59.6976513Z  finished in 5.689
2020-01-18T12:33:59.7167810Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-18T12:33:59.8922226Z 
2020-01-18T12:33:59.8922226Z 
2020-01-18T12:33:59.8923658Z running 39 tests
2020-01-18T12:34:01.9208003Z i..F..F..Fi.F.......FF..F.F.F....i....F
2020-01-18T12:34:01.9208344Z 
2020-01-18T12:34:01.9208614Z ---- [codegen-units] codegen-units/item-collection/drop_in_place_intrinsic.rs stdout ----
2020-01-18T12:34:01.9208646Z 
2020-01-18T12:34:01.9208709Z These items should have been contained but were not:
2020-01-18T12:34:01.9208709Z These items should have been contained but were not:
2020-01-18T12:34:01.9208739Z 
2020-01-18T12:34:01.9209175Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<[drop_in_place_intrinsic::StructWithDtor[0]; 2]> @@ drop_in_place_intrinsic-cgu.0[Internal]
2020-01-18T12:34:01.9209464Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<[drop_in_place_intrinsic::StructWithDtor[0]]> @@ drop_in_place_intrinsic-cgu.0[Internal]
2020-01-18T12:34:01.9209735Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<drop_in_place_intrinsic::StructWithDtor[0]> @@ drop_in_place_intrinsic-cgu.0[Internal]
2020-01-18T12:34:01.9209814Z 
2020-01-18T12:34:01.9209835Z 
2020-01-18T12:34:01.9210310Z These items were contained but should not have been:
2020-01-18T12:34:01.9210355Z 
2020-01-18T12:34:01.9210355Z 
2020-01-18T12:34:01.9210764Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<[drop_in_place_intrinsic::StructWithDtor[0]; 2]> @@ drop_in_place_intrinsic.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9211108Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<drop_in_place_intrinsic::StructWithDtor[0]> @@ drop_in_place_intrinsic.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9211177Z 
2020-01-18T12:34:01.9211541Z thread '[codegen-units] codegen-units/item-collection/drop_in_place_intrinsic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-18T12:34:01.9211606Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-18T12:34:01.9211639Z 
2020-01-18T12:34:01.9211639Z 
2020-01-18T12:34:01.9213181Z ---- [codegen-units] codegen-units/item-collection/generic-drop-glue.rs stdout ----
2020-01-18T12:34:01.9215250Z 
2020-01-18T12:34:01.9215336Z These items should have been contained but were not:
2020-01-18T12:34:01.9215362Z 
2020-01-18T12:34:01.9215760Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<generic_drop_glue::EnumWithDrop[0]<f64, f32>> @@ generic_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9216196Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<generic_drop_glue::EnumWithDrop[0]<i32, i64>> @@ generic_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9216458Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<generic_drop_glue::NonGenericWithDrop[0]> @@ generic_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9216773Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<generic_drop_glue::StructNoDrop[0]<generic_drop_glue::NonGenericWithDrop[0], f64>> @@ generic_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9217068Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<generic_drop_glue::StructWithDrop[0]<&str, generic_drop_glue::NonGenericNoDrop[0]>> @@ generic_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9217550Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<generic_drop_glue::StructWithDrop[0]<i8, char>> @@ generic_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9217614Z 
2020-01-18T12:34:01.9217636Z 
2020-01-18T12:34:01.9217676Z These items were contained but should not have been:
2020-01-18T12:34:01.9217874Z 
2020-01-18T12:34:01.9217874Z 
2020-01-18T12:34:01.9218141Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::EnumWithDrop[0]<f64, f32>> @@ generic_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9218411Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::EnumWithDrop[0]<i32, i64>> @@ generic_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9218694Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::NonGenericWithDrop[0]> @@ generic_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9219000Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::StructNoDrop[0]<generic_drop_glue::NonGenericWithDrop[0], f64>> @@ generic_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9219301Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::StructWithDrop[0]<&str, generic_drop_glue::NonGenericNoDrop[0]>> @@ generic_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9219601Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::StructWithDrop[0]<i8, char>> @@ generic_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9219655Z 
2020-01-18T12:34:01.9220333Z thread '[codegen-units] codegen-units/item-collection/generic-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-18T12:34:01.9220379Z 
2020-01-18T12:34:01.9220973Z ---- [codegen-units] codegen-units/item-collection/instantiation-through-vtable.rs stdout ----
2020-01-18T12:34:01.9220973Z ---- [codegen-units] codegen-units/item-collection/instantiation-through-vtable.rs stdout ----
2020-01-18T12:34:01.9221014Z 
2020-01-18T12:34:01.9221084Z These items should have been contained but were not:
2020-01-18T12:34:01.9221116Z 
2020-01-18T12:34:01.9221472Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<instantiation_through_vtable::Struct[0]<u32>> @@ instantiation_through_vtable-cgu.0[Internal]
2020-01-18T12:34:01.9221840Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<instantiation_through_vtable::Struct[0]<u64>> @@ instantiation_through_vtable-cgu.0[Internal]
2020-01-18T12:34:01.9221917Z 
2020-01-18T12:34:01.9221944Z 
2020-01-18T12:34:01.9221993Z These items were contained but should not have been:
2020-01-18T12:34:01.9222043Z 
2020-01-18T12:34:01.9222043Z 
2020-01-18T12:34:01.9222384Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<instantiation_through_vtable::Struct[0]<u32>> @@ instantiation_through_vtable.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9222733Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<instantiation_through_vtable::Struct[0]<u64>> @@ instantiation_through_vtable.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9222820Z 
2020-01-18T12:34:01.9223286Z thread '[codegen-units] codegen-units/item-collection/instantiation-through-vtable.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-18T12:34:01.9223339Z 
2020-01-18T12:34:01.9223821Z ---- [codegen-units] codegen-units/item-collection/non-generic-drop-glue.rs stdout ----
2020-01-18T12:34:01.9223821Z ---- [codegen-units] codegen-units/item-collection/non-generic-drop-glue.rs stdout ----
2020-01-18T12:34:01.9224098Z 
2020-01-18T12:34:01.9224136Z These items should have been contained but were not:
2020-01-18T12:34:01.9224159Z 
2020-01-18T12:34:01.9224620Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<non_generic_drop_glue::EnumWithDrop[0]> @@ non_generic_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9224886Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<non_generic_drop_glue::StructWithDrop[0]> @@ non_generic_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9224944Z 
2020-01-18T12:34:01.9224982Z 
2020-01-18T12:34:01.9225019Z These items were contained but should not have been:
2020-01-18T12:34:01.9225045Z 
2020-01-18T12:34:01.9225045Z 
2020-01-18T12:34:01.9225301Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<non_generic_drop_glue::EnumWithDrop[0]> @@ non_generic_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9225596Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<non_generic_drop_glue::StructWithDrop[0]> @@ non_generic_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9225656Z 
2020-01-18T12:34:01.9225945Z thread '[codegen-units] codegen-units/item-collection/non-generic-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-18T12:34:01.9225978Z 
2020-01-18T12:34:01.9226192Z ---- [codegen-units] codegen-units/item-collection/transitive-drop-glue.rs stdout ----
2020-01-18T12:34:01.9226192Z ---- [codegen-units] codegen-units/item-collection/transitive-drop-glue.rs stdout ----
2020-01-18T12:34:01.9226221Z 
2020-01-18T12:34:01.9226275Z These items should have been contained but were not:
2020-01-18T12:34:01.9226300Z 
2020-01-18T12:34:01.9226560Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::IntermediateGen[0]<i16>> @@ transitive_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9226827Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::IntermediateGen[0]<u32>> @@ transitive_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9227330Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::Intermediate[0]> @@ transitive_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9227766Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::LeafGen[0]<i16>> @@ transitive_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9228052Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::LeafGen[0]<u32>> @@ transitive_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9228308Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::Leaf[0]> @@ transitive_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9228565Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::RootGen[0]<i16>> @@ transitive_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9228844Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::RootGen[0]<u32>> @@ transitive_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9229104Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::Root[0]> @@ transitive_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9229157Z 
2020-01-18T12:34:01.9229195Z 
2020-01-18T12:34:01.9229233Z These items were contained but should not have been:
2020-01-18T12:34:01.9229264Z 
2020-01-18T12:34:01.9229264Z 
2020-01-18T12:34:01.9229527Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::IntermediateGen[0]<i16>> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9229818Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::IntermediateGen[0]<u32>> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9230080Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::Intermediate[0]> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9230766Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::LeafGen[0]<i16>> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9231208Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::LeafGen[0]<u32>> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9231564Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::Leaf[0]> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9232014Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::RootGen[0]<i16>> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9232351Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::RootGen[0]<u32>> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9232676Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::Root[0]> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9232762Z 
2020-01-18T12:34:01.9233106Z thread '[codegen-units] codegen-units/item-collection/transitive-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-18T12:34:01.9233147Z 
2020-01-18T12:34:01.9233444Z ---- [codegen-units] codegen-units/item-collection/tuple-drop-glue.rs stdout ----
2020-01-18T12:34:01.9233444Z ---- [codegen-units] codegen-units/item-collection/tuple-drop-glue.rs stdout ----
2020-01-18T12:34:01.9233480Z 
2020-01-18T12:34:01.9233529Z These items should have been contained but were not:
2020-01-18T12:34:01.9233561Z 
2020-01-18T12:34:01.9234049Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<(i16, (tuple_drop_glue::Dropped[0], bool))> @@ tuple_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9234302Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<(tuple_drop_glue::Dropped[0], bool)> @@ tuple_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9234570Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<(u32, tuple_drop_glue::Dropped[0])> @@ tuple_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9235038Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<tuple_drop_glue::Dropped[0]> @@ tuple_drop_glue-cgu.0[Internal]
2020-01-18T12:34:01.9235094Z 
2020-01-18T12:34:01.9235117Z 
2020-01-18T12:34:01.9235174Z These items were contained but should not have been:
2020-01-18T12:34:01.9235200Z 
2020-01-18T12:34:01.9235200Z 
2020-01-18T12:34:01.9235480Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<(i16, (tuple_drop_glue::Dropped[0], bool))> @@ tuple_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9235773Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<(tuple_drop_glue::Dropped[0], bool)> @@ tuple_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9236055Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<(u32, tuple_drop_glue::Dropped[0])> @@ tuple_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9236320Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<tuple_drop_glue::Dropped[0]> @@ tuple_drop_glue.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9236393Z 
2020-01-18T12:34:01.9236679Z thread '[codegen-units] codegen-units/item-collection/tuple-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-18T12:34:01.9236715Z 
2020-01-18T12:34:01.9236951Z ---- [codegen-units] codegen-units/item-collection/unsizing.rs stdout ----
2020-01-18T12:34:01.9236951Z ---- [codegen-units] codegen-units/item-collection/unsizing.rs stdout ----
2020-01-18T12:34:01.9236981Z 
2020-01-18T12:34:01.9237028Z These items should have been contained but were not:
2020-01-18T12:34:01.9237055Z 
2020-01-18T12:34:01.9237489Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<bool> @@ unsizing-cgu.0[Internal]
2020-01-18T12:34:01.9237740Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<char> @@ unsizing-cgu.0[Internal]
2020-01-18T12:34:01.9238298Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<f64> @@ unsizing-cgu.0[Internal]
2020-01-18T12:34:01.9238536Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<u32> @@ unsizing-cgu.0[Internal]
2020-01-18T12:34:01.9238587Z 
2020-01-18T12:34:01.9238608Z 
2020-01-18T12:34:01.9238662Z These items were contained but should not have been:
2020-01-18T12:34:01.9238688Z 
2020-01-18T12:34:01.9238688Z 
2020-01-18T12:34:01.9238909Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<bool> @@ unsizing.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9239134Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<char> @@ unsizing.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9239442Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<f64> @@ unsizing.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9239693Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<u32> @@ unsizing.7rcbfp3g-cgu.0[Internal]
2020-01-18T12:34:01.9239808Z 
2020-01-18T12:34:01.9240493Z thread '[codegen-units] codegen-units/item-collection/unsizing.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-18T12:34:01.9240538Z 
2020-01-18T12:34:01.9240806Z ---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----
2020-01-18T12:34:01.9240806Z ---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----
2020-01-18T12:34:01.9240842Z 
2020-01-18T12:34:01.9240909Z These items should have been contained but were not:
2020-01-18T12:34:01.9240941Z 
2020-01-18T12:34:01.9241274Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<cgu_extern_drop_glue::Struct[0]> @@ extern_drop_glue[Internal] extern_drop_glue-mod1[Internal]
2020-01-18T12:34:01.9241358Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<extern_drop_glue::LocalStruct[0]> @@ extern_drop_glue[Internal]
2020-01-18T12:34:01.9241695Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<extern_drop_glue::mod1[0]::LocalStruct[0]> @@ extern_drop_glue-mod1[Internal]
2020-01-18T12:34:01.9241762Z 
2020-01-18T12:34:01.9241813Z 
2020-01-18T12:34:01.9241861Z These items were contained but should not have been:
2020-01-18T12:34:01.9241892Z 
2020-01-18T12:34:01.9241892Z 
2020-01-18T12:34:01.9242243Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<cgu_extern_drop_glue::Struct[0]> @@ extern_drop_glue.3a1fbbbh[Internal] extern_drop_glue.3a1fbbbh-mod1[Internal]
2020-01-18T12:34:01.9242331Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<extern_drop_glue::LocalStruct[0]> @@ extern_drop_glue.3a1fbbbh[Internal]
2020-01-18T12:34:01.9242659Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<extern_drop_glue::mod1[0]::LocalStruct[0]> @@ extern_drop_glue.3a1fbbbh-mod1[Internal]
2020-01-18T12:34:01.9242743Z 
2020-01-18T12:34:01.9243085Z thread '[codegen-units] codegen-units/partitioning/extern-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-18T12:34:01.9243127Z 
2020-01-18T12:34:01.9243392Z ---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
2020-01-18T12:34:01.9243392Z ---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
2020-01-18T12:34:01.9243446Z 
2020-01-18T12:34:01.9243501Z These items should have been contained but were not:
2020-01-18T12:34:01.9243533Z 
2020-01-18T12:34:01.9243955Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<(u32, local_drop_glue::Struct[0])> @@ local_drop_glue-mod1[Internal]
2020-01-18T12:34:01.9244024Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::Outer[0]> @@ local_drop_glue[Internal]
2020-01-18T12:34:01.9244289Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::Struct[0]> @@ local_drop_glue[Internal] local_drop_glue-mod1[Internal]
2020-01-18T12:34:01.9244560Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::mod1[0]::Struct2[0]> @@ local_drop_glue-mod1[Internal]
2020-01-18T12:34:01.9244613Z 
2020-01-18T12:34:01.9244634Z 
2020-01-18T12:34:01.9244679Z These items were contained but should not have been:
2020-01-18T12:34:01.9244722Z 
2020-01-18T12:34:01.9244722Z 
2020-01-18T12:34:01.9244973Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<(u32, local_drop_glue::Struct[0])> @@ local_drop_glue.3a1fbbbh-mod1[Internal]
2020-01-18T12:34:01.9245031Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<local_drop_glue::Outer[0]> @@ local_drop_glue.3a1fbbbh[Internal]
2020-01-18T12:34:01.9245325Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<local_drop_glue::Struct[0]> @@ local_drop_glue.3a1fbbbh[Internal] local_drop_glue.3a1fbbbh-mod1[Internal]
2020-01-18T12:34:01.9245584Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<local_drop_glue::mod1[0]::Struct2[0]> @@ local_drop_glue.3a1fbbbh-mod1[Internal]
2020-01-18T12:34:01.9245637Z 
2020-01-18T12:34:01.9245921Z thread '[codegen-units] codegen-units/partitioning/local-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-18T12:34:01.9245954Z 
2020-01-18T12:34:01.9246252Z ---- [codegen-units] codegen-units/partitioning/vtable-through-const.rs stdout ----
2020-01-18T12:34:01.9246252Z ---- [codegen-units] codegen-units/partitioning/vtable-through-const.rs stdout ----
2020-01-18T12:34:01.9246305Z 
2020-01-18T12:34:01.9246343Z These items should have been contained but were not:
2020-01-18T12:34:01.9246367Z 
2020-01-18T12:34:01.9246515Z MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<u32> @@ vtable_through_const[Internal]
2020-01-18T12:34:01.9246578Z 
2020-01-18T12:34:01.9246599Z 
2020-01-18T12:34:01.9246636Z These items were contained but should not have been:
2020-01-18T12:34:01.9246661Z 
2020-01-18T12:34:01.9246661Z 
2020-01-18T12:34:01.9246719Z MONO_ITEM fn core::ptr[0]::drop_in_place[0]<u32> @@ vtable_through_const.7rcbfp3g[Internal]
2020-01-18T12:34:01.9246768Z 
2020-01-18T12:34:01.9247419Z thread '[codegen-units] codegen-units/partitioning/vtable-through-const.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-18T12:34:01.9247474Z 
2020-01-18T12:34:01.9247496Z 
---
2020-01-18T12:34:01.9251251Z test result: FAILED. 26 passed; 10 failed; 3 ignored; 0 measured; 0 filtered out
2020-01-18T12:34:01.9251291Z 
2020-01-18T12:34:01.9251318Z 
2020-01-18T12:34:01.9251345Z 
2020-01-18T12:34:01.9253053Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-18T12:34:01.9253346Z 
2020-01-18T12:34:01.9253379Z 
2020-01-18T12:34:01.9253818Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-18T12:34:01.9254106Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-18T12:34:01.9254106Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-18T12:34:01.9254151Z Build completed unsuccessfully in 1:02:28
2020-01-18T12:34:01.9273235Z == clock drift check ==
2020-01-18T12:34:01.9296389Z   local time: Sat Jan 18 12:34:01 UTC 2020
2020-01-18T12:34:02.2277739Z   network time: Sat, 18 Jan 2020 12:34:02 GMT
2020-01-18T12:34:02.2291029Z == end clock drift check ==
2020-01-18T12:34:04.5662821Z 
2020-01-18T12:34:04.5776936Z ##[error]Bash exited with code '1'.
2020-01-18T12:34:04.5790155Z ##[section]Finishing: Run build
2020-01-18T12:34:04.5810938Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68335/merge to s
2020-01-18T12:34:04.5813061Z Task         : Get sources
2020-01-18T12:34:04.5813110Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T12:34:04.5813160Z Version      : 1.0.0
2020-01-18T12:34:04.5813222Z Author       : Microsoft
2020-01-18T12:34:04.5813222Z Author       : Microsoft
2020-01-18T12:34:04.5813273Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T12:34:04.5813325Z ==============================================================================
2020-01-18T12:34:05.0214299Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T12:34:05.0253297Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68335/merge to s
2020-01-18T12:34:05.0369362Z Cleaning up task key
2020-01-18T12:34:05.0370864Z Start cleaning up orphan processes.
2020-01-18T12:34:05.0509925Z Terminate orphan process: pid (3669) (python)
2020-01-18T12:34:05.0777014Z ##[section]Finishing: Finalize Job
