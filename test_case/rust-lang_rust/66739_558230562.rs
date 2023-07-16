plain
2019-11-25T16:20:24.4880850Z + /scripts/validate-toolstate.sh
2019-11-25T16:20:24.4929025Z Cloning into 'rust-toolstate'...
2019-11-25T16:20:25.1810062Z <Nothing changed>
2019-11-25T16:20:28.2377616Z Traceback (most recent call last):
2019-11-25T16:20:28.2377938Z   File "../../src/tools/publish_toolstate.py", line 267, in <module>
2019-11-25T16:20:28.2378253Z     validate_maintainers(repo, github_token)
2019-11-25T16:20:28.2378370Z   File "../../src/tools/publish_toolstate.py", line 75, in validate_maintainers
2019-11-25T16:20:28.2379138Z     assignable.extend(user['login'] for user in json.load(response))
2019-11-25T16:20:28.2379259Z   File "/usr/lib/python2.7/json/__init__.py", line 287, in load
2019-11-25T16:20:28.2379375Z     return loads(fp.read(),
2019-11-25T16:20:28.2379476Z   File "/usr/lib/python2.7/socket.py", line 355, in read
2019-11-25T16:20:28.2379554Z     data = self._sock.recv(rbufsize)
2019-11-25T16:20:28.2379653Z   File "/usr/lib/python2.7/httplib.py", line 622, in read
2019-11-25T16:20:28.2379747Z     s = self.fp.read(amt)
2019-11-25T16:20:28.2379823Z   File "/usr/lib/python2.7/socket.py", line 384, in read
2019-11-25T16:20:28.2380125Z     data = self._sock.recv(left)
2019-11-25T16:20:28.2380218Z   File "/usr/lib/python2.7/ssl.py", line 756, in recv
2019-11-25T16:20:28.2380314Z     return self.read(buflen)
2019-11-25T16:20:28.2380390Z   File "/usr/lib/python2.7/ssl.py", line 643, in read
2019-11-25T16:20:28.2380483Z     v = self._sslobj.read(len)
2019-11-25T16:20:28.2380599Z ssl.SSLError: [SSL: DECRYPTION_FAILED_OR_BAD_RECORD_MAC] decryption failed or bad record mac (_ssl.c:1758)
2019-11-25T16:20:28.2430280Z   local time: Mon Nov 25 16:20:28 UTC 2019
2019-11-25T16:20:28.5135812Z   network time: Mon, 25 Nov 2019 16:20:28 GMT
2019-11-25T16:20:28.5137163Z == end clock drift check ==
2019-11-25T16:20:30.6456890Z 
2019-11-25T16:20:30.6456890Z 
2019-11-25T16:20:30.6513574Z ##[error]Bash exited with code '1'.
2019-11-25T16:20:30.6543369Z ##[section]Starting: Checkout
2019-11-25T16:20:30.6545653Z ==============================================================================
2019-11-25T16:20:30.6545737Z Task         : Get sources
2019-11-25T16:20:30.6545835Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
