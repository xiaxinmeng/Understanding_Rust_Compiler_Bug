plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: x86_64-gnu-llvm-10
##[endgroup]
Error: Could not find a backport for `3b2b5b2914d40aa011d189bfe546084cdee53dbe`

All commits in `master` are required to have a corresponding upstream commit. It
looks like the commit `3b2b5b2914d40aa011d189bfe546084cdee53dbe` does not match any commits in `beta`. Was this
intended?

To override, add the text `backport-of: <SHA>` somewhere in the commit
message of `3b2b5b2914d40aa011d189bfe546084cdee53dbe`. <SHA> must be a commit in `master`.
cut: write error: Broken pipe
##[error]Process completed with exit code 1.
Post job cleanup.
