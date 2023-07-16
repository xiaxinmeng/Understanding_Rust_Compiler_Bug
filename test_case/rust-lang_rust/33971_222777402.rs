
$ test -e .git && ./configure | grep RELEASE
configure: CFG_RELEASE_CHANNEL  := dev

$ test -e .git && ./configure --release-channel='beta' | grep RELEASE
configure: CFG_RELEASE_CHANNEL  := beta

$ mv .git .git.backup

$ test ! -e .git && ./configure | grep RELEASE
configure: CFG_RELEASE_CHANNEL  := stable

$ test ! -e .git && ./configure --release-channel=nightly | grep RELEASE
configure: CFG_RELEASE_CHANNEL  := nightly
