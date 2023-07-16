\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/resolve/resolve-self-in-impl.rs","byte_start":820,"byte_end":824,"line_start":25,"line_end":25,"column_start":15,"column_end":19,"is_primary":true,"text":[{"text":"impl Tr for S<Self> {} //~ ERROR cycle detected","highlight_start":15,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which again requires finding type of <impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:25:1: 25:23>, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when finding type of <impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:25:1: 25:23>\n  --> /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:25:15\n   |\nLL | impl Tr for S<Self> {} //~ ERROR cycle detected\n   |               ^^^^\n   |\n   = note: ...which again requires finding type of <impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:25:1: 25:23>, completing the cycle\n\n"}
[00:47:57] {"message":"cycle detected when finding type of <impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:26:1: 26:13>","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example containstravis_time:end:229a95bc:start=1544291982621764179,finish=1544294860400920281,duration=2877779156102
travis_time:start:00e8df37
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec  8 18:47:40 UTC 2018
Sat, 08 Dec 2018 18:47:40 GMT
---
travis_time:end:1d0ae148:start=1544294861514957975,finish=1544294861523039949,duration=8081974
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:213ad9de
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:097d76f8
$ dmesg | grep -i kill
