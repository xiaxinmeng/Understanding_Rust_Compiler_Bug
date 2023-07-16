
git grep '#\[derive' | \
  egrep -v '^(test|doc|libcollectionstest|libcoretest|tools|librust|libunwind|libsyntax|libtest|libterm|librbml|libserialize|libfmt_macros)' | \
  grep PartialEq | \
  grep -v '\bEq'
