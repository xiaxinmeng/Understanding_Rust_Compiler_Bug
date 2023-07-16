plain
  CACHE_DOMAIN: ci-caches.rust-lang.org
  TOOLSTATE_REPO_ACCESS_TOKEN: ***
##[endgroup]
Cloning into 'rust-toolstate'...
/home/runner/work/rust/rust/src/tools/publish_toolstate.py:121: DeprecationWarning: 'U' mode is deprecated
📣 Toolstate changed by rust-lang/rust#82253!

Tested on commit rust-lang/rust@a31c16212d70fcae3ad9d073b00d883951e573ee.
Direct link to PR: <https://github.com/rust-lang/rust/pull/82253>

🎉 miri on windows: test-fail → test-pass (cc @eddyb @RalfJung @oli-obk).
🎉 miri on linux: test-fail → test-pass (cc @eddyb @RalfJung @oli-obk).

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
