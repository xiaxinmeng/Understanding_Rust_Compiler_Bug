plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:055e3b93d15803815fe6f9cbc1b02b11be094e54)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
iiiiiiii.....................i..................i....................................... 264/14385
........................................................................................ 352/14385
........................................................................................ 440/14385
........................................................................................ 528/14385
........................................................................FF.............. 616/14385
...............F...........F.FF.............FFF............F...............F...F.......F 704/14385
.......F..................FF..FF...................F..F.F.......................FF...... 792/14385
...........F..F.F.F.F..F.....F...........F.FFF..FF.F.F.FFF..........F.F................. 880/14385
......................FF..i...F.......F................................................. 968/14385
........................................................................................ 1144/14385
........................................................................................ 1232/14385
........................................................................................ 1320/14385
........................................................................................ 1408/14385
---
..................................................iii................................... 4400/14385
................................................F....................................... 4488/14385
...........................i............................................................ 4576/14385
........................................................................................ 4664/14385
............F...............F.F..F...................................................... 4752/14385
........................................F............................................... 4928/14385
................................i....................................................... 5016/14385
........................................................................................ 5104/14385
..........................................F............................................. 5192/14385
---
---- [ui] tests/ui/associated-types/normalization-debruijn-1.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-types/normalization-debruijn-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/normalization-debruijn-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/normalization-debruijn-1/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued
error: internal compiler error: 
error: internal compiler error: 
                                ItemLocalIds not assigned densely in ::{impl#0}::from_request. Max ItemLocalId = 106, missing IDs = [
                                    "[local_id: 8, owner: ::{impl#0}::from_request]",
                                ]; seens IDs = [
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).0) method from_request in <Option<T> as FromRequest<'r>>::from_request (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).0)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).1) param request: &'r Request<'life0> (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).1)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).2) pat request (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).2)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).3) expr Box::pin(async move {\n            let request = request;\n            match T::from_request(request).await {\n                _ => todo!(),\n            }\n        }) (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).3)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).4) expr Box::pin (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).4)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).5) path segment Box (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).5)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).6) type Box (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).6)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).7) path segment pin (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).7)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).9) type async move {\n            let request = request;\n            match T::from_request(request).await {\n                _ => todo!(),\n            }\n        } (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).9)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).10) pat async move {\n            let request = request;\n            match T::from_request(request).await {\n                _ => todo!(),\n            }\n        } (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).10)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).11) param async move {\n            let request = request;\n            match T::from_request(request).await {\n                _ => todo!(),\n            }\n        } (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).11)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).12) stmt let request = request; (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).12)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).13) expr request (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).13)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).14) path segment request (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).14)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).15) local let request = request; (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).15)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).16) pat request (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).16)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).17) expr match T::from_request(request).await {\n                _ => todo!(),\n            } (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).17)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).18) expr T::from_request(request).await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).18)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).19) expr T::from_request(request) (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).19)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).20) expr T::from_request (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).20)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).21) path segment T (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).21)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).22) type T (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).22)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).23) path segment from_request (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).23)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).24) expr request (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).24)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).25) path segment request (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).25)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).26) pat .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).26)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).27) path segment  (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).27)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).28) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).28)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).29) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).29)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).30) path segment  (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).30)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).31) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).31)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).32) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).32)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).33) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).33)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).34) expr T::from_request(request).await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).34)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).35) expr T::from_request(request).await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).35)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).36) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).36)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).37) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).37)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).38) block .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).38)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).39) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).39)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).40) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).40)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).41) pat T::from_request(request).await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).41)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).42) path segment  (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).42)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).43) expr T::from_request(request).await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).43)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).44) pattern field T::from_request(request).await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).44)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).45) pat .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).45)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).46) expr T::from_request(request).await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).46)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).47) arm T::from_request(request).await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).47)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).48) pat .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).48)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).49) block .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).49)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).50) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).50)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).51) arm .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).51)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).52) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).52)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).53) stmt .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).53)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).54) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).54)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).55) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).55)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).56) path segment  (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).56)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).57) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).57)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).58) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).58)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).59) stmt .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).59)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).60) block .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).60)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).61) arm .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).61)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).62) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).62)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).63) expr .await (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).63)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).64) pat _ (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).64)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).65) arm _ => todo!() (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).65)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).66) expr  (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).66)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).67) expr  (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).67)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).68) path segment  (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).68)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).69) path segment  (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).69)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).70) path segment  (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).70)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).71) expr  (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).71)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).72) block {\n            let request = request;\n            match T::from_request(request).await {\n                _ => todo!(),\n            }\n        } (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).72)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).73) expr {\n            let request = request;\n            match T::from_request(request).await {\n                _ => todo!(),\n            }\n        } (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).73)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).74) expr async move {\n            let request = request;\n            match T::from_request(request).await {\n                _ => todo!(),\n            }\n        } (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).74)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).75) stmt Box::pin(async move {\n            let request = request;\n            match T::from_request(request).await {\n                _ => todo!(),\n            }\n        }); (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).75)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).76) expr  (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).76)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).77) expr  (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).77)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).78) path segment  (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).78)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).79) path segment  (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).79)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).80) path segment  (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).80)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).81) expr  (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).81)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).82) block {\n        Box::pin(async move {\n            let request = request;\n            match T::from_request(request).await {\n                _ => todo!(),\n            }\n        });\n        todo!()\n    } (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).82)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).83) expr {\n        Box::pin(async move {\n            let request = request;\n            match T::from_request(request).await {\n                _ => todo!(),\n            }\n        });\n        todo!()\n    } (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).83)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).84) generic_param <Option<T> as FromRequest<'r>>::from_request::'life0 (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).84)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).85) lifetime 'r (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).85)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).86) type Request<'life0> (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).86)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).87) lifetime 'life0 (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).87)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).88) path segment Request<'life0> (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).88)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).89) type &'r Request<'life0> (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).89)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).90) type Pin<Box<dyn Future<Output = Outcome<Self, Self::Error>>>> (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).90)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).91) type Box<dyn Future<Output = Outcome<Self, Self::Error>>> (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).91)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).92) type Outcome<Self, Self::Error> (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).92)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).93) type Self (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).93)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).94) path segment Self (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).94)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).95) type Self::Error (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).95)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).96) path segment Self (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).96)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).97) type Self (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).97)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).98) path segment Error (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).98)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).99) path segment Outcome<Self, Self::Error> (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).99)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).100) type binding Output = Outcome<Self, Self::Error> (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).100)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).101) path segment Future<Output = Outcome<Self, Self::Error>> (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).101)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).102) trait ref Future<Output = Outcome<Self, Self::Error>> (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).102)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).103) lifetime dyn Future<Output = Outcome<Self, Self::Error>> (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).103)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).104) type dyn Future<Output = Outcome<Self, Self::Error>> (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).104)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).105) path segment Box<dyn Future<Output = Outcome<Self, Self::Error>>> (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).105)))",
                                    "(HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).106) path segment Pin<Box<dyn Future<Output = Outcome<Self, Self::Error>>>> (hir_id=HirId(DefId(0:23 ~ normalization_debruijn_1[7f93]::{impl#0}::from_request).106)))",
   |
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
              1: <rustc_session::session::Session>::delay_span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>
              2: rustc_passes::hir_id_validator::check_crate
              2: rustc_passes::hir_id_validator::check_crate
              3: rustc_interface::passes::analysis
              4: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
              5: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt, rustc_middle::dep_graph::dep_node::DepKind>
              6: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
              7: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
              8: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
              9: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
             10: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             11: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             12: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             14: <unknown>
             15: <unknown>
           


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (20ac38a1b 2023-01-31) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to 2 previous errors
------------------------------------------
------------------------------------------


