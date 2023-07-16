
$ docker run -it --rm ubuntu:16.04 bash
root@7cd2bf5f51eb:/# echo '52.222.171.12 static.rust-lang.org' >> /etc/hosts 
root@7cd2bf5f51eb:/# cat /etc/hosts 
127.0.0.1	localhost
::1	localhost ip6-localhost ip6-loopback
fe00::0	ip6-localnet
ff00::0	ip6-mcastprefix
ff02::1	ip6-allnodes
ff02::2	ip6-allrouters
172.17.0.2	7cd2bf5f51eb
52.222.171.12 static.rust-lang.org
root@7cd2bf5f51eb:/# apt update && apt install curl
Get:1 http://security.ubuntu.com/ubuntu xenial-security InRelease [102 kB]
[...]
root@7cd2bf5f51eb:/# for i in 1 2 3; do curl -sSL https://static.rust-lang.org/dist/2018-01-04/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz | sha256sum; done
9a34b23a82d7f3c91637e10ceefb424539dcfa327c2dcd292ff10c047b1fdc7e  -
9a34b23a82d7f3c91637e10ceefb424539dcfa327c2dcd292ff10c047b1fdc7e  -
9a34b23a82d7f3c91637e10ceefb424539dcfa327c2dcd292ff10c047b1fdc7e  -
