\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/resolve/resolve-self-in-impl-2.rs","byte_start":588,"byte_end":589,"line_start":15,"line_end":15,"column_start":12,"column_end":13,"is_primary":true,"text":[{"text":"impl Self::N for S {} //~ ERROR cannot find trait `N` in `Self`","highlight_start":12,"highlight_end":13}],"label":"not found in `Self`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0405]: cannot find trait `N` in `Self`\n  --> /checkout/src/test/ui/resolve/resolve-self-in-impl-2.rs:15:12\n   |\nLL | impl Self::N for S {} //~ ERROR cannot find trait `N` in `Self`\n   |            ^ not found in `Self`\n\n"}
[00:46:47] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:46:47] {"message":"Some errors occurred: E0405, E0411.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0405, E0411.\n"}
[00:46:47] {"message":"For more information about an error, try `rustc --explain E0405`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --extravis_time:end:08357667:start=1543551401538242800,finish=1543554208959353500,duration=2807421110700
travis_time:start:22e7f98c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 30 05:03:28 UTC 2018
Fri, 30 Nov 2018 05:03:28 GMT
---
travis_time:end:29cfcd02:start=1543554209999886816,finish=1543554210006583458,duration=6696642
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c35e15f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a7dc350
$ dmesg | grep -i kill
