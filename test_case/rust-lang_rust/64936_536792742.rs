plain
2019-09-30T23:24:31.9452884Z Downloaded containers:\n
2019-09-30T23:24:31.9454528Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/dist-x86_64-linux/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-09-30T23:24:32.2212769Z Sending build context to Docker daemon  526.3kB
2019-09-30T23:24:32.2213788Z 
2019-09-30T23:24:32.2625330Z Error response from daemon: Dockerfile parse error line 106: unknown instruction: --SET
2019-09-30T23:24:33.3879864Z Sending build context to Docker daemon  526.3kB
2019-09-30T23:24:33.3879998Z 
2019-09-30T23:24:33.3879998Z 
2019-09-30T23:24:33.4202585Z Error response from daemon: Dockerfile parse error line 106: unknown instruction: --SET
2019-09-30T23:24:35.5523319Z Sending build context to Docker daemon  526.3kB
2019-09-30T23:24:35.5524019Z 
2019-09-30T23:24:35.5524019Z 
2019-09-30T23:24:35.5797314Z Error response from daemon: Dockerfile parse error line 106: unknown instruction: --SET
2019-09-30T23:24:38.9596165Z Sending build context to Docker daemon  526.3kB
2019-09-30T23:24:38.9597508Z 
2019-09-30T23:24:38.9597508Z 
2019-09-30T23:24:38.9873301Z Error response from daemon: Dockerfile parse error line 106: unknown instruction: --SET
2019-09-30T23:24:43.0918514Z Sending build context to Docker daemon  526.3kB
2019-09-30T23:24:43.0918657Z 
2019-09-30T23:24:43.0918657Z 
2019-09-30T23:24:43.1349300Z Error response from daemon: Dockerfile parse error line 106: unknown instruction: --SET
2019-09-30T23:24:43.1349750Z The command has failed after 5 attempts.
2019-09-30T23:24:43.1447906Z ##[error]Bash exited with code '1'.
2019-09-30T23:24:43.1531496Z ##[section]Starting: Upload CPU usage statistics
2019-09-30T23:24:43.1535443Z ==============================================================================
2019-09-30T23:24:43.1535541Z Task         : Bash
2019-09-30T23:24:43.1535663Z Description  : Run a Bash script on macOS, Linux, or Windows
