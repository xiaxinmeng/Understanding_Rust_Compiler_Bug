
install_name_tool -change /usr/local/homebrew/lib/rustlib/x86_64-apple-darwin/lib/libstd-966edb7e-0.10-pre.dylib @rpath/libstd-966edb7e-0.10-pre.dylib libprocmacro-1bc91ebe-0.0.dylib
install_name_tool -change /usr/local/homebrew/lib/rustlib/x86_64-apple-darwin/lib/libsync-7bf3a0fc-0.10-pre.dylib @rpath/libsync-7bf3a0fc-0.10-pre.dylib libprocmacro-1bc91ebe-0.0.dylib

#etc ...
