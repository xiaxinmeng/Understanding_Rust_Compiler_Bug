
$ cat /usr/local/bin/x86_64-unknown-haiku-g*
#!/bin/bash
exec /home/kallisti5/Code/haiku/generated.x86_64/cross-tools-x86_64/bin/x86_64-unknown-haiku-g++ -sysroot= "$@"
#!/bin/bash
exec /home/kallisti5/Code/haiku/generated.x86_64/cross-tools-x86_64/bin/x86_64-unknown-haiku-gcc -sysroot= "$@"
