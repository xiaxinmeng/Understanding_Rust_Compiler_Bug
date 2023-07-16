plain
2019-12-19T20:42:06.4201760Z Build completed successfully in 0:01:42
2019-12-19T20:42:06.4215378Z + /scripts/validate-toolstate.sh
2019-12-19T20:42:06.4267442Z Cloning into 'rust-toolstate'...
2019-12-19T20:42:06.9471734Z Traceback (most recent call last):
2019-12-19T20:42:06.9472639Z   File "../../src/tools/publish_toolstate.py", line 303, in <module>
2019-12-19T20:42:06.9473008Z     cur_datetime
2019-12-19T20:42:06.9473128Z   File "../../src/tools/publish_toolstate.py", line 182, in update_latest
2019-12-19T20:42:06.9474288Z     for os in ['windows', 'linux']
2019-12-19T20:42:06.9474714Z   File "../../src/tools/publish_toolstate.py", line 182, in <dictcomp>
2019-12-19T20:42:06.9475085Z     for os in ['windows', 'linux']
2019-12-19T20:42:06.9475197Z   File "../../src/tools/publish_toolstate.py", line 111, in read_current_status
2019-12-19T20:42:06.9475502Z     (commit, status) = line.split('\t', 1)
2019-12-19T20:42:06.9475605Z ValueError: need more than 1 value to unpack
2019-12-19T20:42:06.9532949Z   local time: Thu Dec 19 20:42:06 UTC 2019
2019-12-19T20:42:07.2165298Z   network time: Thu, 19 Dec 2019 20:42:07 GMT
2019-12-19T20:42:07.2166221Z == end clock drift check ==
2019-12-19T20:42:08.9732570Z 
2019-12-19T20:42:08.9732570Z 
2019-12-19T20:42:08.9838996Z ##[error]Bash exited with code '1'.
2019-12-19T20:42:08.9877369Z ##[section]Starting: Checkout
2019-12-19T20:42:08.9879159Z ==============================================================================
2019-12-19T20:42:08.9879247Z Task         : Get sources
2019-12-19T20:42:08.9879344Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
