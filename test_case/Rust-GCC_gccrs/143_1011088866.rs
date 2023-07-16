
$ git push upstream github/Rust-GCC/gccrs/master:refs/heads/devel/rust/master
Enumerating objects: 15639, done.
Counting objects: 100% (15463/15463), done.
Delta compression using up to 12 threads
Compressing objects: 100% (4018/4018), done.
Writing objects: 100% (14746/14746), 5.61 MiB | 1.64 MiB/s, done.
Total 14746 (delta 11411), reused 13802 (delta 10622)
remote: Resolving deltas: 100% (11411/11411), completed with 395 local objects.
remote: *** Invalid revision history for commit c7c6f785c8e893ec7bcacd1a2319ce309d2450f2:
remote: *** The first line should be the subject of the commit,
remote: *** followed by an empty line.
remote: *** 
remote: *** Below are the first few lines of the revision history:
remote: *** | Adding Rust target hook documentation
remote: *** | Added powerpc target hook and improved aarch64 feature handling
remote: *** | Added DEC Alpha target hook
remote: *** | Added ARC target hook
remote: *** | Created ARM target hook (at least preliminary support)
remote: *** 
remote: *** Please amend the commit's revision history and try again.
remote: error: hook declined to update refs/heads/devel/rust/master
To git+ssh://gcc.gnu.org/git/gcc.git
 ! [remote rejected]         github/Rust-GCC/gccrs/master -> devel/rust/master (hook declined)
error: failed to push some refs to 'git+ssh://gcc.gnu.org/git/gcc.git'
