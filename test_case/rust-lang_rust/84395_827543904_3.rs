
$ RUSTFLAGS="-Z print-link-args -C linker_plugin_lto -C linker=clang \
    -C link_arg=-fuse-ld=lld -C link-arg=-Wl,--plugin-opt=-lto-embed-bitcode=optimized" \
    cargo build --release 
   Compiling world v0.1.0 (/tmp/world)
"clang" "-m64" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-Wl,--as-needed" \
    "-Wl,-plugin-opt=O3" "-Wl,-plugin-opt=mcpu=x86-64" \
    "-L" "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" \
    "/tmp/world/target/release/deps/world-7ae10c96f87cede3.world.8tv0bkhe-cgu.0.rcgu.o" \
    "/tmp/world/target/release/deps/world-7ae10c96f87cede3.world.8tv0bkhe-cgu.1.rcgu.o" \
    "/tmp/world/target/release/deps/world-7ae10c96f87cede3.world.8tv0bkhe-cgu.2.rcgu.o" \
    "-o" "/tmp/world/target/release/deps/world-7ae10c96f87cede3" \
    "/tmp/world/target/release/deps/world-7ae10c96f87cede3.566azmeytlkxgdp0.rcgu.o" \
    "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" \
    "-L" "/tmp/world/target/release/deps" \
    "-L" "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" \
    "-Wl,--start-group" "-Wl,-Bstatic" \
    "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-2a50117481c8f2aa.rlib" \
    "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-dbea9235d0389335.rlib" \
    "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-f8fc3a1fd01a99fc.rlib" \
    "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-60fdb364b9bcdfb1.rlib" \
    "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-b72528d6aa948810.rlib" \
    "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-17ceec21e62ba944.rlib" \
    "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-a618f40af8a64e78.rlib" \
    "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-4f1f1a8ea88df8ed.rlib" \
    "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-407e9bbfdf8e96b6.rlib" \
    "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-a791cad3fe2b88d2.rlib" \
    "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-f9c8522e7861970c.rlib" \
    "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-6ea30e7b99c281a2.rlib" \
    "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-dbd65dd9774f2a51.rlib" \
    "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-dcb2f8ac1eb14dfb.rlib" \
    "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-419b1f5927c75ef4.rlib"\
     "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-49204354bdca6a99.rlib" "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-89b26516c417e255.rlib" \
    "-Wl,--end-group" "/home/vext01/research/yorick/ykrustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-891aaf048eccfa9f.rlib" \
    "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-fuse-ld=lld" \
    "-Wl,--plugin-opt=-lto-embed-bitcode=optimized"
