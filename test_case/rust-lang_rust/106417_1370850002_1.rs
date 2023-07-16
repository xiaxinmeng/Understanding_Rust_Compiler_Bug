sh
#!/bin/sh
set -x 

TRYBUILD=overwrite cargo test

cargo test
