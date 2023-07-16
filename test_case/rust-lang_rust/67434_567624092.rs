plain
2019-12-19T19:13:57.7401411Z Build completed successfully in 0:01:26
2019-12-19T19:13:57.7411915Z + /scripts/validate-toolstate.sh
2019-12-19T19:13:57.7465446Z Cloning into 'rust-toolstate'...
2019-12-19T19:13:58.3513779Z Traceback (most recent call last):
2019-12-19T19:13:58.3513921Z   File "../../src/tools/publish_toolstate.py", line 303, in <module>
2019-12-19T19:13:58.3514023Z     cur_datetime
2019-12-19T19:13:58.3514099Z   File "../../src/tools/publish_toolstate.py", line 182, in update_latest
2019-12-19T19:13:58.3514689Z     for os in ['windows', 'linux']
2019-12-19T19:13:58.3514769Z   File "../../src/tools/publish_toolstate.py", line 182, in <dictcomp>
2019-12-19T19:13:58.3514960Z     for os in ['windows', 'linux']
2019-12-19T19:13:58.3515040Z   File "../../src/tools/publish_toolstate.py", line 111, in read_current_status
2019-12-19T19:13:58.3515246Z     (commit, status) = line.split('\t', 1)
2019-12-19T19:13:58.3515321Z ValueError: need more than 1 value to unpack
2019-12-19T19:13:58.3558351Z   local time: Thu Dec 19 19:13:58 UTC 2019
2019-12-19T19:13:58.4968348Z   network time: Thu, 19 Dec 2019 19:13:58 GMT
2019-12-19T19:13:58.4968772Z == end clock drift check ==
2019-12-19T19:14:00.2394074Z 
2019-12-19T19:14:00.2394074Z 
2019-12-19T19:14:00.2475488Z ##[error]Bash exited with code '1'.
2019-12-19T19:14:00.2513425Z ##[section]Starting: Checkout
2019-12-19T19:14:00.2515018Z ==============================================================================
2019-12-19T19:14:00.2515110Z Task         : Get sources
2019-12-19T19:14:00.2515355Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
