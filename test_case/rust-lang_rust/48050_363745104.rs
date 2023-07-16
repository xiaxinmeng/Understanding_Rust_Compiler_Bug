
$ curl -O https://static.rust-lang.org/dist/2018-01-04/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100  178M  100  178M    0     0  11.2M      0  0:00:15  0:00:15 --:--:-- 11.8M

$ curl -O https://static.rust-lang.org/dist/2018-01-04/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz.sha256
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   110  100   110    0     0    312      0 --:--:-- --:--:-- --:--:--   311

$ openssl sha256 < rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz
9a34b23a82d7f3c91637e10ceefb424539dcfa327c2dcd292ff10c047b1fdc7e

$ cat rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz.sha256 
9a34b23a82d7f3c91637e10ceefb424539dcfa327c2dcd292ff10c047b1fdc7e  rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz
