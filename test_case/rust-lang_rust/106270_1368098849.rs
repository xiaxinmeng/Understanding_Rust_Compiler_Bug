
$ jq 'map(.total_estimate) | add' rustc_query_impl.mono_items-thread_local_macro.json
5153413
$ jq 'map(.total_estimate) | add' rustc_query_impl.mono_items-thread_local_attr.json
4878568
$ python
Python 3.10.6 (main, Nov 14 2022, 16:10:14) [GCC 11.3.0] on linux
>>> 1 - 4878568/5153413
0.05333261665618494
