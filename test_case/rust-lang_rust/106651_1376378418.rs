
$ git clone -o github git@github.com:rust-lang/rust
Cloning into 'rust'...
remote: Enumerating objects: 2121843, done.
remote: Counting objects: 100% (23/23), done.
remote: Total 2121843 (delta 22), reused 22 (delta 22), pack-reused 2121820
Receiving objects: 100% (2121843/2121843), 1022.07 MiB | 11.29 MiB/s, done.
Resolving deltas: 100% (1658141/1658141), done.

$ cd rust

$ git reset --hard  1195b67
HEAD is now at 1195b672fb5 Auto merge of #104757 - camelid:consolidate-lints, r=GuillaumeGomez,jyn514,Manishearth

$ ./configure
configure: processing command line
configure: 
configure: build.configure-args := []
configure: 
configure: writing `config.toml` in current directory
configure: 
configure: run `python /home/user/rust/x.py --help`
