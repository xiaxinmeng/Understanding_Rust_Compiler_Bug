plain
2019-11-25T12:52:29.8663326Z 
2019-11-25T12:52:29.8664364Z 
2019-11-25T12:52:29.8668828Z 
2019-11-25T12:52:29.8674851Z 
2019-11-25T12:52:29.8677003Z (e.g. `T` in `impl MyTrait<T>`). Note that normalization is the only way
2019-11-25T12:52:29.8686258Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-25T12:52:29.8686371Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-25T12:52:29.8686469Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-11-25T12:52:29.8686541Z AGENT_ID=520
---
2019-11-25T12:52:29.8701259Z MAVEN_OPTS=-Xms256m
2019-11-25T12:52:29.8701364Z MSDEPLOY_HTTP_USER_AGENT=VSTS_d439fc94-e01f-4249-b63e-d8392bc0247c_build_10_0
2019-11-25T12:52:29.8701457Z MSMPI_BIN=C:\Program Files\Microsoft MPI\Bin\
2019-11-25T12:52:29.8701544Z MSYSTEM=MINGW64
2019-11-25T12:52:29.8701627Z MyTrait<T>`) with inference variables. We want the opaque type in the
2019-11-25T12:52:29.8701794Z NPM_CONFIG_PREFIX=C:\npm\prefix
2019-11-25T12:52:29.8701881Z NUMBER_OF_PROCESSORS=2
2019-11-25T12:52:29.8701983Z Note that all of this refers to the opaque type (ty::Opaque) and its
2019-11-25T12:52:29.8702062Z OS=Windows_NT
---
2019-11-25T12:52:29.8716554Z function signature - users have no way of getting inference variables
2019-11-25T12:52:29.8716684Z function signature to be eligible to be a defining use of that opaque
2019-11-25T12:52:29.8716780Z inference variables.
2019-11-25T12:52:29.8716872Z into a function signature.
2019-11-25T12:52:29.8716968Z replaced with the corresponding generic parameter in the identity substs
2019-11-25T12:52:29.8717092Z resolved, we will instantiate the generic parameters into fresh
2019-11-25T12:52:29.8717214Z resolving a projection to an opaque type (e.g. `Self::MyType` when
2019-11-25T12:52:29.8717322Z signature after normalization. Any inference variables in the substs are
2019-11-25T12:52:29.8717428Z subst - *not* to the underlying type.
2019-11-25T12:52:29.8717526Z substs now appears to refer to some specific type, rather than a generic
2019-11-25T12:52:29.8717649Z that we can end up with inference variables in opaque substs in a
2019-11-25T12:52:29.8717754Z to replace the explicit generic parameters (e.g. `T` in `impl
2019-11-25T12:52:29.8717878Z type - adding inference variables prevents this, since the opaque type
2019-11-25T12:52:29.8717982Z type.
2019-11-25T12:52:29.8718153Z we have `type MyType = impl SomeTrait`). When the projection is
2019-11-25T12:52:29.8718343Z disk usage:
2019-11-25T12:52:29.9752397Z Filesystem            Size  Used Avail Use% Mounted on
2019-11-25T12:52:29.9755428Z C:/Program Files/Git  256G  140G  116G  55% /
2019-11-25T12:52:29.9756717Z D:                     14G  2.0G   13G  15% /d
---
2019-11-25T12:53:05.1506131Z  84  480M   84  408M    0     0  14.0M      0  0:00:34  0:00:28  0:00:06 22.9M
2019-11-25T12:53:06.6224617Z  90  480M   90  433M    0     0  14.4M      0  0:00:33  0:00:29  0:00:04 22.9M
2019-11-25T12:53:06.7385064Z  95  480M   95  456M    0     0  14.5M      0  0:00:33  0:00:31  0:00:02 23.0M
2019-11-25T12:53:06.7396976Z  95  480M   95  460M    0     0  14.5M      0  0:00:32  0:00:31  0:00:01 22.6M
2019-11-25T12:53:06.7397329Z curl: (56) OpenSSL SSL_read: SSL_ERROR_SYSCALL, errno 10054
2019-11-25T12:53:06.7418300Z 
2019-11-25T12:53:06.7418673Z gzip: stdin: unexpected end of file
2019-11-25T12:53:06.7431177Z tar: Unexpected EOF in archive
2019-11-25T12:53:06.7433506Z tar: Unexpected EOF in archive
2019-11-25T12:53:06.7433845Z tar: Error is not recoverable: exiting now
2019-11-25T12:53:06.7485083Z 
2019-11-25T12:53:06.7572428Z ##[error]Bash exited with code '2'.
2019-11-25T12:53:06.7779867Z ##[section]Starting: Checkout
2019-11-25T12:53:06.7879424Z ==============================================================================
2019-11-25T12:53:06.7879531Z Task         : Get sources
2019-11-25T12:53:06.7879627Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