---- [ui] tests/ui/associated-types/normalization-debruijn-3.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-types/normalization-debruijn-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/normalization-debruijn-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/normalization-debruijn-3/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued
error: internal compiler error: 
error: internal compiler error: 
                                ItemLocalIds not assigned densely in ::main. Max ItemLocalId = 71, missing IDs = [
                                    "[local_id: 2, owner: ::main]",
                                ]; seens IDs = [
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).0) fn main (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).0)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).1) stmt let _ = async {\n        let server = Server;\n        let verification_route = server.and_then(read);\n        run(verification_route).await;\n    }; (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).1)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).3) type async {\n        let server = Server;\n        let verification_route = server.and_then(read);\n        run(verification_route).await;\n    } (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).3)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).4) pat async {\n        let server = Server;\n        let verification_route = server.and_then(read);\n        run(verification_route).await;\n    } (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).4)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).5) param async {\n        let server = Server;\n        let verification_route = server.and_then(read);\n        run(verification_route).await;\n    } (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).5)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).6) stmt let server = Server; (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).6)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).7) expr Server (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).7)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).8) path segment Server (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).8)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).9) local let server = Server; (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).9)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).10) pat server (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).10)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).11) stmt let verification_route = server.and_then(read); (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).11)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).12) expr server.and_then(read) (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).12)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).13) path segment and_then (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).13)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).14) expr server (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).14)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).15) path segment server (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).15)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).16) expr read (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).16)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).17) path segment read (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).17)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).18) local let verification_route = server.and_then(read); (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).18)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).19) pat verification_route (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).19)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).20) expr run(verification_route).await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).20)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).21) expr run(verification_route) (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).21)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).22) expr run (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).22)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).23) path segment run (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).23)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).24) expr verification_route (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).24)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).25) path segment verification_route (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).25)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).26) pat .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).26)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).27) path segment  (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).27)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).28) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).28)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).29) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).29)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).30) path segment  (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).30)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).31) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).31)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).32) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).32)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).33) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).33)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).34) expr run(verification_route).await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).34)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).35) expr run(verification_route).await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).35)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).36) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).36)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).37) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).37)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).38) block .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).38)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).39) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).39)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).40) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).40)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).41) pat run(verification_route).await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).41)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).42) path segment  (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).42)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).43) expr run(verification_route).await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).43)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).44) pattern field run(verification_route).await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).44)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).45) pat .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).45)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).46) expr run(verification_route).await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).46)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).47) arm run(verification_route).await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).47)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).48) pat .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).48)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).49) block .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).49)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).50) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).50)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).51) arm .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).51)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).52) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).52)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).53) stmt .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).53)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).54) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).54)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).55) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).55)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).56) path segment  (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).56)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).57) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).57)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).58) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).58)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).59) stmt .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).59)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).60) block .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).60)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).61) arm .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).61)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).62) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).62)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).63) expr .await (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).63)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).64) stmt run(verification_route).await; (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).64)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).65) block {\n        let server = Server;\n        let verification_route = server.and_then(read);\n        run(verification_route).await;\n    } (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).65)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).66) expr {\n        let server = Server;\n        let verification_route = server.and_then(read);\n        run(verification_route).await;\n    } (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).66)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).67) expr async {\n        let server = Server;\n        let verification_route = server.and_then(read);\n        run(verification_route).await;\n    } (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).67)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).68) local let _ = async {\n        let server = Server;\n        let verification_route = server.and_then(read);\n        run(verification_route).await;\n    }; (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).68)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).69) pat _ (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).69)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).70) block {\n    let _ = async {\n        let server = Server;\n        let verification_route = server.and_then(read);\n        run(verification_route).await;\n    };\n} (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).70)))",
                                    "(HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).71) expr {\n    let _ = async {\n        let server = Server;\n        let verification_route = server.and_then(read);\n        run(verification_route).await;\n    };\n} (hir_id=HirId(DefId(0:28 ~ normalization_debruijn_3[60d8]::main).71)))",
   |
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
              1: <rustc_session::session::Session>::delay_span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>
              2: rustc_passes::hir_id_validator::check_crate
              2: rustc_passes::hir_id_validator::check_crate
              3: rustc_interface::passes::analysis
              4: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
              5: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt, rustc_middle::dep_graph::dep_node::DepKind>
              6: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
              7: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
              8: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
              9: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
             10: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             11: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             12: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             14: <unknown>
             15: <unknown>
           


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (20ac38a1b 2023-01-31) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to 2 previous errors
------------------------------------------
------------------------------------------


