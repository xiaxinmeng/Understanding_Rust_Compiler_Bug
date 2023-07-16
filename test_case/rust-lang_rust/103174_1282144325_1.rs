bash
#!/bin/sh
rustdoc s.rs
cat doc/s/constant.FOO.html | perl -ne 'exit 1 if /static/'
