plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: mingw-check
##[endgroup]
Error: Could not find a backport for `56397e90e178fdc98b48031ceb79e90c61a48e3a`

All commits in `master` are required to have a corresponding upstream commit. It
looks like the commit `56397e90e178fdc98b48031ceb79e90c61a48e3a` does not match any commits in `beta`. Was this
intended?

To override, add the text `backport-of: <SHA>` somewhere in the commit
message of `56397e90e178fdc98b48031ceb79e90c61a48e3a`. <SHA> must be a commit in `master`.
##[error]Process completed with exit code 1.
