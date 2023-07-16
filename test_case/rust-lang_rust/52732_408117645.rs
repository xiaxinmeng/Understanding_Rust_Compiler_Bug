plain
[00:56:55] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:55]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 383 |     assert_decode_utf8!([], "");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 384 |     assert_decode_utf8!([0x41], "A");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 385 |     assert_decode_utf8!([0xC1, 0x81], "��");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 386 |     assert_decode_utf8!([0xE2, 0x99, 0xA5], "♥");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 387 |     assert_decode_utf8!([0xE2, 0x99, 0xA5, 0x41], "♥A");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 388 |     assert_decode_utf8!([0xE2, 0x99], "�");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 389 |     assert_decode_utf8!([0xE2, 0x99, 0x41], "�A");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 390 |     assert_decode_utf8!([0xC0], "�");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 391 |     assert_decode_utf8!([0xC0, 0x41], "�A");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 392 |     assert_decode_utf8!([0x80], "�");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 393 |     assert_decode_utf8!([0x80, 0x41], "�A");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 394 |     assert_decode_utf8!([0xFE], "�");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 395 |     assert_decode_utf8!([0xFE, 0x41], "�A");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 396 |     assert_decode_utf8!([0xFF], "�");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 397 |     assert_decode_utf8!([0xFF, 0x41], "�A");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 398 |     assert_decode_utf8!([0xC0, 0x80], "��");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 401 |     assert_decode_utf8!([0xED, 0x9F, 0xBF], "\u{D7FF}");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 402 |     assert_decode_utf8!([0xED, 0xA0, 0x80], "���");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 403 |     assert_decode_utf8!([0xED, 0xBF, 0x80], "���");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 404 |     assert_decode_utf8!([0xEE, 0x80, 0x80], "\u{E000}");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 407 |     assert_decode_utf8!([0xF4, 0x8F, 0xBF, 0xBF], "\u{10FFFF}");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 408 |     assert_decode_utf8!([0xF4, 0x8F, 0xBF, 0x41], "�A");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 409 |     assert_decode_utf8!([0xF4, 0x90, 0x80, 0x80], "����");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 414 |     assert_decode_utf8!([0xF8, 0x80, 0x80, 0x80, 0x80], "�����");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
[00:57:00] 
[00:57:00] error[E0599]: no function or associated item named `decode_utf8` found for type `char` in the current scope
[00:57:00]     |
[00:57:00]     |
[00:57:00] 373 |             let s = char::decode_utf8(input_bytes.iter().cloned())
[00:57:00]     |                     ^^^^^^^^^^^^^^^^^ function or associated item not found in `char`
[00:57:00] ...
[00:57:00] 415 |     assert_decode_utf8!([0xFC, 0x80, 0x80, 0x80, 0x80, 0x80], "������");
[00:57:00]     |
[00:57:00]     |
[00:57:00]     = help: did you mean `encode_utf8`?
3813752 .
2857552 ./obj
2748512 ./obj/build
2151508 ./obj/build/x86_64-unknown-linux-gnu
---
145432 ./obj/build/bootstrap/debug/incremental
133084 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
133080 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
130564 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02
130560 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02/s-f39kc4ync3-124ayc8-1psc9gui095al
128712 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
122140 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
121700 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
116604 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
---
travis_time:end:0e918dc4:start=1532615535097706373,finish=1532615535106115370,duration=8408997
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01405fb0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:26e95680
travis_time:start:26e95680
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:084d97a0
$ dmesg | grep -i kill
