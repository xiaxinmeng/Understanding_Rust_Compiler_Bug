
git clone git://github.com/joyent/libuv.git
cd libuv/
make
nm libuv.a | grep _uv__fs_event_close
                 U _uv__fs_event_close
nm libuv.a | grep _uv__hrtime
                 U _uv__hrtime
                 U _uv__hrtime
