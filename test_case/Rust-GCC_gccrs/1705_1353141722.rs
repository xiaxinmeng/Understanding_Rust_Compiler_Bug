shell
> arthur@platypus ~/G/r/gccrs (master)> git log 36a9255..HEAD --format=%H -- gcc/rust/ > all_commits.old
> arthur@platypus ~/G/r/gccrs (master)> git log 36a9255..HEAD --format=%H -- gcc/testsuite/rust >> all_commits.old
> arthur@platypus ~/G/r/gccrs (master)> cat all_commits.old | uniq > all_commits
> 