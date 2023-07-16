
dlopen(../app/target/debug/libapp.dylib, 0x00000002)
dyld_image_path_containing_address(0x1019ce000)
  dlopen(../app/target/debug/libapp.dylib) ==> 0x10261b000
dlsym(0x10261b000, get_message)
  dlsym(0x10261b000, get_message) ==> 0x1019cf620
Message: hello world
dlclose(0x10261b000)
dlclose(), found unused image 0x10261b000 libapp.dylib
dlclose(), deleting 0x10261b000 libapp.dylib

dlopen(../app/target/debug/libapp.dylib, 0x00000002)
dyld_image_path_containing_address(0x1019ce000)
  dlopen(../app/target/debug/libapp.dylib) ==> 0x10261b000
dlsym(0x10261b000, get_message)
  dlsym(0x10261b000, get_message) ==> 0x1019cf620
Message: hello world
dlclose(0x10261b000)
dlclose(), found unused image 0x10261b000 libapp.dylib
dlclose(), deleting 0x10261b000 libapp.dylib
