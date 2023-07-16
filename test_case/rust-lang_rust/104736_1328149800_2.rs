
$ rustc +master -Copt-level=3 --crate-type lib --edition 2021 -Zvalidate-mir -Zdump-mir=all -Zdump-mir-graphviz test.rs
$ dot -Tpng mir_dump/test.format_response.001-000.built.after.dot -o built.png
$ dot -Tpng mir_dump/test.format_response_no_try.001-000.built.after.dot -o built_no_try.png
