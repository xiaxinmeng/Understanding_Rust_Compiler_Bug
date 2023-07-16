plain
2019-09-18T16:33:02.3158260Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/aa213ad5-69d3-43d5-8f8e-5f82dc8d87de.sh
2019-09-18T16:33:02.3229971Z Cloning into 'rust-toolstate'...
2019-09-18T16:33:03.1059997Z <Nothing changed>
2019-09-18T16:33:03.3141365Z Traceback (most recent call last):
2019-09-18T16:33:03.3142319Z   File "/home/vsts/work/1/s/src/tools/publish_toolstate.py", line 267, in <module>
2019-09-18T16:33:03.3142435Z     validate_maintainers(repo, github_token)
2019-09-18T16:33:03.3142896Z   File "/home/vsts/work/1/s/src/tools/publish_toolstate.py", line 73, in validate_maintainers
2019-09-18T16:33:03.3144083Z     'Accept': 'application/vnd.github.hellcat-preview+json',
2019-09-18T16:33:03.3144194Z   File "/usr/lib/python2.7/urllib2.py", line 154, in urlopen
2019-09-18T16:33:03.3233991Z     return opener.open(url, data, timeout)
2019-09-18T16:33:03.3234470Z   File "/usr/lib/python2.7/urllib2.py", line 435, in open
2019-09-18T16:33:03.3234600Z     response = meth(req, response)
2019-09-18T16:33:03.3234780Z   File "/usr/lib/python2.7/urllib2.py", line 548, in http_response
2019-09-18T16:33:03.3235490Z     'http', request, response, code, msg, hdrs)
2019-09-18T16:33:03.3235616Z   File "/usr/lib/python2.7/urllib2.py", line 473, in error
2019-09-18T16:33:03.3235750Z     return self._call_chain(*args)
2019-09-18T16:33:03.3235835Z   File "/usr/lib/python2.7/urllib2.py", line 407, in _call_chain
2019-09-18T16:33:03.3237885Z     result = func(*args)
2019-09-18T16:33:03.3238204Z   File "/usr/lib/python2.7/urllib2.py", line 556, in http_error_default
2019-09-18T16:33:03.3238853Z     raise HTTPError(req.get_full_url(), code, msg, hdrs, fp)
2019-09-18T16:33:03.3241458Z urllib2.HTTPError: HTTP Error 401: Unauthorized
2019-09-18T16:33:03.3432349Z ##[error]Bash exited with code '1'.
2019-09-18T16:33:03.3478811Z ##[section]Starting: Upload CPU usage statistics
2019-09-18T16:33:03.3481676Z ==============================================================================
2019-09-18T16:33:03.3481759Z Task         : Bash
2019-09-18T16:33:03.3481874Z Description  : Run a Bash script on macOS, Linux, or Windows