---- [ui] tests/ui/async-await/async-borrowck-escaping-block-error.rs stdout ----

error: failed to compile fixed code
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/async-borrowck-escaping-block-error.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-borrowck-escaping-block-error/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-borrowck-escaping-block-error/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued
error: internal compiler error: 
error: internal compiler error: 
                                ItemLocalIds not assigned densely in ::test_boxed. Max ItemLocalId = 23, missing IDs = [
                                    "[local_id: 10, owner: ::test_boxed]",
                                ]; seens IDs = [
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).0) fn test_boxed (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).0)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).1) stmt let x = 0u32; (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).1)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).2) expr 0u32 (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).2)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).3) local let x = 0u32; (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).3)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).4) pat x (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).4)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).5) expr Box::new(async move { x } ) (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).5)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).6) expr Box::new (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).6)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).7) path segment Box (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).7)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).8) type Box (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).8)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).9) path segment new (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).9)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).11) type async move { x } (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).11)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).12) pat async move { x } (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).12)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).13) param async move { x } (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).13)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).14) expr x (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).14)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).15) path segment x (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).15)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).16) block { x } (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).16)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).17) expr { x } (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).17)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).18) expr async move { x } (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).18)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).19) block {\n    let x = 0u32;\n    Box::new(async move { x } )\n    //~^ ERROR E0373\n} (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).19)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).20) expr {\n    let x = 0u32;\n    Box::new(async move { x } )\n    //~^ ERROR E0373\n} (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).20)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).21) type Box<impl std::future::Future<Output = u32>> (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).21)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).22) type impl std::future::Future<Output = u32> (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).22)))",
                                    "(HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).23) path segment Box<impl std::future::Future<Output = u32>> (hir_id=HirId(DefId(0:3 ~ async_borrowck_escaping_block_error[24da]::test_boxed).23)))",
                                ]
                                ItemLocalIds not assigned densely in ::test_ref. Max ItemLocalId = 21, missing IDs = [
                                    "[local_id: 3, owner: ::test_ref]",
                                ]; seens IDs = [
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).0) fn test_ref (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).0)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).1) param x: &u32 (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).1)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).2) pat x (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).2)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).4) type async move { *x } (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).4)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).5) pat async move { *x } (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).5)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).6) param async move { *x } (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).6)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).7) expr *x (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).7)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).8) expr x (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).8)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).9) path segment x (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).9)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).10) block { *x } (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).10)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).11) expr { *x } (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).11)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).12) expr async move { *x } (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).12)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).13) block {\n    async move { *x }\n    //~^ ERROR E0373\n} (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).13)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).14) expr {\n    async move { *x }\n    //~^ ERROR E0373\n} (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).14)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).15) generic_param test_ref::'_ (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).15)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).16) lifetime  (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).16)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).17) type u32 (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).17)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).18) path segment u32 (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).18)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).19) type &u32 (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).19)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).20) lifetime '_ (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).20)))",
                                    "(HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).21) type impl std::future::Future<Output = u32> + '_ (hir_id=HirId(DefId(0:5 ~ async_borrowck_escaping_block_error[24da]::test_ref).21)))",
   |
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
              1: <rustc_session::session::Session>::delay_span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>
              2: rustc_passes::hir_id_validator::check_crate
              2: rustc_passes::hir_id_validator::check_crate
              3: rustc_interface::passes::analysis
              4: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
              5: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt, rustc_middle::dep_graph::dep_node::DepKind>
              6: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
              7: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
              8: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
              9: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
             10: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             11: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             12: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             14: <unknown>
             15: <unknown>
           


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (20ac38a1b 2023-01-31) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to 2 previous errors
------------------------------------------
------------------------------------------


---- [ui] tests/ui/async-await/async-await.rs#thirunsafeck stdout ----

error in revision `thirunsafeck`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thirunsafeck" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.thirunsafeck/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.thirunsafeck/auxiliary" "-Zthir-unsafeck" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued
error: internal compiler error: 
error: internal compiler error: 
                                ItemLocalIds not assigned densely in ::async_block. Max ItemLocalId = 59, missing IDs = [
                                    "[local_id: 3, owner: ::async_block]",
                                ]; seens IDs = [
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).0) fn async_block (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).0)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).1) param x: u8 (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).1)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).2) pat x (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).2)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).4) type async move {\n        wake_and_yield_once().await;\n        x\n    } (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).4)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).5) pat async move {\n        wake_and_yield_once().await;\n        x\n    } (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).5)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).6) param async move {\n        wake_and_yield_once().await;\n        x\n    } (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).6)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).7) expr wake_and_yield_once().await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).7)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).8) expr wake_and_yield_once() (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).8)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).9) expr wake_and_yield_once (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).9)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).10) path segment wake_and_yield_once (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).10)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).11) pat .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).11)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).12) path segment  (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).12)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).13) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).13)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).14) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).14)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).15) path segment  (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).15)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).16) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).16)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).17) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).17)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).18) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).18)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).19) expr wake_and_yield_once().await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).19)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).20) expr wake_and_yield_once().await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).20)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).21) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).21)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).22) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).22)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).23) block .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).23)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).24) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).24)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).25) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).25)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).26) pat wake_and_yield_once().await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).26)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).27) path segment  (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).27)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).28) expr wake_and_yield_once().await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).28)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).29) pattern field wake_and_yield_once().await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).29)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).30) pat .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).30)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).31) expr wake_and_yield_once().await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).31)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).32) arm wake_and_yield_once().await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).32)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).33) pat .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).33)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).34) block .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).34)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).35) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).35)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).36) arm .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).36)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).37) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).37)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).38) stmt .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).38)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).39) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).39)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).40) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).40)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).41) path segment  (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).41)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).42) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).42)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).43) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).43)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).44) stmt .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).44)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).45) block .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).45)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).46) arm .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).46)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).47) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).47)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).48) expr .await (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).48)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).49) stmt wake_and_yield_once().await; (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).49)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).50) expr x (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).50)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).51) path segment x (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).51)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).52) block {\n        wake_and_yield_once().await;\n        x\n    } (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).52)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).53) expr {\n        wake_and_yield_once().await;\n        x\n    } (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).53)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).54) expr async move {\n        wake_and_yield_once().await;\n        x\n    } (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).54)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).55) block {\n    async move {\n        wake_and_yield_once().await;\n        x\n    }\n} (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).55)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).56) expr {\n    async move {\n        wake_and_yield_once().await;\n        x\n    }\n} (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).56)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).57) type u8 (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).57)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).58) path segment u8 (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).58)))",
                                    "(HirId(DefId(0:27 ~ async_await[eee1]::async_block).59) type impl Future<Output = u8> (hir_id=HirId(DefId(0:27 ~ async_await[eee1]::async_block).59)))",
                                ]
                                ItemLocalIds not assigned densely in ::async_block_with_borrow_named_lifetime. Max ItemLocalId = 64, missing IDs = [
                                    "[local_id: 3, owner: ::async_block_with_borrow_named_lifetime]",
                                ]; seens IDs = [
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).0) fn async_block_with_borrow_named_lifetime (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).0)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).1) param x: &'a u8 (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).1)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).2) pat x (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).2)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).4) type async move {\n        wake_and_yield_once().await;\n        *x\n    } (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).4)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).5) pat async move {\n        wake_and_yield_once().await;\n        *x\n    } (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).5)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).6) param async move {\n        wake_and_yield_once().await;\n        *x\n    } (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).6)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).7) expr wake_and_yield_once().await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).7)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).8) expr wake_and_yield_once() (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).8)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).9) expr wake_and_yield_once (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).9)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).10) path segment wake_and_yield_once (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).10)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).11) pat .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).11)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).12) path segment  (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).12)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).13) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).13)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).14) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).14)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).15) path segment  (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).15)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).16) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).16)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).17) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).17)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).18) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).18)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).19) expr wake_and_yield_once().await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).19)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).20) expr wake_and_yield_once().await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).20)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).21) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).21)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).22) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).22)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).23) block .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).23)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).24) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).24)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).25) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).25)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).26) pat wake_and_yield_once().await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).26)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).27) path segment  (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).27)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).28) expr wake_and_yield_once().await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).28)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).29) pattern field wake_and_yield_once().await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).29)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).30) pat .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).30)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).31) expr wake_and_yield_once().await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).31)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).32) arm wake_and_yield_once().await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).32)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).33) pat .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).33)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).34) block .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).34)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).35) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).35)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).36) arm .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).36)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).37) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).37)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).38) stmt .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).38)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).39) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).39)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).40) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).40)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).41) path segment  (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).41)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).42) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).42)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).43) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).43)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).44) stmt .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).44)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).45) block .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).45)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).46) arm .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).46)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).47) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).47)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).48) expr .await (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).48)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).49) stmt wake_and_yield_once().await; (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).49)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).50) expr *x (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).50)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).51) expr x (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).51)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).52) path segment x (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).52)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).53) block {\n        wake_and_yield_once().await;\n        *x\n    } (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).53)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).54) expr {\n        wake_and_yield_once().await;\n        *x\n    } (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).54)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).55) expr async move {\n        wake_and_yield_once().await;\n        *x\n    } (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).55)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).56) block {\n    async move {\n        wake_and_yield_once().await;\n        *x\n    }\n} (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).56)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).57) expr {\n    async move {\n        wake_and_yield_once().await;\n        *x\n    }\n} (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).57)))",
                                    "(HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).58) generic_param async_block_with_borrow_named_lifetime::'a (hir_id=HirId(DefId(0:29 ~ async_await[eee1]::async_block_with_borrow_named_lifetime).58)))",
