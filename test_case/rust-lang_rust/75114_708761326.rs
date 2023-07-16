
 Traceback (most recent call last):
  File "compare.py", line 106, in <module>
    main(sys.argv[1], sys.argv[2])
  File "compare.py", line 98, in main
    check_subset(expected_main, actual_main)
  File "compare.py", line 65, in check_subset
    _check_subset(expected_main["root"], actual_main["root"], [])
  File "compare.py", line 62, in _check_subset
    _check_subset(expected_index.get(expected, {}), actual_index.get(actual, {}), trace)
  File "compare.py", line 46, in _check_subset
    new_trace = trace.copy()
AttributeError: 'list' object has no attribute 'copy'
