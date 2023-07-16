plain
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at da5f7f109 Auto merge of #81427 - klensy:eat-digits, r=m-ou-se
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
📣 Toolstate changed by rust-lang/rust#81427!
  with open(path, 'rU') as f:

Tested on commit rust-lang/rust@da5f7f10936198bad22c370118bad1ac332d2f46.
Direct link to PR: <https://github.com/rust-lang/rust/pull/81427>

💔 miri on windows: test-fail → build-fail (cc @eddyb @RalfJung @oli-obk).
💔 miri on linux: test-fail → build-fail (cc @eddyb @RalfJung @oli-obk).
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
