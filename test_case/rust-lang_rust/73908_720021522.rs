
me@my-Mac ~ % lipo -info /bin/sh
Architectures in the fat file: /bin/sh are: x86_64 arm64e 
me@my-Mac ~ % arch -arch x86_64 /bin/sh
arch: posix_spawnp: /bin/sh: Bad CPU type in executable
