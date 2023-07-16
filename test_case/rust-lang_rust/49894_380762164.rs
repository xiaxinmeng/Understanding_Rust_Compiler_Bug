plain
Resolving deltas: 100% (614309/614309), completed with 4928 local objects.
---
[00:00:56] configure: rust.quiet-tests     := True
---
[00:33:14] 1362 |         let path = external_path(cx, &cx.tcx.item_name(did),
[00:33:14]      |                                      ^^^^^^^^^^^^^^^^^^^^^^ expected str, found struct `syntax::symbol::InternedString`
---
[00:33:14] 1469 |         let path = external_path(cx, &cx.tcx.item_name(trait_ref.def_id),
[00:33:14]      |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected str, found struct `syntax::symbol::InternedString`
---
[00:33:16] 2796 |                 let path = external_path(cx, &cx.tcx.item_name(did),
[00:33:16]      |                                              ^^^^^^^^^^^^^^^^^^^^^^ expected str, found struct `syntax::symbol::InternedString`
---
[00:33:16] 2807 |                 let path = external_path(cx, &cx.tcx.item_name(did),
[00:33:16]      |                                              ^^^^^^^^^^^^^^^^^^^^^^ expected str, found struct `syntax::symbol::InternedString`
---
[00:33:16] 2825 |                         let path = external_path(cx, &cx.tcx.item_name(did),
[00:33:16]      |                                                      ^^^^^^^^^^^^^^^^^^^^^^ expected str, found struct `syntax::symbol::InternedString`
---
[00:33:17] 2848 |                     let path = external_path(cx, &cx.tcx.item_name(did), Some(did),
[00:33:17]      |                                                  ^^^^^^^^^^^^^^^^^^^^^^ expected str, found struct `syntax::symbol::InternedString`
---
[00:33:19] 264 |                         hir::PathSegment::from_name(Symbol::intern(&param.name))
[00:33:19]     |                                                                    ^^^^^^^^^^^ expected str, found struct `syntax::symbol::InternedString`
---
[00:33:25] Makefile:28: recipe for target 'all' failed
[00:33:25] make: *** [all] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:008b7d54:start=1523530164931732610,finish=1523530164944951894,duration=13219284
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1122f171
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:1122f171:start=1523530164956619488,finish=1523530164969050438,duration=12430950
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:083dd5fa
$ dmesg | grep -i kill
[   10.852881] init: failsafe main process (1092) killed by TERM signal
