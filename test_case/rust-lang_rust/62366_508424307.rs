
info: removing rustup home
error: could not remove 'rustup_home' directory: '/usr/local/rustup'
info: caused by: Permission denied (os error 13)
Attempting to download https://.s3.amazonaws.com/docker/2f511ff28d2c3389996e6646724384337055dbe17514450f68a74b45672fa86ebd4e2373065cd78cea881bbf1f9ccb272982a872df86a3476451bf3eaec60ea7
Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/2f511ff28d2c3389996e6646724384337055dbe17514450f68a74b45672fa86ebd4e2373065cd78cea881bbf1f9ccb272982a872df86a3476451bf3eaec60ea7
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
Command failed. Attempt 2/5:
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
Command failed. Attempt 3/5:
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
Command failed. Attempt 4/5:
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
Command failed. Attempt 5/5:
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
The command has failed after 5 attempts.
open /tmp/rustci_docker_cache: no such file or directory
