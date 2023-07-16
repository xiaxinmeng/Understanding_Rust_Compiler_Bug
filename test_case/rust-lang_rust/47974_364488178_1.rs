
dlopen(../app/target/debug/libapp.dylib, 0x00000002)
dyld_image_path_containing_address(0x10e772000)
  dlopen(../app/target/debug/libapp.dylib) ==> 0x7fe1e0700000
dlsym(0x7fe1e0700000, get_message)
  dlsym(0x7fe1e0700000, get_message) ==> 0x10e773460
Message: hello world
dlclose(0x7fe1e0700000)

dlopen(../app/target/debug/libapp.dylib, 0x00000002)
  dlopen(../app/target/debug/libapp.dylib) ==> 0x7fe1e0700000
dlsym(0x7fe1e0700000, get_mess)
  dlsym(0x7fe1e0700000, get_mess) ==> NULL
dlerror()
Failed to retrieve get_message symbol: dlsym(0x7fe1e0700000, get_mess): symbol not found
