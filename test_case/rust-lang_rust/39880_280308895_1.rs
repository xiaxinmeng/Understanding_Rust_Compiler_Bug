bash
#!/usr/bin/bash
/usr/bin/llvm-config "$@" | sed 's/-lm/-lm -lffi/'
