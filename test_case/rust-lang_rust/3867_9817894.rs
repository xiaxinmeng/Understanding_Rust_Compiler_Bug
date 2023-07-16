
git clone git@github.com:jesse99/rparse.git
cd rparse
git checkout ops
make check1 # should seg fault
uncomment src/tests/test_xml.rs line 170
comment out src/tests/test_xml.rs line 171
make check1 # should pass
