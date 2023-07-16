
./configure ...
make check-stage1-rustdocck
# This will fail but builds the tests search-index
python src/etc/search-index.py dump x86_64-unknown-linux-gnu/test/rustdoc/testname.stage1-x86_64-unknown-linux-gnu/search-index.js

# Or compile manually, taking compiletest directives into account
rm -r doc
rustdoc src/test/rustdoc/testname.rs
python src/etc/search-index.py dump doc/search-index.js
