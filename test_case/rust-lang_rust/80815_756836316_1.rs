
$ git clean -xdff
$ nix-shell -p gcc9 binutils cmake ninja openssl pkgconfig python27 git curl cacert patchelf nix --pure --run './x.py build'
$ nix-shell --keep NIX_PATH -p gcc9 binutils cmake ninja openssl pkgconfig python27 git curl cacert patchelf nix --pure --run './x.py build'
