plain
  CACHE_DOMAIN: ci-caches.rust-lang.org
  TOOLSTATE_REPO_ACCESS_TOKEN: ***
##[endgroup]
Cloning into 'rust-toolstate'...
/home/runner/work/rust/rust/src/tools/publish_toolstate.py:121: DeprecationWarning: 'U' mode is deprecated
📣 Toolstate changed by rust-lang/rust#82150!
  with open(path, 'rU') as f:

Tested on commit rust-lang/rust@9d3deed8a2f7dca16869ccd03a8e328ef627e6eb.
Direct link to PR: <https://github.com/rust-lang/rust/pull/82150>

🎉 miri on windows: test-fail → test-pass (cc @eddyb @oli-obk @RalfJung).
🎉 miri on linux: test-fail → test-pass (cc @eddyb @oli-obk @RalfJung).
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
