
❯❯❯ git log --oneline --grep 'Merge pull ' $(git merge-base upstream-gcc/master HEAD)..upstream-gccrs/gcc-patch-dev                                  
d9f4a1849a3 (upstream-gccrs/gcc-patch-dev, upstream-gccrs-rw/gcc-patch-dev) Merge pull request #1748 from CohenArthur/cl-cleanup-macro-expansion-and-parsing
8e8bf50c1be Merge pull request #1749 from CohenArthur/cl-do-not-lint-public-items
3af69068516 Merge pull request #1743 from Rust-GCC/cls-double-borrows-builtin-overflow-refactor
42d081e810a Merge pull request #1745 from Rust-GCC/dkm/gcc-patch-dev-constexpr
300fd6d7dce Merge pull request #1721 from CohenArthur/first-update-changelog-tryout
92a9a32895e Merge pull request #1746 from CohenArthur/cl-automation
