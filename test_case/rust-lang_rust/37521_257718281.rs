
make rustc-stage1 -j8
make check-stage1-rustdocck -j8 TESTNAME=foo
make check-notidy -j8
