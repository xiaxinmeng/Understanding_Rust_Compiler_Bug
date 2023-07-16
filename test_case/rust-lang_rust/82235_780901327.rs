plain
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at 152f66092 Auto merge of #82235 - GuillaumeGomez:rollup-oflxc08, r=GuillaumeGomez
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
📣 Toolstate changed by rust-lang/rust#82235!

Tested on commit rust-lang/rust@152f6609246558be5e2582e67376194815e6ba0d.
Direct link to PR: <https://github.com/rust-lang/rust/pull/82235>

💔 miri on windows: test-fail → build-fail (cc @RalfJung @eddyb @oli-obk).
💔 miri on linux: test-fail → build-fail (cc @RalfJung @eddyb @oli-obk).

  with open(path, 'rU') as f:
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
