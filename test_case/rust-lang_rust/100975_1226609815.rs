plain
    Finished release [optimized] target(s) in 4.77s
Check compiletest suite=rustdoc-json mode=rustdoc-json (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 60 tests
Some tests failed in compiletest suite=rustdoc-json mode=rustdoc-json host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..........F...........................................F.....

---- [rustdoc-json] src/test/rustdoc-json/glob_import.rs stdout ----

error: check_missing_items failed!
error: check_missing_items failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/check_missing_items.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/glob_import/glob_import.json"
Missing local ID: 0:3:1564
------------------------------------------
stderr: none



---- [rustdoc-json] src/test/rustdoc-json/reexport/private_twice_one_inline.rs stdout ----

error: check_missing_items failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/check_missing_items.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/reexport/private_twice_one_inline/private_twice_one_inline.json"
Missing local ID: 1:3:1566
------------------------------------------
stderr: none

