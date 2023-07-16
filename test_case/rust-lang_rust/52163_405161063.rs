bash
# Build a compiler for us to use, just for the host
x.py build

git clone <some embedded example> && cd $_
<path to built cargo> build --target thumbv7m-none-eabi ...

# some procedure to validate binary produced
verify_bin <path to binary>

exit $?
