
---- [rustdoc-json] rustdoc-json/nested.rs stdout ----

error: compare failed!
status: exit code: 1
command: "/usr/bin/python2.7" "/checkout/src/test/rustdoc-json/compare.py" "/checkout/src/test/rustdoc-json/nested.expected" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/nested/nested.json" "/checkout/src/test/rustdoc-json"
stdout:
------------------------------------------
checking that /checkout/src/test/rustdoc-json/nested.expected is a logical subset of /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/nested/nested.json

------------------------------------------
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/test/rustdoc-json/compare.py", line 129, in <module>
    main(sys.argv[1], sys.argv[2], normalize(sys.argv[3]))
  File "/checkout/src/test/rustdoc-json/compare.py", line 119, in main
    check_subset(expected_main, actual_main, base_dir)
  File "/checkout/src/test/rustdoc-json/compare.py", line 82, in check_subset
    _check_subset(expected_main["root"], actual_main["root"], [])
  File "/checkout/src/test/rustdoc-json/compare.py", line 76, in _check_subset
    expected_index.get(expected, {}), actual_index.get(actual, {}), trace
  File "/checkout/src/test/rustdoc-json/compare.py", line 58, in _check_subset
    _check_subset(expected[key], actual[key], new_trace)
  File "/checkout/src/test/rustdoc-json/compare.py", line 58, in _check_subset
    _check_subset(expected[key], actual[key], new_trace)
  File "/checkout/src/test/rustdoc-json/compare.py", line 72, in _check_subset
    _check_subset(expected, actual, new_trace)
  File "/checkout/src/test/rustdoc-json/compare.py", line 76, in _check_subset
    expected_index.get(expected, {}), actual_index.get(actual, {}), trace
  File "/checkout/src/test/rustdoc-json/compare.py", line 58, in _check_subset
    _check_subset(expected[key], actual[key], new_trace)
  File "/checkout/src/test/rustdoc-json/compare.py", line 58, in _check_subset
    _check_subset(expected[key], actual[key], new_trace)
  File "/checkout/src/test/rustdoc-json/compare.py", line 72, in _check_subset
    _check_subset(expected, actual, new_trace)
  File "/checkout/src/test/rustdoc-json/compare.py", line 76, in _check_subset
    expected_index.get(expected, {}), actual_index.get(actual, {}), trace
  File "/checkout/src/test/rustdoc-json/compare.py", line 58, in _check_subset
    _check_subset(expected[key], actual[key], new_trace)
  File "/checkout/src/test/rustdoc-json/compare.py", line 44, in _check_subset
    "expected type `{}`, got `{}`".format(expected_type, actual_type), trace
  File "/checkout/src/test/rustdoc-json/compare.py", line 25, in __init__
    super().__init__("{}: {}".format(trace, msg))
TypeError: super() takes at least 1 argument (0 given)
