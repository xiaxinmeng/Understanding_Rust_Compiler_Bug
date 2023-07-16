 bash
$ make clean
$ time ./configure --prefix=/home/zazdxscf/build/1nonpkgs/rust/usr/local --disable-rpath --enable-manage-submodules --disable-clang --enable-ccache --enable-dist-host-only --disable-valgrind --disable-helgrind --disable-valgrind-rpass --python=/usr/bin/python2 --enable-optimize --enable-optimize-cxx --enable-optimize-llvm --enable-debug --disable-debuginfo --enable-debug-assertions --enable-debuginfo-tests --enable-llvm-assertions --enable-debug-jemalloc --disable-local-rust --release-channel=dev --host=x86_64-unknown-linux-gnu --target=x86_64-unknown-linux-gnu --build=x86_64-unknown-linux-gnu
...
configure: writing configuration
configure: 
configure: CFG_SRC_DIR          := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_SRC_DIR_RELATIVE := ./ 
configure: CFG_BUILD_DIR        := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_OSTYPE           := unknown-linux-gnu 
configure: CFG_CPUTYPE          := x86_64 
configure: CFG_CONFIGURE_ARGS   := --prefix=/home/zazdxscf/build/1nonp ...
configure: CFG_PREFIX           := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_HOST             := x86_64-unknown-linux-gnu 
configure: CFG_TARGET           := x86_64-unknown-linux-gnu 
configure: CFG_LIBDIR_RELATIVE  := lib 
configure: CFG_DISABLE_MANAGE_SUBMODULES :=  
configure: CFG_AARCH64_LINUX_ANDROID_NDK :=  
configure: CFG_ARM_LINUX_ANDROIDEABI_NDK :=  
configure: CFG_I686_LINUX_ANDROID_NDK :=  
configure: CFG_MANDIR           := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_CCACHE_BASEDIR   := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_LLVM_SRC_DIR     := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_LLVM_BUILD_DIR_x86_64_unknown_linux_gnu := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_LLVM_INST_DIR_x86_64_unknown_linux_gnu := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: 
configure: leaving ./Makefile unchanged
configure: mv -f config.tmp config.mk
configure: 
configure: complete
configure: 
configure: run `make help`
configure: 

real    0m46.124s
user    0m10.727s
sys 0m26.777s
