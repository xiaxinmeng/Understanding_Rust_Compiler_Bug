plain
2019-11-01T23:42:12.8661757Z  ---> 360c029e77d3
2019-11-01T23:42:12.8662756Z Step 4/18 : RUN bash /scripts/emscripten.sh
2019-11-01T23:42:13.0074841Z  ---> Running in 7a20c5d8dc91
2019-11-01T23:42:13.4764759Z + cd /
2019-11-01T23:42:13.4765272Z + curl -fL https://mozilla-games.s3.amazonaws.com/emscripten/releases/emsdk-portable.tar.gz
2019-11-01T23:42:13.4765499Z + tar -xz
2019-11-01T23:42:13.4799300Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-01T23:42:13.4799369Z 
2019-11-01T23:42:13.9464770Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-01T23:42:13.9465997Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-01T23:42:13.9465997Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-01T23:42:13.9537220Z curl: (22) The requested URL returned error: 403 Forbidden
2019-11-01T23:42:13.9555185Z 
2019-11-01T23:42:13.9555872Z gzip: stdin: unexpected end of file
2019-11-01T23:42:13.9556464Z tar: Child returned status 1
2019-11-01T23:42:13.9556794Z tar: Error is not recoverable: exiting now
2019-11-01T23:42:14.3250538Z The command '/bin/sh -c bash /scripts/emscripten.sh' returned a non-zero code: 2
2019-11-01T23:42:15.4348609Z Sending build context to Docker daemon  529.4kB
2019-11-01T23:42:15.4349329Z 
2019-11-01T23:42:15.4556795Z Step 1/18 : FROM ubuntu:16.04
2019-11-01T23:42:15.4563547Z  ---> 5f2bf26e3524
---
2019-11-01T23:42:15.4576481Z  ---> 360c029e77d3
2019-11-01T23:42:15.4576861Z Step 4/18 : RUN bash /scripts/emscripten.sh
2019-11-01T23:42:15.6147324Z  ---> Running in 573e356929ac
2019-11-01T23:42:16.0956883Z + cd /
2019-11-01T23:42:16.0973436Z + curl -fL https://mozilla-games.s3.amazonaws.com/emscripten/releases/emsdk-portable.tar.gz
2019-11-01T23:42:16.0974112Z + tar -xz
2019-11-01T23:42:16.0975331Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-01T23:42:16.0975487Z 
2019-11-01T23:42:16.5077996Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-01T23:42:16.5078726Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-01T23:42:16.5078726Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-01T23:42:16.5156592Z curl: (22) The requested URL returned error: 403 Forbidden
2019-11-01T23:42:16.5175011Z 
2019-11-01T23:42:16.5175496Z gzip: stdin: unexpected end of file
2019-11-01T23:42:16.5181913Z tar: Child returned status 1
2019-11-01T23:42:16.5182430Z tar: Error is not recoverable: exiting now
2019-11-01T23:42:16.8433726Z The command '/bin/sh -c bash /scripts/emscripten.sh' returned a non-zero code: 2
2019-11-01T23:42:18.9435770Z Sending build context to Docker daemon  529.4kB
2019-11-01T23:42:18.9436567Z 
2019-11-01T23:42:18.9720137Z Step 1/18 : FROM ubuntu:16.04
2019-11-01T23:42:18.9721733Z  ---> 5f2bf26e3524
---
2019-11-01T23:42:18.9733632Z  ---> 360c029e77d3
2019-11-01T23:42:18.9734010Z Step 4/18 : RUN bash /scripts/emscripten.sh
2019-11-01T23:42:19.1735501Z  ---> Running in 238013d16f8d
2019-11-01T23:42:19.6043880Z + cd /
2019-11-01T23:42:19.6061571Z + tar -xz
2019-11-01T23:42:19.6061963Z + curl -fL https://mozilla-games.s3.amazonaws.com/emscripten/releases/emsdk-portable.tar.gz
2019-11-01T23:42:19.6120945Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-01T23:42:19.6121284Z 
2019-11-01T23:42:20.0325373Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-01T23:42:20.0326559Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-01T23:42:20.0326559Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-01T23:42:20.0397078Z curl: (22) The requested URL returned error: 403 Forbidden
2019-11-01T23:42:20.0412009Z 
2019-11-01T23:42:20.0412291Z gzip: stdin: unexpected end of file
2019-11-01T23:42:20.0421106Z tar: Child returned status 1
2019-11-01T23:42:20.0421405Z tar: Error is not recoverable: exiting now
2019-11-01T23:42:20.4354114Z The command '/bin/sh -c bash /scripts/emscripten.sh' returned a non-zero code: 2
2019-11-01T23:42:23.5467078Z Sending build context to Docker daemon  529.4kB
2019-11-01T23:42:23.5467331Z 
2019-11-01T23:42:23.5678510Z Step 1/18 : FROM ubuntu:16.04
2019-11-01T23:42:23.5680501Z  ---> 5f2bf26e3524
---
2019-11-01T23:42:23.5692563Z  ---> 360c029e77d3
2019-11-01T23:42:23.5694558Z Step 4/18 : RUN bash /scripts/emscripten.sh
2019-11-01T23:42:23.7264079Z  ---> Running in 1386ba20e87b
2019-11-01T23:42:24.1576282Z + cd /
2019-11-01T23:42:24.1582281Z + curl -fL https://mozilla-games.s3.amazonaws.com/emscripten/releases/emsdk-portable.tar.gz
2019-11-01T23:42:24.1582523Z + tar -xz
2019-11-01T23:42:24.1642088Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-01T23:42:24.1642149Z 
2019-11-01T23:42:24.5702957Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-01T23:42:24.5703474Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-01T23:42:24.5703474Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-01T23:42:24.5776308Z curl: (22) The requested URL returned error: 403 Forbidden
2019-11-01T23:42:24.5792778Z 
2019-11-01T23:42:24.5792886Z gzip: stdin: unexpected end of file
2019-11-01T23:42:24.5793283Z tar: Child returned status 1
2019-11-01T23:42:24.5793368Z tar: Error is not recoverable: exiting now
2019-11-01T23:42:24.9364289Z The command '/bin/sh -c bash /scripts/emscripten.sh' returned a non-zero code: 2
2019-11-01T23:42:29.0626450Z Sending build context to Docker daemon  529.4kB
2019-11-01T23:42:29.0626809Z 
2019-11-01T23:42:29.0877770Z Step 1/18 : FROM ubuntu:16.04
2019-11-01T23:42:29.0887238Z  ---> 5f2bf26e3524
---
2019-11-01T23:42:29.0892551Z  ---> 360c029e77d3
2019-11-01T23:42:29.0892630Z Step 4/18 : RUN bash /scripts/emscripten.sh
2019-11-01T23:42:29.2781668Z  ---> Running in 01c937095830
2019-11-01T23:42:29.7640957Z + cd /
2019-11-01T23:42:29.7649361Z + curl -fL https://mozilla-games.s3.amazonaws.com/emscripten/releases/emsdk-portable.tar.gz
2019-11-01T23:42:29.7649689Z + tar -xz
2019-11-01T23:42:29.7675842Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-01T23:42:29.7675919Z 
2019-11-01T23:42:30.1250567Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-01T23:42:30.1996729Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-01T23:42:30.1996729Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-01T23:42:30.1997160Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-01T23:42:30.2079904Z curl: (22) The requested URL returned error: 403 Forbidden
2019-11-01T23:42:30.2095914Z 
2019-11-01T23:42:30.2096038Z gzip: stdin: unexpected end of file
2019-11-01T23:42:30.2096281Z tar: Child returned status 1
2019-11-01T23:42:30.2096368Z tar: Error is not recoverable: exiting now
2019-11-01T23:42:30.5841472Z The command '/bin/sh -c bash /scripts/emscripten.sh' returned a non-zero code: 2
2019-11-01T23:42:30.5889174Z 
2019-11-01T23:42:30.5889174Z 
2019-11-01T23:42:30.5994794Z ##[error]Bash exited with code '1'.
2019-11-01T23:42:30.6030786Z ##[section]Starting: Upload CPU usage statistics
2019-11-01T23:42:30.6033906Z ==============================================================================
2019-11-01T23:42:30.6033993Z Task         : Bash
2019-11-01T23:42:30.6034073Z Description  : Run a Bash script on macOS, Linux, or Windows
