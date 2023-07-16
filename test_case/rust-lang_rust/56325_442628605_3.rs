\n\nIf you encounter this error you must alter your patterns so that every possible\nvalue of the input type is matched. For types with a small number of variants\n(like enums) you should probably cover all cases explicitly. Alternatively, the\nunderscore `_` wildcard pattern can be added after all other patterns to match\n\"anything etravis_time:end:09df40f1:start=1543441123774753603,finish=1543443926097662977,duration=2802322909374
travis_time:start:18004f95
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 28 22:25:26 UTC 2018
Wed, 28 Nov 2018 22:25:26 GMT
---
travis_time:end:22e75fa8:start=1543443927228914829,finish=1543443927237113722,duration=8198893
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0dfd4baa
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:033febea
$ dmesg | grep -i kill
