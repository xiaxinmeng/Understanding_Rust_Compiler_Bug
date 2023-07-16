
$ docker history -q rust-ci | grep -v missing | xargs docker save | gzip > $HOME/docker/rust-ci.tar.gz
Error response from daemon: no such id: rust-ci
"docker save" requires at least 1 argument(s).
