
docker pull ppc64le/centos;
docker create --name rust_centos -it ppc64le/centos;
docker start rust_centos;
docker exec -it rust_centos bash;
[root@c769e981bada /]# curl https://sh.rustup.rs -sSf | sh;

> info: downloading installer
> /tmp/tmp.awWf7OI1vO/rustup-init: /lib64/ld64.so.2: version `GLIBC_2.22' not found (required by /tmp/tmp.awWf7OI1vO/rustup-init)
> /tmp/tmp.awWf7OI1vO/rustup-init: /lib64/libc.so.6: version `GLIBC_2.18' not found (required by /tmp/tmp.awWf7OI1vO/rustup-init)

