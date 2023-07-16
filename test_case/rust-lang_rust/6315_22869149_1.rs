
gcc --std=c89 -pedantic -Wall -Wextra -Wno-unused-parameter -D_GNU_SOURCE 
    -I/home/huon/rust/src/libuv/include -I/home/huon/rust/src/libuv/include/uv-private 
    -I/home/huon/rust/src/libuv/src -D_LARGEFILE_SOURCE 
    -D_FILE_OFFSET_BITS=64 -DRUST_DEBUG -fno-omit-frame-pointer 
    -DUSE_UTF8 -O2 -m64 -fPIC  -c /home/huon/rust/src/libuv/src/unix/async.c 
    -o src/unix/async.o
gcc --std=c89 -pedantic -Wall -Wextra -Wno-unused-parameter -D_GNU_SOURCE
    -I/home/huon/rust/src/libuv/include -I/home/huon/rust/src/libuv/include/uv-private
    -I/home/huon/rust/src/libuv/src -D_LARGEFILE_SOURCE 
    -D_FILE_OFFSET_BITS=64 -DRUST_DEBUG -fno-omit-frame-pointer
    -DUSE_UTF8 -O2 -m64 -fPIC  -c /home/huon/rust/src/libuv/src/unix/core.c
    -o src/unix/core.o
gcc --std=c89 -pedantic -Wall -Wextra -Wno-unused-parameter -D_GNU_SOURCE
    -I/home/huon/rust/src/libuv/include -I/home/huon/rust/src/libuv/include/uv-private
    -I/home/huon/rust/src/libuv/src -D_LARGEFILE_SOURCE
    -D_FILE_OFFSET_BITS=64 -DRUST_DEBUG -fno-omit-frame-pointer
    -DUSE_UTF8 -O2 -m64 -fPIC  -c /home/huon/rust/src/libuv/src/unix/dl.c
    -o src/unix/dl.o
