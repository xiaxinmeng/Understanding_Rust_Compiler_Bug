
$ /usr/gnu/bin/objdump -j .dynamic -x target/debug/deps/accuracy-b67fe8242b4d4db7 | grep NEEDED
  NEEDED               libc.so.1
  NEEDED               libm.so.2
  NEEDED               libsocket.so.1
  NEEDED               libumem.so.1
  NEEDED               libgcc_s.so.1

$ elfdump -d target/debug/deps/accuracy-b67fe8242b4d4db7 | grep NEEDED
       [0]  NEEDED            0x17c9b0            libc.so.1
       [1]  NEEDED            0x17ca19            libm.so.2
       [2]  NEEDED            0x17ca23            libsocket.so.1
       [3]  NEEDED            0x17ca78            libumem.so.1
       [4]  NEEDED            0x17ca50            libgcc_s.so.1
