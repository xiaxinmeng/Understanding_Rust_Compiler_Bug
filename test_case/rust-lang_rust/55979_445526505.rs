
export LD_LIBRARY_PATH=$(nix-build -E 'import <nixpkgs>' -A 'gcc.cc.lib')/lib64
