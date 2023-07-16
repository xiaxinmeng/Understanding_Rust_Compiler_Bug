
2020-06-03T00:00:38.5324024Z Invoking msys2 shell command: -defterm -no-start -c "ps -ef | grep '[?]' | awk '{print $2}' | xargs -r kill"
2020-06-03T00:00:38.6865418Z mkdir: cannot change permissions of '/dev/shm': Permission denied
2020-06-03T00:00:38.6991154Z mkdir: cannot change permissions of '/dev/mqueue': Permission denied


2020-06-03T00:01:07.9614303Z ###################################################################
2020-06-03T00:01:07.9614638Z #                                                                 #
2020-06-03T00:01:07.9615014Z #                                                                 #
2020-06-03T00:01:07.9615335Z #                   C   A   U   T   I   O   N                     #
2020-06-03T00:01:07.9617067Z #                                                                 #
2020-06-03T00:01:07.9617434Z #                  This is first start of MSYS2.                  #
2020-06-03T00:01:07.9617864Z #       You MUST restart shell to apply necessary actions.        #
2020-06-03T00:01:07.9618228Z #                                                                 #
2020-06-03T00:01:07.9618548Z #                                                                 #
2020-06-03T00:01:07.9618918Z ###################################################################



2020-06-03T00:01:15.0503132Z       0 [main] bash (4656) D:\a\1\s\msys2\usr\bin\bash.exe: *** fatal error - cygheap base mismatch detected - 0x180330408/0x180346408.
2020-06-03T00:01:15.0504511Z This problem is probably due to using incompatible versions of the cygwin DLL.



2020-06-03T00:03:25.3015568Z + rm download-src-doc-book.tar.gz
2020-06-03T00:03:25.3317445Z error: could not lock config file .git/modules/src/doc/edition-guide/config: No such file or directory


2020-06-03T00:05:44.7396921Z Attempting with retry: make prepare
2020-06-03T00:05:45.2840610Z downloading https://static.rust-lang.org/dist/2020-04-22/rust-std-beta-i686-pc-windows-gnu.tar.xz
2020-06-03T00:05:49.7775566Z extracting D:\a\1\s\build\cache\2020-04-22\rust-std-beta-i686-pc-windows-gnu.tar.xz
2020-06-03T00:05:50.1180090Z downloading https://static.rust-lang.org/dist/2020-04-22/rustc-beta-i686-pc-windows-gnu.tar.xz
2020-06-03T00:06:10.8730339Z extracting D:\a\1\s\build\cache\2020-04-22\rustc-beta-i686-pc-windows-gnu.tar.xz
2020-06-03T00:06:11.2194470Z downloading https://static.rust-lang.org/dist/2020-04-22/rust-mingw-beta-i686-pc-windows-gnu.tar.xz
2020-06-03T00:06:13.0653145Z extracting D:\a\1\s\build\cache\2020-04-22\rust-mingw-beta-i686-pc-windows-gnu.tar.xz
2020-06-03T00:06:13.4203770Z downloading https://static.rust-lang.org/dist/2020-04-22/cargo-beta-i686-pc-windows-gnu.tar.xz
2020-06-03T00:06:15.9333299Z extracting D:\a\1\s\build\cache\2020-04-22\cargo-beta-i686-pc-windows-gnu.tar.xz
2020-06-03T00:06:16.3000218Z downloading https://static.rust-lang.org/dist/2020-04-22/rustfmt-nightly-i686-pc-windows-gnu.tar.xz
2020-06-03T00:06:18.9572549Z extracting D:\a\1\s\build\cache\2020-04-22\rustfmt-nightly-i686-pc-windows-gnu.tar.xz
2020-06-03T00:06:20.2241435Z     Updating crates.io index
2020-06-03T00:06:39.5027447Z error: failed to get `cc` as a dependency of package `bootstrap v0.0.0 (D:\a\1\s\src\bootstrap)`
2020-06-03T00:06:39.5028100Z 
2020-06-03T00:06:39.5044855Z Caused by:
2020-06-03T00:06:39.5064607Z   failed to fetch `https://github.com/rust-lang/crates.io-index`
2020-06-03T00:06:39.5098202Z 
2020-06-03T00:06:39.5098465Z Caused by:
2020-06-03T00:06:39.5098664Z   error inflating zlib stream; class=Zlib (5)
2020-06-03T00:06:39.5143192Z failed to run: D:\a\1\s\build\i686-pc-windows-gnu\stage0\bin\cargo.exe build --manifest-path D:\a\1\s\src/bootstrap/Cargo.toml --locked
