plain
2019-07-21T15:54:13.1757580Z To reinstall 5.2.4, run `brew reinstall xz`
2019-07-21T15:54:14.4762690Z ==> Installing dependencies for swig@3: pcre
2019-07-21T15:54:14.4763530Z ==> Installing swig@3 dependency: pcre
2019-07-21T15:54:15.0583420Z ==> Downloading https://homebrew.bintray.com/bottles/pcre-8.43.high_sierra.bottle.tar.gz
2019-07-21T15:54:15.0585240Z ==> Downloading from https://akamai.bintray.com/03/0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861?__gda__=exp=1563725174~hmac=046b7c943d2ae2cc6c93c77f83de8f98c36e71567f4e38e062a225b6e9d0ebc3&response-content-disposition=attachment%3Bfilename%3D%22pcre-8.43.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1_CuGKy0V-BBz6Gtw1VcSQlrA_LWpPcchRpBPVKVHRpzzAfPbrt-PuTdkNYP_Q_ouSa8D3pjKVQAgAU4iAXn2ArGPeugRCJmvK3ZSjv_05esM8Loma5hUb7hCxRjxhQvRFmDVIRt4lUFA&response-X-Checksum-Sha1=c67d4b99bb245f0ea56b34118dd6325b06a7250c&response-X-Checksum-Sha2=0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861
2019-07-21T15:54:17.6005110Z 🍺  /usr/local/Cellar/pcre/8.43: 204 files, 5.5MB
2019-07-21T15:54:17.6005640Z ==> Installing swig@3
2019-07-21T15:54:18.0420190Z ==> Downloading https://homebrew.bintray.com/bottles/swig@3-3.0.12.high_sierra.bottle.tar.gz
2019-07-21T15:54:18.0420190Z ==> Downloading https://homebrew.bintray.com/bottles/swig@3-3.0.12.high_sierra.bottle.tar.gz
2019-07-21T15:54:18.0421950Z ==> Downloading from https://akamai.bintray.com/73/730bd728981cc1534664ef35d08d0b285e79756c286913d868af6afa43f60f4d?__gda__=exp=1563725177~hmac=0fe5253fdef032a507631789d972dc50baeefa2c741be3c96c4d329c6eaf5735&response-content-disposition=attachment%3Bfilename%3D%22swig%403-3.0.12.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1_6npofDjJ2nrMf8FTh4dj6xhKQQtog5Po_ZUPLVvi2PmQUZNYsKUwcPdLq7vWnuNSob4VGRXgrKhRiXaZ6q9XDH9pl6xEUQyhWLnEiqfa7B5mISNR6TVBc6IDs5WxKyxTHB-ZbXVsGqg&response-X-Checksum-Sha1=4dc415ab888a7792f289543bafff9d4ec27cebb3&response-X-Checksum-Sha2=730bd728981cc1534664ef35d08d0b285e79756c286913d868af6afa43f60f4d
2019-07-21T15:54:20.9233160Z ==> Caveats
2019-07-21T15:54:20.9234470Z swig@3 is keg-only, which means it was not symlinked into /usr/local,
2019-07-21T15:54:20.9234700Z because this is an alternate version of another formula.
2019-07-21T15:54:20.9234770Z 
---
2019-07-21T17:24:34.7588640Z -rw-r--r--   1 vsts  staff    3121569 Jul 21 17:15 rustfmt-nightly-x86_64-apple-darwin.tar.gz
2019-07-21T17:24:34.7589400Z -rw-r--r--   1 vsts  staff    2071456 Jul 21 17:15 rustfmt-nightly-x86_64-apple-darwin.tar.xz
2019-07-21T17:24:34.7590740Z Attempting with retry: aws s3 cp --no-progress --recursive --acl public-read ./build/dist s3://rust-lang-ci2/rustc-builds-alt/b6e0c7cc5fded2fdca5817cf32dc2e64309e082b
2019-07-21T17:24:35.6555730Z Traceback (most recent call last):
2019-07-21T17:24:35.6556040Z   File "/usr/local/bin/aws", line 19, in <module>
2019-07-21T17:24:35.6556670Z     import awscli.clidriver
2019-07-21T17:24:35.6557980Z   File "/usr/local/lib/python2.7/site-packages/awscli/clidriver.py", line 36, in <module>
2019-07-21T17:24:35.6561330Z     from awscli.help import ProviderHelpCommand
2019-07-21T17:24:35.6562710Z   File "/usr/local/lib/python2.7/site-packages/awscli/help.py", line 20, in <module>
2019-07-21T17:24:35.6566930Z     from docutils.core import publish_string
2019-07-21T17:24:35.6568300Z   File "/usr/local/lib/python2.7/site-packages/docutils/core.py", line 246
2019-07-21T17:24:35.6569270Z     print('\n::: Runtime settings:', file=self._stderr)
2019-07-21T17:24:35.6569590Z SyntaxError: invalid syntax
2019-07-21T17:24:36.7935060Z Command failed. Attempt 2/5:
2019-07-21T17:24:37.1090670Z Traceback (most recent call last):
2019-07-21T17:24:37.1090670Z Traceback (most recent call last):
2019-07-21T17:24:37.1091340Z   File "/usr/local/bin/aws", line 19, in <module>
2019-07-21T17:24:37.1091850Z     import awscli.clidriver
2019-07-21T17:24:37.1093310Z   File "/usr/local/lib/python2.7/site-packages/awscli/clidriver.py", line 36, in <module>
2019-07-21T17:24:37.1093750Z     from awscli.help import ProviderHelpCommand
2019-07-21T17:24:37.1094550Z   File "/usr/local/lib/python2.7/site-packages/awscli/help.py", line 20, in <module>
2019-07-21T17:24:37.1094810Z     from docutils.core import publish_string
2019-07-21T17:24:37.1095540Z   File "/usr/local/lib/python2.7/site-packages/docutils/core.py", line 246
2019-07-21T17:24:37.1096370Z     print('\n::: Runtime settings:', file=self._stderr)
2019-07-21T17:24:37.1096680Z SyntaxError: invalid syntax
2019-07-21T17:24:39.1259900Z Command failed. Attempt 3/5:
2019-07-21T17:24:39.4443700Z Traceback (most recent call last):
2019-07-21T17:24:39.4443700Z Traceback (most recent call last):
2019-07-21T17:24:39.4444520Z   File "/usr/local/bin/aws", line 19, in <module>
2019-07-21T17:24:39.4445280Z     import awscli.clidriver
2019-07-21T17:24:39.4446910Z   File "/usr/local/lib/python2.7/site-packages/awscli/clidriver.py", line 36, in <module>
2019-07-21T17:24:39.4447220Z     from awscli.help import ProviderHelpCommand
2019-07-21T17:24:39.4447960Z   File "/usr/local/lib/python2.7/site-packages/awscli/help.py", line 20, in <module>
2019-07-21T17:24:39.4448220Z     from docutils.core import publish_string
2019-07-21T17:24:39.4448900Z   File "/usr/local/lib/python2.7/site-packages/docutils/core.py", line 246
2019-07-21T17:24:39.4449660Z     print('\n::: Runtime settings:', file=self._stderr)
2019-07-21T17:24:39.4450000Z SyntaxError: invalid syntax
2019-07-21T17:24:42.5683050Z Command failed. Attempt 4/5:
2019-07-21T17:24:42.8802010Z Traceback (most recent call last):
2019-07-21T17:24:42.8802010Z Traceback (most recent call last):
2019-07-21T17:24:42.8803340Z   File "/usr/local/bin/aws", line 19, in <module>
2019-07-21T17:24:42.8803920Z     import awscli.clidriver
2019-07-21T17:24:42.8805450Z   File "/usr/local/lib/python2.7/site-packages/awscli/clidriver.py", line 36, in <module>
2019-07-21T17:24:42.8805840Z     from awscli.help import ProviderHelpCommand
2019-07-21T17:24:42.8806630Z   File "/usr/local/lib/python2.7/site-packages/awscli/help.py", line 20, in <module>
2019-07-21T17:24:42.8806880Z     from docutils.core import publish_string
2019-07-21T17:24:42.8807600Z   File "/usr/local/lib/python2.7/site-packages/docutils/core.py", line 246
2019-07-21T17:24:42.8808410Z     print('\n::: Runtime settings:', file=self._stderr)
2019-07-21T17:24:42.8808790Z SyntaxError: invalid syntax
2019-07-21T17:24:47.0051050Z Command failed. Attempt 5/5:
2019-07-21T17:24:47.3270640Z Traceback (most recent call last):
2019-07-21T17:24:47.3270640Z Traceback (most recent call last):
2019-07-21T17:24:47.3271050Z   File "/usr/local/bin/aws", line 19, in <module>
2019-07-21T17:24:47.3271910Z     import awscli.clidriver
2019-07-21T17:24:47.3273530Z   File "/usr/local/lib/python2.7/site-packages/awscli/clidriver.py", line 36, in <module>
2019-07-21T17:24:47.3273840Z     from awscli.help import ProviderHelpCommand
2019-07-21T17:24:47.3274900Z   File "/usr/local/lib/python2.7/site-packages/awscli/help.py", line 20, in <module>
2019-07-21T17:24:47.3275180Z     from docutils.core import publish_string
2019-07-21T17:24:47.3275900Z   File "/usr/local/lib/python2.7/site-packages/docutils/core.py", line 246
2019-07-21T17:24:47.3276670Z     print('\n::: Runtime settings:', file=self._stderr)
2019-07-21T17:24:47.3276970Z SyntaxError: invalid syntax
2019-07-21T17:24:47.3405110Z The command has failed after 5 attempts.
2019-07-21T17:24:47.3405110Z The command has failed after 5 attempts.
2019-07-21T17:24:47.3603750Z ##[error]Bash exited with code '1'.
2019-07-21T17:24:47.3642770Z ##[section]Starting: Upload CPU usage statistics
2019-07-21T17:24:47.3647810Z ==============================================================================
2019-07-21T17:24:47.3648040Z Task         : Bash
2019-07-21T17:24:47.3648210Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-21T17:24:47.5604300Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-21T17:24:47.5631030Z ========================== Starting Command Output ===========================
2019-07-21T17:24:47.5658770Z [command]/bin/bash --noprofile --norc /Users/vsts/agent/2.154.3/work/_temp/45d31b0f-a06c-4a0d-ad8d-d5a754497f49.sh
2019-07-21T17:24:47.8863760Z Traceback (most recent call last):
2019-07-21T17:24:47.8864970Z   File "/usr/local/bin/aws", line 19, in <module>
2019-07-21T17:24:47.8870970Z     import awscli.clidriver
2019-07-21T17:24:47.8873070Z   File "/usr/local/lib/python2.7/site-packages/awscli/clidriver.py", line 36, in <module>
2019-07-21T17:24:47.8873980Z     from awscli.help import ProviderHelpCommand
2019-07-21T17:24:47.8875550Z   File "/usr/local/lib/python2.7/site-packages/awscli/help.py", line 20, in <module>
2019-07-21T17:24:47.8875990Z     from docutils.core import publish_string
2019-07-21T17:24:47.8877530Z   File "/usr/local/lib/python2.7/site-packages/docutils/core.py", line 246
2019-07-21T17:24:47.8878310Z     print('\n::: Runtime settings:', file=self._stderr)
2019-07-21T17:24:47.8878640Z SyntaxError: invalid syntax
2019-07-21T17:24:47.8878640Z SyntaxError: invalid syntax
2019-07-21T17:24:47.9077230Z ##[error]Bash exited with code '1'.
2019-07-21T17:24:47.9124910Z ##[section]Starting: Checkout
2019-07-21T17:24:47.9127890Z ==============================================================================
2019-07-21T17:24:47.9128100Z Task         : Get sources
2019-07-21T17:24:47.9128280Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
