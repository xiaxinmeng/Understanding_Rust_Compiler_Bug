plain
2019-08-05T04:35:24.6162990Z tidy check
2019-08-05T04:35:34.6073100Z * 578 error codes
2019-08-05T04:35:34.6073730Z * highest error code: E0733
2019-08-05T04:35:35.3997340Z * 262 features
2019-08-05T04:35:36.7158710Z Stray file with UI testing output: "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui/huge-array-simple.stderr"
2019-08-05T04:35:36.8061790Z some tidy checks failed
2019-08-05T04:35:36.8069050Z 
2019-08-05T04:35:36.8069050Z 
2019-08-05T04:35:36.8070910Z command did not execute successfully: "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/tidy" "/Users/vsts/agent/2.155.1/work/1/s/src" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage0/bin/cargo" "--no-vendor"
2019-08-05T04:35:36.8072460Z 
2019-08-05T04:35:36.8073110Z 
2019-08-05T04:35:36.8081660Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-08-05T04:35:36.8082420Z Build completed unsuccessfully in 0:01:45
2019-08-05T04:35:36.8082420Z Build completed unsuccessfully in 0:01:45
2019-08-05T04:35:36.8590920Z ##[error]Bash exited with code '1'.
2019-08-05T04:35:36.8634030Z ##[section]Starting: Upload CPU usage statistics
2019-08-05T04:35:36.8638370Z ==============================================================================
2019-08-05T04:35:36.8638480Z Task         : Bash
2019-08-05T04:35:36.8638550Z Description  : Run a Bash script on macOS, Linux, or Windows
