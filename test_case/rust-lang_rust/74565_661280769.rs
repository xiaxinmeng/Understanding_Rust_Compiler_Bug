
2020-07-20T17:46:47.9653699Z + /scripts/validate-toolstate.sh
2020-07-20T17:46:47.9705032Z Cloning into 'rust-toolstate'...
2020-07-20T17:46:48.5729316Z <Nothing changed>
2020-07-20T17:46:48.7351930Z Traceback (most recent call last):
2020-07-20T17:46:48.7352697Z   File "../../src/tools/publish_toolstate.py", line 286, in <module>
2020-07-20T17:46:48.7353324Z     validate_maintainers(repo, github_token)
2020-07-20T17:46:48.7353970Z   File "../../src/tools/publish_toolstate.py", line 87, in validate_maintainers
2020-07-20T17:46:48.7355615Z     'Accept': 'application/vnd.github.hellcat-preview+json',
2020-07-20T17:46:48.7356332Z   File "/usr/lib/python3.6/urllib/request.py", line 223, in urlopen
2020-07-20T17:46:48.7356917Z     return opener.open(url, data, timeout)
2020-07-20T17:46:48.7357511Z   File "/usr/lib/python3.6/urllib/request.py", line 532, in open
2020-07-20T17:46:48.7358065Z     response = meth(req, response)
2020-07-20T17:46:48.7358662Z   File "/usr/lib/python3.6/urllib/request.py", line 642, in http_response
2020-07-20T17:46:48.7359637Z     'http', request, response, code, msg, hdrs)
2020-07-20T17:46:48.7360565Z   File "/usr/lib/python3.6/urllib/request.py", line 570, in error
2020-07-20T17:46:48.7361135Z     return self._call_chain(*args)
2020-07-20T17:46:48.7361767Z   File "/usr/lib/python3.6/urllib/request.py", line 504, in _call_chain
2020-07-20T17:46:48.7362328Z     result = func(*args)
2020-07-20T17:46:48.7362895Z   File "/usr/lib/python3.6/urllib/request.py", line 650, in http_error_default
2020-07-20T17:46:48.7363576Z     raise HTTPError(req.full_url, code, msg, hdrs, fp)
2020-07-20T17:46:48.7364203Z urllib.error.HTTPError: HTTP Error 401: Unauthorized