---
Future breakage diagnostic:
warning: trailing semicolon in macro used in expression position
  --> fake-test-src-base/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:9:13
   |
LL |         true; //~  WARN trailing semicolon in macro
...
...
LL |         let _ = foo!(third);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>
note: the lint level is defined here
---
Future breakage diagnostic:
warning: trailing semicolon in macro used in expression position
  --> fake-test-src-base/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:9:13
   |
LL |         true; //~  WARN trailing semicolon in macro
...
...
LL |         let _ = foo!(fourth);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>
note: the lint level is defined here
---
Future breakage diagnostic:
warning: trailing semicolon in macro used in expression position
  --> fake-test-src-base/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:9:13
   |
LL |         true; //~  WARN trailing semicolon in macro
...
...
LL |         foo!(warn_in_block)
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>
   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>
   = note: macro invocations at the end of a block are treated as expressions
   = note: to ignore the value produced by the macro, add a semicolon after the invocation of `foo`
  --> fake-test-src-base/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:4:9
   |
LL | #![warn(semicolon_in_expressions_from_macros)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this warning originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)

Future breakage diagnostic:
warning: trailing semicolon in macro used in expression position
  --> fake-test-src-base/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:9:13
   |
LL |         true; //~  WARN trailing semicolon in macro
...
...
LL |     let _ = foo!(warn_in_expr);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>
note: the lint level is defined here
---
Future breakage diagnostic:
warning: trailing semicolon in macro used in expression position
  --> fake-test-src-base/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:9:13
   |
