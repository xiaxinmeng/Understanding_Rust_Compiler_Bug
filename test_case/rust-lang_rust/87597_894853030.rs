plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: mingw-check
##[endgroup]
Error: Could not find a backport for `fe96fc8907e82c8423352c8e2d1130d02549613a`

All commits in `HEAD` are required to have a corresponding upstream commit.
It looks like the commit `fe96fc8907e82c8423352c8e2d1130d02549613a` does not match any commits in `master`. Was
this intended?

To override, add the text `backport-of: <SHA>` somewhere in the commit
message of `fe96fc8907e82c8423352c8e2d1130d02549613a`. <SHA> must be a commit in `master`.
##[error]Process completed with exit code 1.
Post job cleanup.
