plain
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 12  361M   12 44.4M    0     0  63.9M      0  0:00:05 --:--:--  0:00:05 63.8M
 34  361M   34  124M    0     0  73.5M      0  0:00:04  0:00:01  0:00:03 73.5M
 35  361M   35  130M    0     0  73.7M      0  0:00:04  0:00:01  0:00:03 73.6M
curl: (56) OpenSSL SSL_read: SSL_ERROR_SYSCALL, errno 104
** Resuming transfer from byte position 136346506
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed


  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 35  231M   35 82.9M    0     0  90.4M      0  0:00:02 --:--:--  0:00:02 90.3M
 41  231M   41 95.2M    0     0  90.0M      0  0:00:02  0:00:01  0:00:01 90.0M
curl: (56) OpenSSL SSL_read: SSL_ERROR_SYSCALL, errno 104
** Resuming transfer from byte position 236272540
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

---
Caused by:
  failed to open: /checkout/Cargo.lock

Caused by:
  Read-only file system (os error 30)
Build completed unsuccessfully in 0:00:36
