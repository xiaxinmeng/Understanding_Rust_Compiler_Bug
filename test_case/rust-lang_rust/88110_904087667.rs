$ echo "pub fn main(){}" | rustc --crate-type="bin" --crate-name="testcrate" -o libtestcrate.rmeta --emit="metadata" -
$ ls -la libtestcrate.rmeta
-rw-rw-r-- 1 user user 0 Aug 23 16:00 libtestcrate.rmeta
$ echo "pub fn main(){}" | rustc --crate-type="lib" --crate-name="testcrate" -o libtestcrate.rmeta --emit="metadata" -
$ ls -la libtestcrate.rmeta
-rw-rw-r-- 1 user user 1659 Aug 23 16:00 libtestcrate.rmeta