LL |         true; //~  WARN trailing semicolon in macro
...
...
LL |     let _ = #[allow(semicolon_in_expressions_from_macros)] foo!(allow_does_not_work);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>
note: the lint level is defined here
---
---- [ui] tests/ui/liveness/liveness-upvars.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/liveness/liveness-upvars.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-upvars" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-upvars/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
warning: value assigned to `last` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:10:9
   |
LL |         last = Some(s); //~  WARN value assigned to `last` is never read
   |
   |
   = help: maybe it is overwritten before being read?
  --> fake-test-src-base/liveness/liveness-upvars.rs:4:9
   |
LL | #![warn(unused)]
   |         ^^^^^^
   |         ^^^^^^
   = note: `#[warn(unused_assignments)]` implied by `#[warn(unused)]`

warning: unused variable: `last`
  --> fake-test-src-base/liveness/liveness-upvars.rs:10:9
   |
LL |         last = Some(s); //~  WARN value assigned to `last` is never read
   |
   = help: did you mean to capture by reference instead?
   = note: `#[warn(unused_variables)]` implied by `#[warn(unused)]`


warning: unused variable: `sum`
  --> fake-test-src-base/liveness/liveness-upvars.rs:22:9
   |
LL |         sum += x; //~ WARN unused variable: `sum`
   |
   = help: did you mean to capture by reference instead?

warning: value captured by `c` is never read
warning: value captured by `c` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:32:9
   |
LL |         c = 1; //~ WARN value captured by `c` is never read
   |
   = help: did you mean to capture by reference instead?

warning: value captured by `c` is never read
warning: value captured by `c` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:36:9
   |
LL |         c = 1; //~ WARN value captured by `c` is never read
   |
   = help: did you mean to capture by reference instead?

warning: unused variable: `c`
warning: unused variable: `c`
  --> fake-test-src-base/liveness/liveness-upvars.rs:42:9
   |
LL |         c += 1; //~ WARN unused variable: `c`
   |
   = help: did you mean to capture by reference instead?

warning: value assigned to `c` is never read
warning: value assigned to `c` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:45:9
   |
LL |         c += 1; //~  WARN value assigned to `c` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: unused variable: `c`
  --> fake-test-src-base/liveness/liveness-upvars.rs:45:9
   |
   |
LL |         c += 1; //~  WARN value assigned to `c` is never read
   |
   = help: did you mean to capture by reference instead?

warning: value assigned to `c` is never read
warning: value assigned to `c` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:58:9
   |
LL |         c += 1; //~  WARN value assigned to `c` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: value assigned to `c` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:64:9
   |
   |
LL |         c += 1; //~  WARN value assigned to `c` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: value assigned to `d` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:73:13
   |
   |
LL |             d = Some("d1"); //~ WARN value assigned to `d` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: value assigned to `e` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:77:13
   |
   |
LL |             e = Some("e1"); //~  WARN value assigned to `e` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: value assigned to `e` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:79:13
   |
   |
LL |             e = Some("e2"); //~  WARN value assigned to `e` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: unused variable: `e`
  --> fake-test-src-base/liveness/liveness-upvars.rs:77:13
   |
   |
LL |             e = Some("e1"); //~  WARN value assigned to `e` is never read
   |
   = help: did you mean to capture by reference instead?


warning: value assigned to `v` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:87:13
   |
LL |             v = T::default(); //~ WARN value assigned to `v` is never read
   |
   |
   = help: maybe it is overwritten before being read?

warning: value assigned to `z` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:99:17
   |
LL |                 z = T::default(); //~  WARN value assigned to `z` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: unused variable: `z`
  --> fake-test-src-base/liveness/liveness-upvars.rs:99:17
   |
   |
LL |                 z = T::default(); //~  WARN value assigned to `z` is never read
   |
   = help: did you mean to capture by reference instead?

warning: value assigned to `state` is never read
warning: value assigned to `state` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:125:9
   |
LL |         state = 4;  //~  WARN value assigned to `state` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: value assigned to `state` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:128:9
   |
   |
LL |         state = 5;  //~ WARN value assigned to `state` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: unused variable: `state`
  --> fake-test-src-base/liveness/liveness-upvars.rs:125:9
   |
   |
LL |         state = 4;  //~  WARN value assigned to `state` is never read
   |
   = help: did you mean to capture by reference instead?

warning: value assigned to `s` is never read
warning: value assigned to `s` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:137:9
   |
LL |         s = 1; //~ WARN value assigned to `s` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: value assigned to `s` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:139:9
   |
   |
