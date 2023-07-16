
% rustc -C prefer-dynamic a.rs
% otool -lvv a | grep -A 2 RPATH
# no output
% ./a
dyld: Library not loaded: @rpath/libstd-330229df6d027e2b.dylib
  Referenced from: /private/tmp/./a
  Reason: image not found
zsh: abort      ./a
