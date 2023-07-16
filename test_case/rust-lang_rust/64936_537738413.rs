plain
2019-10-03T00:41:17.1874828Z Downloaded containers:\n
2019-10-03T00:41:17.1876648Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/dist-x86_64-linux/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-10-03T00:41:17.3635157Z Sending build context to Docker daemon  526.3kB
2019-10-03T00:41:17.3635328Z 
2019-10-03T00:41:17.3900464Z Error response from daemon: Dockerfile parse error line 106: unknown instruction: --SET
2019-10-03T00:41:18.5313220Z Sending build context to Docker daemon  526.3kB
2019-10-03T00:41:18.5313451Z 
2019-10-03T00:41:18.5313451Z 
2019-10-03T00:41:18.5775556Z Error response from daemon: Dockerfile parse error line 106: unknown instruction: --SET
2019-10-03T00:41:20.8365835Z Sending build context to Docker daemon  526.3kB
2019-10-03T00:41:20.8366520Z 
2019-10-03T00:41:20.8366520Z 
2019-10-03T00:41:20.8622878Z Error response from daemon: Dockerfile parse error line 106: unknown instruction: --SET
2019-10-03T00:41:23.9510902Z Sending build context to Docker daemon  526.3kB
2019-10-03T00:41:23.9511377Z 
2019-10-03T00:41:23.9511377Z 
2019-10-03T00:41:23.9733345Z Error response from daemon: Dockerfile parse error line 106: unknown instruction: --SET
2019-10-03T00:41:28.1137423Z Sending build context to Docker daemon  526.3kB
2019-10-03T00:41:28.1138405Z 
2019-10-03T00:41:28.1138405Z 
2019-10-03T00:41:28.1445055Z Error response from daemon: Dockerfile parse error line 106: unknown instruction: --SET
2019-10-03T00:41:28.1457148Z The command has failed after 5 attempts.
2019-10-03T00:41:28.1612143Z ##[error]Bash exited with code '1'.
2019-10-03T00:41:28.1647724Z ##[section]Starting: Upload CPU usage statistics
2019-10-03T00:41:28.1651455Z ==============================================================================
2019-10-03T00:41:28.1651607Z Task         : Bash
2019-10-03T00:41:28.1651684Z Description  : Run a Bash script on macOS, Linux, or Windows
