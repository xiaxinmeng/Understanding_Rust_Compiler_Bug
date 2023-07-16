plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: mingw-check
##[endgroup]
usage: git merge-base [-a | --all] <commit> <commit>...
   or: git merge-base [-a | --all] --octopus <commit>...
   or: git merge-base --independent <commit>...
   or: git merge-base --is-ancestor <commit> <commit>
   or: git merge-base --fork-point <ref> [<commit>]

Error: Could not find a valid backport for `56397e90e178fdc98b48031ceb79e90c61a48e3a`

`56397e90e178fdc98b48031ceb79e90c61a48e3a` has a commit message marked:

    `backport-of: `
    -a, --all             output all common ancestors
    --octopus             find ancestors for a single n-way merge
    --independent         list revs not reachable from others
    --is-ancestor         is the first one ancestor of the other?
    --fork-point          find where <commit> forked from reflog of <ref>


but could not find `` in `master`.
##[error]Process completed with exit code 1.
