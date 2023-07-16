
$ CXX='g++ -arch i386 -arch x86_64' CC='gcc -arch i386 -arch x86_64' CFLAGS='-arch i386 -arch x86_64' CXXFLAGS='-arch i386 -arch x86_64' LDFLAGS='-arch i386 -arch x86_64' ./configure --disable-bindings --{build,host,target}=i686-apple-darwin --enable-targets=x86,x86_64,cbe --enable-optimized
checking for i686-apple-darwin-clang... gcc -arch i386 -arch x86_64
checking for C compiler default output file name... a.out
checking whether the C compiler works... yes
checking whether we are cross compiling... no
checking for suffix of executables... 
checking for suffix of object files... o
checking whether we are using the GNU C compiler... yes
checking whether gcc -arch i386 -arch x86_64 accepts -g... yes
checking for gcc -arch i386 -arch x86_64 option to accept ISO C89... none needed
checking whether we are using the GNU C++ compiler... yes
checking whether g++ -arch i386 -arch x86_64 accepts -g... yes
checking how to run the C preprocessor... /lib/cpp
configure: error: C preprocessor "/lib/cpp" fails sanity check
See `config.log' for more details.