LL |         s = yield (); //~ WARN value assigned to `s` is never read
   |
   |
   = help: maybe it is overwritten before being read?

error: internal compiler error: no errors encountered even though `delay_span_bug` issued
error: internal compiler error: 
error: internal compiler error: 
                                ItemLocalIds not assigned densely in ::f. Max ItemLocalId = 240, missing IDs = [
                                    "[local_id: 43, owner: ::f]",
                                    "[local_id: 95, owner: ::f]",
                                    "[local_id: 199, owner: ::f]",
                                ]; seens IDs = [
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).0) fn f (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).0)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).1) stmt let mut c = 0; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).1)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).2) expr 0 (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).2)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).3) local let mut c = 0; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).3)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).4) pat mut c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).4)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).5) stmt let _ = move || {\n        c = 1; //~ WARN value captured by `c` is never read\n        println!(\"{}\", c);\n    }; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).5)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).6) expr move || (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).6)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).7) expr {\n        c = 1; //~ WARN value captured by `c` is never read\n        println!(\"{}\", c);\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).7)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).8) expr c = 1 (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).8)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).9) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).9)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).10) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).10)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).11) expr 1 (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).11)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).12) stmt c = 1; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).12)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).13) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).13)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).14) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).14)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).15) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).15)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).16) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).16)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).17) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).17)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).18) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).18)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).19) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).19)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).20) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).20)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).21) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).21)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).22) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).22)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).23) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).23)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).24) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).24)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).25) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).25)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).26) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).26)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).27) type c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).27)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).28) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).28)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).29) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).29)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).30) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).30)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).31) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).31)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).32) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).32)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).33) type  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).33)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).34) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).34)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).35) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).35)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).36) stmt  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).36)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).37) block  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).37)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).38) stmt  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).38)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).39) block {\n        c = 1; //~ WARN value captured by `c` is never read\n        println!(\"{}\", c);\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).39)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).40) local let _ = move || {\n        c = 1; //~ WARN value captured by `c` is never read\n        println!(\"{}\", c);\n    }; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).40)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).41) pat _ (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).41)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).42) stmt let _ = async move {\n        c = 1; //~ WARN value captured by `c` is never read\n        println!(\"{}\", c);\n    }; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).42)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).44) type async move {\n        c = 1; //~ WARN value captured by `c` is never read\n        println!(\"{}\", c);\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).44)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).45) pat async move {\n        c = 1; //~ WARN value captured by `c` is never read\n        println!(\"{}\", c);\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).45)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).46) param async move {\n        c = 1; //~ WARN value captured by `c` is never read\n        println!(\"{}\", c);\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).46)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).47) expr c = 1 (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).47)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).48) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).48)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).49) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).49)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).50) expr 1 (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).50)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).51) stmt c = 1; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).51)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).52) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).52)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).53) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).53)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).54) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).54)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).55) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).55)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).56) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).56)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).57) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).57)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).58) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).58)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).59) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).59)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).60) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).60)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).61) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).61)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).62) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).62)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).63) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).63)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).64) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).64)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).65) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).65)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).66) type c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).66)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).67) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).67)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).68) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).68)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).69) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).69)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).70) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).70)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).71) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).71)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).72) type  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).72)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).73) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).73)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).74) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).74)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).75) stmt  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).75)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).76) block  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).76)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).77) stmt  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).77)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).78) block {\n        c = 1; //~ WARN value captured by `c` is never read\n        println!(\"{}\", c);\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).78)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).79) expr {\n        c = 1; //~ WARN value captured by `c` is never read\n        println!(\"{}\", c);\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).79)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).80) expr async move {\n        c = 1; //~ WARN value captured by `c` is never read\n        println!(\"{}\", c);\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).80)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).81) local let _ = async move {\n        c = 1; //~ WARN value captured by `c` is never read\n        println!(\"{}\", c);\n    }; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).81)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).82) pat _ (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).82)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).83) stmt let _ = move || {\n        c += 1; //~ WARN unused variable: `c`\n    }; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).83)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).84) expr move || (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).84)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).85) expr {\n        c += 1; //~ WARN unused variable: `c`\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).85)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).86) expr c += 1 (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).86)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).87) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).87)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).88) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).88)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).89) expr 1 (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).89)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).90) stmt c += 1; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).90)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).91) block {\n        c += 1; //~ WARN unused variable: `c`\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).91)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).92) local let _ = move || {\n        c += 1; //~ WARN unused variable: `c`\n    }; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).92)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).93) pat _ (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).93)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).94) stmt let _ = async move {\n        c += 1; //~  WARN value assigned to `c` is never read\n                //~| WARN unused variable: `c`\n    }; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).94)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).96) type async move {\n        c += 1; //~  WARN value assigned to `c` is never read\n                //~| WARN unused variable: `c`\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).96)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).97) pat async move {\n        c += 1; //~  WARN value assigned to `c` is never read\n                //~| WARN unused variable: `c`\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).97)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).98) param async move {\n        c += 1; //~  WARN value assigned to `c` is never read\n                //~| WARN unused variable: `c`\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).98)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).99) expr c += 1 (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).99)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).100) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).100)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).101) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).101)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).102) expr 1 (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).102)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).103) stmt c += 1; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).103)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).104) block {\n        c += 1; //~  WARN value assigned to `c` is never read\n                //~| WARN unused variable: `c`\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).104)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).105) expr {\n        c += 1; //~  WARN value assigned to `c` is never read\n                //~| WARN unused variable: `c`\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).105)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).106) expr async move {\n        c += 1; //~  WARN value assigned to `c` is never read\n                //~| WARN unused variable: `c`\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).106)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).107) local let _ = async move {\n        c += 1; //~  WARN value assigned to `c` is never read\n                //~| WARN unused variable: `c`\n    }; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).107)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).108) pat _ (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).108)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).109) stmt let _ = move || {\n        println!(\"{}\", c);\n        // Value is read by closure itself on later invocations.\n        c += 1;\n    }; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).109)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).110) expr move || (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).110)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).111) expr {\n        println!(\"{}\", c);\n        // Value is read by closure itself on later invocations.\n        c += 1;\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).111)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).112) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).112)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).113) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).113)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).114) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).114)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).115) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).115)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).116) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).116)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).117) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).117)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).118) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).118)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).119) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).119)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).120) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).120)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).121) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).121)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).122) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).122)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).123) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).123)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).124) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).124)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).125) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).125)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).126) type c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).126)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).127) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).127)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).128) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).128)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).129) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).129)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).130) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).130)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).131) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).131)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).132) type  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).132)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).133) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).133)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).134) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).134)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).135) stmt  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).135)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).136) block  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).136)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).137) stmt  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).137)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).138) expr c += 1 (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).138)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).139) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).139)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).140) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).140)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).141) expr 1 (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).141)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).142) stmt c += 1; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).142)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).143) block {\n        println!(\"{}\", c);\n        // Value is read by closure itself on later invocations.\n        c += 1;\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).143)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).144) local let _ = move || {\n        println!(\"{}\", c);\n        // Value is read by closure itself on later invocations.\n        c += 1;\n    }; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).144)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).145) pat _ (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).145)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).146) stmt let b = Box::new(42); (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).146)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).147) expr Box::new(42) (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).147)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).148) expr Box::new (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).148)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).149) path segment Box (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).149)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).150) type Box (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).150)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).151) path segment new (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).151)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).152) expr 42 (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).152)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).153) local let b = Box::new(42); (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).153)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).154) pat b (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).154)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).155) stmt let _ = move || {\n        println!(\"{}\", c);\n        // Never read because this is FnOnce closure.\n        c += 1; //~  WARN value assigned to `c` is never read\n        drop(b);\n    }; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).155)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).156) expr move || (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).156)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).157) expr {\n        println!(\"{}\", c);\n        // Never read because this is FnOnce closure.\n        c += 1; //~  WARN value assigned to `c` is never read\n        drop(b);\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).157)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).158) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).158)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).159) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).159)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).160) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).160)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).161) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).161)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).162) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).162)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).163) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).163)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).164) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).164)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).165) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).165)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).166) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).166)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).167) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).167)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).168) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).168)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).169) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).169)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).170) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).170)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).171) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).171)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).172) type c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).172)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).173) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).173)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).174) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).174)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).175) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).175)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).176) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).176)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).177) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).177)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).178) type  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).178)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).179) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).179)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).180) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).180)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).181) stmt  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).181)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).182) block  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).182)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).183) stmt  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).183)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).184) expr c += 1 (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).184)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).185) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).185)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).186) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).186)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).187) expr 1 (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).187)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).188) stmt c += 1; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).188)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).189) expr drop(b) (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).189)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).190) expr drop (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).190)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).191) path segment drop (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).191)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).192) expr b (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).192)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).193) path segment b (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).193)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).194) stmt drop(b); (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).194)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).195) block {\n        println!(\"{}\", c);\n        // Never read because this is FnOnce closure.\n        c += 1; //~  WARN value assigned to `c` is never read\n        drop(b);\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).195)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).196) local let _ = move || {\n        println!(\"{}\", c);\n        // Never read because this is FnOnce closure.\n        c += 1; //~  WARN value assigned to `c` is never read\n        drop(b);\n    }; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).196)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).197) pat _ (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).197)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).198) stmt let _ = async move {\n        println!(\"{}\", c);\n        // Never read because this is a generator.\n        c += 1; //~  WARN value assigned to `c` is never read\n    }; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).198)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).200) type async move {\n        println!(\"{}\", c);\n        // Never read because this is a generator.\n        c += 1; //~  WARN value assigned to `c` is never read\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).200)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).201) pat async move {\n        println!(\"{}\", c);\n        // Never read because this is a generator.\n        c += 1; //~  WARN value assigned to `c` is never read\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).201)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).202) param async move {\n        println!(\"{}\", c);\n        // Never read because this is a generator.\n        c += 1; //~  WARN value assigned to `c` is never read\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).202)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).203) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).203)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).204) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).204)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).205) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).205)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).206) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).206)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).207) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).207)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).208) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).208)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).209) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).209)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).210) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).210)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).211) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).211)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).212) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).212)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).213) expr \"{}\" (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).213)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).214) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).214)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).215) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).215)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).216) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).216)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).217) type c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).217)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).218) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).218)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).219) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).219)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).220) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).220)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).221) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).221)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).222) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).222)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).223) type  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).223)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).224) path segment  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).224)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).225) expr  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).225)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).226) stmt  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).226)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).227) block  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).227)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).228) stmt  (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).228)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).229) expr c += 1 (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).229)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).230) expr c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).230)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).231) path segment c (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).231)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).232) expr 1 (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).232)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).233) stmt c += 1; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).233)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).234) block {\n        println!(\"{}\", c);\n        // Never read because this is a generator.\n        c += 1; //~  WARN value assigned to `c` is never read\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).234)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).235) expr {\n        println!(\"{}\", c);\n        // Never read because this is a generator.\n        c += 1; //~  WARN value assigned to `c` is never read\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).235)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).236) expr async move {\n        println!(\"{}\", c);\n        // Never read because this is a generator.\n        c += 1; //~  WARN value assigned to `c` is never read\n    } (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).236)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).237) local let _ = async move {\n        println!(\"{}\", c);\n        // Never read because this is a generator.\n        c += 1; //~  WARN value assigned to `c` is never read\n    }; (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).237)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).238) pat _ (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).238)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).239) block {\n    let mut c = 0;\n\n    // Captured by value, but variable is dead on entry.\n    let _ = move || {\n        c = 1; //~ WARN value captured by `c` is never read\n        println!(\"{}\", c);\n    };\n    let _ = async move {\n        c = 1; //~ WARN value captured by `c` is never read\n        println!(\"{}\", c);\n    };\n\n    // Read and written to, but never actually used.\n    let _ = move || {\n        c += 1; //~ WARN unused variable: `c`\n    };\n    let _ = async move {\n        c += 1; //~  WARN value assigned to `c` is never read\n                //~| WARN unused variable: `c`\n    };\n\n    let _ = move || {\n        println!(\"{}\", c);\n        // Value is read by closure itself on later invocations.\n        c += 1;\n    };\n    let b = Box::new(42);\n    let _ = move || {\n        println!(\"{}\", c);\n        // Never read because this is FnOnce closure.\n        c += 1; //~  WARN value assigned to `c` is never read\n        drop(b);\n    };\n    let _ = async move {\n        println!(\"{}\", c);\n        // Never read because this is a generator.\n        c += 1; //~  WARN value assigned to `c` is never read\n    };\n} (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).239)))",
                                    "(HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).240) expr {\n    let mut c = 0;\n\n    // Captured by value, but variable is dead on entry.\n    let _ = move || {\n        c = 1; //~ WARN value captured by `c` is never read\n        println!(\"{}\", c);\n    };\n    let _ = async move {\n        c = 1; //~ WARN value captured by `c` is never read\n        println!(\"{}\", c);\n    };\n\n    // Read and written to, but never actually used.\n    let _ = move || {\n        c += 1; //~ WARN unused variable: `c`\n    };\n    let _ = async move {\n        c += 1; //~  WARN value assigned to `c` is never read\n                //~| WARN unused variable: `c`\n    };\n\n    let _ = move || {\n        println!(\"{}\", c);\n        // Value is read by closure itself on later invocations.\n        c += 1;\n    };\n    let b = Box::new(42);\n    let _ = move || {\n        println!(\"{}\", c);\n        // Never read because this is FnOnce closure.\n        c += 1; //~  WARN value assigned to `c` is never read\n        drop(b);\n    };\n    let _ = async move {\n        println!(\"{}\", c);\n        // Never read because this is a generator.\n        c += 1; //~  WARN value assigned to `c` is never read\n    };\n} (hir_id=HirId(DefId(0:7 ~ liveness_upvars[0e6a]::f).240)))",
                                ]
                                ItemLocalIds not assigned densely in ::async_generator. Max ItemLocalId = 182, missing IDs = [
                                    "[local_id: 8, owner: ::async_generator]",
                                    "[local_id: 119, owner: ::async_generator]",
                                ]; seens IDs = [
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).0) fn async_generator (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).0)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).1) stmt let mut state: u32 = 0; (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).1)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).2) type u32 (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).2)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).3) path segment u32 (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).3)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).4) expr 0 (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).4)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).5) local let mut state: u32 = 0; (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).5)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).6) pat mut state (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).6)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).7) stmt let _ = async {\n        state = 1;\n        yield_now().await;\n        state = 2;\n        yield_now().await;\n        state = 3;\n    }; (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).7)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).9) type async {\n        state = 1;\n        yield_now().await;\n        state = 2;\n        yield_now().await;\n        state = 3;\n    } (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).9)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).10) pat async {\n        state = 1;\n        yield_now().await;\n        state = 2;\n        yield_now().await;\n        state = 3;\n    } (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).10)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).11) param async {\n        state = 1;\n        yield_now().await;\n        state = 2;\n        yield_now().await;\n        state = 3;\n    } (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).11)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).12) expr state = 1 (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).12)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).13) expr state (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).13)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).14) path segment state (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).14)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).15) expr 1 (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).15)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).16) stmt state = 1; (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).16)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).17) expr yield_now().await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).17)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).18) expr yield_now() (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).18)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).19) expr yield_now (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).19)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).20) path segment yield_now (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).20)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).21) pat .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).21)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).22) path segment  (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).22)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).23) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).23)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).24) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).24)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).25) path segment  (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).25)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).26) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).26)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).27) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).27)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).28) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).28)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).29) expr yield_now().await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).29)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).30) expr yield_now().await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).30)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).31) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).31)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).32) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).32)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).33) block .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).33)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).34) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).34)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).35) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).35)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).36) pat yield_now().await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).36)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).37) path segment  (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).37)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).38) expr yield_now().await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).38)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).39) pattern field yield_now().await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).39)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).40) pat .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).40)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).41) expr yield_now().await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).41)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).42) arm yield_now().await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).42)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).43) pat .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).43)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).44) block .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).44)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).45) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).45)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).46) arm .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).46)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).47) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).47)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).48) stmt .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).48)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).49) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).49)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).50) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).50)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).51) path segment  (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).51)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).52) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).52)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).53) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).53)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).54) stmt .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).54)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).55) block .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).55)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).56) arm .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).56)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).57) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).57)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).58) expr .await (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).58)))",
                                    "(HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).59) stmt yield_now().await; (hir_id=HirId(DefId(0:27 ~ liveness_upvars[0e6a]::async_generator).59)))",
