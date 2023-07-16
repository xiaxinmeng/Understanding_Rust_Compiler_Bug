
#!/bin/bash
PREFIX=/home/john/rust
export LD_LIBRARY_PATH=$PREFIX/lib
exec $PREFIX/bin/cargo "$@"
