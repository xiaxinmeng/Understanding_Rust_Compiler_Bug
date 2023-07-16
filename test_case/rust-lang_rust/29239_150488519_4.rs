 bash
$ git clean -dfx
//$ patched with the above ^
$ ./configure --enable-ccache --enable-dist-host-only --release-channel=dev --host=x86_64-unknown-linux-gnu --target=x86_64-unknown-linux-gnu --build=x86_64-unknown-linux-gnu
...
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage0/isaac
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage0/sync
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage0/test
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage0/arch/x86_64
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage1
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage1/isaac
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage1/sync
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage1/test
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage1/arch/x86_64
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage2
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage2/isaac
...
configure: configuring LLVM with:
configure: --enable-targets=x86_64 --enable-optimized --disable-assertions --disable-docs --enable-bindings=none --disable-terminfo --disable-zlib --disable-libffi --build=x86_64-unknown-linux-gnu                         --host=x86_64-unknown-linux-gnu --target=x86_64-unknown-linux-gnu --with-python=/usr/bin/python2.7
...
configure: writing configuration
configure: 
configure: CFG_SRC_DIR          := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_SRC_DIR_RELATIVE := ./ 
configure: CFG_BUILD_DIR        := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_OSTYPE           := unknown-linux-gnu 
configure: CFG_CPUTYPE          := x86_64 
configure: CFG_CONFIGURE_ARGS   := --enable-ccache --enable-dist-host- ...
configure: CFG_PREFIX           := /usr/local 
configure: CFG_HOST             := x86_64-unknown-linux-gnu 
configure: CFG_TARGET           := x86_64-unknown-linux-gnu 
configure: CFG_LIBDIR_RELATIVE  := lib 
configure: CFG_DISABLE_MANAGE_SUBMODULES :=  
configure: CFG_AARCH64_LINUX_ANDROID_NDK :=  
configure: CFG_ARM_LINUX_ANDROIDEABI_NDK :=  
configure: CFG_I686_LINUX_ANDROID_NDK :=  
configure: CFG_MANDIR           := /usr/local/share/man 
configure: CFG_CCACHE_BASEDIR   := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_LLVM_SRC_DIR     := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_LLVM_BUILD_DIR_x86_64_unknown_linux_gnu := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_LLVM_INST_DIR_x86_64_unknown_linux_gnu := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: 
configure: cp -f /home/zazdxscf/build/1nonpkgs/rust/rust/Makefile.in ./Makefile
configure: mv -f config.tmp config.mk
configure: 
configure: configured in release mode. for development consider --enable-debug
configure: 
configure: run `make help`
configure: 
