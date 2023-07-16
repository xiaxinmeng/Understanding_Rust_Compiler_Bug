
git checkout doc_hidden
./configure
make check-stage1-rustdocck -j 12
 cd src/librustdoc/
../../x86_64-unknown-linux-gnu/stage1/bin/rustdoc ~/dev/docs_test_case/src/lib.rs
