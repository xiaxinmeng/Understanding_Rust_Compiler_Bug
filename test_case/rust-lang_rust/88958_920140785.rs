plain
Build completed successfully in 0:00:42
+ /scripts/validate-toolstate.sh
Cloning into 'rust-toolstate'...
<Nothing changed>
HTTPError: HTTP Error 403: Forbidden
b'{"message":"Must have admin access to view repository collaborators.","documentation_url":"https://docs.github.com/rest/reference/repos#list-repository-collaborators"}'
Traceback (most recent call last):
  File "../../src/tools/publish_toolstate.py", line 302, in <module>
    validate_maintainers(repo, github_token)
  File "../../src/tools/publish_toolstate.py", line 93, in validate_maintainers
    'Accept': 'application/vnd.github.hellcat-preview+json',
  File "/usr/lib/python3.6/urllib/request.py", line 223, in urlopen
    return opener.open(url, data, timeout)
  File "/usr/lib/python3.6/urllib/request.py", line 532, in open
    response = meth(req, response)
  File "/usr/lib/python3.6/urllib/request.py", line 642, in http_response
    'http', request, response, code, msg, hdrs)
  File "/usr/lib/python3.6/urllib/request.py", line 570, in error
    return self._call_chain(*args)
  File "/usr/lib/python3.6/urllib/request.py", line 504, in _call_chain
    result = func(*args)
  File "/usr/lib/python3.6/urllib/request.py", line 650, in http_error_default
    raise HTTPError(req.full_url, code, msg, hdrs, fp)
urllib.error.HTTPError: HTTP Error 403: Forbidden
