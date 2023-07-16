
$ git fetch upstream refs/meta/config
[...]
From git+ssh://gcc.gnu.org/git/gcc
 * branch                    refs/meta/config -> FETCH_HEAD
$ git checkout -f FETCH_HEAD
$ $EDITOR project.config
$ git commit -v -- project.config
[...]
[detached HEAD 15e03be6fc6] Enable 'no-precommit-check' for GCC/Rust development branches, 'devel/rust/*'
 1 file changed, 6 insertions(+)
$ git push upstream HEAD:refs/meta/config
[...]
To git+ssh://gcc.gnu.org/git/gcc.git
   f03579097c2..15e03be6fc6  HEAD -> refs/meta/config
