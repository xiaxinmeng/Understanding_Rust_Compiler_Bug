
(view as text)

Traceback (most recent call last):
  File "/usr/local/lib/python2.7/dist-packages/Twisted-14.0.0-py2.7-linux-x86_64.egg/twisted/internet/defer.py", line 382, in callback
    self._startRunCallbacks(result)
  File "/usr/local/lib/python2.7/dist-packages/Twisted-14.0.0-py2.7-linux-x86_64.egg/twisted/internet/defer.py", line 490, in _startRunCallbacks
    self._runCallbacks()
  File "/usr/local/lib/python2.7/dist-packages/Twisted-14.0.0-py2.7-linux-x86_64.egg/twisted/internet/defer.py", line 577, in _runCallbacks
    current.result = callback(current.result, *args, **kw)
  File "/usr/local/lib/python2.7/dist-packages/Twisted-14.0.0-py2.7-linux-x86_64.egg/twisted/internet/defer.py", line 1155, in gotResult
    _inlineCallbacks(r, g, deferred)
--- <exception caught here> ---
  File "/usr/local/lib/python2.7/dist-packages/Twisted-14.0.0-py2.7-linux-x86_64.egg/twisted/internet/defer.py", line 1097, in _inlineCallbacks
    result = result.throwExceptionIntoGenerator(g)
  File "/usr/local/lib/python2.7/dist-packages/Twisted-14.0.0-py2.7-linux-x86_64.egg/twisted/spread/pb.py", line 470, in throwExceptionIntoGenerator
    return g.throw(RemoteError(self.type, self.value, self.traceback))
  File "/usr/local/lib/python2.7/dist-packages/buildbot-0.8.10_pre_86_g94ddde8-py2.7.egg/buildbot/process/buildstep.py", line 552, in runCommand
    res = yield command.run(self, self.remote)
twisted.spread.pb.RemoteError: [Error 32] The process cannot access the file because it is being used by another process: 'c:\\users\\rustbu~1\\appdata\\local\\temp\\tmptmsble'
