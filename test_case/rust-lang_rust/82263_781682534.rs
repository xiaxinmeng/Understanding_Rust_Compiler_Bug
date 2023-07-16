plain
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at 0148b971c Auto merge of #82263 - Dylan-DPC:rollup-cypm2uw, r=Dylan-DPC
##[group]Run src/ci/publish_toolstate.sh
src/ci/publish_toolstate.sh
env:
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  DEPLOY_BUCKET: rust-lang-ci2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---
  CACHE_DOMAIN: ci-caches.rust-lang.org
  TOOLSTATE_REPO_ACCESS_TOKEN: ***
##[endgroup]
Cloning into 'rust-toolstate'...
/home/runner/work/rust/rust/src/tools/publish_toolstate.py:121: DeprecationWarning: 'U' mode is deprecated
📣 Toolstate changed by rust-lang/rust#82263!
  with open(path, 'rU') as f:

Tested on commit rust-lang/rust@0148b971c921a0831fbf3357e5936eec724e3566.
Direct link to PR: <https://github.com/rust-lang/rust/pull/82263>

💔 miri on windows: test-fail → build-fail (cc @RalfJung @oli-obk @eddyb).
💔 miri on linux: test-fail → build-fail (cc @RalfJung @oli-obk @eddyb).
Traceback (most recent call last):
Traceback (most recent call last):
  File "/home/runner/work/rust/rust/src/tools/publish_toolstate.py", line 338, in <module>
    response = urllib2.urlopen(urllib2.Request(
  File "/usr/lib/python3.8/urllib/request.py", line 222, in urlopen
    return opener.open(url, data, timeout)
  File "/usr/lib/python3.8/urllib/request.py", line 522, in open
    req = meth(req)
  File "/usr/lib/python3.8/urllib/request.py", line 1281, in do_request_
    raise TypeError(msg)
TypeError: POST data should be bytes, an iterable of bytes, or a file object. It cannot be of type str.
##[error]Process completed with exit code 1.
