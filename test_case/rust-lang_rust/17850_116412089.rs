
+ local _ostype=unknown-linux-gnu
+ local _cputype=x86_64
+ [ unknown-linux-gnu = unknown-linux-gnu -a x86_64 = x86_64 ]
+ local _bin_to_probe=zsh
+ [ -z zsh -a -e /usr/bin/env ]
+ [ -n zsh ]
+ grep -q x86[_-]64
+ file -L zsh
+ [ 1 != 0 ]
+ local _cputype=i686
+ local _arch=i686-unknown-linux-gnu
