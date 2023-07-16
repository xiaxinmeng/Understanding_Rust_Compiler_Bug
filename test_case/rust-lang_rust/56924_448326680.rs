plain
travis_time:end:0a0007cd:start=1545153619781869436,finish=1545153622389397821,duration=2607528385
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:45] 
[00:56:45] running 119 tests
[00:57:10] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:57:14] i......iii.i.....ii
[00:57:14] 
[00:57:14]  finished in 28.643
[00:57:14] travis_fold:end:test_debuginfo

---
[01:23:17] travis_fold:end:stage0-linkchecker

[01:23:17] travis_time:end:stage0-linkchecker:start=1545158628871379693,finish=1545158630937027102,duration=2065647409

[01:23:17] cargo/print.html:3617: id is not unique: `_name`
[01:23:17] cargo/print.html:3622: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:3630: id is not unique: `_description`
[01:23:17] cargo/print.html:3661: id is not unique: `_options`
[01:23:17] cargo/print.html:3891: id is not unique: `_display_options`
[01:23:17] cargo/print.html:3960: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:3985: id is not unique: `_common_options`
[01:23:17] cargo/print.html:4046: id is not unique: `_environment`
[01:23:17] cargo/print.html:4055: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:4072: id is not unique: `_examples`
[01:23:17] cargo/print.html:4097: id is not unique: `_see_also`
[01:23:17] cargo/print.html:4123: id is not unique: `_name`
[01:23:17] cargo/print.html:4128: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:4136: id is not unique: `_description`
[01:23:17] cargo/print.html:4144: id is not unique: `_options`
[01:23:17] cargo/print.html:4147: id is not unique: `_package_selection`
[01:23:17] cargo/print.html:4177: id is not unique: `_target_selection`
[01:23:17] cargo/print.html:4241: id is not unique: `_feature_selection`
[01:23:17] cargo/print.html:4267: id is not unique: `_compilation_options`
[01:23:17] cargo/print.html:4320: id is not unique: `_output_options`
[01:23:17] cargo/print.html:4342: id is not unique: `_display_options`
[01:23:17] cargo/print.html:4410: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:4435: id is not unique: `_common_options`
[01:23:17] cargo/print.html:4452: id is not unique: `_miscellaneous_options`
[01:23:17] cargo/print.html:4468: id is not unique: `_profiles`
[01:23:17] cargo/print.html:4513: id is not unique: `_environment`
[01:23:17] cargo/print.html:4522: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:4539: id is not unique: `_examples`
[01:23:17] cargo/print.html:4564: id is not unique: `_see_also`
[01:23:17] cargo/print.html:4590: id is not unique: `_name`
[01:23:17] cargo/print.html:4595: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:4603: id is not unique: `_description`
[01:23:17] cargo/print.html:4615: id is not unique: `_options`
[01:23:17] cargo/print.html:4618: id is not unique: `_package_selection`
[01:23:17] cargo/print.html:4648: id is not unique: `_target_selection`
[01:23:17] cargo/print.html:4712: id is not unique: `_feature_selection`
[01:23:17] cargo/print.html:4738: id is not unique: `_compilation_options`
[01:23:17] cargo/print.html:4799: id is not unique: `_output_options`
[01:23:17] cargo/print.html:4813: id is not unique: `_display_options`
[01:23:17] cargo/print.html:4872: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:4897: id is not unique: `_common_options`
[01:23:17] cargo/print.html:4914: id is not unique: `_miscellaneous_options`
[01:23:17] cargo/print.html:4930: id is not unique: `_profiles`
[01:23:17] cargo/print.html:4975: id is not unique: `_environment`
[01:23:17] cargo/print.html:4984: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:5001: id is not unique: `_examples`
[01:23:17] cargo/print.html:5026: id is not unique: `_see_also`
[01:23:17] cargo/print.html:5052: id is not unique: `_name`
[01:23:17] cargo/print.html:5057: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:5065: id is not unique: `_description`
[01:23:17] cargo/print.html:5077: id is not unique: `_options`
[01:23:17] cargo/print.html:5080: id is not unique: `_package_selection`
[01:23:17] cargo/print.html:5162: id is not unique: `_display_options`
[01:23:17] cargo/print.html:5204: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:5229: id is not unique: `_common_options`
[01:23:17] cargo/print.html:5248: id is not unique: `_environment`
[01:23:17] cargo/print.html:5257: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:5274: id is not unique: `_examples`
[01:23:17] cargo/print.html:5299: id is not unique: `_see_also`
[01:23:17] cargo/print.html:5325: id is not unique: `_name`
[01:23:17] cargo/print.html:5330: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:5338: id is not unique: `_description`
[01:23:17] cargo/print.html:5347: id is not unique: `_options`
[01:23:17] cargo/print.html:5369: id is not unique: `_package_selection`
[01:23:17] cargo/print.html:5399: id is not unique: `_target_selection`
[01:23:17] cargo/print.html:5429: id is not unique: `_feature_selection`
[01:23:17] cargo/print.html:5455: id is not unique: `_compilation_options`
[01:23:17] cargo/print.html:5508: id is not unique: `_output_options`
[01:23:17] cargo/print.html:5522: id is not unique: `_display_options`
[01:23:17] cargo/print.html:5581: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:5606: id is not unique: `_common_options`
[01:23:17] cargo/print.html:5623: id is not unique: `_miscellaneous_options`
[01:23:17] cargo/print.html:5639: id is not unique: `_profiles`
[01:23:17] cargo/print.html:5684: id is not unique: `_environment`
[01:23:17] cargo/print.html:5693: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:5710: id is not unique: `_examples`
[01:23:17] cargo/print.html:5728: id is not unique: `_see_also`
[01:23:17] cargo/print.html:5754: id is not unique: `_name`
[01:23:17] cargo/print.html:5759: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:5767: id is not unique: `_description`
[01:23:17] cargo/print.html:5785: id is not unique: `_options`
[01:23:17] cargo/print.html:5836: id is not unique: `_display_options`
[01:23:17] cargo/print.html:5878: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:5903: id is not unique: `_common_options`
[01:23:17] cargo/print.html:5922: id is not unique: `_environment`
[01:23:17] cargo/print.html:5931: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:5948: id is not unique: `_examples`
[01:23:17] cargo/print.html:5965: id is not unique: `_see_also`
[01:23:17] cargo/print.html:5991: id is not unique: `_name`
[01:23:17] cargo/print.html:5996: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:6004: id is not unique: `_description`
[01:23:17] cargo/print.html:6051: id is not unique: `_options`
[01:23:17] cargo/print.html:6090: id is not unique: `_package_selection`
[01:23:17] cargo/print.html:6120: id is not unique: `_target_selection`
[01:23:17] cargo/print.html:6184: id is not unique: `_feature_selection`
[01:23:17] cargo/print.html:6210: id is not unique: `_compilation_options`
[01:23:17] cargo/print.html:6271: id is not unique: `_output_options`
[01:23:17] cargo/print.html:6285: id is not unique: `_display_options`
[01:23:17] cargo/print.html:6344: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:6369: id is not unique: `_common_options`
[01:23:17] cargo/print.html:6386: id is not unique: `_miscellaneous_options`
[01:23:17] cargo/print.html:6402: id is not unique: `_profiles`
[01:23:17] cargo/print.html:6447: id is not unique: `_environment`
[01:23:17] cargo/print.html:6456: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:6473: id is not unique: `_examples`
[01:23:17] cargo/print.html:6506: id is not unique: `_see_also`
[01:23:17] cargo/print.html:6532: id is not unique: `_name`
[01:23:17] cargo/print.html:6537: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:6545: id is not unique: `_description`
[01:23:17] cargo/print.html:6558: id is not unique: `_options`
[01:23:17] cargo/print.html:6561: id is not unique: `_package_selection`
[01:23:17] cargo/print.html:6578: id is not unique: `_target_selection`
[01:23:17] cargo/print.html:6598: id is not unique: `_feature_selection`
[01:23:17] cargo/print.html:6624: id is not unique: `_compilation_options`
[01:23:17] cargo/print.html:6677: id is not unique: `_output_options`
[01:23:17] cargo/print.html:6691: id is not unique: `_display_options`
[01:23:17] cargo/print.html:6750: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:6775: id is not unique: `_common_options`
[01:23:17] cargo/print.html:6792: id is not unique: `_miscellaneous_options`
[01:23:17] cargo/print.html:6808: id is not unique: `_profiles`
[01:23:17] cargo/print.html:6853: id is not unique: `_environment`
[01:23:17] cargo/print.html:6862: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:6879: id is not unique: `_examples`
[01:23:17] cargo/print.html:6904: id is not unique: `_see_also`
[01:23:17] cargo/print.html:6930: id is not unique: `_name`
[01:23:17] cargo/print.html:6935: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:6943: id is not unique: `_description`
[01:23:17] cargo/print.html:6969: id is not unique: `_options`
[01:23:17] cargo/print.html:6972: id is not unique: `_package_selection`
[01:23:17] cargo/print.html:6989: id is not unique: `_target_selection`
[01:23:17] cargo/print.html:7052: id is not unique: `_feature_selection`
[01:23:17] cargo/print.html:7078: id is not unique: `_compilation_options`
[01:23:17] cargo/print.html:7131: id is not unique: `_output_options`
[01:23:17] cargo/print.html:7145: id is not unique: `_display_options`
[01:23:17] cargo/print.html:7204: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:7229: id is not unique: `_common_options`
[01:23:17] cargo/print.html:7246: id is not unique: `_miscellaneous_options`
[01:23:17] cargo/print.html:7262: id is not unique: `_profiles`
[01:23:17] cargo/print.html:7307: id is not unique: `_environment`
[01:23:17] cargo/print.html:7316: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:7333: id is not unique: `_examples`
[01:23:17] cargo/print.html:7359: id is not unique: `_see_also`
[01:23:17] cargo/print.html:7385: id is not unique: `_name`
[01:23:17] cargo/print.html:7390: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:7398: id is not unique: `_description`
[01:23:17] cargo/print.html:7424: id is not unique: `_options`
[01:23:17] cargo/print.html:7427: id is not unique: `_documentation_options`
[01:23:17] cargo/print.html:7438: id is not unique: `_package_selection`
[01:23:17] cargo/print.html:7455: id is not unique: `_target_selection`
[01:23:17] cargo/print.html:7520: id is not unique: `_feature_selection`
[01:23:17] cargo/print.html:7546: id is not unique: `_compilation_options`
[01:23:17] cargo/print.html:7599: id is not unique: `_output_options`
[01:23:17] cargo/print.html:7613: id is not unique: `_display_options`
[01:23:17] cargo/print.html:7672: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:7697: id is not unique: `_common_options`
[01:23:17] cargo/print.html:7714: id is not unique: `_miscellaneous_options`
[01:23:17] cargo/print.html:7730: id is not unique: `_profiles`
[01:23:17] cargo/print.html:7775: id is not unique: `_environment`
[01:23:17] cargo/print.html:7784: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:7801: id is not unique: `_examples`
[01:23:17] cargo/print.html:7818: id is not unique: `_see_also`
[01:23:17] cargo/print.html:7844: id is not unique: `_name`
[01:23:17] cargo/print.html:7849: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:7857: id is not unique: `_description`
[01:23:17] cargo/print.html:7891: id is not unique: `_options`
[01:23:17] cargo/print.html:7912: id is not unique: `_package_selection`
[01:23:17] cargo/print.html:7942: id is not unique: `_target_selection`
[01:23:17] cargo/print.html:8047: id is not unique: `_feature_selection`
[01:23:17] cargo/print.html:8073: id is not unique: `_compilation_options`
[01:23:17] cargo/print.html:8126: id is not unique: `_output_options`
[01:23:17] cargo/print.html:8140: id is not unique: `_display_options`
[01:23:17] cargo/print.html:8209: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:8234: id is not unique: `_common_options`
[01:23:17] cargo/print.html:8251: id is not unique: `_miscellaneous_options`
[01:23:17] cargo/print.html:8277: id is not unique: `_profiles`
[01:23:17] cargo/print.html:8331: id is not unique: `_environment`
[01:23:17] cargo/print.html:8340: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:8357: id is not unique: `_examples`
[01:23:17] cargo/print.html:8382: id is not unique: `_see_also`
[01:23:17] cargo/print.html:8409: id is not unique: `_name`
[01:23:17] cargo/print.html:8414: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:8422: id is not unique: `_description`
[01:23:17] cargo/print.html:8436: id is not unique: `_options`
[01:23:17] cargo/print.html:8439: id is not unique: `_display_options`
[01:23:17] cargo/print.html:8481: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:8506: id is not unique: `_common_options`
[01:23:17] cargo/print.html:8525: id is not unique: `_environment`
[01:23:17] cargo/print.html:8534: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:8551: id is not unique: `_examples`
[01:23:17] cargo/print.html:8568: id is not unique: `_see_also`
[01:23:17] cargo/print.html:8594: id is not unique: `_name`
[01:23:17] cargo/print.html:8599: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:8607: id is not unique: `_description`
[01:23:17] cargo/print.html:8620: id is not unique: `_options`
[01:23:17] cargo/print.html:8623: id is not unique: `_display_options`
[01:23:17] cargo/print.html:8665: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:8677: id is not unique: `_common_options`
[01:23:17] cargo/print.html:8696: id is not unique: `_environment`
[01:23:17] cargo/print.html:8705: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:8722: id is not unique: `_examples`
[01:23:17] cargo/print.html:8739: id is not unique: `_see_also`
[01:23:17] cargo/print.html:8765: id is not unique: `_name`
[01:23:17] cargo/print.html:8770: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:8778: id is not unique: `_description`
[01:23:17] cargo/print.html:9010: id is not unique: `_options`
[01:23:17] cargo/print.html:9013: id is not unique: `_output_options`
[01:23:17] cargo/print.html:9030: id is not unique: `_feature_selection`
[01:23:17] cargo/print.html:9056: id is not unique: `_display_options`
[01:23:17] cargo/print.html:9098: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:9123: id is not unique: `_common_options`
[01:23:17] cargo/print.html:9142: id is not unique: `_environment`
[01:23:17] cargo/print.html:9151: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:9168: id is not unique: `_examples`
[01:23:17] cargo/print.html:9185: id is not unique: `_see_also`
[01:23:17] cargo/print.html:9211: id is not unique: `_name`
[01:23:17] cargo/print.html:9216: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:9224: id is not unique: `_description`
[01:23:17] cargo/print.html:9285: id is not unique: `_options`
[01:23:17] cargo/print.html:9288: id is not unique: `_package_selection`
[01:23:17] cargo/print.html:9300: id is not unique: `_display_options`
[01:23:17] cargo/print.html:9342: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:9367: id is not unique: `_common_options`
[01:23:17] cargo/print.html:9386: id is not unique: `_environment`
[01:23:17] cargo/print.html:9395: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:9412: id is not unique: `_examples`
[01:23:17] cargo/print.html:9445: id is not unique: `_see_also`
[01:23:17] cargo/print.html:9471: id is not unique: `_name`
[01:23:17] cargo/print.html:9476: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:9484: id is not unique: `_description`
[01:23:17] cargo/print.html:9494: id is not unique: `_options`
[01:23:17] cargo/print.html:9531: id is not unique: `_display_options`
[01:23:17] cargo/print.html:9573: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:9598: id is not unique: `_common_options`
[01:23:17] cargo/print.html:9617: id is not unique: `_environment`
[01:23:17] cargo/print.html:9626: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:9643: id is not unique: `_examples`
[01:23:17] cargo/print.html:9676: id is not unique: `_see_also`
[01:23:17] cargo/print.html:9702: id is not unique: `_name`
[01:23:17] cargo/print.html:9707: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:9715: id is not unique: `_description`
[01:23:17] cargo/print.html:9737: id is not unique: `_options`
[01:23:17] cargo/print.html:9740: id is not unique: `_display_options`
[01:23:17] cargo/print.html:9782: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:9807: id is not unique: `_common_options`
[01:23:17] cargo/print.html:9826: id is not unique: `_environment`
[01:23:17] cargo/print.html:9835: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:9852: id is not unique: `_examples`
[01:23:17] cargo/print.html:9869: id is not unique: `_see_also`
[01:23:17] cargo/print.html:9896: id is not unique: `_name`
[01:23:17] cargo/print.html:9901: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:9909: id is not unique: `_description`
[01:23:17] cargo/print.html:9993: id is not unique: `_options`
[01:23:17] cargo/print.html:10037: id is not unique: `_display_options`
[01:23:17] cargo/print.html:10079: id is not unique: `_common_options`
[01:23:17] cargo/print.html:10098: id is not unique: `_environment`
[01:23:17] cargo/print.html:10107: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:10124: id is not unique: `_examples`
[01:23:17] cargo/print.html:10141: id is not unique: `_see_also`
[01:23:17] cargo/print.html:10167: id is not unique: `_name`
[01:23:17] cargo/print.html:10172: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:10183: id is not unique: `_description`
[01:23:17] cargo/print.html:10237: id is not unique: `_options`
[01:23:17] cargo/print.html:10310: id is not unique: `_feature_selection`
[01:23:17] cargo/print.html:10336: id is not unique: `_compilation_options`
[01:23:17] cargo/print.html:10388: id is not unique: `_miscellaneous_options`
[01:23:17] cargo/print.html:10402: id is not unique: `_display_options`
[01:23:17] cargo/print.html:10444: id is not unique: `_common_options`
[01:23:17] cargo/print.html:10463: id is not unique: `_environment`
[01:23:17] cargo/print.html:10472: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:10489: id is not unique: `_examples`
[01:23:17] cargo/print.html:10514: id is not unique: `_see_also`
[01:23:17] cargo/print.html:10540: id is not unique: `_name`
[01:23:17] cargo/print.html:10545: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:10553: id is not unique: `_description`
[01:23:17] cargo/print.html:10630: id is not unique: `_options`
[01:23:17] cargo/print.html:10674: id is not unique: `_display_options`
[01:23:17] cargo/print.html:10716: id is not unique: `_common_options`
[01:23:17] cargo/print.html:10735: id is not unique: `_environment`
[01:23:17] cargo/print.html:10744: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:10761: id is not unique: `_examples`
[01:23:17] cargo/print.html:10778: id is not unique: `_see_also`
[01:23:17] cargo/print.html:10804: id is not unique: `_name`
[01:23:17] cargo/print.html:10809: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:10817: id is not unique: `_description`
[01:23:17] cargo/print.html:10827: id is not unique: `_options`
[01:23:17] cargo/print.html:10853: id is not unique: `_display_options`
[01:23:17] cargo/print.html:10895: id is not unique: `_common_options`
[01:23:17] cargo/print.html:10914: id is not unique: `_environment`
[01:23:17] cargo/print.html:10923: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:10940: id is not unique: `_examples`
[01:23:17] cargo/print.html:10957: id is not unique: `_see_also`
[01:23:17] cargo/print.html:10983: id is not unique: `_name`
[01:23:17] cargo/print.html:10988: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:10996: id is not unique: `_description`
[01:23:17] cargo/print.html:11032: id is not unique: `_options`
[01:23:17] cargo/print.html:11035: id is not unique: `_install_options`
[01:23:17] cargo/print.html:11055: id is not unique: `_display_options`
[01:23:17] cargo/print.html:11097: id is not unique: `_common_options`
[01:23:17] cargo/print.html:11116: id is not unique: `_environment`
[01:23:17] cargo/print.html:11125: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:11142: id is not unique: `_examples`
[01:23:17] cargo/print.html:11159: id is not unique: `_see_also`
[01:23:17] cargo/print.html:11186: id is not unique: `_name`
[01:23:17] cargo/print.html:11191: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:11199: id is not unique: `_description`
[01:23:17] cargo/print.html:11219: id is not unique: `_options`
[01:23:17] cargo/print.html:11242: id is not unique: `_display_options`
[01:23:17] cargo/print.html:11284: id is not unique: `_common_options`
[01:23:17] cargo/print.html:11303: id is not unique: `_environment`
[01:23:17] cargo/print.html:11312: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:11329: id is not unique: `_examples`
[01:23:17] cargo/print.html:11346: id is not unique: `_see_also`
[01:23:17] cargo/print.html:11372: id is not unique: `_name`
[01:23:17] cargo/print.html:11377: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:11387: id is not unique: `_description`
[01:23:17] cargo/print.html:11409: id is not unique: `_options`
[01:23:17] cargo/print.html:11450: id is not unique: `_display_options`
[01:23:17] cargo/print.html:11492: id is not unique: `_common_options`
[01:23:17] cargo/print.html:11511: id is not unique: `_environment`
[01:23:17] cargo/print.html:11520: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:11537: id is not unique: `_examples`
[01:23:17] cargo/print.html:11570: id is not unique: `_see_also`
[01:23:17] cargo/print.html:11596: id is not unique: `_name`
[01:23:17] cargo/print.html:11601: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:11609: id is not unique: `_description`
[01:23:17] cargo/print.html:11664: id is not unique: `_options`
[01:23:17] cargo/print.html:11692: id is not unique: `_compilation_options`
[01:23:17] cargo/print.html:11747: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:11772: id is not unique: `_miscellaneous_options`
[01:23:17] cargo/print.html:11786: id is not unique: `_display_options`
[01:23:17] cargo/print.html:11828: id is not unique: `_common_options`
[01:23:17] cargo/print.html:11847: id is not unique: `_environment`
[01:23:17] cargo/print.html:11856: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:11873: id is not unique: `_examples`
[01:23:17] cargo/print.html:11890: id is not unique: `_see_also`
[01:23:17] cargo/print.html:11916: id is not unique: `_name`
[01:23:17] cargo/print.html:11921: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:11929: id is not unique: `_description`
[01:23:17] cargo/print.html:11973: id is not unique: `_options`
[01:23:17] cargo/print.html:12011: id is not unique: `_compilation_options`
[01:23:17] cargo/print.html:12066: id is not unique: `_manifest_options`
[01:23:17] cargo/print.html:12091: id is not unique: `_miscellaneous_options`
[01:23:17] cargo/print.html:12105: id is not unique: `_display_options`
[01:23:17] cargo/print.html:12147: id is not unique: `_common_options`
[01:23:17] cargo/print.html:12166: id is not unique: `_environment`
[01:23:17] cargo/print.html:12175: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:12192: id is not unique: `_examples`
[01:23:17] cargo/print.html:12209: id is not unique: `_see_also`
[01:23:17] cargo/print.html:12235: id is not unique: `_name`
[01:23:17] cargo/print.html:12240: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:12248: id is not unique: `_description`
[01:23:17] cargo/print.html:12271: id is not unique: `_options`
[01:23:17] cargo/print.html:12274: id is not unique: `_owner_options`
[01:23:17] cargo/print.html:12305: id is not unique: `_display_options`
[01:23:17] cargo/print.html:12347: id is not unique: `_common_options`
[01:23:17] cargo/print.html:12366: id is not unique: `_environment`
[01:23:17] cargo/print.html:12375: id is not unique: `_exit_status`
[01:23:17] cargo/print.html:12392: id is not unique: `_examples`
[01:23:17] cargo/print.html:12409: id is not unique: `_see_also`
[01:23:17] cargo/print.html:12436: id is not unique: `_name`
[01:23:17] cargo/print.html:12441: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:12449: id is not unique: `_description`
[01:23:17] cargo/print.html:12457: id is not unique: `_examples`
[01:23:17] cargo/print.html:12482: id is not unique: `_see_also`
[01:23:17] cargo/print.html:12508: id is not unique: `_name`
[01:23:17] cargo/print.html:12513: id is not unique: `_synopsis`
[01:23:17] cargo/print.html:12521: id is not unique: `_description`
[01:23:17] cargo/print.html:12529: id is not unique: `_options`
[01:23:17] cargo/print.html:12543: id is not unique: `_examples`
[01:23:17] cargo/print.html:12577: id is not unique: `_see_also`
[01:23:17] cargo/commands/generated/cargo-update.html:19: broken link - cargo/commands/generated/commands/cargo-build.html
[01:23:17] cargo/commands/generated/cargo-update.html:19: broken link - cargo/commands/generated/commands/cargo-generate-lockfile.html
[01:23:17] cargo/commands/generated/cargo-update.html:34: broken link - cargo/commands/generated/commands/cargo-pkgid.html
[01:23:17] cargo/commands/generated/cargo-update.html:70: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-update.html:96: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-update.html:150: broken link - cargo/commands/generated/reference/environment-variables.html
[01:23:17] cargo/commands/generated/cargo-update.html:209: broken link - cargo/commands/generated/commands/index.html
[01:23:17] cargo/commands/generated/cargo-update.html:209: broken link - cargo/commands/generated/commands/cargo-generate-lockfile.html
[01:23:17] cargo/commands/generated/cargo-build.html:39: broken link - cargo/commands/generated/commands/cargo-pkgid.html
[01:23:17] cargo/commands/generated/cargo-build.html:184: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-build.html:205: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-build.html:229: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-build.html:255: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-build.html:337: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-build.html:351: broken link - cargo/commands/generated/reference/manifest.html
[01:23:17] cargo/commands/generated/cargo-build.html:394: broken link - cargo/commands/generated/reference/environment-variables.html
[01:23:17] cargo/commands/generated/cargo-build.html:445: broken link - cargo/commands/generated/commands/index.html
[01:23:17] cargo/commands/generated/cargo-build.html:445: broken link - cargo/commands/generated/commands/cargo-rustc.html
[01:23:17] cargo/commands/generated/cargo-version.html:73: broken link - cargo/commands/generated/commands/index.html
[01:23:17] cargo/commands/generated/cargo-new.html:81: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-new.html:85: broken link - cargo/commands/generated/commands/cargo-init.html
[01:23:17] cargo/commands/generated/cargo-new.html:144: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-new.html:170: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-new.html:199: broken link - cargo/commands/generated/reference/environment-variables.html
[01:23:17] cargo/commands/generated/cargo-new.html:242: broken link - cargo/commands/generated/commands/index.html
[01:23:17] cargo/commands/generated/cargo-new.html:242: broken link - cargo/commands/generated/commands/cargo-init.html
[01:23:17] cargo/commands/generated/cargo-owner.html:25: broken link - cargo/commands/generated/commands/cargo-login.html
[01:23:17] cargo/commands/generated/cargo-owner.html:32: broken link - cargo/commands/generated/reference/publishing.html
[01:23:17] cargo/commands/generated/cargo-owner.html:88: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-owner.html:114: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-owner.html:143: broken link - cargo/commands/generated/reference/environment-variables.html
[01:23:17] cargo/commands/generated/cargo-owner.html:202: broken link - cargo/commands/generated/commands/index.html
[01:23:17] cargo/commands/generated/cargo-owner.html:202: broken link - cargo/commands/generated/commands/cargo-login.html
[01:23:17] cargo/commands/generated/cargo-owner.html:202: broken link - cargo/commands/generated/commands/cargo-publish.html
[01:23:17] cargo/commands/generated/cargo-fix.html:25: broken link - cargo/commands/generated/commands/cargo-check.html
[01:23:17] cargo/commands/generated/cargo-fix.html:114: broken link - cargo/commands/generated/commands/cargo-pkgid.html
[01:23:17] cargo/commands/generated/cargo-fix.html:259: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-fix.html:288: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-fix.html:304: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-fix.html:330: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-fix.html:403: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-fix.html:417: broken link - cargo/commands/generated/reference/manifest.html
[01:23:17] cargo/commands/generated/cargo-fix.html:460: broken link - cargo/commands/generated/reference/environment-variables.html
[01:23:17] cargo/commands/generated/cargo-fix.html:519: broken link - cargo/commands/generated/commands/index.html
[01:23:17] cargo/commands/generated/cargo-fix.html:519: broken link - cargo/commands/generated/commands/cargo-check.html
[01:23:17] cargo/commands/generated/cargo-init.html:88: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-init.html:92: broken link - cargo/commands/generated/commands/cargo-new.html
[01:23:17] cargo/commands/generated/cargo-init.html:151: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-init.html:177: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-init.html:206: broken link - cargo/commands/generated/reference/environment-variables.html
[01:23:17] cargo/commands/generated/cargo-init.html:249: broken link - cargo/commands/generated/commands/index.html
[01:23:17] cargo/commands/generated/cargo-init.html:249: broken link - cargo/commands/generated/commands/cargo-new.html
[01:23:17] cargo/commands/generated/cargo-uninstall.html:17: broken link - cargo/commands/generated/commands/cargo-install.html
[01:23:17] cargo/commands/generated/cargo-uninstall.html:19: broken link - cargo/commands/generated/commands/cargo-pkgid.html
[01:23:17] cargo/commands/generated/cargo-uninstall.html:37: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-uninstall.html:82: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-uninstall.html:108: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-uninstall.html:137: broken link - cargo/commands/generated/reference/environment-variables.html
[01:23:17] cargo/commands/generated/cargo-uninstall.html:180: broken link - cargo/commands/generated/commands/index.html
[01:23:17] cargo/commands/generated/cargo-uninstall.html:180: broken link - cargo/commands/generated/commands/cargo-install.html
[01:23:17] cargo/commands/generated/cargo-search.html:59: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-search.html:85: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-search.html:114: broken link - cargo/commands/generated/reference/environment-variables.html
[01:23:17] cargo/commands/generated/cargo-search.html:157: broken link - cargo/commands/generated/commands/index.html
[01:23:17] cargo/commands/generated/cargo-search.html:157: broken link - cargo/commands/generated/commands/cargo-install.html
[01:23:17] cargo/commands/generated/cargo-search.html:157: broken link - cargo/commands/generated/commands/cargo-publish.html
[01:23:17] cargo/commands/generated/cargo-metadata.html:301: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-metadata.html:327: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-metadata.html:381: broken link - cargo/commands/generated/reference/environment-variables.html
[01:23:17] cargo/commands/generated/cargo-metadata.html:424: broken link - cargo/commands/generated/commands/index.html
[01:23:17] cargo/commands/generated/cargo-publish.html:39: broken link - cargo/commands/generated/commands/cargo-package.html
[01:23:17] cargo/commands/generated/cargo-publish.html:49: broken link - cargo/commands/generated/commands/cargo-login.html
[01:23:17] cargo/commands/generated/cargo-publish.html:52: broken link - cargo/commands/generated/reference/publishing.html
[01:23:17] cargo/commands/generated/cargo-publish.html:135: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-publish.html:144: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-publish.html:183: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-publish.html:199: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-publish.html:225: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-publish.html:254: broken link - cargo/commands/generated/reference/environment-variables.html
[01:23:17] cargo/commands/generated/cargo-publish.html:297: broken link - cargo/commands/generated/commands/index.html
[01:23:17] cargo/commands/generated/cargo-publish.html:297: broken link - cargo/commands/generated/commands/cargo-package.html
[01:23:17] cargo/commands/generated/cargo-publish.html:297: broken link - cargo/commands/generated/commands/cargo-login.html
[01:23:17] cargo/commands/generated/cargo-login.html:18: broken link - cargo/commands/generated/commands/cargo-publish.html
[01:23:17] cargo/commands/generated/cargo-login.html:66: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-login.html:92: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-login.html:121: broken link - cargo/commands/generated/reference/environment-variables.html
[01:23:17] cargo/commands/generated/cargo-login.html:164: broken link - cargo/commands/generated/commands/index.html
[01:23:17] cargo/commands/generated/cargo-login.html:164: broken link - cargo/commands/generated/commands/cargo-publish.html
[01:23:17] cargo/commands/generated/cargo-check.html:43: broken link - cargo/commands/generated/commands/cargo-pkgid.html
[01:23:17] cargo/commands/generated/cargo-check.html:188: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-check.html:217: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-check.html:233: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-check.html:259: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-check.html:332: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-check.html:346: broken link - cargo/commands/generated/reference/manifest.html
[01:23:17] cargo/commands/generated/cargo-check.html:389: broken link - cargo/commands/generated/reference/environment-variables.html
[01:23:17] cargo/commands/generated/cargo-check.html:440: broken link - cargo/commands/generated/commands/index.html
[01:23:17] cargo/commands/generated/cargo-check.html:440: broken link - cargo/commands/generated/commands/cargo-build.html
[01:23:17] cargo/commands/generated/cargo-rustc.html:35: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-rustc.html:53: broken link - cargo/commands/generated/commands/cargo-pkgid.html
[01:23:17] cargo/commands/generated/cargo-rustc.html:188: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-rustc.html:209: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-rustc.html:225: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-rustc.html:251: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-rustc.html:324: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-rustc.html:338: broken link - cargo/commands/generated/reference/manifest.html
[01:23:17] cargo/commands/generated/cargo-rustc.html:381: broken link - cargo/commands/generated/reference/environment-variables.html
[01:23:17] cargo/commands/generated/cargo-rustc.html:433: broken link - cargo/commands/generated/commands/index.html
[01:23:17] cargo/commands/generated/cargo-rustc.html:433: broken link - cargo/commands/generated/commands/cargo-build.html
[01:23:17] cargo/commands/generated/cargo-verify-project.html:48: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-verify-project.html:74: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-verify-project.html:128: broken link - cargo/commands/generated/reference/environment-variables.html
[01:23:17] cargo/commands/generated/cargo-verify-project.html:171: broken link - cargo/commands/generated/commands/index.html
[01:23:17] cargo/commands/generated/cargo-verify-project.html:171: broken link - cargo/commands/generated/commands/cargo-package.html
[01:23:17] cargo/commands/generated/cargo-rustdoc.html:64: broken link - cargo/commands/generated/commands/cargo-pkgid.html
[01:23:17] cargo/commands/generated/cargo-rustdoc.html:201: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-rustdoc.html:222: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-rustdoc.html:238: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-rustdoc.html:264: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-rustdoc.html:337: broken link - cargo/commands/generated/reference/config.html
[01:23:17] cargo/commands/generated/cargo-rustdoc.html:351: broken link - cargo/commands/generated/reference/manifest.html
[01:23:17] cargo/commands/generated/cargo-rustdoc.html:394: broken link - cargo/commands/generated/reference/environment-variables.html
[01:23:17] cargo/commands/generated/cargo-rustdoc.html:437: broken link - cargo/commands/generated/commands/index.html
[01:23:17] cargo/commands/generated/cargo-rustdoc.html:437: broken link - cargo/commands/generated/commands/cargo-doc.html
[01:23:17] cargo/commands/generated/cargo.html:33: broken link - cargo/commands/generated/commands/cargo-bench.html
[01:23:17] cargo/commands/generated/cargo.html:37: broken link - cargo/commands/generated/commands/cargo-build.html
---
travis_time:end:0014e3e4:start=1545158638348513129,finish=1545158638355239652,duration=6726523
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00f59ae7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2b125cbc
travis_time:start:2b125cbc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01bc6528
$ dmesg | grep -i kill
