rust
2019-11-22T00:10:44.1792166Z agent.jobstatus=Succeeded
2019-11-22T00:10:44.1792219Z match res {
2019-11-22T00:10:44.1792219Z match res {
2019-11-22T00:10:44.1792301Z r? @oli-obk -- This is WIP, but I'd appreciate feedback. :)
2019-11-22T00:10:44.1792438Z 
2019-11-22T00:10:44.1796114Z disk usage:
2019-11-22T00:10:44.2281915Z Filesystem            Size  Used Avail Use% Mounted on
2019-11-22T00:10:44.2282059Z C:/Program Files/Git  256G  140G  116G  55% /
---
2019-11-22T00:10:55.3411300Z  30  480M   30  146M    0     0  22.1M      0  0:00:21  0:00:06  0:00:15 22.3M
2019-11-22T00:10:56.7898963Z  36  480M   36  173M    0     0  22.7M      0  0:00:21  0:00:07  0:00:14 23.3M
2019-11-22T00:10:56.8221928Z  38  480M   38  186M    0     0  20.6M      0  0:00:23  0:00:09  0:00:14 20.1M
2019-11-22T00:10:56.8249797Z  39  480M   39  187M    0     0  20.6M      0  0:00:23  0:00:09  0:00:14 20.0M
2019-11-22T00:10:56.8250879Z curl: (56) OpenSSL SSL_read: SSL_ERROR_SYSCALL, errno 10054
2019-11-22T00:10:56.8271581Z 
2019-11-22T00:10:56.8271869Z gzip: stdin: unexpected end of file
2019-11-22T00:10:56.8277445Z tar: Unexpected EOF in archive
2019-11-22T00:10:56.8277542Z tar: Unexpected EOF in archive
2019-11-22T00:10:56.8277621Z tar: Error is not recoverable: exiting now
2019-11-22T00:10:56.8334979Z 
2019-11-22T00:10:56.8412459Z ##[error]Bash exited with code '2'.
2019-11-22T00:10:56.8597332Z ##[section]Starting: Checkout
2019-11-22T00:10:56.8693690Z ==============================================================================
2019-11-22T00:10:56.8693798Z Task         : Get sources
2019-11-22T00:10:56.8693864Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
