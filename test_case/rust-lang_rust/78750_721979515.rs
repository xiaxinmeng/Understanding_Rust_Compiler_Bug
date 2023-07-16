sh
./x.py test src/test/rustdoc 2>&1 | grep '^    \[rustdoc\] ' | awk '{print $2}' | xargs printf "src/test/%s\n" | xargs -n1 diff-toolchain nightly stage1 >differences.log
