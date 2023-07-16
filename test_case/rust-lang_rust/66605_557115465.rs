plain
2019-11-21T13:00:20.8576826Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-21T13:00:21.4980495Z ##[command]git config gc.auto 0
2019-11-21T13:00:21.4986718Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-21T13:00:21.4991279Z ##[command]git config --get-all http.proxy
2019-11-21T13:00:21.4994050Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66605/merge:refs/remotes/pull/66605/merge
---
2019-11-21T13:06:01.4636533Z Successfully built afea3e5a34f9
2019-11-21T13:06:01.5816052Z Successfully tagged rust-ci:latest
2019-11-21T13:06:01.6453091Z Built container sha256:afea3e5a34f918d45141a9a881f8cdd73c282c51ce7d95414d70e9e818651d19
2019-11-21T13:06:01.6468993Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/2781c8e6f0d7ff5702addf75da3252d9fe2c3b87f17f4cf2f6886d0417f523eab2b6e8b3a56c5580ab060276d097ffa9209fe0fe08f69277e52760e31cdece60
2019-11-21T13:07:04.0959445Z upload failed: - to s3://rust-lang-ci-sccache2/docker/2781c8e6f0d7ff5702addf75da3252d9fe2c3b87f17f4cf2f6886d0417f523eab2b6e8b3a56c5580ab060276d097ffa9209fe0fe08f69277e52760e31cdece60 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2019-11-21T13:07:05.1197349Z [CI_JOB_NAME=mingw-check]
2019-11-21T13:07:05.1230153Z == clock drift check ==
2019-11-21T13:07:05.1239639Z   local time: Thu Nov 21 13:07:05 UTC 2019
2019-11-21T13:07:05.4026416Z   network time: Thu, 21 Nov 2019 13:07:05 GMT
---
2019-11-21T13:12:31.9720499Z configure: rust.verify-llvm-ir  := True
2019-11-21T13:12:31.9720557Z configure: build.submodules     := False
2019-11-21T13:12:31.9720731Z configure: rust.dist-src        := False
2019-11-21T13:12:31.9720770Z configure: llvm.ccache          := sccache
2019-11-21T13:12:31.9721011Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-11-21T13:12:31.9721097Z configure: writing `config.toml` in current directory
2019-11-21T13:12:31.9721136Z configure: 
2019-11-21T13:12:31.9721370Z configure: run `python3 /checkout/x.py --help`
2019-11-21T13:12:31.9721410Z configure: 
---
2019-11-21T13:21:06.9148519Z Build completed successfully in 0:01:44
2019-11-21T13:21:06.9242313Z + /scripts/validate-toolstate.sh
2019-11-21T13:21:06.9275486Z Cloning into 'rust-toolstate'...
2019-11-21T13:21:07.6008403Z Traceback (most recent call last):
2019-11-21T13:21:07.6008521Z   File "../../src/tools/publish_toolstate.py", line 303, in <module>
2019-11-21T13:21:07.6008658Z     cur_datetime
2019-11-21T13:21:07.6008711Z   File "../../src/tools/publish_toolstate.py", line 178, in update_latest
2019-11-21T13:21:07.6008764Z     latest = json.load(f, object_pairs_hook=collections.OrderedDict)
2019-11-21T13:21:07.6008838Z   File "/usr/lib/python3.5/json/__init__.py", line 268, in load
2019-11-21T13:21:07.6008891Z     parse_constant=parse_constant, object_pairs_hook=object_pairs_hook, **kw)
2019-11-21T13:21:07.6008944Z   File "/usr/lib/python3.5/json/__init__.py", line 312, in loads
2019-11-21T13:21:07.6009010Z     s.__class__.__name__))
2019-11-21T13:21:07.6009798Z TypeError: the JSON object must be str, not 'bytes'
2019-11-21T13:21:07.6104774Z   local time: Thu Nov 21 13:21:07 UTC 2019
2019-11-21T13:21:07.8861508Z   network time: Thu, 21 Nov 2019 13:21:07 GMT
2019-11-21T13:21:07.8869120Z == end clock drift check ==
2019-11-21T13:21:09.5330129Z 
2019-11-21T13:21:09.5330129Z 
2019-11-21T13:21:09.5425446Z ##[error]Bash exited with code '1'.
2019-11-21T13:21:09.5454163Z ##[section]Starting: Checkout
2019-11-21T13:21:09.5455610Z ==============================================================================
2019-11-21T13:21:09.5455655Z Task         : Get sources
2019-11-21T13:21:09.5455693Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
