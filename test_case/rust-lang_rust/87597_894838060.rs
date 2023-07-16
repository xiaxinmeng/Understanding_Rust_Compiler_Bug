plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: mingw-check
##[endgroup]
Mukund 1
+ 56397e90e178fdc98b48031ceb79e90c61a48e3a
+ b212f7961c4679e1f2ce6a0de2a9aa5944d54d6d
- 9e299abf05070ccff80cab0f8c914f1c9166e921
+ 2b2bf6c88a0d9bf72534cc4ce1a22971af77b137
+ 8ebc97f7a3a34ae1b5b8a330b4fc3b5483239edd
- 87d2c5934d5d7c8331b3859d5ed9b5523b99bb0e
+ 29ed642e4832bf19c49ee75eb1ce52ee4ee68ab5
+ eb17931b42c78faa3a43c356dbb68120e1e97068
Mukund 2
Mukund 3
Error: Could not find a backport for `56397e90e178fdc98b48031ceb79e90c61a48e3a`

All commits in `beta` are required to have a corresponding upstream commit. It
looks like the commit `56397e90e178fdc98b48031ceb79e90c61a48e3a` does not match any commits in `master`. Was this
intended?

To override, add the text `backport-of: <SHA>` somewhere in the commit
message of `56397e90e178fdc98b48031ceb79e90c61a48e3a`. <SHA> must be a commit in `master`.
##[error]Process completed with exit code 1.
Post job cleanup.
