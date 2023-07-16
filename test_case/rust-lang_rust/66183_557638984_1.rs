rust
2019-11-22T18:21:19.6629715Z agent.jobstatus=Succeeded
2019-11-22T18:21:19.6629817Z enum E { pub U } // OK
2019-11-22T18:21:19.6629886Z trait Foo { pub fn bar(); } // OK
2019-11-22T18:21:19.6630003Z disk usage:
2019-11-22T18:21:19.7268712Z Filesystem            Size  Used Avail Use% Mounted on
2019-11-22T18:21:19.7274620Z C:/Program Files/Git  256G  141G  116G  55% /
2019-11-22T18:21:19.7275810Z D:                     14G  2.0G   13G  15% /d
---
2019-11-22T18:21:31.9379460Z  29  480M   29  139M    0     0  20.8M      0  0:00:22  0:00:06  0:00:16 21.7M
2019-11-22T18:21:33.2914590Z  34  480M   34  164M    0     0  21.3M      0  0:00:22  0:00:07  0:00:15 22.0M
2019-11-22T18:21:33.3637205Z  38  480M   38  186M    0     0  20.5M      0  0:00:23  0:00:09  0:00:14 20.3M
2019-11-22T18:21:33.3649227Z  39  480M   39  188M    0     0  20.6M      0  0:00:23  0:00:09  0:00:14 19.2M
2019-11-22T18:21:33.3649600Z curl: (56) OpenSSL SSL_read: SSL_ERROR_SYSCALL, errno 10054
2019-11-22T18:21:33.3670392Z 
2019-11-22T18:21:33.3679353Z gzip: stdin: unexpected end of file
2019-11-22T18:21:33.3679579Z tar: Unexpected EOF in archive
2019-11-22T18:21:33.3679662Z tar: Unexpected EOF in archive
2019-11-22T18:21:33.3679772Z tar: Error is not recoverable: exiting now
2019-11-22T18:21:33.3758739Z 
2019-11-22T18:21:33.3837962Z ##[error]Bash exited with code '2'.
2019-11-22T18:21:33.4040992Z ##[section]Starting: Checkout
2019-11-22T18:21:33.4144165Z ==============================================================================
2019-11-22T18:21:33.4144281Z Task         : Get sources
2019-11-22T18:21:33.4144370Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
