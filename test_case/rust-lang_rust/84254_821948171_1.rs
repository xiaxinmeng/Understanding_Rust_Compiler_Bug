
$ elfdump target/debug/deps/accuracy-b67fe8242b4d4db7  | grep NEEDED
       [0]  NEEDED            0x17c9b3            libc.so.1
       [1]  NEEDED            0x17ca1c            libm.so.2
       [2]  NEEDED            0x17ca26            libsocket.so.1
       [3]  NEEDED            0x17ca7b            libumem.so.1
       [4]  NEEDED            0x17ca53            libgcc_s.so.1
