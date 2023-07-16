
2020-07-20T21:36:43.3315998Z   File "../../src/tools/publish_toolstate.py", line 286, in <module>
2020-07-20T21:36:43.3316489Z     validate_maintainers(repo, github_token)
2020-07-20T21:36:43.3316982Z   File "../../src/tools/publish_toolstate.py", line 87, in validate_maintainers
2020-07-20T21:36:43.3318330Z     'Accept': 'application/vnd.github.hellcat-preview+json',
2020-07-20T21:36:43.3318845Z   File "/usr/lib/python3.6/urllib/request.py", line 223, in urlopen
2020-07-20T21:36:43.3319312Z     return opener.open(url, data, timeout)
2020-07-20T21:36:43.3319759Z   File "/usr/lib/python3.6/urllib/request.py", line 532, in open
2020-07-20T21:36:43.3320200Z     response = meth(req, response)
2020-07-20T21:36:43.3320894Z   File "/usr/lib/python3.6/urllib/request.py", line 642, in http_response
2020-07-20T21:36:43.3321668Z     'http', request, response, code, msg, hdrs)
2020-07-20T21:36:43.3322148Z   File "/usr/lib/python3.6/urllib/request.py", line 570, in error
2020-07-20T21:36:43.3322573Z     return self._call_chain(*args)
2020-07-20T21:36:43.3323031Z   File "/usr/lib/python3.6/urllib/request.py", line 504, in _call_chain
2020-07-20T21:36:43.3323468Z     result = func(*args)
2020-07-20T21:36:43.3323901Z   File "/usr/lib/python3.6/urllib/request.py", line 650, in http_error_default
2020-07-20T21:36:43.3324433Z     raise HTTPError(req.full_url, code, msg, hdrs, fp)
2020-07-20T21:36:43.3324907Z urllib.error.HTTPError: HTTP Error 403: Forbidden
